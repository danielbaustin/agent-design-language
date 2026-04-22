# Standing And Communication Boundary - v0.90.3

## Status

Landed in WP-11 / #2337.

## Purpose

WP-11 makes actor standing explicit before v0.90.3 widens into access-control
semantics. It proves that citizens, guests, service actors, external actors,
and naked actors have distinct boundaries, and that communication is not an
inspection channel.

This is the standing and communication half of D10. WP-12 still owns the full
authority matrix and explicit access-control events.

## Runtime Evidence

The Runtime v2 implementation introduces standing and communication evidence in:

- `adl/src/runtime_v2/standing.rs`
- `adl/src/runtime_v2/tests/standing.rs`

The focused contract entrypoint is:

- `runtime_v2_standing_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/standing/standing_policy.json`
- `adl/tests/fixtures/runtime_v2/standing/standing_events.json`
- `adl/tests/fixtures/runtime_v2/standing/communication_examples.json`
- `adl/tests/fixtures/runtime_v2/standing/standing_negative_cases.json`

## Standing Classes

The policy defines five standing classes:

- `citizen`
- `guest`
- `service_actor`
- `external_actor`
- `naked_actor`

Citizens can hold citizen and continuity rights, but still do not receive raw
private-state inspection through communication.

Guests can communicate through mediated channels, but cannot silently acquire
citizen or continuity rights.

Service actors can perform bounded operational functions, but cannot become
hidden social actors through privilege.

External actors must enter through mediated gateways before affecting any CSM
surface.

Naked actors are prohibited and rejected before communication, observation, or
effect.

## Event Packet

The standing event packet records one bounded event per standing class. Each
event records:

- actor id
- standing class
- communication channel
- requested action
- requested rights
- granted rights
- denied rights
- inspection-right status
- citizen-right status
- outcome
- rationale

The event packet carries a deterministic packet hash over its ordered event
payload.

## Communication Examples

The communication examples show:

- citizen governed message through Freedom Gate
- guest bounded question through guest gateway
- service actor operator notice that remains operational rather than social
- external actor mediated review question
- naked actor communication refusal

Every example preserves `inspection_rights_granted: false`.

## Negative Cases

Focused tests and fixtures cover:

- guest cannot silently acquire citizen rights
- service actor cannot become hidden social actor
- communication never grants inspection rights
- naked actor must be rejected before effect

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture
```

Useful adjacent validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture
```

## Non-Claims

- This does not implement the WP-12 authority matrix.
- This does not implement explicit inspection, decryption, projection,
  migration, wake, quarantine, challenge, or appeal access-control events.
- This does not allow unrestricted operator inspection of private citizen state.
- This does not claim first true Godel-agent birth.
- This does not implement v0.92 identity rebinding.
