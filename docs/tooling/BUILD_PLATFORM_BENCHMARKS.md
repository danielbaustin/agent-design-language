# Build Platform Benchmarks

`adl/tools/run_build_platform_benchmark.sh` runs the same small build/test
workload on each build platform so WP-06 timing comparisons use one shape.

The workload is:

```text
cargo build --manifest-path <repo>/adl/Cargo.toml --locked --bin adl-pr-doctor
cargo test --manifest-path <repo>/adl/Cargo.toml --locked --lib provider_communication -- --nocapture
```

The helper writes a JSON summary and logs under the selected artifact directory,
then prints one summary line:

```text
ADL_BUILD_PLATFORM_BENCHMARK platform=<name> build_seconds=<n> test_seconds=<n> total_seconds=<n> status=passed
```

## Local And Remote Platforms

Wuji:

```bash
bash adl/tools/run_build_platform_benchmark.sh \
  --platform wuji \
  --cache-posture linked_target_cache_warm \
  --out .adl/tmp/build-platform-benchmark/wuji/summary.json \
  --artifact-dir .adl/tmp/build-platform-benchmark/wuji
```

Nessus:

```bash
bash adl/tools/run_nessus_remote_validation.sh \
  --run-id <run-id> \
  --git-ref <branch-or-ref> \
  --local-artifact-dir .adl/tmp/build-platform-benchmark/nessus \
  --command 'if ! command -v clang >/dev/null 2>&1; then sudo apt-get update && sudo apt-get install -y clang; fi; export CC=clang; bash adl/tools/run_build_platform_benchmark.sh --platform nessus --cache-posture remote_target_sccache_warm --out .adl/tmp/build-platform-benchmark/nessus/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/nessus'
```

AWS Spot:

```bash
bash adl/tools/run_aws_spot_remote_validation_lane.sh \
  --run \
  --check-account \
  --command 'bash adl/tools/run_build_platform_benchmark.sh --platform aws_spot --cache-posture warm_ebs_cache --out .adl/tmp/build-platform-benchmark/aws-spot-ebs/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/aws-spot-ebs' \
  --git-ref <branch-or-ref> \
  --out .adl/tmp/aws-spot-remote-validation/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-spot-remote-validation/<run-id>/artifacts \
  --instance-type m7a.2xlarge \
  --json
```

CodeBuild is owned by `#4838`. Do not use this `#4837` Spot branch as the
source of CodeBuild operational commands; merge or inspect the CodeBuild issue
branch for that lane's wrapper and current proof.

## Cache Postures

- `wuji`: linked local target cache.
- `nessus`: remote target cache plus `sccache`; use `CC=clang` for the current
  Linux AWS LC build surface.
- `aws_spot`: retained warm EBS cache mounted at `/mnt/adl-cache`; this volume
  has a standing AWS storage cost and the run summary must show
  `cache_volume.attachment_state: "attached"`.
- `codebuild`: tracked separately by `#4838`; this `#4837` branch does not
  contain the CodeBuild wrapper and should not be used as CodeBuild runbook
  truth.

## Current Comparison Snapshot

These are WP-06 working measurements, not universal performance claims:

| Platform | Cache posture | Build | Test | Total | Notes |
| --- | --- | ---: | ---: | ---: | --- |
| Wuji | `linked_target_cache_warm` | 11s | 0s | 11s | Shared helper, warm local linked target cache. |
| Nessus | `remote_target_sccache_warm` | 3s | 0s | 3s | Shared helper through Nessus SSH wrapper with `CC=clang`. |
| AWS Spot | `warm_ebs_cache` | 65s | 55s | 120s | Accepted warm-EBS proof; summary records `cache_volume.attachment_state: attached`; this run recorded `ssh_debug_degraded`, and the wrapper default was then updated to the matching retained key. |
| AWS Spot baseline | `no_explicit_ebs_cache` | 221s | 190s | 411s | Historical baseline run completed and cleaned up; not accepted as warm-EBS proof. |
| CodeBuild | tracked by `#4838` | pending in this branch | pending in this branch | pending in this branch | This Spot branch does not contain the CodeBuild wrapper or current CodeBuild proof. |

Refresh this table only from retained summaries or logs. Do not infer a cache
posture from command labels alone.
