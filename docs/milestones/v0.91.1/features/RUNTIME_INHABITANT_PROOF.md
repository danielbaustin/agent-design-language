# Runtime Inhabitant Proof

## Metadata

- Feature Name: Runtime Inhabitant Integration And Observatory Proof
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-15 and WP-16
- Source Docs: `.adl/docs/TBD/v0.91_1_runtime_observatory_dependency_note.md`
- Proof Modes: demo, trace, artifacts, review

## Purpose

Prove that ADL can run an agent-shaped inhabitant inside the CSM/polis boundary
with standing, state, communication, trace, and Observatory projection. This is
the key v0.91.1 closeout proof before v0.92 identity/birthday work begins.

## Scope

In scope:

- Integrated agent-shaped runtime path.
- Runtime state, communication, trace, and Observatory projection artifacts.
- Redaction proof.
- Operator-facing proof report.

Out of scope:

- First true birthday.
- Full autonomy or federation.
- Public exposure of private state.

## Acceptance Criteria

- Demo is deterministic in fixture mode.
- Artifacts are repo-relative and reviewable.

## Landed Surface

- `adl/src/runtime_v2/runtime_inhabitant_integration.rs`
- `adl/src/runtime_v2/tests/runtime_inhabitant_integration.rs`
- `adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_integration.json`
- `adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_operator_report.md`

## Proof Route

- WP-15 lands a bounded runtime inhabitant integration packet plus deterministic
  operator report that bind:
  - standing transition/event mediation
  - citizen state substrate
  - lifecycle state contract
  - memory identity
  - theory-of-mind
  - landed capability harness bundle
  - intelligence metric
  - governed learning
  - access and observatory evidence
  - ACIP hardening and A2A boundary
  - integrated CSM run trace spine
- WP-16 remains the observatory-visible flagship demo follow-on and is not
  absorbed by the WP-15 packet.

## Non-Claims

- Does not claim first true birthday or personhood.
- Does not claim unbounded autonomy or external federation readiness.
- Does not bypass lifecycle, Freedom Gate, ACC, trace, redaction, or review
  boundaries.
- Does not replace the WP-16 flagship demo.
- Proof report states exactly what was proven and what remains downstream.
