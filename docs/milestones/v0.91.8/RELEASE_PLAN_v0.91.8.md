# v0.91.8 Release Plan

## Release Posture

`v0.91.8` is not released by this planning package. Release requires merged
implementation issues, exact-revision acceptance evidence, review, remediation,
and lifecycle closeout.

## Gates

1. Architecture and denominator approval.
2. Characterization and parity corpus acceptance.
3. ADL v2 implementation proof.
4. Runtime v3 adapter and deployment proof.
5. C-SDLC v2 lifecycle deployment proof.
6. Rollback and reversible selector proof.
7. Deletion eligibility and post-deletion validation.
8. WP-14A acceptance and deployment.
9. Demo, quality, docs, internal/external review, remediation, and preflight.
10. WP-21 exact-revision handoff and release ceremony closeout.

The release-tail review sequence must remain `WP-21 -> WP-21A -> WP-22` before
release ceremony: WP-21 aligns feature-list and planning truth, future v0.91.8
WP-21A `#5355` prepares next-milestone handoff/review alignment, and WP-22
reviews that packet. Current v0.91.7 WP-21A `#5489` only prepares this
documentation package and does not execute v0.91.8 work.

The release plan must consume current blocker/non-claim truth explicitly:
`#5408` is closed/remediated via PR #5419, while #4906 remains retained
blocked-with-evidence unless separately dispositioned.

## Rollback

Rollback must restore the previous generation selector and stable binary path
state. The release cannot rely on Cargo target directories or local build cache
state as operational truth.
