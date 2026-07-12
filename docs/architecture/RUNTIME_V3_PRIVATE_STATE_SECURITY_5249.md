# Runtime v3 Private-State Security Proof (#5249)

As of v0.91.7, Runtime v3 has an issue-local private-state security boundary
for cutover review. The proof is intentionally invariant-based rather than raw
Runtime v2 format equivalence.

The Runtime v3 boundary provides:

- signed private-state records using `ed25519-dalek`
- BLAKE3 sealed payload hashes rather than projected raw private bytes
- append-only lineage checks with exact next-sequence enforcement
- same-position anti-equivocation checks
- projection provenance checks against the signed record hash
- redacted projection only
- principal authorization
- sanctuary-level policy enforcement

This resolves the previous "no Runtime v3 private-state proof exists" blocker
as an accepted intentional divergence: Runtime v3 preserves the security
semantics needed for cutover review without reusing Runtime v2 internals or
claiming raw wire-format compatibility.

## Proof

- `adl-runtime-kernel/src/private_state.rs`
- `adl-runtime-kernel/tests/private_state.rs`
- `docs/architecture/runtime_v3_private_state_security_5249.v1.json`

Focused validation:

- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test private_state -- --nocapture`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test private_state -- -D warnings`

## Non-Claims

- This does not authorize Runtime v3 as the default runtime.
- This does not delete Runtime v2.
- This does not claim Runtime v2 raw private-state format equivalence.
- Production key custody, rotation, and storage hardening remain deployment
  concerns outside this issue-local invariant proof.
