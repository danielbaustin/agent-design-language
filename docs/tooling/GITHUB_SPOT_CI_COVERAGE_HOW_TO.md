# GitHub Spot CI And Coverage HOW-TO

## Rollout Model

GitHub Actions remains the control plane. A small hosted runner authenticates
to the Agent Logic AWS account with OIDC, launches the ADL Spot lane, receives
logs and retained proof, publishes artifacts, and reports the check result.
Rust build, test, and coverage work runs inside the immutable builder container
on Spot with the retained 500 GiB EBS cache.

Roll out in four stages:

1. Run manual `adl-ci-and-coverage` as the operational shadow: both profiles
   execute concurrently inside one Spot launch, one retained cache, and one
   cleanup scope.
2. Use the individual `adl-ci` and `adl-coverage` profiles only for bounded
   diagnosis. The GitHub cutover path uses exactly one Spot job,
   `adl-ci-and-coverage`, for both stable checks.
3. Rehearse failure, cancellation, interruption, and hosted rollback.
4. Change the required workflow routing only after all prior gates pass.

The existing `.github/workflows/ci.yaml` remains the rollback path during this
issue. Do not remove its hosted jobs.

## Cutover Runbook

Advance only when the current phase is green:

1. **Image qualification.** Publish from a full 40-hex source commit, verify
   CodeBuild resolved that exact commit, retain the ECR digest proof, and pass
   the non-root Spot toolchain preflight.
2. **Shadow qualification.** Run `adl-ci` against at least two existing issue
   commits and run `adl-coverage` twice against one historically green commit.
   Retain source SHA, image digest, cache state, validation/remote/total time,
   cost estimate, test counts, cleanup, and hosted comparison.
3. **Failure qualification.** Prove failed tests, cancellation, and Spot
   interruption still terminate EC2, remove temporary IAM/network resources,
   return the retained EBS volume to `available`, sanitize artifacts, and leave
   stable checks red rather than skipped-success.
4. **Canary route.** Set `ADL_HEAVY_CI_BACKEND=spot` for one controlled
   same-repository PR and verify exactly one Spot launch runs both profiles;
   the stable `adl-ci` and `adl-coverage` contexts aggregate that same result.
   Fork PRs and non-PR coverage must remain hosted.
5. **Broad route.** Keep the variable at `spot` for trusted same-repository PRs
   only after two consecutive canary PRs complete without operator repair.
   Monitor launch latency, workload time, interruption rate, cache headroom,
   and cleanup residue.
6. **Rollback rehearsal.** Set `ADL_HEAVY_CI_BACKEND=hosted`, rerun checks, and
   verify the hosted graph owns both stable contexts. Deleting the variable
   must produce the same fail-safe result.

Stop the rollout and use hosted runners when source/image identity cannot be
verified, the retained volume is not exclusively available, required tools are
missing, cleanup is incomplete, artifact sanitization fails, or a stable check
has no selected backend. Do not remove the hosted implementation during this
milestone.

## AWS And GitHub Setup

Configure a GitHub OIDC role in the Agent Logic account. Restrict trust to this
repository and approved branches/environments. The workflow requires:

- `id-token: write`
- `contents: read`
- repository secret `AWS_SPOT_REMOTE_VALIDATION_ROLE_ARN`
- repository variable `AWS_SPOT_REMOTE_VALIDATION_REGION` (defaults to
  `us-west-2`)
- repository variable `AWS_SPOT_REMOTE_VALIDATION_SSH_ALLOWED_CIDR`, set to the
  operator's current public `/32` address
- protected GitHub environment `adl-spot-ci`; configure it before cutover and
  do not rely on GitHub's unprotected auto-created environment default

Configure `adl-spot-ci` with **Selected branches and tags**, branch patterns
`main` and `codex/*`, and no tag patterns. Do not add required reviewers to
this CI environment because unattended required checks must not wait for a
deployment approval. Fork pull requests still cannot match these repository
branch rules and are explicitly routed to hosted runners by the workflow.

Do not store AWS access keys in GitHub. The role needs the bounded EC2, EBS,
SSM, IAM, ECR-read, and cleanup permissions already created for the Spot lane.
Its OIDC trust includes the dedicated `adl-spot-ci` environment subject so
automatic pull-request calls do not require a repository-wide
`pull_request` subject. Keep environment deployment policy restricted to the
trusted repository branches that may run Spot.

The live workflow always enables port 22 using the configured operator CIDR;
it does not fall back to the ephemeral GitHub runner address. The retained
passphraseless key is the same key used by the local SSH recovery command.
GitHub-side execution uses SSM as a second, retained log channel when the
operator-only SSH ingress is not reachable from the hosted runner.

This is a public repository. Keep privileged Spot dispatch on trusted
`workflow_dispatch`, protected branches, or an approval-gated environment.
Never run privileged self-hosted work directly for untrusted fork PR code.

## Independent Shadow Runs

Open **Actions -> aws-spot-remote-validation -> Run workflow**.

For the operational apples-to-apples run:

- `mode`: `start-run`
- `profile`: `adl-ci-and-coverage`
- `remote_ref`: pushed branch under test; blank uses the workflow branch
- `git_ref`: exact immutable commit under test
- `base_ref`: merge base or `origin/main`
- `source_event_name`: `pull_request` for a PR shadow
- `instance_type`: `m7a.2xlarge`
- `validation_command`: blank

The combined profile runs path-policy `adl-ci` first and authoritative
`adl-coverage` second on the same host and retained target. It emits separate
profile timings and pass records, then one total run timing. A successful CI
only run or coverage-only run is not a substitute for this combined proof.

