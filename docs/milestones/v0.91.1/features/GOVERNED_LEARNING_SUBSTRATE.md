# Governed Learning Substrate

## Metadata

- Feature Name: Governed Learning Substrate
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-10
- Source Docs: `.adl/docs/TBD/learning_model/`
- Proof Modes: fixtures, policy, review

## Purpose

Bound learning updates, feedback, adaptation, and rollback under explicit
policy. ADL should learn from evidence without allowing hidden self-modification
or unreviewable drift.

## Scope

In scope:

- Learning update and feedback contract.
- Policy boundary for adaptation and rollback.
- Fixtures for accepted feedback, rejected feedback, and unsafe update claims.
- Integration with capability, intelligence, and ANRM evidence.

Out of scope:

- Autonomous retraining.
- Hidden model mutation.
- Grand unified learning theory.

## Acceptance Criteria

- Unsafe or hidden learning claims fail closed.
- Accepted learning updates preserve evidence and rollback references.
- Reviewers can inspect what changed and why.
