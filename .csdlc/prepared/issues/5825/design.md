# Issue 5825 Design: Birthday Contract And Negative Cases

## Outcome And Sources

Define the deterministic WP-08 birth decision consumed by the Birthday sprint. The contract is grounded in `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`, the negative suite in `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`, and the WP-08 row in `docs/milestones/v0.92/WBS_v0.92.md`.

## Owned Surface

Implementation may update the WP-08 feature contract and add an issue-local birthday contract/schema, valid fixture, negative fixtures, validator, and retained validation report. Candidate protected paths are `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`, `adl/src/runtime_v2/`, `adl/tests/fixtures/runtime_v2/birthday/`, `adl/src/runtime_v2/tests/`, and `.csdlc/evidence/5825/`; exact new module names must be narrowed before editing shared Runtime files.

## Contract

A birth result requires stable name and identity root, continuity head across bounded cycles, redaction-safe memory grounding, capability envelope, bounded ACP evidence, inherited moral context, witness set, receipt, and reviewer-visible validation. Process startup, task execution, snapshot, wake/resume, restore, admission, copied state, dormant rehydration, simulation, migration, suspension, restart, provisional citizenship, or any packet missing a required evidence surface must return a stable rejection reason.

## Dependencies And Invariants

WP-01/#5818 and WP-02A/#5819 must be terminally proven before execution. The decision is deterministic over canonical inputs, fails closed on missing or contradictory evidence, and never upgrades existing v0.91.x birthday non-claims by implication.

## Validation And Rollback

Focused schema/fixture tests prove one valid packet. A table-driven negative lane proves every listed disqualifier and missing-evidence case. A claim-boundary scan rejects personhood, consciousness, production citizenship, governance, migration, and transport overclaims. Rollback removes the new contract/fixtures and restores the feature doc without altering historical evidence.

## Non-Goals

WP-09 identity construction, WP-10 continuity implementation, WP-11 memory behavior, WP-12 capability production, WP-13 profiles, WP-15 witnesses, WP-16 packet assembly, public launch, and v0.93 governance are outside this issue.
