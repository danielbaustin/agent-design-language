# GitHub Spot CI And Coverage HOW-TO

## Rollout Model

GitHub Actions remains the control plane. A small hosted runner authenticates
to the Agent Logic AWS account with OIDC, launches the ADL Spot lane, receives
logs and retained proof, publishes artifacts, and reports the check result.
Rust build, test, and coverage work runs inside the immutable builder container
on Spot with the retained 1000 GiB EBS cache.

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

### Runtime And Capacity Guardrails

Every live Spot invocation has a hard manager wall-clock limit. The workflow
default is 1800 seconds; the remote command itself remains bounded at 600
seconds. A timeout returns a failed check and runs the cleanup step rather than
leaving a paid builder running indefinitely. The limit can be lowered for a
known-fast shadow, but the workflow rejects values below 300 seconds or above
3600 seconds; the lower-level wrapper also fail-closes values below 30 seconds.

The workflow passes an ordered capacity pool instead of relying on one EC2
shape. The default pool is `c7a.8xlarge,m7a.8xlarge,c7i.8xlarge`; Spot tries
each type in order and never silently falls back to on-demand because the
workflow invokes the launcher with `--spot-only`. Keep the pool in the same
availability-zone-compatible topology as the retained EBS volume.

New builders receive both `adl:managed=true` and
`adl:lane=spot-remote-validation` tags. The cleanup step runs
`adl/tools/sweep_aws_spot_orphans.sh --run --run-id <exact-run-id>
--max-age-minutes 90` after the run-specific cleanup. The live sweep is
deliberately narrow: it only considers those two tags plus the exact run ID,
requires a 30-minute minimum age, records hashed instance identities, and never
modifies the retained EBS volume. A broad sweep without `--run` is a dry-run
and is the preferred diagnostic mode.

The exact-run cleanup path still honors the age gate: it proves ownership with
the generated run ID and can terminate a genuinely stale instance after
primary cleanup fails. A broad live sweep is intentionally unsupported; use
the default dry-run to discover candidates for operator review.

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

If the retained cache is unavailable, unhealthy, or fails its identity and
mount checks, the lane fails closed before EC2 launch. For a deliberate cold
cache measurement, use a separately named proof run and record it as cold; do
not detach, recreate, or overwrite the production retained volume during a
normal PR validation. A builder image or Rust toolchain change requires a new
immutable image digest and a new cache qualification run.

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

Configure `adl-spot-ci` with no deployment branch policy (GitHub API:
`deployment_branch_policy: null`). GitHub's branch-pattern mode evaluates the
PR deployment as `refs/pull/<number>/merge` and rejects it even when the source
branch is `codex/*`; no-policy mode is required for PR merge refs. Do not add
required reviewers to this CI environment because unattended required checks
must not wait for a deployment approval. The workflow still requires a
same-repository PR head and explicitly routes fork pull requests to hosted
runners, so no untrusted fork code can enter the Spot job.

Do not store AWS access keys in GitHub. The role needs the bounded EC2, EBS,
SSM, IAM, ECR-read, and cleanup permissions already created for the Spot lane.
Its OIDC trust includes the dedicated `adl-spot-ci` environment subject so
automatic pull-request calls do not require a repository-wide
`pull_request` subject. Keep the no-policy environment setting paired with the
workflow's same-repository guard; do not remove that guard.

The live workflow always enables port 22 using the configured operator CIDR;
it does not widen ingress to the ephemeral GitHub runner address. The retained
passphraseless key is the same key used by the local SSH recovery command.
GitHub-side execution uses SSM as a second, retained log channel because the
operator-only SSH ingress is intentionally not reachable from the hosted
runner. The finalizer requires either a successful SSH probe and tail (the
local/operator path) or the explicit operator-allowlist SSM fallback with
both stdout and stderr records (the GitHub path). This keeps SSH recovery
available to the operator without weakening the security group.

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
- `instance_type`: `c7a.8xlarge` (36 vCPUs; the production parallel profile)
- `instance_types`: `c7a.8xlarge,m7a.8xlarge,c7i.8xlarge` (ordered Spot
  capacity pool)
- `max_run_seconds`: `1800` (hard manager limit)
- `validation_command`: blank

The combined profile starts path-policy `adl-ci` and policy-selected
`adl-coverage` concurrently on the same host and retained target. It emits
separate profile timings and pass records, then one total run timing. A
successful CI-only run or coverage-only run is not a substitute for this
combined proof.

For CI-only diagnostics:

- `mode`: `start-run`
- `profile`: `adl-ci`
- `remote_ref`: pushed branch under test; blank uses the workflow branch
- `git_ref`: exact immutable commit under test
- `base_ref`: merge base or `origin/main`
- `source_event_name`: event semantics to reproduce; use `pull_request` for a
  PR shadow
- `instance_type`: `c7a.8xlarge`
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
preinstalled coverage toolchain and runs the focused coverage-impact lane for
pull requests, including when path policy marks full coverage as required.
The plan records `mode=pr-fast-sla full_policy=true`; this is the bounded PR
proof proven on `c7a.8xlarge` in 257 seconds with CI and 1238/1238 focused
coverage tests passing. Full authoritative coverage remains required for
push/main and non-PR evidence events. The full lane uses two concurrent
nextest partitions by default, with 18 test threads per partition on the
36-vCPU `c7a.8xlarge` builder. Both partitions must pass before the single
coverage result is emitted; override `ADL_AUTHORITATIVE_COVERAGE_PARTITIONS`
and `ADL_AUTHORITATIVE_COVERAGE_TEST_THREADS` only for a measured builder
shape.

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

## Retained Cache Policy

The EBS cache is retained for the value that actually accelerates repeated
Rust work:

- `/cache-root/target` keeps the normal Cargo target and fingerprints.
- `/cache-root/sccache` keeps compiler artifacts.
- `/cache-root/cargo-home` keeps the Cargo registry and Git checkouts.

The immutable builder image also sets `SCCACHE_CACHE_SIZE=20G`, so compiler
cache eviction is handled by `sccache` rather than by deleting the cache root.

The derived `/cache-root/target/coverage` tree is reset at the start of each
coverage lane. It contains instrumented and LLVM coverage outputs, so retaining
it alongside the normal target causes duplicate artifacts and unbounded cache
growth without improving the next normal build. The reset emits an
`ADL_SPOT_CACHE_PRUNE` record with the removed byte count and explicitly names
the three preserved cache roots. `.profraw` files are also removed by the
coverage lane's existing cleanup trap. This policy preserves warm compilation
value while making repeated CI-plus-coverage runs disk-bounded.

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

The Spot wrapper enforces a 600-second remote-command deadline. The operational
performance target remains 300 seconds, but the larger kill threshold allows a
warm-cache GitHub run to finish and report its actual timing instead of being
cut off at the end of the target window. On timeout or instance loss it
requests SSM `CancelCommand` before terminating the builder, so a canceled
GitHub job does not leave a validation command running during teardown. The
timeout override is constrained to the same 600-second maximum.
