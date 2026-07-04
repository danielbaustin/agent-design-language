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
  - Configures an S3 cache for cargo, rustup, cargo-installed binaries, and
    `sccache` artifacts.
  - Installs `sccache` from a pinned prebuilt release instead of compiling it in
    CodeBuild.
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

Follow-up optimization on 2026-07-04 configured S3 cache posture
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

## Result Truth

The integrated repo lane is present, locally proven, and live-proven through the
Agent Logic CodeBuild project. The GitHub Actions path still requires the
repository variable/secret values from
`.adl/tmp/aws-codefriend-build-resource-setup/github-actions-config.env` before
workflow-dispatched `start-build` can be used from GitHub.
