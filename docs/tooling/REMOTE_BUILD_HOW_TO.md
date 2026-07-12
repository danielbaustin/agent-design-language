# Remote Build How-To

Use this playbook to run ADL validation or repeated builds on Nessus, AWS Spot,
or AWS CodeBuild without rediscovering the setup. Keep runs issue-bound,
explicit, and proof-producing.

## 1. Standard Setup

Run from the issue worktree.

```sh
export ADL_WORKTREE=$(git rev-parse --show-toplevel)
export ADL_BIN=${ADL_BIN:-/Users/daniel/git/agent-design-language/adl/target/debug/adl}
export ADL_AWS_PROFILE=agent-logic-admin
export ADL_ARTIFACT_DIR=.adl/local-artifacts/remote-build
mkdir -p "$ADL_ARTIFACT_DIR"
```

Rules:

- Do not do tracked issue work on root `main`.
- Use `agent-logic-admin` for ADL AWS work.
- Keep `ADL_WORKTREE` pointed at the bound issue worktree. `ADL_BIN` may point
  at the approved repo-owned binary; do not rebuild it unless the issue requires
  a new binary.
- Keep AWS Spot and CodeBuild live runs explicit; both may cost money.
- Use the pre-published ADL builder image. Do not rebuild the image inside each
  build job.
- The ADL builder image must include `cargo-nextest`; PR-fast Rust lanes depend
  on it.
- Use the canonical repaired tag, `adl-builder:v0.91.7-fixed`, or an ECR image
  URI that points at that same image build. Do not route nextest-backed remote
  validation through the stale `v0.91.7` builder tag.
- Put scratch outputs under ignored `.adl/local-artifacts/`.
- Record the platform, cache posture, benchmark line, and proof artifact path.

## 2. Choose A Lane

| Lane | Use when | Cache posture to prove |
| --- | --- | --- |
| Nessus | Fast no-cloud-cost remote validation on operator hardware | persistent remote target/cache; state cold or warm |
| AWS Spot | Fast AWS/EC2 validation | fixed builder image plus retained warm EBS cache |
| CodeBuild XLARGE | Scalable isolated CodeFriend-style repeated builds | immutable ECR digest, S3 `sccache`, compatibility-keyed S3 target archive |
| Wuji/local | Local ARM work | no image-backed parity until ARM64 or multi-arch image exists |

For scheduler routing without launching paid work:

```sh
bash adl/tools/validation_manager.sh --platform-routing
```

## 3. Nessus

Remote host: `daniel@nessus.local`.

Select the lane first. Use committed refs for remote proof; the remote host
checks out the advertised git ref and cannot see local uncommitted worktree
changes.

```sh
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --base origin/main \
  --head HEAD \
  --remote-git-ref <branch-or-commit> \
  --remote-artifact-dir "$ADL_ARTIFACT_DIR/nessus"
```

Run it:

```sh
ADL_NESSUS_BUILDER_IMAGE=<pullable-image-uri> \
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --run \
  --base origin/main \
  --head HEAD \
  --remote-git-ref <branch-or-commit> \
  --remote-artifact-dir "$ADL_ARTIFACT_DIR/nessus"
```

If the image is already loaded on Nessus as a local tag, do not let the wrapper
try to pull it from a registry:

```sh
ADL_NESSUS_BUILDER_IMAGE=adl-builder:v0.91.7-fixed \
ADL_NESSUS_BUILDER_PULL_POLICY=never \
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --run \
  --base origin/main \
  --head HEAD \
  --remote-git-ref <branch-or-commit> \
  --remote-artifact-dir "$ADL_ARTIFACT_DIR/nessus"
```

Raw-host Nessus PR-fast runs require `cargo nextest --version` to work on the
remote host. If it does not, use the builder image path. A local-only image tag
without `ADL_NESSUS_BUILDER_PULL_POLICY=never` is a setup error, not validation
proof.

Record:

- remote git ref
- fetched summary/log artifact paths
- cold image-backed or warm target-cache posture
- benchmark line if `run_build_platform_benchmark.sh` ran

## 4. AWS Spot

No-cost, self-verifying preflight:

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh preflight \
  --git-ref <pushed-branch-or-commit>
```

Asynchronous live run:

```sh
RUN_ID="adl-spot-$(date -u +%Y%m%d%H%M%S)"

bash adl/tools/run_aws_spot_remote_validation_lane.sh launch \
  --run-id "$RUN_ID" \
  --git-ref <pushed-branch-or-commit> \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture fixed_builder_image_warm_ebs_cache --out .adl/local-artifacts/build-platform/aws-spot-summary.json --artifact-dir .adl/local-artifacts/build-platform/aws-spot' \
  --instance-type m7a.2xlarge
