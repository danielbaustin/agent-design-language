# Issue 5828 Design: First Memory Palace Context-Topology Slice

## Outcome And Sources

Implement the Runtime v3 Memory Palace boundary in `adl-runtime-kernel`, using the existing ObsMem contract in `adl/src/obsmem_contract/` and trace fields in `adl-runtime-kernel/src/observability.rs` as read-only input authorities. The retained `adl/src/memory_palace.rs` slice is compatibility evidence, not the implementation target. The design follows `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md` and the redaction boundary in `MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`.

## Owned Paths

- `adl-runtime-kernel/src/memory_palace.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/memory_palace.rs`
- `adl-runtime-kernel/tests/fixtures/memory_palace`
- `docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md`
- `.csdlc/prepared/issues/5828/validate-obsmem-trace-integration.rb`
- `.csdlc/prepared/issues/5828/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5828/produce-native-receipt.rb`
- `.csdlc/evidence/5828`

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
  }
]
```

## Contract

Declared memory records and citation hashes form a canonical context topology. Bounded selection produces a stable working set and records overflow rather than consuming beyond the limit. Required identity/continuity bindings, source provenance, temporal anchors, and redaction checks gate every loaded item. Missing references, hash mismatch, stale context, continuity mismatch, private/absolute paths, unauthorized private-state access, and nondeterministic ordering fail closed.

## Dependencies And Invariants

WP-09/#5826 and WP-10/#5827 must be terminal. Before editing, record exact source revisions and digests for `adl/src/obsmem_contract/models.rs`, `adl-runtime-kernel/src/observability.rs`, and `adl-runtime-kernel/src/proof.rs` in `obsmem-trace-integration-receipt.json`; that receipt also names the fixture digest, trace ID, ObsMem citation IDs, and Runtime v3 test output digest. Same normalized ObsMem records, trace refs, identity root, continuity head, observation time, and bounds produce byte-equivalent semantic output. No raw private state enters the packet; all references remain repo-relative and witnessed.

## Validation And Rollback

The exact `memory_palace` Runtime v3 integration-test target must run a nonzero count and prove normalized ObsMem ingestion, trace/receipt binding, deterministic replay, bounded overflow, and stale/hash/continuity/redaction failures. `validate-obsmem-trace-integration.rb` recomputes source, authority, fixture-tree, and output digests and binds exact HEAD, argv, runner, trace, and citation identity rather than trusting declared fields. The issue-local native producer must run the same target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent native validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden. Rollback removes the new Runtime v3 module and fixture schema while preserving the integration receipt and emitted historical packets as evidence.

## Non-Goals

Distributed or unbounded Memory Palace, semantic search, raw private-memory browsing, replacing ObsMem, and birthday completion are excluded.
