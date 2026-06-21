# ADL CI Validation Cost Review

Status: `review_packet`
Date: 2026-06-21
Issue: `#4322`
Milestone: `v0.91.6`

This packet reviews the current `adl-ci` workflow and adjacent validation
selection surfaces, then records the bounded CI fixes implemented under
`#4322`. The goal is to reduce validation friction without weakening release,
proof, or workflow-control safety.

## Summary

The current CI design already has the right safety posture: pull requests are
classified by changed paths, docs-only work can avoid Rust validation, slow
proofs are kept out of ordinary PRs, and full coverage remains available for
push, schedule, workflow-dispatch, and fail-closed situations.

The main cost problem is that several different checks are still routed through
one broad switch, `ci_contracts_required`. When that switch is true, the
workflow runs CI policy contracts, proof-lane contracts, skill contract checks,
runtime budget checks, cache/linker checks, and selected dataset tooling checks
as one bundle. That is safe, but it is increasingly expensive and makes it hard
to know which check actually proves the changed surface.

The best next step is to converge `ci.yaml` on the existing validation-manager
and lane-selector manifest, then split the wide contract bundle into named
contract outputs. Do not replace fail-closed behavior with ad hoc string
skips.

## Scope Reviewed

- `.github/workflows/ci.yaml`
- `.github/workflows/nightly-coverage-ratchet.yaml`
- `adl/tools/ci_path_policy.sh`
- `adl/tools/check_coverage_impact.sh`
- `adl/tools/run_pr_fast_test_lane.sh`
- `adl/tools/validation_manager.py`
- `adl/config/validation_lane_selector.v0.91.6.json`
- related retained review packets:
  - `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md`
  - `docs/milestones/v0.91.6/review/V0916_BUILD_THROUGHPUT_MINI_SPRINT_REVIEW_4310.md`
  - `docs/milestones/v0.91.6/review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md`

## Current Job And Step Inventory

| Job / step group | Purpose | Owner | Trigger / gate | Cost evidence available | Recommendation |
|---|---|---|---|---|---|
| `adl-ci` path classification | Classify changed paths and emit validation booleans. | tools | Every CI event. | Deterministic shell/Python policy; no fresh Actions timing collected in this issue. | Keep mandatory. Make the validation-manager profile output the primary source of lane identity. |
| Rust toolchain, cache, `sccache`, `lld`, acceleration | Prepare Rust validation when needed. | tools | `rust_required == true`. | Build-throughput review recorded local child validation seconds; no per-step Actions timing collected here. | Keep conditional. Add a follow-on to reduce apt dependency for `lld` and report setup time explicitly. |
| Shell syntax and retired wrapper guardrails | Catch broken shell scripts and retired wrapper resurrection. | tools | Always. | Expected tiny deterministic checks. | Keep always. These are cheap workflow safety checks. |
| Coverage-impact and path-policy contracts | Keep path classification and coverage-impact routing from drifting. | tools | Always in `adl-ci`. | Expected small deterministic shell checks. | Keep always until the selector manifest owns them; then keep as named `ci_path_policy_contracts` / `coverage_impact_contracts`. |
| `ci_contracts_required` bundle | Runs PVF, tracked-proof-lane, PR-fast-lane, slow-proof-lane, authoritative-coverage-lane, skill, dataset-tooling, runtime-budget, and cache/linker contract checks. It does not itself run the actual slow-proof shard job or full coverage execution. | tools / review / docs / runtime | Any path classified as requiring CI contracts. | Source shows broad fan-out; no fresh average timing collected. | Split into named outputs so each changed surface pays for only its proving contract. |
| Legacy refs and PR closing linkage guardrails | Preserve migration and issue-linkage invariants. | tools | Always. | Expected small, but linkage check may need GitHub API. | Keep mandatory until the typed Rust/PVF replacement fully lands and is reviewed. Do not remove as a cost shortcut. |
| `fmt`, `clippy`, PR-fast nextest, doc tests | Rust compile and ordinary PR regression proof. | shared Rust / owning area | `rust_required == true` and not covered by full coverage. | User observed high test count; `run_pr_fast_test_lane.sh` narrows many Rust surfaces but can still fall back broad. | Keep. Improve manifest-backed mapping and record selected filter/profile in PR evidence. |
| Demo smoke | Demo/runtime smoke proof. | demos/runtime | `demo_smoke_required == true`. | Expected non-trivial but bounded. | Keep for runtime/demo surfaces; do not run for docs/tooling-only changes. |
| Tracked proof validation lane | Validate v0.91.3 proof surfaces. | proof/review | `v0913_proof_required == true`. | Bounded proof packet validation. | Keep targeted; consider retiring v0.91.3-specific routing only after a replacement proof namespace exists. |
| `adl-slow-proof` | Four-shard slow proof nextest lane. | runtime | Push, schedule, workflow dispatch only. | Known high resource lane by design. | Keep out of ordinary PRs. Preserve as release/nightly proof, not PR-fast proof. |
| `adl-coverage` path classification and PR fast coverage | Produce coverage evidence only when needed. | tools / shared Rust | Every CI event, with expensive steps gated by coverage outputs. | Coverage job has explicit skip/defer branches; no fresh Actions timing collected. | Keep. Remove duplicate full-coverage-deferred messaging and ensure PR policy surfaces are explicit. |
| Full authoritative coverage and coverage gates | Workspace and per-file coverage evidence. | release/tools | Full coverage required; enforcement on non-PR events. | High cost by definition. | Keep as release/push/schedule/fail-closed authority. Avoid running as ordinary docs or narrow tooling proof. |
| Nightly coverage ratchet | Scheduled and manual coverage watchdog/report. | tools | `workflow_dispatch` and daily `schedule`. | High cost by design; no fresh Actions timing collected here. | Keep scheduled now that the workflow name is truthful; review cadence separately if cost becomes noisy. |

