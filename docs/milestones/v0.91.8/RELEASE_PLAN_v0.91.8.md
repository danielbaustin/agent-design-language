# v0.91.8 Release Plan

## Release Posture

`v0.91.8` is not released by this documentation package. WP-16 merged at
`2e9d2dd7c4260dcf6ec6af954b0eea97554212df` and records an integrated platform
quality-gate pass. WP-17 documentation alignment and both WP-18 internal
reviews are closed. Final release still requires formal external review,
remediation, next-milestone
handoff/review, and release ceremony work.

## Gates

1. Architecture and denominator approval.
2. Characterization and parity corpus acceptance.
3. ADL v2 implementation proof.
4. Runtime v3 adapter and deployment proof.
5. C-SDLC v2 lifecycle deployment proof.
6. Rollback and reversible selector proof.
7. Deletion eligibility and post-deletion validation.
8. WP-14A acceptance and deployment.
9. Demo and integrated quality gate. WP-16 is merged at `2e9d2dd7c` with 67
   audited issues, 0 unacceptable outcomes, 0 release blockers, and focused,
   integrated, and complete lanes passing.
10. WP-17 documentation and release-truth alignment.
11. WP-18 first internal milestone review.
12. WP-18 final second pass `#5791` after residual coding.
13. WP-19 independent external review.
14. WP-20 remediation and release preflight.
15. WP-21 exact-revision v0.92 handoff ledger.
16. WP-21A next-milestone closeout plan.
17. WP-22 next-milestone planning review.
18. WP-23 release ceremony and lifecycle closeout.

The release-tail review sequence must preserve closed WP-17 documentation
alignment, closed first-pass WP-18 review, final WP-18 second-pass review after
residual coding, WP-19 external review, WP-20 remediation/preflight, WP-21
next-milestone handoff, WP-21A closeout planning, WP-22 planning review, and
WP-23 release ceremony. Current
v0.91.7 WP-21A `#5489` is historical preparation evidence and does not execute
v0.91.8 work.

The release plan must consume current blocker/non-claim truth explicitly:
`#5408` is closed/remediated via PR #5419, while #4906 remains retained
blocked-with-evidence unless separately dispositioned.

## Rollback

Rollback must restore the previous generation selector and stable binary path
state. The release cannot rely on Cargo target directories or local build cache
state as operational truth.

## Current Non-Claims

- Final `v0.91.8` release approval is not claimed.
- Both WP-18 reviews are complete; formal third-party milestone review is not
  claimed complete.
- v0.92 birthday activation is not claimed.
- Partial or ambiguous release-tail, umbrella, and lifecycle-drift items
  recorded by WP-16 remain explicit limitations unless later evidence closes
  them.
