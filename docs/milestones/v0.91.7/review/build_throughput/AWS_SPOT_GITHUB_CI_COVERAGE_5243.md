# AWS Spot GitHub CI And Coverage Proof

## Scope

Issue `#5243` moves heavy `adl-ci` and `adl-coverage` execution to ephemeral
AWS Spot builders while GitHub Actions remains the orchestration, artifact, and
stable-check reporting surface. The hosted graph remains the default and
immediate rollback backend until all live gates below pass.

## Current Disposition

Status: `live_combined_profile_proven; pr-open-environment-recheck-pending`

Earlier scaled attempts exceeded the ceiling or failed safely before validation
because the retained cache was below its free-space floor. The retained cache
has now been expanded to 1000 GiB. The PR-fast combined
profile is live-proven on `c7a.8xlarge`: CI and focused coverage ran in parallel,
all 1238 selected coverage tests passed, the JSON report completed, live SSH/SSM
logs were observed, and the full launch-through-cleanup lifecycle completed in
257 seconds. Full authoritative coverage remains reserved for push/main.

The `adl-spot-ci` GitHub environment is configured with no deployment branch
policy (`deployment_branch_policy: null`), no environment secrets, no manual
approval gate, and no tag rule. GitHub's selected-branch mode rejects a
pull-request deployment represented as `refs/pull/<number>/merge`; no-policy
mode is required for this trusted same-repository workflow. The Agent Logic OIDC role trust was then applied and independently
read back with exactly these subjects:

- `repo:danielbaustin/agent-design-language:ref:refs/heads/main`
- `repo:danielbaustin/agent-design-language:ref:refs/heads/codex/*`
- `repo:danielbaustin/agent-design-language:environment:adl-spot-ci`

No repository-wide `pull_request` subject is present. Production routing remains
Spot-selected but the PR must pass its environment recheck before required-check
cutover is considered complete; the immediate rollback is the existing
`ADL_HEAVY_CI_BACKEND=hosted` setting.

The first live run of the partitioned profile on commit `4e5465af` completed
Spot launch, SSH/SSM, immutable image, retained-cache, toolchain, CI, and
cleanup proof in 241 seconds. CI passed in 26 seconds. Coverage failed
immediately because the initial implementation placed `--partition` after
nextest's `--` test-binary separator; the remote logs retained the exact
command and exit status 96. The instance terminated and the retained volume
returned to `available`. The fix moves the partition option before the
separator; a fresh live run is required.

