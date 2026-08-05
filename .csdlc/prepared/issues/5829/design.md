# Issue 5829 Design: Capability Envelope

## Outcome And Sources

Define WP-12's birthday-consumable provider, model, tool, skill, authority, and limit envelope from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, current provider/profile surfaces under `adl/src/provider/`, `adl/src/provider_adapter.rs`, and the retained #4761 envelope at `.csdlc/evidence/4761/capability-envelope/`.

## Owned Paths

The complete writable protected-path set is:

- `adl-runtime-kernel/src/capability_envelope.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/capability_envelope.rs`
- `adl-runtime-kernel/tests/fixtures/capability_envelope/`
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `.csdlc/prepared/issues/5829/validate-native-receipts.rb`
- `.csdlc/evidence/5829/`

Provider inventory and #4761 evidence are read-only input authorities. WP-12 may develop its nonshared paths in parallel with WP-11, but it must not claim or edit `adl-runtime-kernel/src/lib.rs` until WP-11/#5828 has landed and released that path; its registration edit is then a one-line serialized step.

## Contract

Each envelope binds identity root and evidence revision to explicit provider/model identifiers, tools, skills, authority grants, denials, resource/recurrence limits, provenance refs, and unsupported claims. Canonical ordering makes equivalent inputs deterministic. Unknown provider/model, stale source digest, undeclared tool/skill, authority escalation, missing limits, credential material, or absolute host paths are rejected.

## Dependencies And Invariants

WP-08/#5825 and WP-09/#5826 must be terminal, and #4761 evidence must remain verifiable. The canonical wave row and live issue must add WP-08 before execution so they exactly match this stricter card contract. The `lib.rs` registration claim is additionally serialized after WP-11/#5828 integration; this is a write-collision gate, not a semantic substitution for WP-08 or WP-09. Capability is descriptive and bounded; it does not grant authority, prove invocation, expose credentials, or imply unlimited capacity.

## Validation And Rollback

The exact `capability_envelope` integration-test target must run a nonzero count proving complete deterministic envelopes and stale-provenance, unsupported-provider/model, unauthorized-capability, omitted-limit, secret-like-content, and path-portability failures. Native Linux CI and a retained native macOS receipt bind the exact source SHA, test argv, fixture-tree digest, output digest, runner identity, and recomputed native artifact digest before portability is claimed. Rollback removes the v0.92 envelope while preserving #4761 evidence unchanged.

## Non-Goals

Provider execution, credential setup, remote deployment, reputation, identity creation, Memory Palace completion, and birthday approval are excluded.
