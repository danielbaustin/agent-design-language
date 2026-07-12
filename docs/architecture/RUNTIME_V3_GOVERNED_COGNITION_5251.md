# Runtime v3 Governed Cognition Proof (#5251)

As of v0.91.7, Runtime v3 has an issue-local governed cognition adapter proof
for cutover review. The proof is deterministic and contract-based. It does not
reuse Runtime v2 internals, call a model/provider, or claim subjective
cognition.

The Runtime v3 boundary provides:

- deterministic service contracts for moral/affect/wellbeing and
  curiosity/intelligence/theory-of-mind surfaces
- bounded score validation for affect, wellbeing, curiosity, intelligence
  confidence, and theory-of-mind confidence
- explicit policy-hash binding
- review-required dispositions that stay distinct from allow/refuse
- review records bound to subject, policy, evidence, and context review hash
- JSON-safe decisions that do not embed prompt or provider payloads

This resolves the previous #5251 blockers as accepted intentional divergences:
Runtime v3 preserves cutover-critical governance and contract semantics without
claiming Runtime v2 raw cognition wire-format equivalence.

## Proof

- `adl-runtime-kernel/src/cognition.rs`
- `adl-runtime-kernel/tests/cognition.rs`
- `docs/architecture/runtime_v3_governed_cognition_5251.v1.json`

Focused validation:

- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test cognition -- --nocapture`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test cognition -- -D warnings`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test parity -- --nocapture`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`

## Non-Claims

- This does not authorize Runtime v3 as the default runtime.
- This does not delete Runtime v2.
- This does not claim subjective cognition, model-backed reasoning, or Runtime
  v2 raw cognition wire-format compatibility.
- Broader production-like soak and shared live fixture expansion remain routed
  through #5253.
