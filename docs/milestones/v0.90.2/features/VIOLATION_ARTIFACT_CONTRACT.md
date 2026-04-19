# Violation Artifact Contract

## Purpose

Make invariant failures stable, reviewable, and useful for demos and release
evidence.

## Minimum Fields

- artifact version
- invariant id
- attempted action
- actor or test harness
- kernel stage
- trace anchor
- decision
- resulting state
- recovery eligibility
- quarantine requirement

## Required Proof

At least one violation artifact should be generated from a real negative test
and checked for stable shape.
