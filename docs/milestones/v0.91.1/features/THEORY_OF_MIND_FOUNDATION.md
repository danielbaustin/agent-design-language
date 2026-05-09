# Theory Of Mind Foundation

## Metadata

- Feature Name: Theory of Mind Foundation
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-08
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

## Landed Artifacts

- `adl/src/runtime_v2/theory_of_mind_foundation.rs`
- `adl/src/runtime_v2/tests/theory_of_mind_foundation.rs`
- `adl/tests/fixtures/runtime_v2/theory_of_mind/theory_of_mind_foundation.json`

## Proof Notes

- Shares the `memory_tom_evidence_demo` route with WP-07 so memory and ToM stay evidence-bound together.
- Preserves correction, unknown, and privacy-restricted fixtures without claiming mind-reading.