For CI-only diagnostics:

- `mode`: `start-run`
- `profile`: `adl-ci`
- `remote_ref`: pushed branch under test; blank uses the workflow branch
- `git_ref`: exact immutable commit under test
- `base_ref`: merge base or `origin/main`
- `source_event_name`: event semantics to reproduce; use `pull_request` for a
  PR shadow
- `instance_type`: `m7a.2xlarge`
- `validation_command`: blank

For coverage-only diagnostics, use `profile: adl-coverage`. The named profiles
reject a custom command so the proof cannot silently run a cheaper substitute. Use
`profile: custom` only for explicit operator diagnostics.

The remote commands are owned by
`adl/tools/run_aws_spot_ci_profile.sh`. `adl-ci` applies the same path policy as
hosted CI, runs formatting and clippy when Rust is selected, runs focused tests
and doc tests only when coverage has not taken ownership, and runs selected
demo or tracked-proof lanes. It does not force full nextest when hosted CI
would delegate that work to `adl-coverage`. `adl-coverage` verifies the
preinstalled coverage toolchain and runs the authoritative coverage lane with
its instrumented target rooted under retained EBS.

### Shadow An Existing Issue Without Affecting Its PR

Dispatch this workflow from its trusted implementation branch, then set:

- `remote_ref` to the existing issue's advertised branch name
- `git_ref` to that branch's exact pushed commit SHA
- `base_ref` to the exact base commit used for comparison
- `issue_number` to the proof-owning issue number

The workflow performs a narrow fetch of `remote_ref`, verifies `git_ref`, and
requires the advertised branch tip to equal the requested immutable commit.
The remote builder independently checks its `HEAD` against the same commit
before mounting source into the container. A branch update during launch fails
closed instead of validating one tree while reporting another SHA. The launcher
embeds the reviewed Spot runner, image wrapper, and named-profile script as a
trusted control bundle; the container mounts that bundle read-only at
`/adl-control` and the selected source separately at `/workspace`. This allows
an older target branch to run the current trusted lane without rewriting that
branch or depending on helper scripts that are absent from it. It does not
push, edit the target PR, publish a replacement required check, or change the
target issue's lifecycle records. Results remain attached to the independent
workflow-dispatch run. This makes it suitable for hosted-versus-Spot comparison
before cutover.

Do not dispatch untrusted fork code through this privileged path. A selected
ref must be an advertised branch in this repository, and the immutable commit
must resolve from that branch before EC2 launches.

## Image Gate

Before the first coverage run, publish an immutable builder image containing:

- `cargo-nextest`
- `cargo-llvm-cov`
- GitHub CLI (`gh`), because lifecycle tests exercise its process boundary
- `llvm-tools-preview`
- `sccache`
- `lld`

The profile fails if any required tool is absent. It never installs them during
the paid validation run. The operational default is
`adl-builder:v0.91.7-coverage-5243`; every live run resolves the tag to an
immutable digest before launch. Keep the previous digest available for
rollback.

## Required Proof Before Cutover

For the combined profile, retain two consecutive same-commit runs showing:

- exact source commit and image digest
- cache target pre-existing size and free space
- command, launch, SSM, validation, teardown, and total timings
- separate `adl-ci` build/test success and `adl-coverage` success in the same
  Spot lifecycle
- returned logs and coverage summary evidence
- instance termination and temporary-resource cleanup
- estimated compute cost

Also prove cancellation or interruption cleanup and confirm the retained volume
returns to `available`. The workflow's always-run artifact step sanitizes
partial JSON, JSONL, and text logs before fail-closed identifier verification,
so cancellation cannot upload a raw AWS identifier merely because normal
finalization was interrupted.

## Cutover And Required Checks

Do not rename `adl-ci` or `adl-coverage`; branch protection depends on those
stable contexts. The stable aggregators accept exactly one selected backend:
the existing hosted job graph or the reusable Spot workflow. Repository
variable `ADL_HEAVY_CI_BACKEND` is the route switch. Its absent/default value
is `hosted`; set it to `spot` only after the proof gates below pass. Same-repo
PR checks may use Spot. Push-to-main, schedule/nightly, and workflow-dispatch
coverage remains hosted so LCOV and Codecov publication behavior is unchanged.
Fork PRs always remain on
the unprivileged hosted path even when the repository variable is `spot`. The
reusable call forwards the original event name so pull-request, push, schedule,
and workflow-dispatch policy remains aligned with the hosted lane.

The aggregators verify backend selection instead of treating every skipped job
as success. A selected Spot lane must succeed whenever path policy requires its
work. On the hosted path, required Rust format/clippy and test jobs must each
succeed, while required demo/proof work is checked independently. Success in
one category cannot mask a skipped required category.

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

The fastest rollback requires no source edit: set repository variable
`ADL_HEAVY_CI_BACKEND=hosted`, then rerun the failed checks. The workflow also
defaults to hosted when the variable is absent or has any value other than
`spot`, so deleting the variable is fail-safe rollback.

## Concurrency And Operations

The retained volume allows one attachment at a time. Keep the global workflow
concurrency group `aws-spot-remote-validation-ebs-cache` with
`cancel-in-progress: false`. CI and coverage shadow jobs are independent but
serialize when they use this volume.

Use the Spot HOW-TO for status, live logs, SSH, stop, cleanup, cache recovery,
image rollback, and alternate instance selection:

- [AWS Spot Remote Execution HOW-TO](AWS_SPOT_REMOTE_EXECUTION_HOW_TO.md)
