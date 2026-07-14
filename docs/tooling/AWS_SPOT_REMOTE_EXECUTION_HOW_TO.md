# AWS Spot Remote Execution HOW-TO

## Purpose

Use ADL's repo-native AWS Spot lane for build and validation work on an
ephemeral EC2 instance. The host runs a pinned ADL builder container and mounts
the retained EBS cache. The command verifies the Agent Logic account, source
commit, image digest, cache identity, SSH recovery, live logs, and cleanup.

The proven baseline is `m7a.2xlarge` in `us-west-2a` with the retained 500 GiB
cache. Another instance type is allowed only when it is available in the same
cache-volume availability zone and its architecture matches the selected image.
The current CI-and-coverage image, `adl-builder:v0.91.7-coverage-5243`, is amd64.

## Safety Rules

- Use AWS profile `agent-logic-admin` for operator runs.
- Run `preflight` before every paid run.
- Use a pushed branch, tag, or advertised remote ref. Do not use `HEAD`.
- SSH recovery is always enabled on port 22 with the configured passphraseless
  debug key and restricted to the operator's configured public `/32`; do not
  rely on the hosted runner as the only observation path.
- Keep the image pinned by digest. A tag is resolved once before launch.
- Never delete or recreate the retained EBS volume during routine cleanup.
- Do not launch two jobs against the retained volume concurrently.
- Treat a red wrapper summary as a failed run even if the remote command passed.

## Preflight

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh preflight \
  --profile agent-logic-admin \
  --git-ref <pushed-branch-or-tag> \
  --instance-type m7a.2xlarge \
  --cache-volume-size-gib 500 \
  --command 'bash adl/tools/run_pr_fast_test_lane.sh'
```

Preflight does not create EC2 resources. Confirm that it reports the business
account match, retained cache availability, immutable image, SSH recovery, and
the expected source commit. It also requires the machine-readable
`embedded_control_bundle_v1` and `spot_only_v1` binary capabilities before any
paid launch. Install the current owner binary, or select a reviewed issue-owned
binary with `ADL_AWS_REMOTE_VALIDATION_BIN`, when that guard fires.

## Run

```sh
RUN_ID="adl-spot-$(date -u +%Y%m%d%H%M%S)"

bash adl/tools/run_aws_spot_remote_validation_lane.sh run \
  --run \
  --run-id "$RUN_ID" \
  --profile agent-logic-admin \
  --git-ref <pushed-branch-or-tag> \
  --instance-type m7a.2xlarge \
  --cache-volume-size-gib 500 \
  --command 'bash adl/tools/run_pr_fast_test_lane.sh' \
  --out ".adl/local-artifacts/$RUN_ID/summary.json" \
  --artifact-dir ".adl/local-artifacts/$RUN_ID/artifacts" \
  --json
```

This synchronous path is the canonical operator command. It waits through
termination and temporary-resource cleanup before returning.

For automation or a control terminal with a bounded lifetime, replace `run`
with `launch`. The command performs the same fail-closed preflight, starts a
detached manager in a new process session, writes `manager.pid`, and returns.
The manager independently owns launch, live evidence, finalization, and
cleanup; use `status` and `logs` below until `wrapper-final-summary.json`
appears. Do not background the synchronous command with shell `&`.
Run IDs are single-use. A duplicate active, incomplete, or terminal run ID is
rejected; inspect or clean up the existing run and choose a new run ID.

## Observe And Recover

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh status --run-id "$RUN_ID" --out ".adl/local-artifacts/$RUN_ID/summary.json" --artifact-dir ".adl/local-artifacts/$RUN_ID/artifacts" --json
bash adl/tools/run_aws_spot_remote_validation_lane.sh logs --run-id "$RUN_ID" --out ".adl/local-artifacts/$RUN_ID/summary.json" --artifact-dir ".adl/local-artifacts/$RUN_ID/artifacts" --follow
bash adl/tools/run_aws_spot_remote_validation_lane.sh ssh --run-id "$RUN_ID" --out ".adl/local-artifacts/$RUN_ID/summary.json" --artifact-dir ".adl/local-artifacts/$RUN_ID/artifacts"
```

If the controlling terminal is interrupted, inspect status first. Then stop
and clean up by run ID:

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh stop --run-id "$RUN_ID" --json
bash adl/tools/run_aws_spot_remote_validation_lane.sh cleanup --run-id "$RUN_ID" --json
```

Cleanup must show the instance terminated and temporary IAM/security resources
removed. The retained cache volume remains available and billable. During the
run, use SSH from the configured operator `/32` allowlist or the SSM progress
log to observe every stage; the final summary is not the only activity record.

## Cache Operations

Routine cache checks are part of `preflight` and the builder proof:

- retained volume identity and availability zone
- mount source distinct from the root filesystem
- writable probe
- configured size and filesystem resize
- free-space threshold
- pre-existing target entries and bytes

Low space fails closed. Use only the wrapper's explicit bounded target-cleanup
recovery; do not manually delete the volume or broad cache roots. After an
interrupted run, verify the volume is `available` before launching again.

## Image Operations

The canonical definition is `adl/docker/adl-builder/Dockerfile`. It must include
Rust, `cargo-nextest`, `cargo-llvm-cov`, `llvm-tools-preview`, `sccache`, `lld`,
AWS CLI, Git, and the native build dependencies.

Use [ADL Builder Image](ADL_BUILDER_IMAGE.md) for publication. Publish a new
tag, verify it, then deliberately move the operational tag. Never build the
image inside a validation run. Keep the previous digest for immediate rollback:

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh preflight \
  --builder-image <ecr-uri@previous-sha256-digest> \
  --git-ref <pushed-branch-or-tag>
```

## Instance Selection

Pass another compatible type with `--instance-type`. Check these constraints:

- same availability zone as the retained EBS volume
- enough memory and disk throughput for Rust and coverage
- architecture matches the immutable image
- Spot capacity and price are acceptable
- SSH, SSM, ECR, and EBS permissions remain unchanged

An ARM/Graviton instance requires an arm64 or multi-architecture builder image.
Do not use the amd64 digest and weaken architecture verification.

## Proof Checklist

Retain the wrapper summary and confirm:

- `status: passed`
- every `self_verification` field is true and `failures` is empty
- source commit and immutable image are verified
- cache identity, mount, and writability are verified
- live SSH logs are verified
- build/test command passed
- final instance state is `terminated`
- estimated cost and all phase timings are present
