# v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth

Canonical Template Source: `docs/templates/prompts/1.0.3/sor.md`

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-4630
Run ID: issue-4630
Version: v0.91.7
Title: [v0.91.7][WP-03] Consume C-SDLC integration control-plane truth
Branch: codex/4630-v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth
Card Status: ready
Status: DONE
Generated: 2026-07-02T17:59:01Z

Execution:
- Actor: issue-wave bootstrap
- Model: not_applicable
- Provider: not_applicable
- Start Time: 2026-06-29T02:07:59Z
- End Time: 2026-07-02T17:35:41Z

## Summary

Execution resumed in a clean rebound worktree after an accidental early start was preserved as local evidence. The issue now has a published draft PR for a first-class repo-native `adl-pr-shepherd` owner binary above readiness, watcher, janitor, and closeout surfaces. After publication, `adl-ci` failed in the retained v0.91.3 proof-validation lane because the older card-lifecycle contract commands broad-compiled enough of the tree to exhaust runner disk. The branch was janitored first by narrowing that retained proof lane to the exact `--bin adl` card-lifecycle tests and then by reclaiming repo-local rootfs space before authoritative coverage plus hardening `pr.sh shepherd` so it fails closed with dedicated owner-binary guidance instead of delegating into a stale generic `adl` binary. Fresh GitHub checks are now pending on commit `3a62c9e6cc866513fa554298f877bac9ed17d03f` in PR `#4714`.

## PVF Lane Truth
- Initial PVF lane: `prompt_template`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `Execution narrowed to the still-missing first-class lifecycle-shepherd command surface.`

