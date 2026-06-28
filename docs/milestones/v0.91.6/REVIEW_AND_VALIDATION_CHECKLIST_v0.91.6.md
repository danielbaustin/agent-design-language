# v0.91.6 Review And Validation Checklist

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Setup issue: `#3800`

## Status

Release-tail review checklist for docs/planning and first-tranche bridge work.
WP-11 `#3976`, WP-12 `#3977`, WP-13 `#3978`, and WP-14A `#4582` have merged.
Current docs-truth repair and final-preflight consumers should use the same
checklist while keeping WP-15 failed-review truth and WP-16 remediation state
explicit.

## Required Local Validation

For planning-doc-only changes:

- `git diff --check`
- `python3 adl/tools/check_repo_quality_staleness.py --milestone v0.91.6`
- required-file check for `README.md`, `WBS_v0.91.6.md`,
  `FEATURE_DOCS_v0.91.6.md`, `MILESTONE_CHECKLIST_v0.91.6.md`,
  `REVIEW_AND_VALIDATION_CHECKLIST_v0.91.6.md`, and
  `WP_ISSUE_WAVE_v0.91.6.yaml`
- added-line scan for host-local paths, obvious secret markers, and local
  authoring-workspace links
- required-surface scan for resilience, logging/tooling, public prompt records,
  provider/model, Gemma, ACIP/A2A, protobuf, security, CAV,
  identity/continuity, capability selector, Observatory/Unity, AEE,
  Memory/ObsMem, ACP/cognitive profiles, `#3801`, `#3780`, and `v0.92`

For runtime or tooling changes opened from this milestone:

- use the focused owner lane or exact validation command named by the issue
- if reviewer-facing repo or milestone docs changed, run
  `python3 adl/tools/check_repo_quality_staleness.py --milestone v0.91.6`
  before broader owner-lane proof
- record local proof separately from deferred CI proof
- state whether slow proof is skipped, expected, or required

## Review Questions

- Does the change preserve the `#3778` bridge-ledger contract?
- Does any doc claim `v0.92` readiness without evidence?
- Are first-tranche outputs concrete enough for C-SDLC issue execution?
- Are `v0.91.7` residuals explicit?
- Are security and ACIP/A2A treated as activation-path work?
- Are public prompt records export/redaction/indexing boundaries explicit?
- Does provider/model reliability include Gemma and multi-agent limits?
- Are tooling reliability issues `#3802`-`#3805` fixed or routed?
- Are AEE completion, Memory/ObsMem handoff, and ACP/cognitive profiles
  accounted before `v0.92` activation refresh?

## Finding Dispositions

Allowed dispositions:

- `fixed_in_scope`
- `routed_to_named_issue`
- `accepted_deferred_with_risk`
- `blocked_pending_operator_decision`

Disallowed dispositions:

- vague future work
- hidden runtime implementation inside planning docs
- closing by narrative without tracked evidence

## Closeout Truth

Closeout must record:

- what `v0.91.6` completed
- what `v0.91.6` deferred or blocked
- what `#3801` must consume
- what `#3780` may consume for `v0.92`
- which validations ran locally
- which CI checks were relied on after PR publication

## WP-13 Entry Review

Before internal review starts, WP-13 must leave these entrypoints aligned:

- root `README.md`
- root `CHANGELOG.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `README.md`, `WBS`, `SPRINT_PLAN`, `WP_ISSUE_WAVE`, `MILESTONE_CHECKLIST`,
  `RELEASE_PLAN`, and `RELEASE_NOTES` under `docs/milestones/v0.91.6/`
- `docs/milestones/v0.91.6/review/internal_review/V0916_INTERNAL_REVIEW_PLAN_2026-06-23.md`
- `docs/milestones/v0.91.6/review/V0916_WP12_QUALITY_GATE_3977.md`
- `docs/milestones/v0.91.6/review/sprint_execution_packets/ISSUE_3977_QUALITY_GATE_LIVE_STATE_2026-06-27.md`

Allowed result: aligned docs may still say `v0.91.6` is not release-ready.
They must not say `#3979` is active, `v0.91.5` is active, or that `v0.92`
activation is open.
