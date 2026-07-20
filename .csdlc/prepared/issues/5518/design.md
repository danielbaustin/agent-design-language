# Atomic terminal plan-step repair

## Problem

A closed-out issue can retain a stale SPP step even when its terminal SOR and
receipt prove that the action completed. Direct card edits would invalidate
the record and receipt digests.

## Design

Add one typed cross-issue repair operation that requires an active authority
claim and a closed-out, claim-free target. The operation may only advance one
existing SPP step from `pending` or `in_progress` to `completed`. It refreshes
all card projections, appends an audit event, recomputes the target digest, and
refreshes the terminal receipt atomically with rollback on failure.

## Boundaries

- No general terminal card editing.
- No historical audit rewriting.
- No runtime or Runtime v2 source changes.
- No AWS execution.
