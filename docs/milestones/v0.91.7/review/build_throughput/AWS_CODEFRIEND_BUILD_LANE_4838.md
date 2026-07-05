# AWS CodeFriend Build Lane - Issue #4838

## Status

Implemented with local contract proof, Agent Logic AWS resource setup, and a
successful live CodeBuild run on `BUILD_GENERAL1_LARGE`.

## Implemented Surfaces

- `.github/workflows/aws-codefriend-build.yaml`
  - Manual `workflow_dispatch` only.
  - `dry-run` mode renders the CodeBuild request and uploads artifacts.
  - `start-build` mode requires GitHub OIDC, an account-hash secret, and a
    configured CodeBuild project.
  - Uses `contents: read` and `id-token: write`.
- `adl/tools/run_aws_codefriend_build_lane.sh`
  - Repo-native wrapper for dry-run and `codebuild start-build`.
  - Defaults local AWS authority to `agent-logic-admin` and `us-west-2`.
  - Verifies STS account identity by SHA-256 hash before live AWS work.
  - Does not print fresh account IDs, ARNs, user IDs, credentials, or tokens.
- `adl/tools/test_run_aws_codefriend_build_lane.sh`
  - Fake-AWS contract test for STS account hash check, CodeBuild request
    rendering, live `start-build` argument shape, dry-run no-AWS behavior,
    mismatch failure, and workflow trigger/secret contract.
- `adl/tools/setup_aws_codefriend_build_resources.sh`
  - Idempotently creates or updates the Agent Logic CodeBuild project, service
    role, GitHub Actions OIDC start-build role, and local GitHub configuration
    handoff file.
  - Supports `--compute-type` so large and xlarge runs are intentional setup
    states rather than manual AWS console edits.
  - Configures CodeBuild S3 cache for cargo/rustup state and native S3
    `sccache` for compiler artifacts.
  - Installs `sccache` 0.16 from a pinned prebuilt release instead of compiling
    it in CodeBuild.
  - Uses `lld`, disables incremental compilation, and normalizes the checkout
    through `/workspace` so compiler-cache keys are stable across CodeBuild
    source directories.
  - Restricts GitHub OIDC trust to repository `main` and `codex/*` refs.
- `docs/tooling/AWS_CODEFRIEND_BUILD_LANE.md`
  - Operator runbook for dry-run, live boundary, required GitHub configuration,
    setup/update commands, benchmark command, compute size selection, and
    failure handling.

## Local Proof

Commands run from `/Users/daniel/git/agent-design-language/.worktrees/adl-wp-4838`:

```sh
bash -n adl/tools/run_aws_codefriend_build_lane.sh
bash -n adl/tools/test_run_aws_codefriend_build_lane.sh
python3 -m json.tool adl/config/validation_lane_selector.v0.91.6.json >/dev/null
bash adl/tools/test_run_aws_codefriend_build_lane.sh
bash adl/tools/test_select_validation_lanes.sh
bash -n adl/tools/*.sh
ruby -e 'require "yaml"; YAML.load_file(".github/workflows/aws-codefriend-build.yaml"); puts "PASS workflow yaml parse"'
```

Observed results:

- `PASS test_run_aws_codefriend_build_lane`
- `PASS test_select_validation_lanes`
- `PASS workflow yaml parse`

## AWS Account Boundary Proof

Command:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --dry-run \
  --check-account \
  --project-name adl-codefriend-build \
  --source-version HEAD \
  --out .adl/tmp/aws-codefriend-build/account-check-summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build/account-check
```

Observed result:

```text
PASS account_profile_resolved profile=agent-logic-admin account_matches_retained_proof=true
PASS aws_codefriend_build_dry_run project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin
```

This proves the local AWS profile resolved to the same Agent Logic account hash
as the retained issue `#4603` proof. It did not start CodeBuild.

## Agent Logic AWS Resource Setup

Setup helper:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/setup_aws_codefriend_build_resources.sh \
  --apply \
  --project-name adl-codefriend-build \
  --compute-type BUILD_GENERAL1_LARGE
```

Observed result:

```text
PASS aws_codefriend_resources_ready project=adl-codefriend-build region=us-west-2 profile=agent-logic-admin compute_type=BUILD_GENERAL1_LARGE
```

The helper creates or updates:

- the GitHub Actions OIDC provider if missing
- the CodeBuild service role
- the GitHub Actions start-build role
- the `adl-codefriend-build` CodeBuild project
- the S3 cache bucket/prefix used by the CodeBuild project
- `.adl/tmp/aws-codefriend-build-resource-setup/github-actions-config.env`

The local GitHub configuration file contains variable/secret values for the
operator and is not committed.

Review hardening after pre-PR subagent review:

- The workflow rejects ambiguous `HEAD`; blank `source_version` resolves to the
  workflow run SHA.
- The GitHub Actions starter role includes `codebuild:StopBuild`.
- The local wrapper requests `stop-build` when `--wait` times out, then fails
  truthfully.

## Live CodeBuild Boundary

Large compute benchmark command shape:

```sh
ADL_AWS_PROFILE=agent-logic-admin \
bash adl/tools/run_aws_codefriend_build_lane.sh \
  --run \
  --check-account \
  --wait \
  --project-name adl-codefriend-build \
  --source-version codex/4838-v0-91-7-wp-06-aws-add-github-actions-aws-codefriend-build-lane \
  --env ADL_CODEFRIEND_BUILD_COMMAND='<benchmark command>' \
  --out .adl/tmp/aws-codefriend-build/<run-id>/summary.json \
  --artifact-dir .adl/tmp/aws-codefriend-build/<run-id>