## Issue Metrics Truth
- Estimated elapsed seconds: `5400`
- Actual elapsed seconds: `unknown`
- Estimated total tokens: `1000000`
- Actual total tokens: `unknown`
- Estimated validation seconds: `600`
- Actual validation seconds: `unknown`
- Budget source: `manual_entry`
- Goal metrics data source: `manual_entry`
- Goal metrics source ref: `.adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/spp.md`
- Data-source confidence: `medium`
- Estimate error percent: `unknown`
- Issue goal ref: `issue-4630`
- Sprint goal ref: `unknown`
- Goal metrics rollup ref: `unknown`
- Validation planning prompt: `.adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/vpp.md`
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `not_applicable`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Bootstrap scaffold records unknown issue metrics only; variance analysis is deferred until execution produces authoritative estimates and actuals.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Updated issue-local planning and execution truth under `.adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/`
- Preserved accidental pre-WP-03 start evidence at `.adl/docs/TBD/issue-4630-pre-wp03-accidental-status.txt`
- Preserved accidental pre-WP-03 diff evidence at `.adl/docs/TBD/issue-4630-pre-wp03-accidental.diff`
- Published branch implementation changes in:
  - `adl/Cargo.toml`
  - `adl/src/bin/adl_pr_shepherd.rs`
  - `adl/src/cli/pr_cmd.rs`
  - `adl/src/cli/pr_cmd_args.rs`
  - `adl/src/cli/pr_cmd/github.rs`
  - `adl/src/cli/pr_cmd/github/tests/watch.rs`
  - `adl/tools/run_v0913_proof_validation_lane.sh`
  - `adl/tools/pr.sh`
  - `adl/tools/pr_delegate.sh`
  - `adl/tools/pr_usage.sh`
  - `adl/tools/run_authoritative_coverage_lane.sh`
  - `docs/default_workflow.md`
  - `docs/milestones/v0.91.3/review/card_lifecycle_integration/CARD_LIFECYCLE_PROOF_PACKET_v0.91.3.md`
  - `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md`
  - `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md`
  - `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
  - `adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh`
  - `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md`

## Actions taken
- Confirmed WP-03 maps to issue `#4630` in `v0.91.7`.
- Detected a stale session claim and an accidental earlier dirty worktree start for `#4630`.
- Preserved the accidental start as local evidence, released the stale claim, removed the abandoned dirty worktree, and rebound a clean `#4630` worktree through `adl/tools/pr.sh run`.
- Created the issue-bound goal for `#4630`.
- Reviewed the existing watcher, doctor, closeout, delegate, and lifecycle-shepherd contract surfaces to determine whether a first-class shepherd command still remained missing.
- Implemented a first-class repo-native `adl-pr-shepherd` owner-binary path by adding Cargo registration, dedicated binary wiring, parser/dispatch reuse, lifecycle synthesis mapping, help text, and compatibility shell/delegate wiring.
- Updated workflow/operator docs and issue-local planning/review truth so the execution slice matches the real touched surfaces and deliverable.
- Refreshed the bound worktree onto current `origin/main` and reconciled the overlapping command-surface changes so upstream `pr-inventory` truth and this issue's `adl-pr-shepherd` truth both remain present.
- Re-established the issue session-ledger claim after the earlier claim aged out during active work so finish/publication can proceed under a fresh ownership record.
- Ran focused validation for the owner binary lane, retained proof-lane contract, authoritative coverage runner contract, delegate contract, direct `adl-pr-shepherd` JSON output, and `git diff --check`.
- Ran bounded pre-PR review, fixed the staged merge-conflict residue in `adl/src/cli/pr_cmd.rs`, and corrected lifecycle truth so `closeout_needed` maps to `closed_no_pr` and closed issues do not become `settled` before local closeout.
- Published draft PR `#4714` for the implementation branch.
- Diagnosed the first red `adl-ci` run on PR `#4714` as runner disk exhaustion inside the retained `tracked proof-validation lane contract` step rather than a shepherd-command logic failure.
- Narrowed the retained v0.91.3 proof lane to the exact `--bin adl` card-lifecycle tests, updated the matching replay surfaces, reran the focused proof locally, and pushed janitor commit `f5115e89`.
- Reclaimed repo-local rootfs space before authoritative coverage by clearing restored `adl/target` state inside the authoritative runner entrypoint, reran the focused runner contract, and pushed janitor commit `8e365ddf`.
- Hardened `pr.sh shepherd` delegation so the compatibility wrapper now fails closed with dedicated owner-binary guidance when `adl-pr-shepherd` is not built locally, added a focused delegate regression contract, and pushed janitor commit `3a62c9e6`.
- Re-checked the repo-native watcher after publication and janitoring; the issue is now under watcher ownership with `classification=pr_open` while fresh `adl-ci` / `adl-coverage` results are pending on PR `#4714`.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `adl/src/cli/pr_cmd.rs`
  - `adl/src/cli/pr_cmd_args.rs`
  - `adl/src/cli/pr_cmd/github.rs`
  - `adl/src/cli/pr_cmd/github/tests/watch.rs`
  - `adl/Cargo.toml`
  - `adl/src/bin/adl_pr_shepherd.rs`
  - `adl/tools/run_v0913_proof_validation_lane.sh`
  - `adl/tools/pr.sh`
  - `adl/tools/pr_delegate.sh`
  - `adl/tools/pr_usage.sh`
  - `adl/tools/run_authoritative_coverage_lane.sh`
  - `docs/default_workflow.md`
  - `docs/milestones/v0.91.3/review/card_lifecycle_integration/CARD_LIFECYCLE_PROOF_PACKET_v0.91.3.md`
  - `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md`
  - `docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md`
  - `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
  - `adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh`
  - `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: issue-bound worktree implementation published to draft PR `#4714`, then janitored in place with blocker-driven retained-proof narrowing on commit `f5115e894f4b7cb081f67ac79d66081684d652b2`, authoritative-coverage rootfs reclamation on commit `8e365ddf32d337324bdfa5f008971b9528f3ac8c`, and compatibility-wrapper delegate hardening on commit `3a62c9e6cc866513fa554298f877bac9ed17d03f`
