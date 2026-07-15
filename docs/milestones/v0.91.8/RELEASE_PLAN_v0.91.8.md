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
8. WP-14A acceptance/deployment handoff.
9. Demo, quality, docs, internal/external review, remediation, and preflight.
10. Release ceremony closeout.

## Rollback

Rollback must restore the previous generation selector and stable binary path
state. The release cannot rely on Cargo target directories or local build cache
state as operational truth.

