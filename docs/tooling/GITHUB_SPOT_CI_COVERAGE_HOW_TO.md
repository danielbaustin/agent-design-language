# GitHub Spot CI And Coverage HOW-TO

## Rollout Model

GitHub Actions remains the control plane. A small hosted runner authenticates
to the Agent Logic AWS account with OIDC, launches the ADL Spot lane, receives
logs and retained proof, publishes artifacts, and reports the check result.
Rust build, test, and coverage work runs inside the immutable builder container
on Spot with the retained EBS cache.

Roll out in four stages:

1. Run manual `adl-ci` and `adl-coverage` shadow profiles independently.
2. Run each profile twice at one commit and compare artifacts with hosted CI.
3. Rehearse failure, cancellation, interruption, and hosted rollback.
4. Change the required workflow routing only after all prior gates pass.

The existing `.github/workflows/ci.yaml` remains the rollback path during this
issue. Do not remove its hosted jobs.

## AWS And GitHub Setup

Configure a GitHub OIDC role in the Agent Logic account. Restrict trust to this
repository and approved branches/environments. The workflow requires:

- `id-token: write`
- `contents: read`
- repository secret `AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN`
- repository variable `AWS_SPOT_REMOTE_VALIDATION_REGION` (defaults to
  `us-west-2`)

Do not store AWS access keys in GitHub. The role needs the bounded EC2, EBS,
SSM, IAM, ECR-read, and cleanup permissions already created for the Spot lane.

This is a public repository. Keep privileged Spot dispatch on trusted
`workflow_dispatch`, protected branches, or an approval-gated environment.
Never run privileged self-hosted work directly for untrusted fork PR code.

## Independent Shadow Runs

Open **Actions -> aws-spot-remote-validation -> Run workflow**.

For CI:

- `mode`: `start-run`
- `profile`: `adl-ci`
- `git_ref`: pushed branch or commit under test
- `base_ref`: merge base or `origin/main`
- `instance_type`: `m7a.2xlarge`
- `validation_command`: blank

For coverage, repeat with `profile: adl-coverage`. The named profiles reject a
custom command so the proof cannot silently run a cheaper substitute. Use
`profile: custom` only for explicit operator diagnostics.

The remote commands are owned by
`adl/tools/run_aws_spot_ci_profile.sh`. `adl-ci` runs formatting, clippy,
PR-fast/full-nextest selection, and doc tests. `adl-coverage` verifies the
preinstalled coverage toolchain and runs the authoritative coverage lane with
its instrumented target rooted under retained EBS.

## Image Gate

Before the first coverage run, publish an immutable builder image containing:

- `cargo-nextest`
- `cargo-llvm-cov`
- `llvm-tools-preview`
- `sccache`
- `lld`

The profile fails if any required tool is absent. It never installs them during
the paid validation run. Verify the new digest with a dry run and keep the
previous digest available for rollback.

## Required Proof Before Cutover

For both profiles, retain two consecutive same-commit runs showing:

- exact source commit and image digest
- cache target pre-existing size and free space
- command, launch, SSM, validation, teardown, and total timings
- build/test or coverage success
- returned logs and coverage summary evidence
- instance termination and temporary-resource cleanup
- estimated compute cost

Also prove cancellation or interruption cleanup and confirm the retained volume
returns to `available`.

## Cutover And Required Checks

Do not rename `adl-ci` or `adl-coverage`; branch protection depends on those
stable contexts. Implement the final route as one explicit workflow/config
switch whose alternate branch is the existing hosted job graph.

Before enabling Spot by default:

1. Validate workflow contracts locally.
2. Complete independent CI and coverage shadow proofs.
3. Compare hosted and Spot exit status, summaries, and artifacts.
4. Confirm the aggregator remains fail closed.
5. Confirm fork PRs stay on the unprivileged hosted path.
6. Rehearse the rollback switch.

## Immediate Rollback

If launch, cache attachment, image verification, test/coverage execution,
artifact return, check reporting, or cleanup regresses:

1. Revert only the routing-switch commit or set the routing switch to `hosted`.
2. Rerun the existing `.github/workflows/ci.yaml` hosted path.
3. Stop and clean up the failed Spot run by run ID.
4. Verify zero issue-tagged instances and an `available` retained volume.
5. Leave the Spot shadow workflow available for diagnosis; do not repair the
   required check in place while other sessions are blocked.

The fallback is not removed in this issue. Removing it requires later sustained
operational evidence.

## Concurrency And Operations

The retained volume allows one attachment at a time. Keep the global workflow
concurrency group `aws-spot-remote-validation-ebs-cache` with
`cancel-in-progress: false`. CI and coverage shadow jobs are independent but
serialize when they use this volume.

Use the Spot HOW-TO for status, live logs, SSH, stop, cleanup, cache recovery,
image rollback, and alternate instance selection:

- [AWS Spot Remote Execution HOW-TO](AWS_SPOT_REMOTE_EXECUTION_HOW_TO.md)

