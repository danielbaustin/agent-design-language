# Issue 5827 Design: Continuity Across Bounded Cycles

## Outcome And Sources

Implement the WP-10 continuity record defined by `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md` and `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, consuming existing lineage and wake-continuity evidence without converting those bounded proofs into birthday truth.

## Owned Paths

- `adl-runtime-kernel/src/birthday_continuity.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birthday_continuity.rs`
- `adl-runtime-kernel/tests/fixtures/birthday_continuity`
- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `.csdlc/prepared/issues/5827/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5827/produce-native-receipt.rb`
- `.csdlc/evidence/5827`

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
    "id": "v092-identity-feature-doc-v1",
    "paths": [
      "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"
    ],
    "issues": [
      5826,
      5827
    ],
    "order": [
      5826,
      5827
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5827-continuity-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md"
    ],
    "issues": [
      5827,
      5843
    ],
    "order": [
      5827,
      5843
    ]
  }
]
```

## Contract

The next continuity head is derived from canonical predecessor head and current cycle evidence. Replays of identical inputs match; missing predecessor, root substitution, discontinuous cycle order, forged witness, duplicate cycle, copied state without lineage, or narrative-only continuity fails closed.

## Dependencies And Invariants

WP-09/#5826 must be terminal. Existing private-state lineage and wake evidence remain inputs, not replacement authority. Continuity never exposes raw private state and never treats restart, wake, restore, or snapshot as sufficient by itself.

## Validation

The exact `birthday_continuity` integration-test target must run a nonzero test count proving a two-or-more-cycle chain, deterministic head derivation, substitution/discontinuity/duplicate/reordered/missing-evidence failures, and copied-state rejection. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden.

## Rollback

Remove only the WP-10 continuity module, registration, integration test,
fixtures, and owned feature-document edits. Retain predecessor identity and
birth evidence plus every rejected chain and native receipt; rollback must not
rewrite a continuity head or manufacture an uninterrupted history.

## Non-Goals

Memory Palace retrieval, capability profiles, migration, metaphysical sameness, citizenship, and birthday approval are outside WP-10.
