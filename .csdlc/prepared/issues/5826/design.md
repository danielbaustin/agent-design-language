# Issue 5826 Design: Stable Name And Identity Root

## Outcome And Sources

Define the WP-09 identity record from `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`, the candidate birthday record in `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`, and current Runtime v3 lineage/private-state authority in `adl-runtime-kernel/src/identity_memory.rs` and `adl-runtime-kernel/src/private_state.rs`. Retained Runtime v2 lineage is compatibility evidence only.

## Owned Paths

- `adl-runtime-kernel/src/birthday_identity.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birthday_identity.rs`
- `adl-runtime-kernel/tests/fixtures/birthday_identity`
- `docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md`
- `.csdlc/prepared/issues/5826/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5826/produce-native-receipt.rb`
- `.csdlc/evidence/5826`

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
  }
]
```

## Contract

Stable name is a label bound to an identity root, never the root itself. Aliases are ordered, provenance-bearing additions and cannot silently replace the root. Identity creation rejects empty or ambiguous roots, duplicate/conflicting aliases, missing origin evidence, path-unsafe references, and continuity heads that do not bind to prior evidence.

## Dependencies And Invariants

WP-08/#5825 must be terminal before implementation; prior citizen-state lineage remains authoritative substrate. Serialization and identity-root derivation are deterministic. Raw private state is never required for review, and a display name, boot admission, wake, snapshot, or copied state cannot establish identity alone.

## Validation

The exact `birthday_identity` integration-test target must run a nonzero test count covering canonical records, deterministic ordering, missing roots, alias collision, provenance mismatch, substituted continuity heads, and private-path disclosure. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden.

## Rollback

Remove only the WP-09 identity record module, registration, integration test,
fixtures, and owned feature-document edits. Preserve prior lineage primitives,
WP-08 outputs, rejected identity records, and native receipt evidence; rollback
must not rename an identity root or rewrite continuity history.

## Non-Goals

This issue does not prove multi-cycle continuity, migration, citizenship, reputation, legal personhood, or the birthday event.
