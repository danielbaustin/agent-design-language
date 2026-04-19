# Invariant And Security Boundary

## Purpose

Define the invariant evidence and one security-boundary proof required for
Runtime v2 foundation review.

## Required Invariant Proof

The prototype should intentionally attempt one illegal state transition, reject
it, and emit a violation artifact.

Candidate violations:

- duplicate active citizen id
- episode execution while paused
- wake from invalid snapshot
- operator resume before invariant check

## Required Security Proof

The prototype should attempt one invalid action through the normal kernel/policy
path and prove it is refused.

The security proof must include:

- actor
- attempted action
- evaluated policy/invariant
- refusal reason
- trace ref
- resulting state

## Boundary

This is safety evidence for the polis. It is not the full red/blue/purple
security ecology and should not distort the Runtime v2 core thesis.
