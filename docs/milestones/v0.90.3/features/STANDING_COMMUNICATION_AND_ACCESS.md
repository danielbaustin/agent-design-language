# Standing, Communication, And Access

## Status

Planning contract for v0.90.3. WP-11 has landed the standing and
communication boundary proof. WP-12 has landed the authority matrix,
access-event packet, and denial fixtures for the full access-control proof.

## Purpose

Bind citizen state to actor standing, communication rights, inspection rights,
and access-control events.

## Standing Classes

v0.90.3 should make the following classes explicit:

- citizen: durable identity-bearing member of a polis
- guest: bounded temporary or externally originated participant
- service actor: runtime substrate actor with delegated authority but no social
  rights
- external actor: outside participant mediated through a gateway or guest path
- naked actor: prohibited unclassified actor with influence

## Core Rules

- Any actor with influence must have declared standing.
- Citizen and guest standing must differ in rights, duties, duration, trace,
  and continuity.
- A guest cannot silently acquire citizen rights.
- A service actor cannot become a hidden social actor through privileges.
- Communication is a governed action.
- Communication never grants inspection rights.
- Inspection, projection, migration, wake, quarantine, challenge, appeal, and
  decryption must be explicit access events.

## WP-11 Runtime Evidence

The WP-11 proof is recorded in
`docs/milestones/v0.90.3/STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md`.

Runtime evidence:

- `adl/src/runtime_v2/standing.rs`
- `adl/src/runtime_v2/tests/standing.rs`

Fixture evidence:

- `adl/tests/fixtures/runtime_v2/standing/standing_policy.json`
- `adl/tests/fixtures/runtime_v2/standing/standing_events.json`
- `adl/tests/fixtures/runtime_v2/standing/communication_examples.json`
- `adl/tests/fixtures/runtime_v2/standing/standing_negative_cases.json`

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture
```

## Access-Control Events

Required event families:

- inspection requested
- inspection denied
- decryption requested
- decryption denied
- projection requested
- projection denied
- wake requested
- wake denied
- migration requested
- migration denied
- quarantine requested
- quarantine release requested
- challenge raised
- appeal raised

## Acceptance Tests

- naked actor cannot communicate, observe, or affect shared state
- guest cannot silently acquire citizen continuity
- service actor cannot initiate arbitrary communication
- operator cannot inspect private state without an access event
- projection access does not become raw state access

WP-11 satisfies the first three acceptance tests directly. WP-12 owns operator
inspection events and projection-access denial semantics.

## WP-12 Runtime Evidence

The WP-12 proof is recorded in
`docs/milestones/v0.90.3/ACCESS_CONTROL_SEMANTICS_v0.90.3.md`.

Runtime evidence:

- `adl/src/runtime_v2/access_control.rs`
- `adl/src/runtime_v2/tests/access_control.rs`

Fixture evidence:

- `adl/tests/fixtures/runtime_v2/access_control/authority_matrix.json`
- `adl/tests/fixtures/runtime_v2/access_control/access_events.json`
- `adl/tests/fixtures/runtime_v2/access_control/denial_fixtures.json`

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture
```

WP-12 satisfies operator inspection-event and projection-access denial
semantics. WP-13 still owns challenge and appeal due-process behavior.
