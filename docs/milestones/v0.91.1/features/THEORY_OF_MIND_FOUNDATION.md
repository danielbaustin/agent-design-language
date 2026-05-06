# Theory Of Mind Foundation

## Metadata

- Feature Name: Theory of Mind Foundation
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-07
- Source Docs: `.adl/docs/TBD/ToM/`
- Proof Modes: schema, fixtures, review

## Purpose

Introduce bounded agent-model and update-event surfaces so ADL can represent
hypotheses about other agents without claiming mind-reading, bypassing policy,
or exposing private state.

## Scope

In scope:

- Agent-model schema.
- ToM update-event contract.
- Evidence and uncertainty requirements.
- Fixtures for updates, corrections, unknown states, and privacy restrictions.

Out of scope:

- Reputation scoring.
- Consciousness claims.
- Policy override based on inferred mental state.

## Acceptance Criteria

- ToM updates cite evidence or policy-authorized state.
- Uncertainty is preserved.
- ToM can inform review without granting hidden inspection authority.