Two subsequent live attempts proved the corrected partition command reached
the real workload but did not finish before the 300-second SSM deadline on the
32-vCPU builder: the isolated-target attempt took 380 seconds end to end, and
the shared-warm-target attempt took 379 seconds end to end. Both terminated
cleanly and preserved the retained EBS volume. A 64-vCPU preflight was clean,
but its Spot launch was rejected before compute by `MaxSpotInstanceCountExceeded`.
The PR Spot profile routes pull-request coverage to the existing focused
coverage-impact lane, records `mode=pr-fast-sla full_policy=true`, and retains
full authoritative coverage for push/main evidence. That explicit routing is
live-proven by the `c7a.8xlarge` run below.

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
| `adl-ci` current open-PR shadow | `ab166ff0` (PR `#5158`) | replacement `v0.91.7-coverage-5243` | retained 500 GiB EBS | 311s | 415s | 473s | passed; exact current head/base, image, source, cache, and toolchain verified |
| `adl-coverage` same-commit repeat 1 | `9346f230` | replacement `v0.91.7-coverage-5243` | retained 500 GiB EBS | 736s | 835s | 912s | passed; 2165/2165 tests, 2 skipped |
| `adl-coverage` same-commit repeat 2 | `9346f230` | replacement `v0.91.7-coverage-5243` | retained 500 GiB EBS | 767s | 862s | 919s | passed; 2108/2108 tests, 2 skipped |
| `adl-coverage` final-profile repeat | `9346f230` | replacement `v0.91.7-coverage-5243` | retained 500 GiB EBS | 636s | 738s | 815s | passed; 2108/2108 tests, 2 skipped; final trusted profile |
| `adl-ci` GitHub workflow run 30 | `bc76182c` | immutable digest `sha256:20831e3...` | retained 500 GiB EBS | 249s | 304s | 03:20-03:27Z | passed; 135 requests, 100% Rust cache hits, 343 GB pre-existing target |
| `adl-coverage` GitHub workflow run 31 | `bc76182c` | immutable digest `sha256:20831e3...` | retained 500 GiB EBS | not run | not run | canceled before launch | superseded; operational proof must run both profiles in one lifecycle |
| `adl-ci-and-coverage` GitHub workflow run 32 | `d3adb15f` | immutable digest `sha256:20831e358...` | retained 500 GiB EBS | failed in coverage | failed | failed | stopped on separate `#5267` runtime fixture defect; instance and cache cleanup passed |
| `adl-ci-and-coverage` GitHub workflow run 33 | `9346f230` | immutable digest `sha256:20831e358...` | retained 500 GiB EBS | 728s | 785s | passed | `adl-ci` and `adl-coverage` passed in one lifecycle; 2108/2108 tests, 2 skipped; instance terminated and volume available |
| `adl-ci-and-coverage` GitHub workflow run 38 | `bf79a69d` | immutable digest `sha256:20831e358...` | retained 500 GiB EBS | >300s | canceled | failed target | one `m7a.8xlarge` host; CI and coverage processes were concurrent; stopped at the 300s ceiling |
| `adl-ci-and-coverage` GitHub workflow run 39 | `ca1cb62c` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS resize | preflight | 45s | failed before validation because the retained filesystem was below the 10 GiB floor; Spot cleanup passed |
| `adl-ci-and-coverage` GitHub workflow run 40 | `f511f52a` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | >300s | canceled | failed target | warm repeat reached parallel full-coverage compilation but did not finish before the 300s ceiling; Spot cleanup passed |
| `adl-ci-and-coverage` GitHub workflow run 41 (`29323862776`) | `5e240483` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | CI 28s; coverage failed at 195s | 166s | 225s | post-resize Spot launch, SSM, retained-cache mount, and cleanup passed; coverage returned exit 1 and its preserved profile log is required for diagnosis |
| `adl-ci-and-coverage` direct Spot run (`adl-wp-5243-direct-4e5465af6`) | `4e5465af` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | CI 26s; coverage argument error | 138s | 241s | Spot launch, SSH/SSM, cache/image/toolchain checks, and cleanup passed; coverage exited 96 before tests because partition placement was wrong; retained cache preserved |
| `adl-ci-and-coverage` direct Spot run (`adl-wp-5243-direct-94a3d8f7`) | `94a3d8f7` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | CI 27s; coverage tool-option error | 139s | 195s | Spot launch and cleanup passed; `cargo llvm-cov` rejected incompatible `--no-report` plus `--no-clean`; no coverage tests ran |
| `adl-ci-and-coverage` direct Spot run (`adl-wp-5243-direct-ee2d02ae`) | `ee2d02ae` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | CI passed; isolated coverage timed out | n/a | 380s | Spot launch, SSH/SSM, image, cache, and cleanup passed; duplicated instrumented targets exceeded the 300s remote deadline |
| `adl-ci-and-coverage` direct Spot run (`adl-wp-5243-direct-f4531680`) | `f4531680` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | CI passed; shared coverage timed out | n/a | 379s | Shared warm target and concurrent profiles reached the full workload but exceeded the 300s remote deadline; Spot cleanup passed |
| `adl-ci-and-coverage` direct Spot run (`5243-prfast-bcb316f43`) | `bcb316f43` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | CI passed; 1238 focused coverage tests passed; report path failed | 108s | 165s | Container checkout was `/workspace`, not `/mnt/adl-source`; cleanup passed; no combined pass claim |
| `adl-ci-and-coverage` direct Spot run (`5243-prfast-6969fcaec`) | `6969fcaec` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | no terminal validation output | >300s | 394s | Report-directory fix was deployed; SSM timed out before validation output; cleanup passed; no combined pass claim |
| `adl-ci-and-coverage` direct Spot run (`5243-prfast-6969fcaec-warm`) | `6969fcaec` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS; warm path | no terminal validation output | >300s | 409s | `ADL_RUST_WARM_CACHE=0` removed relink work; SSH stopped responding during container validation; cleanup passed; no combined pass claim |
| `adl-ci-and-coverage` direct Spot run (`5243-prfast-86fabf9a6-c7a`) | `86fabf9a6` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS; warm path | CI passed; 1238/1238 focused coverage tests; report passed | 155s | 257s | `c7a.8xlarge`; live logs, immutable image, source binding, SSH recovery, Spot purchase, and cleanup all verified |
| `adl-ci-and-coverage` 64-vCPU preflight (`adl-wp-5243-direct-m7a16-f4531680`) | `f4531680` | immutable digest `sha256:20831e358...` | retained 1000 GiB EBS | not launched | n/a | 1s | AWS rejected the Spot request with `MaxSpotInstanceCountExceeded`; no compute or validation ran |

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

