# AWS Spot Remote Validation Lane

`adl/tools/run_aws_spot_remote_validation_lane.sh` is the canonical operator
entry point for the existing Agent Logic Spot validation lane. It extends the
launch, warm-EBS, SSH/logging, builder-image, interruption, and teardown work
from `#4837`, `#4679`, `#4879`, and `#4955`; it does not create a
parallel lane.

The default contract is:

- AWS profile `agent-logic-admin`, checked against retained account proof
- Spot `m7a.2xlarge`
- the retained hot-cache volume and subnet identified by prior live proof
- `/mnt/adl-cache` with at least 10 GiB free and a writable filesystem
- `adl-builder:v0.91.7-fixed`, resolved once to an immutable ECR digest
- the dedicated `tools/aws_remote_validation` owner binary
- passphraseless SSH and live remote-tail logging
- explicit source-commit verification
- automatic compute termination while the retained EBS volume survives
- redacted retained evidence plus mode-600 private recovery state

## Quick Start

Push the commit that will run. The remote checkout cannot consume uncommitted
local changes.

No-cost preflight:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh preflight \
  --git-ref <pushed-branch-or-commit>
```

The preflight performs read-only checks only. It verifies the Agent Logic
account, immutable image digest, retained hot-cache identity and availability,
cache/subnet availability-zone match, EBS shape, current AL2023 AMI, SSH key,
source commit, and current Spot price. It creates no AWS resources.

Asynchronous live run:

```bash
RUN_ID="adl-spot-$(date -u +%Y%m%d%H%M%S)"

bash adl/tools/run_aws_spot_remote_validation_lane.sh launch \
  --run-id "$RUN_ID" \
  --git-ref <pushed-branch-or-commit> \
  --command 'cargo nextest run --manifest-path adl/Cargo.toml --workspace --locked' \
  --instance-type m7a.2xlarge \
  --json
```

`launch` is an explicit paid action. It returns the run id and background
manager PID immediately. The default artifact root is:

```text
.adl/tmp/aws-spot-remote-validation/<run-id>/
```

Use a synchronous run when the calling process should wait:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh run --run \
  --run-id "$RUN_ID" \
  --git-ref <pushed-branch-or-commit> \
  --command 'cargo nextest run --manifest-path adl/Cargo.toml --workspace --locked' \
  --instance-type m7a.2xlarge \
  --json
```

The repeated `run --run` spelling is intentional. The action chooses the
synchronous lifecycle, and `--run` is the paid-resource confirmation.

## Operate A Run

All lifecycle actions use the same run id:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh status --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh logs --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh logs --follow --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh ssh --run-id "$RUN_ID"
```

`logs` redacts account ids, ARNs, EC2/EBS/network resource ids, and IP
addresses while streaming. `ssh` resolves the private endpoint from local
control state, checks key mode, proves the key has no passphrase, and uses
`BatchMode=yes`.

Emergency stop:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh stop --run-id "$RUN_ID"
```

`stop` verifies the Agent Logic account and the instance's `adl:run_id` tag
before termination. It refuses a tag mismatch and never deletes the cache
volume.

Cleanup verification:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh cleanup --run-id "$RUN_ID"
```

`cleanup` stops an active run when necessary, then verifies that the retained
cache still exists in `available` or `in-use` state. The normal integrated
runner also deletes its temporary security group, role, and instance profile.

## Builder Image And Cache

The wrapper resolves the canonical ECR tag to a digest before launch. An
explicit override must use:

```text
<registry>/<repository>@sha256:<64-hex-digest>
```

Mutable tags are rejected for live execution. The instance pulls that digest
and verifies:

- `rustc`
- `cargo`
- `cargo-nextest`
- `sccache`
- `lld`
- AWS CLI
- image and runtime architecture

Rust validation tools are not installed on the ephemeral host. Docker and AWS
CLI are host transport dependencies and may be installed when the selected AMI
does not already provide them. The builder image itself is never rebuilt by a
validation run. The image includes `lld`, but the Spot lane leaves `RUSTFLAGS`
empty to preserve compatibility with the established warm-EBS Cargo target.
Changing compiler flags creates a different Cargo cache identity and requires
separate, explicit migration proof.

The retained cache is mounted at `/mnt/adl-cache`. Container-backed state is
under:

```text
/mnt/adl-cache/adl-aws-remote-validation/shared/target
/mnt/adl-cache/adl-aws-remote-validation/shared/sccache
/mnt/adl-cache/adl-aws-remote-validation/shared/cargo-home
/mnt/adl-cache/adl-aws-remote-validation/shared/tmp
/mnt/adl-cache/adl-aws-remote-validation/shared/source/agent-design-language
```

The container maps the retained `tmp` directory to `/tmp`, preventing large builds and
disk-sensitive tests from consuming the small ephemeral root filesystem.
These are the original warm-cache directories established by `#4837`; the
builder image must reuse them rather than create a parallel container-specific
target namespace. The runner also reuses the EBS-backed tracked checkout. It
fetches and checks out only when the requested commit changes, so same-commit
runs preserve source paths and mtimes while still verifying the exact commit
before execution.

The immutable builder image owns `rustc`, Cargo, nextest, sccache, and `lld`.
Do not set `RUSTUP_HOME` to the retained volume: doing so would make toolchain
provenance mutable and could invalidate Cargo fingerprints. After changing any
cache identity, treat the first successful run as migration evidence, not warm
proof. Warm proof requires a second same-commit run with no unexpected
compilation and materially reused target artifacts; nonzero preexisting bytes
alone are not sufficient.