```

Observed live result on `BUILD_GENERAL1_LARGE`:

```text
CODEFRIEND_BENCHMARK build_seconds=394 test_seconds=322 total_seconds=716 status=passed
```

The earlier `BUILD_GENERAL1_MEDIUM` run failed after roughly fifteen minutes
with the compiler killed while building a large AWS SDK dependency. The project
was updated to large compute for the successful run.

Follow-up optimization on 2026-07-04 first configured S3 cache posture
`s3_sccache_binary`:

- cache bucket/prefix: `adl-codefriend-build-cache/codebuild/cache`
- cached paths: cargo registry, cargo git, cargo bin, rustup, and sccache
- excluded path: `target/**/*`
- reason for excluding `target`: whole-target S3 cache upload left the build
  stuck in post-build and was slower than useful repeated-build behavior
- first successful S3/sccache binary run on `BUILD_GENERAL1_LARGE`:
  - install phase: `1s`
  - build phase: `560s`
  - post-build cache upload: `57s`
  - terminal status: `SUCCEEDED`

A second repeated cached run was started to capture steady-state cache benefit.
It remained in BUILD past the seed run's build duration and was stopped
intentionally to avoid burning the full CodeBuild timeout:

- run posture: `s3_sccache_binary_repeat`
- terminal wrapper status: `STOPPED`
- conclusion: S3/sccache cache restore and upload are bounded, but this cache
  posture did not produce a fast repeated-build path for the benchmark workload
  on `BUILD_GENERAL1_LARGE`

The requested xlarge comparison was attempted after updating the project to
`BUILD_GENERAL1_XLARGE`, but CodeBuild failed before launch with an account
limit error because the Agent Logic Linux/XLarge concurrent-build quota is `0`.
The quota is adjustable, and a minimal increase request to `1` is pending. The
project was restored to `BUILD_GENERAL1_LARGE` so the live operational lane
continues to work while xlarge waits on AWS quota approval.

After quota approval, the lane was updated to use native S3 `sccache`, `lld`,
`CARGO_INCREMENTAL=0`, and stable `/workspace` paths. The successful repeated
xlarge run is retained at
`docs/milestones/v0.91.7/review/build_throughput/codebuild-xlarge-native-sccache-s3-repeat-20260704.md`.

Observed `BUILD_GENERAL1_XLARGE` native S3 `sccache` repeat result:

```text
ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild build_seconds=101 test_seconds=79 total_seconds=180 status=passed
```

CodeBuild phase timings:

- provisioning: `4s`
- download source: `13s`
- install: `11s`
- build: `180s`
- post build: `32s`

Native S3 `sccache` evidence:

- client: `0.16.0`
- cache location: `adl-codefriend-build-cache/codebuild/cache/sccache/x86_64-unknown-linux-gnu`
- compile requests: `845`
- Rust cache hits: `366`
- Rust cache misses: `2`
- Rust cache hit rate: `99.46%`
- cache read errors: `0`
- cache write errors: `0`

Current live re-run after applying the xlarge native S3 project buildspec:

- Build id: `adl-codefriend-build:5e4ab258-a121-4561-8196-61249e60f793`
- Status: `SUCCEEDED`
- CodeBuild wall-clock: `310s`
- Benchmark line:
  - `ADL_BUILD_PLATFORM_BENCHMARK platform=codebuild build_seconds=96 test_seconds=75 total_seconds=171 status=passed`
- Rust cache hit rate: `99.73%`
- Cache read/write errors: `0`

This current run still installed `lld`/`clang` during the install phase, so the
shared builder image from `#4879` remains the expected next setup-time
optimization.

Post-review hardening replaced the buildspec's temporary credential file with
fileless `eval "$(aws configure export-credentials --format env)"`. The live
CodeBuild project buildspec was re-applied and verified to contain fileless
credential export, no `codebuild-aws-credentials.env` path, native S3
`sccache`, `lld`, and `/workspace` target normalization.

## Result Truth

The integrated repo lane is present, locally proven, and live-proven through the
Agent Logic CodeBuild project. The current fast repeated-build posture is
`BUILD_GENERAL1_XLARGE` with native S3 `sccache`; a custom builder image from
follow-up issue `#4879` is expected to reduce the remaining install/setup
overhead while preserving this compiler-cache path. The GitHub Actions path
still requires the repository variable/secret values from
`.adl/tmp/aws-codefriend-build-resource-setup/github-actions-config.env` before
workflow-dispatched `start-build` can be used from GitHub.