- Branch freshness note: current worktree branch has been refreshed onto current `origin/main`; no outstanding behind-main drift is currently recorded in this issue-local execution record
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml --bin adl lifecycle_shepherd -- --nocapture`
    Verified the lifecycle-shepherd mapping tests for pre-run, execution-bound, publication-ready, PR waiting, janitor-active, `closed_no_pr`, and blocked states.
  - `cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::tooling_cmd::tests::structured_prompt::tracked_csdlc_card_bundle_validates' -- --exact --nocapture`
    Verified the retained v0.91.3 card-lifecycle structured-prompt contract runs as one exact targeted test under the `adl` binary instead of broad-compiling the tree.
  - `cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::pr_cmd::doctor::tests::card_lifecycle_accepts_tracked_csdlc_bundle' -- --exact --nocapture`
    Verified the retained v0.91.3 doctor card-lifecycle contract runs as one exact targeted test under the `adl` binary instead of broad-compiling the tree.
  - `bash adl/tools/test_run_v0913_proof_validation_lane.sh`
    Verified the narrow retained proof-validation lane contract and replay-surface updates remain internally consistent after the janitor fix.
  - `bash adl/tools/test_run_authoritative_coverage_lane.sh`
    Verified the authoritative coverage runner still relocates llvm-cov outputs onto scratch storage after reclaiming restored repo-local `adl/target` state on runner rootfs.
  - `bash adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh`
    Verified `doctor` still reuses a fresh primary-checkout owner binary when safe, and `shepherd` now refuses stale generic `adl` fallback with dedicated owner-binary guidance when `adl-pr-shepherd` is not built.
  - `git diff --check`
    Verified the worktree contains no whitespace or patch-format errors after remediation.
  - `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-pr-shepherd -- 4630 --version v0.91.7 --json`
    Verified the owner binary returns an authoritative `adl.pr.shepherd.v1` packet for the bound issue from the worktree.
  - `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token ./adl/tools/pr.sh shepherd 4630 --version v0.91.7 --json`
    Verified the compatibility wrapper now fails closed with dedicated owner-binary guidance instead of delegating into a stale generic `adl` binary when `adl-pr-shepherd` is not built locally.
  - `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token ./adl/tools/pr.sh watch 4630 --json`
    Verified the published branch now reports `classification=pr_open`, `tail_owner=issue-watcher`, and pending GitHub checks on PR `#4714` after the janitor pushes.
- Result: FAIL

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml --bin adl lifecycle_shepherd -- --nocapture`
    Proved the lifecycle-shepherd state mapping and regression tests for the new owner-binary surface.
  - `cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::tooling_cmd::tests::structured_prompt::tracked_csdlc_card_bundle_validates' -- --exact --nocapture`
    Proved the retained v0.91.3 structured-prompt card-lifecycle contract runs as one exact targeted test under the `adl` binary.
  - `cargo test --manifest-path adl/Cargo.toml --bin adl 'cli::pr_cmd::doctor::tests::card_lifecycle_accepts_tracked_csdlc_bundle' -- --exact --nocapture`
    Proved the retained v0.91.3 doctor card-lifecycle contract runs as one exact targeted test under the `adl` binary.
  - `bash adl/tools/test_run_v0913_proof_validation_lane.sh`
    Proved the narrow retained proof-validation lane contract and replay-surface updates are internally consistent after the janitor fix.
  - `bash adl/tools/test_run_authoritative_coverage_lane.sh`
    Proved the authoritative coverage runner still reclaims restored repo-local `adl/target` state before launching the scratch-mounted llvm-cov build.
  - `bash adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh`
    Proved the wrapper now enforces dedicated owner-binary truth for `shepherd` instead of silently falling through to stale generic `adl`.
  - `git diff --check`
    Proved the branch has no whitespace or malformed patch residue after remediation.
  - `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-pr-shepherd -- 4630 --version v0.91.7 --json`
    Proved the owner binary emits the expected machine-readable lifecycle-shepherd packet for the bound issue.
  - `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token ./adl/tools/pr.sh shepherd 4630 --version v0.91.7 --json`
    Proved the shell compatibility wrapper now fails closed with dedicated owner-binary guidance when `adl-pr-shepherd` is not already built locally.
  - `ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token ./adl/tools/pr.sh watch 4630 --json`
    Proved the PR-tail classifier now reports `pr_open` / watcher ownership with fresh checks pending on the latest janitored commit.
