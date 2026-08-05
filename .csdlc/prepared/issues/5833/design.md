# Issue 5833 Design: Birth Witnesses And Citizen Receipt

## Outcome And Sources

Define WP-15 Runtime v3 witness and receipt contracts from `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`, the #4762 retained handoff under `docs/milestones/v0.91.8/review/v092_handoff_4762/`, and current private-state authority in `adl-runtime-kernel/src/private_state.rs`. Retained `adl/src/runtime_v2/private_state_witness.rs` is read-only compatibility evidence.

## Owned Paths

- `adl-runtime-kernel/src/birth_witness.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birth_witness.rs`
- `adl-runtime-kernel/tests/fixtures/birth_witness`
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `.csdlc/prepared/issues/5833/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5833/produce-native-receipt.rb`
- `.csdlc/evidence/5833`

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
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5833-witness-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md"
    ],
    "issues": [
      5833,
      5843
    ],
    "order": [
      5833,
      5843
    ]
  }
]
```

## Contract

Witnesses must be distinct where policy requires, bind the exact birthday candidate digest, and agree on the reviewed evidence set. Missing, duplicate, stale, forged, equivocal, unauthorized, or candidate-mismatched witnesses fail closed. The receipt is deterministically derived from the validated decision and cannot claim a birth while `birth_event_status` remains `not_claimed`.

## Dependencies And Invariants

WP-09/#5826 through WP-13/#5830 must be terminal as required by sprint gate 3, and #4762 evidence remains an input rather than current birth proof. Receipts are review surfaces, not authority substitutes.

## Validation

The exact `birth_witness` Runtime v3 integration-test target must run a nonzero count proving valid witness sets, deterministic receipts, and equivocation, duplicate identity, stale digest, missing authority, forged integrity ref, redaction leakage, and premature-birth rejection. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden.

## Rollback

Remove only the WP-15 witness module, registration, integration test, fixtures,
and owned feature-document edits. Preserve retained #4762 evidence, all emitted
audit receipts, rejected witness sets, and native CI receipts; rollback must
not erase equivocation evidence or authorize a premature birth.

## Non-Goals

Public launch, birthday decision ownership, legal attestation, citizenship, governance authority, raw private-state disclosure, and rewriting #4762 are excluded.
