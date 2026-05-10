# A2A Adapter Boundary

## Metadata

- Feature Name: A2A Adapter Boundary And Compatibility Plan
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-14
- Source Docs: `.adl/docs/TBD/a2a/`
- Proof Modes: fixtures, schema, tests, review

## Purpose

Define A2A as an adapter over ADL's communication substrate, not a competing
communication architecture. A2A compatibility must preserve ADL identity,
authority, redaction, trace, and local/external transport boundaries.

## Scope

In scope:

- A2A-over-ACIP compatibility mapping.
- Adapter boundary and non-claims.
- Fixtures for allowed, denied, and unsupported adapter cases.
- Relationship to ACIP and ACC authority.

Out of scope:

- External federation readiness.
- Bypassing ACIP or ACC.
- Treating A2A as canonical internal comms.

## Acceptance Criteria

- Adapter flow cannot bypass ADL authority checks.
- Compatibility docs preserve one communication model.
- External transport remains gated until security posture is implemented.

## Landed Surface

- `adl/src/runtime_v2/a2a_adapter_boundary.rs`
- `adl/src/runtime_v2/tests/a2a_adapter_boundary.rs`
- `adl/src/agent_comms/a2a.inc`
- `adl/tests/fixtures/runtime_v2/comms/a2a_adapter_boundary.json`
- `adl/src/runtime_v2/acip_hardening.rs`

## Proof Route

- Contract accessor: `runtime_v2_a2a_adapter_boundary_contract()`
- Generated packet path: `runtime_v2/acip/a2a_adapter_boundary_packet.json`
- Tracked proof surfaces:
  - `adl/src/runtime_v2/a2a_adapter_boundary.rs`
  - `adl/src/runtime_v2/tests/a2a_adapter_boundary.rs`
  - `adl/src/agent_comms/a2a.inc`
  - `adl/tests/fixtures/runtime_v2/comms/a2a_adapter_boundary.json`
- Focused validation:
  - `cargo test --manifest-path adl/Cargo.toml runtime_v2_a2a_adapter_boundary -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml acip_a2a_adapter -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml runtime_v2_acip_hardening -- --nocapture`

## Non-Claims

- This slice does not prove external federation or cross-polis routing.
- This slice does not create a second canonical internal communication model.
- This slice does not grant execution authority from Agent Card claims.
- This slice does not bypass ACIP, ACC, lifecycle, trace, or redaction boundaries.