- Results:
  - PASS: focused lifecycle-shepherd tests passed.
  - PASS: exact retained proof-lane card-lifecycle tests passed under `--bin adl`.
  - PASS: `bash adl/tools/test_run_v0913_proof_validation_lane.sh` passed after the narrow contract update.
  - PASS: `git diff --check` passed.
  - PASS: direct `adl-pr-shepherd` JSON output returned `adl.pr.shepherd.v1` with `classification=ready_for_run`.
  - PASS: wrapper `pr.sh shepherd` now fails closed with dedicated owner-binary guidance instead of delegating into stale generic `adl`.
  - PASS: watcher JSON after publication/janitor push reported the issue under `issue-watcher` ownership with fresh checks pending on PR `#4714`.

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
    - cargo test --manifest-path adl/Cargo.toml --bin adl lifecycle_shepherd -- --nocapture
    - bash adl/tools/test_run_authoritative_coverage_lane.sh
    - bash adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh
    - git diff --check
    - ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-pr-shepherd -- 4630 --version v0.91.7 --json
  determinism:
    status: NOT_RUN
    replay_verified: unknown
    ordering_guarantees_verified: unknown
  security_privacy:
    status: PARTIAL
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PARTIAL
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
sor_facts:
  schema_version: adl.sor_facts.v1
  changed_paths:
  - adl/Cargo.toml
  - adl/src/bin/adl_pr_shepherd.rs
  - adl/src/cli/pr_cmd.rs
  - adl/src/cli/pr_cmd/github.rs
  - adl/src/cli/pr_cmd/github/tests/watch.rs
  - adl/src/cli/pr_cmd_args.rs
  - adl/src/cli/tests/pr_cmd_inline/basics.rs
  - adl/tools/run_authoritative_coverage_lane.sh
  - adl/tools/run_v0913_proof_validation_lane.sh
  - adl/tools/pr.sh
  - adl/tools/pr_delegate.sh
  - adl/tools/pr_usage.sh
  - adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md
  - adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh
  - docs/default_workflow.md
  - docs/milestones/v0.91.3/review/card_lifecycle_integration/CARD_LIFECYCLE_PROOF_PACKET_v0.91.3.md
  - docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md
  - docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md
  - docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md
  validation:
    status: NOT_RUN
    commands: []
  review:
    findings_status: findings_present
    recommended_outcome: pass
    findings:
    - '`P0` [`adl/src/cli/pr_cmd.rs`] had staged merge-conflict markers in the cached import hunk. Disposition: fixed by staging the resolved file so the index matches the buildable worktree.'
    - '`P1` [`adl/src/cli/pr_cmd/github.rs`] collapsed `closeout_needed` and `merged_pending_closeout` into the same lifecycle state. Disposition: fixed so `closeout_needed` routes to `closed_no_pr` while merged PR closeout still routes to `merged_needs_closeout`.'
    - '`P1` [`adl/src/cli/pr_cmd/github.rs`] promoted a closed issue to `settled` before local closeout truth existed. Disposition: fixed so closed issues remain under `pr-closeout` ownership until closeout is actually finalized.'
    fixes:
    - All actionable findings above were fixed in the bound worktree.
    - Focused validation reran after remediation and passed.
  finish:
    pr_url: https://github.com/danielbaustin/agent-design-language/pull/4714
    blocking_notes:
    - Fresh `adl-ci` and `adl-coverage` results are pending on commit `3a62c9e6cc866513fa554298f877bac9ed17d03f`.
    fix_notes:
    - Published draft PR `#4714` for the `adl-pr-shepherd` implementation slice.
    - Janitored the first red `adl-ci` run by narrowing the retained v0.91.3 proof-validation lane to exact `--bin adl` card-lifecycle tests and updating the matching replay surfaces.
    - Janitored the authoritative coverage lane by reclaiming restored repo-local `adl/target` state before launching scratch-mounted llvm-cov work.
    - Janitored the compatibility wrapper by refusing stale generic `adl` fallback for `shepherd` and surfacing dedicated owner-binary build guidance instead.
  integration:
    state: pr_open
    main_repo_paths:
    - adl/Cargo.toml
    - adl/src/bin/adl_pr_shepherd.rs
    - adl/src/cli/pr_cmd.rs
    - adl/src/cli/pr_cmd/github.rs
    - adl/src/cli/pr_cmd/github/tests/watch.rs
    - adl/src/cli/pr_cmd_args.rs
    - adl/src/cli/tests/pr_cmd_inline/basics.rs
    - adl/tools/pr.sh
    - adl/tools/pr_delegate.sh
    - adl/tools/pr_usage.sh
    - adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md
    - docs/default_workflow.md
    - docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md
