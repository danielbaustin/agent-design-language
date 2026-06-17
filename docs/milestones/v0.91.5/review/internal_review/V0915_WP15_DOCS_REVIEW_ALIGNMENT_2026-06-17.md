# v0.91.5 WP-15 Docs And Review Alignment

Date: `2026-06-17`
Issue: `#3579`
Sprint: `#3574`

## Summary

WP-15 aligns the reviewer-facing milestone surfaces to the actual Sprint 4
frontier after WP-14 and the completed first remediation tranche.

This packet does not perform second-pass internal review, external review,
final preflight, or release ceremony. It only makes those later steps easier to
start from tracked docs instead of chat reconstruction.

## Source Evidence

- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP15_INPUT_FROM_GAP_REVIEW_2026-06-17.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md`
- live Sprint 4 issue truth for `#3574`, `#3575`, `#3579`, `#3576`, `#3580`,
  `#3577`, `#3581`, `#3578`, and `#3923`

## Alignment Changes

The following release-facing docs were normalized from opening-era or earlier
mid-milestone status language to the current Sprint 4 release-tail posture:

- `docs/milestones/v0.91.5/README.md`
- `docs/milestones/v0.91.5/RELEASE_PLAN_v0.91.5.md`
- `docs/milestones/v0.91.5/RELEASE_NOTES_v0.91.5.md`
- `docs/milestones/v0.91.5/VISION_v0.91.5.md`
- `docs/milestones/v0.91.5/DESIGN_v0.91.5.md`
- `docs/milestones/v0.91.5/DECISIONS_v0.91.5.md`
- `docs/milestones/v0.91.5/features/README.md`
- `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md`
- `docs/milestones/v0.91.5/features/V092_ACTIVATION_READINESS_v0.91.5.md`

## What Is Now Explicit

- Sprint 1 through Sprint 3 delivery is materially landed.
- First-pass internal review already happened.
- The first remediation wave closed through `#3899`.
- Sprint 4 is the active release-tail frontier.
- WP-14 is the applied quality gate and remains blocked for release.
- WP-15 is docs/review alignment.
- Second-pass internal review remains downstream and should not be inferred as
  already complete.
- The retired shell-helper closed-issue audit still lacks a repo-native
  replacement lane.

## Reviewer Entry Points

Reviewers should be able to orient from these tracked surfaces without
reconstructing milestone history from chat:

- milestone README
- sprint plan
- quality gate
- first internal-review findings register
- WP-14 quality-gate application packet
- second-pass internal-review handoff plan
- issue wave / WBS / activation map / demo matrix

## Residual Gaps

WP-15 intentionally does not solve:

- second-pass internal review execution
- external review
- final remediation and v0.92 preflight
- release ceremony
- the missing repo-native replacement for the retired shell-helper closeout
  audit path

## Bottom Line

After WP-15, the release-tail docs should tell one coherent story: v0.91.5 is
substantial, not hollow; still open, not release-ready; and already past the
first review/remediation cycle rather than stuck in WP-01-era planning language.
