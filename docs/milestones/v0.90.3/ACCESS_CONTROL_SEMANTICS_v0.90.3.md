# Access-Control Semantics - v0.90.3

## Status

Landed in WP-12 / #2338.

## Purpose

WP-12 completes the D10 standing, communication, and access-control proof. It
defines the authority matrix and access-event evidence for sensitive
citizen-state paths without granting raw private-state inspection or hidden
continuity mutation.

WP-11 proved standing and communication boundaries. WP-12 proves that sensitive
access paths are explicit, auditable, and fail closed when denied.

## Runtime Evidence

The Runtime v2 implementation introduces access-control evidence in:

- `adl/src/runtime_v2/access_control.rs`
- `adl/src/runtime_v2/tests/access_control.rs`

The focused contract entrypoint is:

- `runtime_v2_access_control_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/access_control/authority_matrix.json`
- `adl/tests/fixtures/runtime_v2/access_control/access_events.json`
- `adl/tests/fixtures/runtime_v2/access_control/denial_fixtures.json`

## Authority Matrix

The matrix covers the required WP-12 sensitive paths:

- `inspection`
- `decryption`
- `projection`
- `migration`
- `wake`
- `quarantine`
- `challenge`
- `appeal`
- `release`

Every path requires an auditable event. The matrix does not grant raw
private-state inspection, decryption cleartext, or silent continuity mutation.

Projection is modeled as redacted Observatory visibility, not raw inspection.
Quarantine is modeled as evidence-preserving safety state, not release
authority. Challenge and appeal are recorded as access events, but WP-13 still
owns due-process behavior and review-resolution artifacts.

## Access Events

The event packet records one access event for each sensitive path. Each event
records:

- access path
- event kind
- actor id
- standing class
- decision
- requested authority
- granted authority
- denied authority
- evidence refs
- raw-private-state disclosure flag
- continuity mutation flag
- continuity sequence before and after
- rationale

The packet carries a deterministic hash over the ordered event payload.

## Denial Fixtures

Focused tests and fixtures cover:

- missing access event is rejected
- denied inspection cannot leak raw private state
- denied decryption cannot return cleartext
- denied migration cannot advance continuity
- denied wake cannot change the active head
- denied release cannot grant quarantine release authority

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture
```

Useful adjacent validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture
```

## Non-Claims

- This does not implement WP-13 challenge or appeal due-process behavior.
- This does not implement review-resolution artifacts.
- This does not allow unrestricted operator inspection of private citizen state.
- This does not claim first true Godel-agent birth.
- This does not implement v0.92 identity rebinding, migration, or birthday.