## Path-Policy Escalation Audit

The path policy has a healthy fail-closed core:

- Non-PR events become authoritative full coverage.
- Missing or unavailable PR SHAs become authoritative full coverage.
- Empty or unavailable diffs become authoritative full coverage.
- Validation-manager failure becomes authoritative full coverage.

These fail-closed rules should stay.

The weaker part is not fail-closed escalation. It is coarse positive routing.
Examples:

- `mark_pr_fast_coverage` sets `rust_required`, `coverage_required`,
  `demo_smoke_required`, and `ci_contracts_required` together.
- `mark_policy_surface_full_coverage` sets `ci_contracts_required` and
  `full_coverage_required` together for coverage-policy surfaces.
- Any `.github/workflows/*` or `adl/tools/*` fallback can set
  `ci_contracts_required` without distinguishing which contract family proves
  the change.
- `ci.yaml` then interprets `ci_contracts_required` as a single broad bundle.

The validation-manager is already present and can select specific lanes such as
`ci_path_policy_contracts`, `coverage_impact_contracts`, `docs_diff_check`, and
owner lanes. The audit result is therefore:

1. Keep `ci_path_policy.sh` as the fail-closed CI entrypoint.
2. Move from one broad `ci_contracts_required` output to multiple named outputs.
3. Keep release-gate and slow-proof escalation explicit in the manifest.
4. Require every named output to have a deterministic contract test.

## Implemented Fixes

The following bounded fixes were implemented in the same issue after review:

1. `adl-ci` now writes the selected validation profile, status, run lanes, and
   escalation state into the GitHub Actions step summary.
2. `adl-coverage` now writes coverage lane, coverage authority, selected
   profile, run lanes, and escalation state into the GitHub Actions step
   summary.
3. The duplicate full-coverage-deferred PR message was removed from
   `adl-coverage`; the single retained message remains.
4. `nightly-coverage-ratchet` now has an actual scheduled trigger instead of
   being manual-only while named nightly.
5. `ci_path_policy.sh` now treats validation-summary-only workflow edits and
   nightly-schedule-only workflow edits as bounded CI-contract work instead of
   forcing authoritative coverage.
6. Tooling-only CI policy-surface edits now run CI contract validation without
   forcing the expensive authoritative coverage execution; mixed
   runtime-plus-policy changes still escalate to authoritative coverage.
7. `test_ci_runtime_contracts.sh` now guards the summary steps, duplicate
   message removal, and nightly schedule truth.
8. `test_ci_path_policy.sh` now guards the bounded workflow-summary,
   nightly-schedule, tooling-only policy-surface, and mixed runtime-policy
   routing behavior.

## Remaining Quick Wins

