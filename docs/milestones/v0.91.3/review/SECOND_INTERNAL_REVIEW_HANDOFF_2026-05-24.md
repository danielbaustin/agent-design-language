# v0.91.3 Second Internal Review Handoff

## Status

`active_remediation_handoff`

## Purpose

This handoff records the replayable public map for the second internal-review
repair pass. It exists because some review-control material is intentionally
kept under ignored `.adl/reviews/` paths, while third-party reviewers need a
tracked surface that explains what is available, what is historical, and what
must not be treated as publication proof.

## Source-Backed State

- `WP-13` / `#3208` opened the v0.91.3 internal-review cycle.
- v0.91.3 is no longer pre-WP-13 in root documentation.
- The second internal-review remediation wave is active before external review.
- The retained WP-13 review lane error register is
  `.adl/reviews/v0.91.3/internal/wp-13/REVIEW_LANE_ERRORS.md`.
- The tracked public review evidence for milestone claims remains under
  `docs/milestones/v0.91.3/review/`.

## Non-Replayable Or Local-Only Surfaces

The following paths were referenced by issue `#3327` as expected review-control
inputs, but they are not present in this bound worktree:

- `.adl/reviews/v0.91.3/internal/pass-2/FINDINGS_REGISTER.md`
- `.adl/reviews/v0.91.3/internal/pass-2/ERROR_REGISTER.md`
- `.adl/reviews/v0.91.3/internal/pass-2/SECOND_INTERNAL_REVIEW_PLAN.md`

These paths must therefore not be cited as durable proof in tracked milestone
documentation unless a later issue creates or restores them intentionally.

## Reviewer Guidance

- Treat ignored `.adl/reviews/` material as local review-control state unless a
  tracked handoff says otherwise.
- Treat `docs/milestones/v0.91.3/review/` as the durable public review evidence
  namespace.
- Do not synthesize review-lane errors into product findings unless the error
  is independently confirmed against the reviewed baseline.
- Keep root docs honest about current milestone phase: v0.91.3 is in
  post-WP-13 internal-review remediation, not waiting for internal review to
  begin.

## Remaining Blockers

- This handoff does not claim external review readiness.
- This handoff does not replace the pass-2 review plan if that plan is later
  restored as a durable `.adl/reviews/` control artifact.
- Any future external-review packet should cite this tracked handoff when
  explaining local-only pass-2 review-control paths.
