# v0.91.6 Review And Validation Checklist

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Setup issue: `#3800`

## Status

Candidate review checklist for docs/planning and first-tranche bridge work.

## Required Local Validation

For planning-doc-only changes:

- `git diff --check`
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
