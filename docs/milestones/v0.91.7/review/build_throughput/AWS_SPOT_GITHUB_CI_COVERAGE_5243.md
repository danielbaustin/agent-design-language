# AWS Spot GitHub CI And Coverage Proof

## Scope

Issue `#5243` moves heavy `adl-ci` and `adl-coverage` execution to ephemeral
AWS Spot builders while GitHub Actions remains the orchestration, artifact, and
stable-check reporting surface. The hosted graph remains the default and
immediate rollback backend until all live gates below pass.

## Current Disposition

Status: `shadow_proof_in_progress`

The Spot CI lane is live-proven. The authoritative coverage lane is not yet
green. The first full run found a runtime fixture baseline defect tracked by
`#5267`; a separate run against a historically green commit then proved that
the immutable builder image lacked `gh`, which a lifecycle test executes. The
image contract now requires and verifies `gh`, but a replacement immutable
digest and two same-commit coverage repeats are still required. No
required-check cutover claim is made.

The `adl-spot-ci` GitHub environment is configured with selected-branch rules
for `main` and `codex/*`, no environment secrets, no manual approval gate, and
no tag rule. The Agent Logic OIDC role trust was then applied and independently
read back with exactly these subjects:

- `repo:danielbaustin/agent-design-language:ref:refs/heads/main`
- `repo:danielbaustin/agent-design-language:ref:refs/heads/codex/*`
- `repo:danielbaustin/agent-design-language:environment:adl-spot-ci`

No repository-wide `pull_request` subject is present. Production routing is
still disabled while coverage proof is incomplete.

## Live Runs

| Surface | Source | Image | Cache | Validation | Remote | Total | Result |
|---|---|---|---|---:|---:|---:|---|
| `adl-ci` GitHub shadow | `f0b86deb` | prior immutable CI image | retained EBS | 27s | 124s | 226s | passed |
| `adl-ci` GitHub shadow repeat | `f0b86deb` | prior immutable CI image | retained EBS | 27s | 124s | 195s | passed |
| `adl-ci` existing-PR shadow | `cd6aded` (`#5258`) | `v0.91.7-coverage-5243` | retained 500 GiB EBS | 42s | 144s | 252s | passed |
| `adl-ci` existing-PR shadow repeat | `cd6aded` (`#5258`) | `v0.91.7-coverage-5243` | retained 500 GiB EBS | 42s | 156s | 255s | passed |
| `adl-ci` existing-PR shadow | `461f2cac` (PR `#5158`) | `v0.91.7-coverage-5243` | retained 500 GiB EBS | 305s | 406s | 501s | passed |
| `adl-coverage` existing-PR shadow | `cd6aded` (`#5258`) | `v0.91.7-coverage-5243` | retained 500 GiB EBS | 1212s remote workload | 1212s | 1301s | failed on runtime fixture; `#5267` |
| `adl-coverage` historical-green control | `9346f230` | `v0.91.7-coverage-5243` | retained 500 GiB EBS | 679s before fail-fast | 864s | 970s | failed because image lacked `gh`; 1777/1778 executed tests passed |
| `adl-coverage` replacement-image control | `9346f230` | replacement `v0.91.7-coverage-5243` | retained 500 GiB EBS | 2.7s before fail-fast | 109s | 214s | image preflight passed; two tests exposed cross-run shared `TMPDIR` fixture collision |

Both existing-PR CI shadows ran 54 focused tests, doc tests, and demo smoke
successfully, with identical 42-second validation time. They used the exact source commit and merge base without modifying
the target branch or PR. The instance terminated, temporary AWS resources were
removed, and the retained volume remained available. Estimated Spot compute
cost was approximately `$0.015` per run using observed instance lifetime and
the lane's pre-run hourly estimate.

The PR `#5158` shadow used exact head
`461f2cac80a6adc30f21ae25f57b09837ddce7d4` and base
`14f72c2b0e4d372e86c241fd57813265d6eb1d1c`. Path policy selected Rust
format/clippy and demo smoke while delegating authoritative full coverage to
the coverage profile. The immutable container reported 305 seconds of
validation, including 122 seconds for the warm-target clippy build; the remote
command took 406 seconds and the complete launch-through-cleanup run took 501
seconds. The retained target contained 38,095 pre-existing entries and
307,950,690,304 bytes, with 107,650,170,880 bytes free. Estimated Spot compute
cost was `$0.028738`. The instance terminated, the temporary security group,
instance profile, and role were deleted, and the retained volume returned to
`available`.

