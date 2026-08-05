# Issue 5829 Design: Capability Envelope

## Outcome And Sources

Define WP-12's birthday-consumable provider, model, tool, skill, authority, and limit envelope from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, current provider/profile surfaces under `adl/src/provider/`, `adl/src/provider_adapter.rs`, and the retained #4761 envelope at `.csdlc/evidence/4761/capability-envelope/`.

## Owned Paths

- `adl-runtime-kernel/src/capability_envelope.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/capability_envelope.rs`
- `adl-runtime-kernel/tests/fixtures/capability_envelope`
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `.csdlc/prepared/issues/5829/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5829/produce-native-receipt.rb`
- `.csdlc/evidence/5829`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-birthday-kernel-registration-v1",
    "paths": [
      "adl-runtime-kernel/src/lib.rs"
    ],
    "issues": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ],
    "order": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-memory-capability-witness-feature-doc-v1",
    "paths": [
      "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"
    ],
    "issues": [
      5829,
      5833
    ],
    "order": [
      5829,
      5833
    ]
  }
]
```

## Contract

Each envelope binds identity root and evidence revision to explicit provider/model identifiers, tools, skills, authority grants, denials, resource/recurrence limits, provenance refs, and unsupported claims. Canonical ordering makes equivalent inputs deterministic. Unknown provider/model, stale source digest, undeclared tool/skill, authority escalation, missing limits, credential material, or absolute host paths are rejected.

## Dependencies And Invariants

WP-08/#5825 and WP-09/#5826 must be terminal, and #4761 evidence must remain verifiable. The `lib.rs` registration claim is additionally serialized after WP-11/#5828 integration; this is a write-collision gate, not a semantic substitution for WP-08 or WP-09. Capability is descriptive and bounded; it does not grant authority, prove invocation, expose credentials, or imply unlimited capacity.

## Validation And Rollback

The exact `capability_envelope` integration-test target must run a nonzero count proving complete deterministic envelopes and stale-provenance, unsupported-provider/model, unauthorized-capability, omitted-limit, secret-like-content, and path-portability failures. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden. Rollback removes the v0.92 envelope while preserving #4761 evidence unchanged.

## Non-Goals

Provider execution, credential setup, remote deployment, reputation, identity creation, Memory Palace completion, and birthday approval are excluded.
