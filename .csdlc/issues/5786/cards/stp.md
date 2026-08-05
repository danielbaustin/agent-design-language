# Structured Task Prompt

Template: 1.0.0

Issue: 5786

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver behavior-preserving cleanup with exact deletion denominator.

## Deliverables

- behavior-preserving cleanup with exact deletion denominator
- reviewed deletion manifest, parity proof, and retained-surface inventory

## Acceptance

1. AC-1: The execution SHA proves WP-20 ancestry, rollback-window expiry, rollback disposition, selector health, and a clean supported install before deletion.
2. AC-2: Every remaining adl/src file and active reverse reference has exactly one accountable disposition with replacement evidence or a timed owner/reason/expiry exception.
3. AC-3: The supported adl command resolves to the thin ADL v2 CLI; Runtime v2 and obsolete compatibility routes are absent only after positive, negative, artifact, trace, failure, and rollback parity.
4. AC-4: Exact physical LoC/file accounting against the pinned 355,675-line baseline proves at least 80% deletion; 80-89% retention has owner, reason, and expiry, and below 80% is failure.
5. AC-5: Focused replacement, clean-install, reverse-reference, macOS, Linux, and rollback proof passes at the exact deletion head without unsupported behavior loss.
6. AC-6: One exact-head independent review has no unresolved actionable finding and the PR uses Closes #5786 without claiming WP-21A or release completion.

## Dependencies

- WP-20

## Inputs

- Live issue #5786 objective, rollback gate, baseline, and acceptance criteria
- docs/milestones/v0.92/WBS_v0.92.md and docs/milestones/v0.92/QUALITY_GATE_v0.92.md
- docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md
- docs/milestones/v0.91.8/RUNTIME_V3_FUNCTIONAL_PARITY_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json and 5346-post-deletion-validation.v1.json

## Non Goals

- Deleting before 2026-08-12T09:04:24Z or while rollback/selector health is unresolved
- Treating crate movement, build exclusion, or unowned compatibility retention as reduction
- WP-21A refactoring, WP-22 gate approval, new features, or downstream release work