1. Split `ci_contracts_required` into named booleans such as
   `ci_path_policy_contracts_required`, `coverage_impact_contracts_required`,
   `proof_lane_contracts_required`, `skill_contracts_required`,
   `runtime_ci_contracts_required`, and `cache_linker_contracts_required`.
2. Stop running unrelated skill contract checks for pure CI-policy changes once
   the split outputs exist.
3. Add explicit timing capture for CI setup groups, especially Rust setup,
   `lld` installation, nextest installation, PR-fast tests, and coverage.

## High-Risk Checks Not To Remove

- Fail-closed full validation for non-PR, missing-SHA, empty-diff, and
  validation-manager failure cases.
- PR closing linkage guardrails until the typed replacement is merged, reviewed,
  and wired as the active gate.
- Legacy reference guardrail while migration residue can still reappear.
- Coverage-impact changed-source gate for Rust source changes.
- Full coverage authority on push-to-main, schedule/workflow-dispatch, and
  fail-closed cases.
- Slow-proof lanes for release/nightly proof. They should remain outside
  ordinary PR-fast validation, not disappear.
- Demo smoke for runtime/demo surfaces.

## Follow-On Issue Candidates

### Candidate 1: Split `ci_contracts_required` into named CI contract outputs

Acceptance:

- `ci_path_policy.sh` emits named contract booleans instead of one broad
  contract switch.
- `ci.yaml` runs only the contract groups selected by those booleans.
- Existing fail-closed behavior remains intact.
- Focused tests prove at least docs-only, CI-policy-only, Rust PR-fast,
  coverage-policy, and slow-proof policy examples.
- Existing `v0913` tracked proof-surface routing remains explicitly preserved
  until a replacement proof namespace is implemented and reviewed.

### Candidate 2: Publish validation-manager profile summaries in PR CI output

Acceptance:

- CI writes a concise profile summary showing selected lanes, skipped lanes,
  escalation state, and reason.
- The profile summary is generated from `validation_manager.py --json`, not from
  hand-maintained duplicate strings.
- PR evidence can cite the selected validation profile without reading raw logs.

### Candidate 3: Add CI runtime budget measurement for setup and validation groups

Acceptance:

- GitHub Actions job/step timing is summarized into a retained artifact or
  check summary.
- Setup time is separated from proof time.
- `lld`, `sccache`, nextest, PR-fast tests, coverage, and contract bundles are
  distinct budget buckets.
- Unknown timing is recorded as unknown, not zero.

### Candidate 4: Replace brittle diff-snippet policy exceptions with manifest rules

Acceptance:

- Reporting-only coverage workflow changes, bounded PR-fast coverage policy
  changes, and slow-proof policy changes are represented in a manifest or
  structured selector rules.
- Existing string-snippet tests remain until parity is proven, then retire.
- The replacement has deterministic fixtures for true positive and true
  negative cases.

### Candidate 5: Clean up coverage workflow duplication and scheduling truth

Acceptance:

- Duplicate PR full-coverage-deferred messaging in `adl-coverage` is removed or
  consolidated.
- `nightly-coverage-ratchet` is either scheduled as named or renamed/documented
  as manual-only.
- Coverage artifact publication boundaries remain visible.

## Recommended Sequencing

1. Do Candidate 1 first. It creates the cost control surface.
2. Do Candidate 2 next. It makes PR validation truth readable.
3. Do Candidate 3 after that. Timing data should verify whether the split is
   improving real CI cost.
4. Do Candidates 4 and 5 as cleanup/hardening once the named outputs are stable.

## Lifecycle Notes

`pr.sh init` and `pr.sh run` for `#4322` were blocked by current lifecycle
tooling behavior around bootstrap `SOR` validation and the open tools PR wave.
The issue cards were structurally ready for design-time execution, and a
bounded issue worktree was created manually on
`codex/4322-review-adl-ci-checks-and-reduce-unnecessary-validation-cost` to
keep tracked work off `main`. This packet records that tooling wrinkle rather
than hiding it.

## Validation

- `git diff --check`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_ci_path_policy.sh`
- bounded 5.4 subagent review of this packet and the CI changes

## Non-Claims

- This issue does not claim current average GitHub Actions runtime by step.
- This issue does not remove any release, proof, security, or linkage gate.
- This issue does not replace the validation-manager agent planned for later
  work.