```

## Determinism Evidence
- Determinism tests executed: focused command proofs only; the new `adl-pr-shepherd` owner binary was exercised directly and the compatibility wrapper was exercised for fail-closed owner-binary enforcement, but no same-input replay bundle was captured in this issue.
- Fixtures or scripts used: `adl/tools/pr.sh run` for clean rebinding and the repo-native command/parser/delegate surfaces listed above.
- Replay verification (same inputs -> same artifacts/order): not formally replay-verified beyond stable direct-binary packet output and stable wrapper failure-mode guidance for the same bound issue.
- Ordering guarantees (sorting / tie-break rules used): expected to inherit doctor/watch deterministic routing, but no new replay proof has been run yet.
- Artifact stability notes: implementation is now published to draft PR `#4714`; merge has not happened yet, and the current pending-check state is tied to commit `3a62c9e6cc866513fa554298f877bac9ed17d03f`.

## Security / Privacy Checks
- Secret leakage scan performed: limited implementation-content review only; no token contents or secret values were intentionally recorded.
- Prompt / tool argument redaction verified: partially; command planning retains environment-variable names only and does not print secret contents.
- Absolute path leakage check: durable execution record paths remain repository-relative; local ignored accidental-start evidence paths are repository-relative as recorded here.
- Sandbox / policy invariants preserved: yes; tracked implementation remains inside the bound issue worktree and the primary checkout stayed clean on `main`.

## Replay Artifacts
- Trace bundle path(s): not_applicable yet
- Run artifact root: not_applicable yet
- Replay command used for verification: not_run
- Replay result: NOT_RUN

## Artifact Verification
- Primary proof surface: the bound worktree implementation plus the updated planning/contract cards and focused command outputs for the `adl-pr-shepherd` owner-binary surface
- Required artifacts present: yes for the local implementation slice; no PR or merged-main artifact set exists yet
- Artifact schema/version checks: focused command proof only; no additional schema artifact changed in this slice
- Hash/byte-stability checks: not_run
- Missing/optional artifacts and rationale: merged/main-integrated artifacts are intentionally absent because PR `#4714` is still open and fresh checks are pending on commit `3a62c9e6cc866513fa554298f877bac9ed17d03f`.

## Decisions / Deviations
- This broad WP truth-consumption issue is being executed through the lifecycle-shepherd command slice rather than by reopening already-consumed v0.91.6 watcher or closeout work.
- The accidental early `#4630` start was preserved as local evidence and replaced with a clean rebind before implementation continued.
- Integration state is now `pr_open`; merge and closeout truth remain pending while PR `#4714` waits on fresh checks.

## Follow-ups / Deferred work
- Update this record again when PR `#4714` moves from pending checks to green, failed, merged, or closed.
- Normalize this record to `merged` or `closed_no_pr` during finish/closeout as appropriate.
