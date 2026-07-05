# AWS CodeFriend Build Lane

## Status

Implemented for issue `#4838` as a manual GitHub Actions plus AWS CodeBuild
lane. The lane is intentionally separate from the AWS Spot EC2 remote validation
lane.

## Entry Points

- GitHub Actions workflow:
  `.github/workflows/aws-codefriend-build.yaml`
- AWS resource setup helper:
  `adl/tools/setup_aws_codefriend_build_resources.sh`
- Repo-native wrapper:
  `adl/tools/run_aws_codefriend_build_lane.sh`
- Local contract test:
  `adl/tools/test_run_aws_codefriend_build_lane.sh`

## AWS Resource Setup

Check current Agent Logic resources without mutation:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/setup_aws_codefriend_build_resources.sh \
  --check \
  --project-name adl-codefriend-build
```

Create or update the CodeBuild project, service role, and GitHub OIDC start
role. The project includes an S3 cache for cargo/rustup state, uses native S3
`sccache` for compiler artifacts, installs `sccache` 0.16 from a pinned
prebuilt release, enables `lld`, disables incremental compilation, and
normalizes the checkout through `/workspace` for stable compiler-cache keys. The
buildspec exports CodeBuild credentials for `sccache` through an in-memory
`aws configure export-credentials` evaluation and does not write credential
material to a temp file:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/setup_aws_codefriend_build_resources.sh \
  --apply \
  --project-name adl-codefriend-build \
  --compute-type BUILD_GENERAL1_LARGE \
  --cache-bucket adl-codefriend-build-cache \
  --cache-prefix codebuild/cache
```

For xlarge comparison runs, update the project deliberately:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/setup_aws_codefriend_build_resources.sh \
  --apply \
  --project-name adl-codefriend-build \
  --compute-type BUILD_GENERAL1_XLARGE
```

As of the later WP-06 proof pass on 2026-07-04, the Linux/XLarge quota was
approved and `BUILD_GENERAL1_XLARGE` produced the current fastest retained
CodeBuild result with native S3 `sccache`. Use large only when intentionally
testing the smaller/cheaper fallback.

The setup helper writes the GitHub variable/secret values to:

```text
.adl/tmp/aws-codefriend-build-resource-setup/github-actions-config.env
```

That file is local operator material and must not be committed.

## Trigger Contract

The workflow only uses `workflow_dispatch`. It does not run on ordinary pull
requests or pushes.

The workflow rejects ambiguous `HEAD` input. Leave `source_version` blank to use
the workflow run SHA, or pass an explicit branch, tag, or commit.

Dispatch input `mode` controls the boundary:

- `dry-run` renders the CodeBuild request and uploads the request/summary
  artifact without calling AWS.
- `start-build` assumes the configured Agent Logic AWS role through GitHub OIDC,
  verifies the STS account hash, then calls `codebuild start-build`.

## Required GitHub Configuration

Repository variable:

- `AWS_CODEFRIEND_CODEBUILD_PROJECT`: CodeBuild project name.
- `AWS_CODEFRIEND_REGION`: optional region override; defaults to `us-west-2`.

Repository secrets:

- `AWS_CODEFRIEND_BUILD_ROLE_ARN`: OIDC role for starting the CodeFriend
  CodeBuild project.
- `AWS_CODEFRIEND_ACCOUNT_SHA256`: SHA-256 of the approved Agent Logic AWS
  account ID.

The workflow uses `contents: read` and `id-token: write`. It does not require a
long-lived AWS access key. The GitHub Actions path calls the wrapper with
`--profile env` so the AWS CLI uses the OIDC credentials exported by
`aws-actions/configure-aws-credentials`.

The setup helper restricts the OIDC trust to repository `main` and `codex/*`
refs. If another branch namespace needs live AWS builds, update and reapply the
setup helper intentionally.

## Local AWS Account Contract

Local live runs default to:

```sh
ADL_AWS_PROFILE=agent-logic-admin
ADL_AWS_REGION=us-west-2
```

The wrapper compares the live STS account ID hash against the retained Agent
Logic account proof from issue `#4603` unless
`ADL_AWS_CODEFRIEND_ACCOUNT_SHA256` or `--expected-account-sha256` is provided.
It reports only whether the hash matched; it does not print fresh account IDs,
ARNs, user IDs, credentials, or token values.

## Local Dry-Run Proof

```sh
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --dry-run \
  --project-name adl-codefriend-build \
  --source-version HEAD \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_pr_fast_test_lane.sh' \
  --out .adl/tmp/aws-codefriend-build/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build \
  --print-command
```

## Live Start-Build Boundary

Live execution may incur AWS CodeBuild charges and requires the Agent Logic
business account:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --project-name "$ADL_AWS_CODEFRIEND_CODEBUILD_PROJECT" \
  --source-version HEAD \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_pr_fast_test_lane.sh' \
  --out .adl/tmp/aws-codefriend-build/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build
```

Use `--wait` to make the local wrapper block until CodeBuild reaches a terminal
state and to retain a redacted status artifact:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --wait \
  --project-name adl-codefriend-build \
  --source-version <branch-or-ref> \
  --env ADL_CODEFRIEND_BUILD_COMMAND='bash adl/tools/run_build_platform_benchmark.sh --platform codebuild --cache-posture codebuild_xlarge_no_persistent_cache --out .adl/tmp/build-platform-benchmark/codebuild-xlarge/summary.json --artifact-dir .adl/tmp/build-platform-benchmark/codebuild-xlarge' \
  --out .adl/tmp/aws-codefriend-build/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build/<run-id>
```

If `--wait` times out, the wrapper requests `codebuild stop-build` before
returning failure. The GitHub Actions starter role includes `codebuild:StopBuild`
for that cleanup path.

The current lane uses native S3 `sccache` for compiler artifacts. It
intentionally does not cache the full `target/` tree through CodeBuild's S3
cache because that made post-build cache upload slower than the build work.
CodeBuild's S3 cache is limited to reusable toolchain state:

```text
/root/.cargo/registry/**/*
/root/.cargo/git/**/*
/root/.cargo/bin/**/*
/root/.rustup/**/*
```

Compiler artifacts are stored by `sccache` itself under:

```text
s3://adl-codefriend-build-cache/codebuild/cache/sccache/x86_64-unknown-linux-gnu
```

## Current Live Timing

Issue `#4838` has a successful live `BUILD_GENERAL1_LARGE` run:

```text
CODEFRIEND_BENCHMARK build_seconds=394 test_seconds=322 total_seconds=716 status=passed
```

That run proved the project can execute the ADL build/test benchmark on large
compute.

The first S3/sccache binary-cache run also succeeded on
`BUILD_GENERAL1_LARGE`:

```text
phase timings: install=1s build=560s post_build=57s status=SUCCEEDED
```

A repeated cached run was attempted after the seed run. It remained in BUILD
past the seed build duration and was stopped intentionally to avoid burning the
full CodeBuild timeout. That older local-cache/archive posture is operational
and bounded, but not the fast repeated-build path.

The xlarge native S3 `sccache` repeat proof supersedes the older large/no-fast
cache result for repeated-build planning:

```text
ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild build_seconds=101 test_seconds=79 total_seconds=180 status=passed
```

A current re-run after applying the xlarge/native-S3 project buildspec was
slightly faster:

```text
ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild build_seconds=96 test_seconds=75 total_seconds=171 status=passed
```

That run reported Rust cache hit rate `99.73%` with zero cache read/write
errors. CodeBuild wall-clock from start to end was `310s`, including source
download, install/setup, benchmark execution, post-build sccache stats, and S3
cache upload.

Retained proof:

```text
docs/milestones/v0.91.7/review/build_throughput/codebuild-xlarge-native-sccache-s3-repeat-20260704.md
```

Prefer warm-EBS Spot when lowest repeated latency is the goal and the retained
EBS cache is attached. Use CodeBuild xlarge when scalable, isolated GitHub/AWS
dispatch is more important than the absolute fastest warm-cache latency.

## Failure Handling

The lane fails closed when:

- the CodeBuild project is missing
- live mode is requested without the OIDC role secret or account-hash secret
- STS resolves to the wrong account hash
- `aws codebuild start-build` fails
- artifact upload cannot find generated artifacts

The resulting summary and request/response JSON are retained as workflow
artifacts for review.