The two green coverage repeats used exact head
`9346f230c96053dd90fdb60cbf6e04fcc3ffcff8` and base
`363e3f0e8afefbc845408bca3f7ca2a4ccba2e51`. Repeat 1 ran 2,165 tests with a
657.5-second nextest summary and 736-second validation time. Repeat 2 ran
2,108 tests with a 657.5-second nextest summary and 767-second validation
time. Full launch-through-cleanup totals were 912 and 919 seconds. Both runs
verified the same immutable image, source commit, 500 GiB EBS identity, and
per-run temporary directory isolation; both instances terminated cleanly.

The final trusted-profile repeat used the same head and base, completed all
2,108 tests with two skips, and took 634 seconds in the profile, 738 seconds
for the remote command, and 815 seconds end to end. It also verified clean
instance termination and retained-volume cleanup.

The current open-PR shadow used exact head
`ab166ff0bf31366f80cd03e25a9de4ce4523c9c8` and base
`a193ea7c7a4dbd841c5b86dacffb7045c017566a`. It completed the selected
`adl-ci` path policy, including format/clippy, doc tests, and five demo-smoke
cases. Validation took 311 seconds, the remote command took 415 seconds, and
launch-through-cleanup took 473 seconds. The retained target had 47,587
pre-existing entries and 324,113,203,200 bytes, with 90,875,449,344 bytes
free. Estimated Spot compute cost was approximately `$0.027` at the observed
`$0.2065/hour` estimate. The instance terminated and the retained volume
detached successfully.

The final GitHub combined-profile control used exact head
`9346f230c96053dd90fdb60cbf6e04fcc3ffcff8` and base
`363e3f0e8afefbc845408bca3f7ca2a4ccba2e51` on `m7a.2xlarge`. The immutable
builder proof reported 344,309,071,872 pre-existing target bytes, 66,262
pre-existing entries, 69,869,932,544 free cache bytes, a writable verified
mount, and `sccache` at 100% Rust cache hit rate (84 hits, 0 misses, 0 cache
errors). The combined profile recorded `adl-ci=0s` from the retained cache and
`adl-coverage=727s`; the remote command took 785s and the launch-through-
cleanup workflow completed successfully. Coverage ran 2,108 tests with two
skips, and the instance terminated with both retained volumes available.

The workflow's operator-only SSH ingress was verified from the configured
`47.146.81.109/32` source. The operator SSH key matched the EC2 key pair and
live progress/test output was readable during execution. The GitHub runner
used SSM for the control path; future runs stream incremental SSM output into
the retained `remote-tail.log` while preserving the direct operator SSH path.

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
- `spot`: one reusable Spot `adl-ci-and-coverage` lane for trusted same-repo
  pull requests; it runs CI and coverage concurrently on one warm
  `c7a.8xlarge` host with a 300-second wall-time target; the retained proof
  completed in 257 seconds.
- fork pull requests: hosted lanes regardless of the variable

Push-to-main, schedule/nightly, and workflow-dispatch coverage remains hosted
to preserve LCOV and Codecov publication behavior.

Stable `adl-ci` and `adl-coverage` aggregators preserve branch-protection check
names. Immediate rollback is setting `ADL_HEAVY_CI_BACKEND=hosted` or deleting
the variable and rerunning the checks.

## Publication Gate

The live PR-fast proof is complete. Publication still requires the bounded
pre-PR review, a draft PR with the exact pushed source ref, and repository
variable cutover to `spot` only after the draft checks are routed to Spot.
Setting `ADL_HEAVY_CI_BACKEND=hosted` remains the immediate rollback path.
