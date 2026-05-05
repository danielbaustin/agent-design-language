# WP-22 Internal Review - v0.90.5

## Status

Completed internal review record for `WP-22` / `#2587`.

This review is findings-first and evidence-bound. It does not approve release
readiness, replace external review, or absorb remediation.

## Scope Reviewed

- milestone reviewer-entry docs
- demo/proof surfaces
- quality-gate and release-tail truth
- current issue/PR dependency posture through WP-21

Primary evidence:

- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/MILESTONE_CHECKLIST_v0.90.5.md`
- `docs/milestones/v0.90.5/QUALITY_GATE_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_PLAN_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_NOTES_v0.90.5.md`
- `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md`
- merged issue / PR truth through `WP-21`

## Executive Judgment

`v0.90.5` has a real, reviewer-facing governed-tools proof package and the
direct dependency chain through `WP-21` is landed. The milestone is not
release-ready. The current `main` coverage exception remains open, and the
review-tail gates after internal review are still incomplete by design.

The package is strong enough to proceed to `WP-23` external review, provided
the release-tail truth remains explicit and the current quality exception is not
smoothed over.

## Findings

### P1: Authoritative main-branch coverage gate is still red

Evidence:

- `docs/milestones/v0.90.5/QUALITY_GATE_v0.90.5.md` records push-to-main run
  `25272620889` with `adl-ci` green and `adl-coverage` failing at coverage
  policy enforcement.

Why it matters:

The milestone cannot be described as release-ready while the canonical
authoritative coverage posture on `main` is explicitly red.

Recommended route:

- carry as an explicit release-tail blocker into `WP-24`
- keep the exception visible in `WP-23` external review materials

### P2: Public-spec / privacy gate items are still open and need explicit review disposition

Evidence:

- `docs/milestones/v0.90.5/MILESTONE_CHECKLIST_v0.90.5.md` still leaves these
  review gates unchecked:
  - public-spec language checked for overclaiming
  - UTS validity never described as execution authority
  - privacy and redaction claims backed by tests or explicit deferrals

Why it matters:

The milestone already has strong non-claims, but the checklist truth says these
specific review gates are not yet fully dispositioned. External reviewers
should see that as open review work rather than assumed complete.

Recommended route:

- carry forward into `WP-23` as named review prompts
- route accepted follow-up work into `WP-24`

## No-Finding Notes

- The direct dependency gate for `WP-22` is satisfied: `WP-21` / `#2586` is
  closed and PR `#2702` is merged.
- The demo/proof package is explicit and reviewer-facing rather than hidden in
  implementation-only surfaces.
- Release notes now read as milestone-grounded with a clear release-tail caveat
  instead of pretending the milestone is already complete.

## Review Outcome

- internal review complete: `yes`
- release ready: `no`
- proceed to external review: `yes`
- remediation still required: `yes`
