# #5468 Terminal SRP Status Reconciliation

## Problem

Terminal reconciliation projects retained review evidence into a durable
`closed_out` issue, but it preserves a stale `pre_phase` SRP card status. The
result contradicts the same card's completed passing review evidence.

## Design

Extend the existing typed `reconcile_terminal` transaction to normalize only
the SRP status when terminal evidence proves the issue is closed out and the
retained review evidence is completed. A passing review or a completed review
whose findings are all dispositioned projects SRP status `complete`; unresolved
review truth remains fail closed under existing card validation.

The normalization occurs before cross-card validation and receipt refresh, so
the issue projection and retained receipt advance atomically under the existing
generation, digest, audit, and rollback transaction.

## Boundaries

- No arbitrary post-closeout card mutation.
- No SOR follow-up behavior changes.
- No runtime, AWS, CI, or workflow changes.
- #5452 is regenerated only after the focused typed-v2 regression passes.