Validation runs as the remote host's non-root UID/GID so permission-negative
tests remain meaningful. The container also sets
`AWS_EC2_METADATA_DISABLED=true`: AWS launch, EBS, SSM, ECR, logging, and
teardown stay on the host control path, while ordinary Rust tests cannot
silently discover the disposable instance role and turn unit tests into live
AWS calls.

Older experiments may leave `container-target`, `container-sccache`,
`container-cargo-home`, or `container-tmp` beside the canonical directories.
They are not consumed by this lane and are preserved until a separately
verified maintenance operation removes them. Do not delete retained EBS paths
as part of an ordinary validation run.

Historical AWS state contains two preserved volumes with the same Name tag in
different availability zones. Do not select or delete either by name. The
wrapper reads the prior hot-cache proof, pins its subnet and hashed volume
identity, verifies one matching volume in that availability zone, and fails
closed on ambiguity or shape drift.

## Evidence

Public evidence:

- `summary.json`: recursively redacted AWS runner summary
- `artifacts/wrapper-final-summary.json`: self-verifying lane result
- `artifacts/events.jsonl`: redacted lifecycle events
- `artifacts/remote-tail.log`: redacted live SSH tail
- `artifacts/resume-state.json`: interruption and retry history when present

Private emergency recovery state:

```text
artifacts/.private/control-summary.json
artifacts/.private/command-status.log
```

The private directory is mode 700 and its files are mode 600. It can contain
raw control identifiers needed to terminate a failed run. Do not copy or upload
it. The GitHub workflow explicitly excludes hidden files.

Automatic retries are bounded and restricted to classified capacity, transient
network, and SSM failures, plus provider-confirmed Spot interruptions. A test or
validation failure is terminal and is never relaunched as an infrastructure
retry. `stop` discovers the latest attempt control log and verifies the
instance's `adl:run_id` tag before termination.

The canonical wrapper is Spot-only and tries its ordered x86 builder pool in
the retained cache volume's availability zone. It never falls back to
on-demand implicitly. The lower-level owner binary retains an explicit
compatibility path for callers that intentionally allow on-demand fallback.

If target artifacts from an incompatible linker/toolchain generation exhaust
the retained volume, run the canonical lane with the exact maintenance command
`cargo clean --manifest-path adl/Cargo.toml`. That exact command alone may run
below the free-space threshold; it clears only the mounted Cargo target while
preserving the EBS volume, `sccache`, and Cargo-home caches. All other commands
still require 10 GiB of pre-run headroom.

A successful `wrapper-final-summary.json` proves:

- Spot purchase option
- immutable builder image and toolchain
- exact source commit
- retained cache identity, attachment, mount, writability, and free space
- passphraseless SSH recovery and live tail startup
- no host Rust validation-tool installation
- compute termination
- boot, launch, SSM, validation, teardown, and total timing
- estimated compute cost from pre-run Spot price and observed lifetime

For warm-cache proof, also inspect
`cache_target_preexisting_entries` and
`cache_target_preexisting_bytes`. A label saying "warm" is not proof.

## GitHub Actions

`.github/workflows/aws-spot-remote-validation.yaml` is manual
`workflow_dispatch` only and serializes runs around the retained cache.

Required repository secrets:

```text
AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN
AWS_SPOT_REMOTE_VALIDATION_SSH_PRIVATE_KEY_B64
```

The SSH secret is the base64 encoding of the retained passphraseless private
key. The workflow decodes it under `RUNNER_TEMP`, verifies it with
`ssh-keygen -P ''`, and never places it under the upload root.

Refresh the existing OIDC role:

```bash
bash adl/tools/setup_aws_spot_remote_validation_github_resources.sh \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2
```

The role may launch/terminate compute and manage ephemeral launch surfaces. It
does not receive `CreateVolume` or `DeleteVolume`; the retained cache must
already exist.

## Recovery

1. Run `status` and `logs --follow`.
2. Use `ssh` when application-level diagnosis is needed.
3. Use `stop` if the manager is stuck or a validation must be cancelled.
4. Run `cleanup` and require `retained_cache_preserved=true`.
5. Inspect `wrapper-final-summary.json` for the failure classification.
6. Keep `.private` local until compute termination is confirmed, then remove
   it according to local artifact-retention policy.

Never delete or recreate the retained cache as a recovery shortcut.
When the retained EBS volume is expanded, the remote runner grows its ext4
filesystem with `resize2fs` after mounting it. EBS size alone is not usable
capacity until that filesystem-growth step succeeds.

## Focused Contract Tests

```bash
bash adl/tools/test_run_aws_spot_remote_validation_lane.sh
bash adl/tools/test_run_aws_spot_builder_image_validation.sh
bash adl/tools/test_aws_spot_artifact_finalize.sh
bash adl/tools/test_aws_spot_lifecycle_controls.sh
cargo test --manifest-path tools/aws_remote_validation/Cargo.toml \
  --bin adl-aws-remote-validation
```

These tests cover immutable-image enforcement, missing tools, wrong source,
cache mount and capacity failures, architecture mismatch, validation failure,
interruption/resume, SSH recovery, redaction, tag-guarded stop, teardown
failure, and retained-cache cleanup.

## Prior Proof

The prior hot-cache authority remains:

```text
docs/milestones/v0.91.7/review/build_throughput/remote_validation_4603/live_run_summary_retry11_agentlogic_hotcache.json
```

The fixed builder-image and warm-EBS benchmark proof remains in:

```text
docs/milestones/v0.91.7/review/build_throughput/ADL_BUILDER_IMAGE_4879.md
```

Issue `#5191` adds new same-commit repeated live proof; do not claim those
runs until their retained evidence exists.