```

Use `m7a.2xlarge` as the normal Spot builder shape for Rust validation that
touches the AWS SDK dependency graph. Retained WP-06 proof shows this shape can
complete the benchmark with the warm EBS cache. Do not use `c7i.large` for ADL
Rust/AWS builds: #4998 retained negative evidence shows `c7i.large` reached
compile and then failed with `sccache: Compiler killed by signal 9` while
compiling `aws-sdk-ec2`. Smaller or cheaper Spot shapes count only after a
retained successful run for the same class of workload.

The wrapper resolves `adl-builder:v0.91.7-fixed` to an immutable digest and
runs the validation inside it. Mutable image references are rejected. The
retained hot-cache proof pins the intended subnet and EBS identity; do not pick
one of the duplicated historical Name tags manually.

Operate the run with:

```sh
bash adl/tools/run_aws_spot_remote_validation_lane.sh status --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh logs --follow --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh ssh --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh stop --run-id "$RUN_ID"
bash adl/tools/run_aws_spot_remote_validation_lane.sh cleanup --run-id "$RUN_ID"
```

Record:

- account check passed
- advertised git ref
- selected instance type and why it is appropriate for the workload
- immutable builder image digest hash and verified toolchain
- retained EBS identity, attachment, mount health, free space, and preexisting target entries
- benchmark line
- cleanup/termination completed
- `resume-state.json` with interrupted attempts when Spot is reclaimed
- `wrapper-final-summary.json` with final self-verification status plus explicit
  interruption and resume counts
- redacted logs/artifacts and excluded mode-600 `.private` recovery state

Do not call a Spot row warm unless the retained EBS cache is attached in the
AWS-side summary.

If Spot is interrupted, keep the retained artifact directory and rerun from the
same issue/ref/command context. A successful retry must be recorded as
`resumed_after_interruption`, with the previous interrupted attempt visible in
`resume-state.json`; do not rewrite it as an ordinary `passed` run.
Validation/test failures do not retry. Only classified infrastructure failures
or a provider-confirmed Spot interruption may consume a bounded retry.

## 5. CodeBuild / CodeFriend

Run the wrapper dry-run/account check first:

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --dry-run \
  --check-account \
  --profile agent-logic-admin \
  --project-name adl-codefriend-build \
  --source-version HEAD \
  --out "$ADL_ARTIFACT_DIR/codebuild-dry-run-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/codebuild-dry-run"
```

Create or update the project only after the wrapper account check passes. This
step mutates AWS IAM, S3, and CodeBuild resources.

```sh
bash adl/tools/setup_aws_codefriend_build_resources.sh \
  --apply \
  --profile agent-logic-admin \
  --region us-west-2 \
  --compute-type BUILD_GENERAL1_XLARGE \
  --artifact-dir "$ADL_ARTIFACT_DIR/codebuild-setup"
```

Live run:

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --profile agent-logic-admin \
  --project-name adl-codefriend-build \
  --source-version <40-character-commit-sha> \
  --region us-west-2 \
  --full-nextest \
  --out "$ADL_ARTIFACT_DIR/codebuild-live-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/codebuild-live" \
  --wait \
  --poll-seconds 15 \
  --timeout-seconds 900
```

Record:

- project `adl-codefriend-build`
- compute type
- immutable builder image digest
- stable `/codebuild/adl-source` and `/codebuild/adl-target` paths
- 18 Cargo build jobs
- 18 nextest workers, matching the lane's minimum half-vCPU concurrency policy
- no-fail-fast broad execution so one paid run reports every failing test
- S3 `sccache` and compatibility-keyed S3 target archive posture
- redacted live CloudWatch stream attached by `--wait`
- benchmark line
- terminal CodeBuild status
- redacted logs/artifacts

Do not report nested Docker-in-CodeBuild, image-built-inside-job, or S3-only
diagnostic rows as the operational CodeBuild path.

## 6. Shared Benchmark Command

Use this inside any lane when comparing platforms:

```sh
bash adl/tools/run_build_platform_benchmark.sh \
  --platform <platform> \
  --cache-posture <cache-posture> \
  --out "$ADL_ARTIFACT_DIR/<platform>-benchmark-summary.json" \
  --artifact-dir "$ADL_ARTIFACT_DIR/<platform>-benchmark"
```

Accepted current comparison rows live in
[Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md).

## 7. Proof Checklist

Every reported remote-build result should include:

- issue/worktree and git ref
- platform and cache posture
- command or wrapper used
- dry-run versus paid live run
- build seconds, test seconds, total seconds, status
- cache proof: warm EBS, stable CodeBuild target cache, S3 `sccache`, or Nessus
  warm target cache
- artifact path under `.adl/local-artifacts/`
- cleanup status for AWS resources
- explicit non-claims for any missing live proof

## 8. Troubleshooting

- Wrong AWS account: rerun the wrapper with `--check-account --profile agent-logic-admin`.
- CodeBuild too slow: verify the immutable ECR digest, 18 Cargo jobs, S3 target
  cache restore result, and S3 `sccache` hit rate. Do not cache Cargo binaries,
  rustup, or `/codebuild/adl-target` through CodeBuild local custom cache.
- Spot interrupted: inspect `resume-state.json` and
  `wrapper-final-summary.json`, then rerun the same issue/ref/command context.
- Spot too slow: verify retained EBS cache attachment and cleanup status.
- Nessus SSH asks for a password/passphrase: fix the operator SSH key before
  treating the lane as operational.
- `0s` timing looks suspicious: rerun with precise timing before reporting it.
- Wuji image parity is requested: stop until an ARM64 or multi-arch builder
  image exists.

## Related Docs

- [Validation Platform Routing](VALIDATION_PLATFORM_ROUTING.md)
- [ADL Builder Image](ADL_BUILDER_IMAGE.md)
- [AWS CodeFriend Build Lane](AWS_CODEFRIEND_BUILD_LANE.md)
- [AWS Spot Remote Validation Lane](AWS_SPOT_REMOTE_VALIDATION_LANE.md)
- [Nessus Validation Manager Lane](NESSUS_VALIDATION_MANAGER_LANE.md)
- [Build Platform Benchmarks](BUILD_PLATFORM_BENCHMARKS.md)