The replacement image was published from exact source commit
`8b678bd933bbd73aa8481f3dd4173b54761f0518`; the publisher verified
CodeBuild's resolved source commit and retained a digest proof. Its first Spot
control passed the real image toolchain gate including `gh`. Two
`long_lived_agent` tests then observed prior fixture state because the retained
cache also retained a shared container `TMPDIR`. Cargo target, Cargo home, and
`sccache` remain shared, while temporary state is now isolated under a unique
run-id directory. That directory is cleared before every attempt, including a
retry after abrupt interruption, and removed after a completed container run.

## Control And Source Separation

The launcher embeds compressed copies of the reviewed remote runner, immutable
image wrapper, and named CI profile. The Spot host materializes that control
bundle outside the source checkout. The validation container mounts it
read-only at `/adl-control` and mounts the selected branch at `/workspace`.
This permits independent testing of an older PR commit without copying current
scripts into that PR or invalidating its retained source/cache layout.

## Cleanup And Failure Evidence

- Successful runs terminated the instance and removed temporary IAM and
  security-group resources.
- A cancelled GitHub shadow terminated its launched instance and returned the
  retained EBS volume to `available`.
- The failed full-coverage run also completed teardown and retained the cache.
- The historical-green control run terminated its instance, deleted its
  temporary IAM and security-group resources, and returned the retained cache
  after the missing-`gh` failure.
- The replacement-image control also completed teardown and returned the
  retained cache after detecting the shared-`TMPDIR` collision.
- The always-run artifact step now sanitizes partial JSON, JSONL, and text
  evidence before independently failing closed on retained AWS identifiers.
  Artifact upload is conditional on the sanitizer step succeeding.
- Embedded JSON proof records are parsed before redaction so numeric cache and
  timing metrics remain valid JSON rather than being mistaken for account ids.

## Offline Contract Proof

- `actionlint v1.7.12 .github/workflows/ci.yaml .github/workflows/aws-spot-remote-validation.yaml`
- `bash adl/tools/test_run_aws_spot_remote_validation_lane.sh`
- `bash adl/tools/test_run_aws_spot_builder_image_validation.sh`
- `bash adl/tools/test_run_aws_spot_ci_profile.sh`
- `bash adl/tools/test_aws_spot_artifact_finalize.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `cargo test --manifest-path tools/aws_remote_validation/Cargo.toml`

The remote-validation binary now exposes
`adl.aws_remote_validation.capabilities.v1`. Paid-run preflight requires both
`embedded_control_bundle_v1` and `spot_only_v1`; a historical binary that only
recognizes `--spot-only` is rejected before account, topology, image, or launch
work. The focused negative fixture and the current binary capability probe both
pass. Final bounded review reported no remaining P1/P2 finding on this guard.

The sanitizer also passed against a copied 7.3 MiB live artifact tree from the
successful `#5258` shadow, followed by an independent fail-closed verification
pass.

The final bounded subagent review reproduced the prior skipped-Rust routing
case and confirmed that it now fails closed. It reported no remaining P1/P2
findings after verifying sanitizer-gated upload, advertised-ref binding,
requirement-aware backend selection, embedded-JSON metric preservation,
fork/OIDC boundaries, cleanup, rollback, and the retained `#5158` proof.

## Production Routing And Rollback

Repository variable `ADL_HEAVY_CI_BACKEND` controls the heavy backend:

- absent, `hosted`, or any value other than `spot`: existing hosted lanes
- `spot`: reusable Spot `adl-ci` and `adl-coverage` lanes for trusted same-repo
  pull requests
- fork pull requests: hosted lanes regardless of the variable

Push-to-main, schedule/nightly, and workflow-dispatch coverage remains hosted
to preserve LCOV and Codecov publication behavior.

Stable `adl-ci` and `adl-coverage` aggregators preserve branch-protection check
names. Immediate rollback is setting `ADL_HEAVY_CI_BACKEND=hosted` or deleting
the variable and rerunning the checks.

## Remaining Proof Before Cutover

1. Publish and resolve a replacement immutable image that passes the expanded
   toolchain preflight, including `gh`.
2. Run two consecutive same-commit green `adl-coverage` shadows against the
   historical-green control commit; `#5267` remains independent product work.
3. Run the production routing switch in Spot mode and verify stable check names.
4. Run an environment-backed workflow shadow and verify the constrained OIDC
   subject is accepted end to end.
5. Rehearse variable-only rollback to hosted checks.
6. Retain redacted artifacts and final cost/timing comparison.
