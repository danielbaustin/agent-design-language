# Runtime v3 Identity And Memory Continuity Proof (#5250)

As of v0.91.7, Runtime v3 has an issue-local identity and memory-continuity
adapter proof for cutover review. The proof is invariant-based and does not
reuse Runtime v2 internals.

The Runtime v3 boundary provides:

- signed citizen identity bindings using `ed25519-dalek`
- citizen, runtime, and continuity id binding before memory append,
  checkpoint, lifelog, and restore access
- same-continuity owner substitution refusal across citizen, runtime, and
  signing-key identity
- append-only memory events over a continuity head
- checkpoint summaries with accepted-through, head hash, public facts, and
  private-state reference ids
- lifelog projections that expose only allowed visible fields and redact
  private-state references
- restore checks that refuse mismatched citizen/runtime/continuity state,
  preserve checkpoint summaries across the next checkpoint, and require the
  expected head before continued append

This resolves the previous #5250 blockers for `clock.checkpoint_lifelog` and
`citizen.identity_memory` as accepted intentional divergences: Runtime v3 keeps
the cutover-critical continuity semantics without claiming Runtime v2 raw
identity or memory wire-format equivalence.

## Proof

- `adl-runtime-kernel/src/identity_memory.rs`
- `adl-runtime-kernel/tests/identity_memory.rs`
- `docs/architecture/runtime_v3_identity_memory_5250.v1.json`

Focused validation:

- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test identity_memory -- --nocapture`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test identity_memory -- -D warnings`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test parity -- --nocapture`

## Non-Claims

- This does not authorize Runtime v3 as the default runtime.
- This does not delete Runtime v2.
- This does not claim Runtime v2 raw identity or memory wire-format
  compatibility.
- Broader production-like soak and shared live fixture expansion remain routed
  through #5253.
