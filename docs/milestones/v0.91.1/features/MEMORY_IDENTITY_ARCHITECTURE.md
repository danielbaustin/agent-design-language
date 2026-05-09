# Memory And Identity Architecture

## Metadata

- Feature Name: Memory and Identity Architecture
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-07
- Source Docs: `.adl/docs/TBD/memory_identity/`
- Proof Modes: architecture, fixtures, tests, review

## Purpose

Prepare memory and identity evidence surfaces for v0.92 without claiming full
identity continuity early. v0.91.1 should clarify what memory evidence exists,
how it is cited, and what remains birthday-bound.

## Scope

In scope:

- Memory/identity architecture packet.
- Evidence references for memory surfaces.
- Boundary language for v0.92 identity and birthday work.
- Review fixtures showing cited evidence rather than hidden continuity claims.

Out of scope:

- Full memory model completion.
- Birthday event semantics.
- Metaphysical identity claims.

## Acceptance Criteria

- Memory references are traceable and reviewable.
- Identity claims remain bounded and non-final.
- v0.92 receives a concrete handoff rather than a loose idea pile.

## Proof Route

- `adl/src/runtime_v2/memory_identity_architecture.rs`
- `adl/src/runtime_v2/tests/memory_identity_architecture.rs`
- `adl/tests/fixtures/runtime_v2/memory_identity/memory_identity_architecture.json`
- dependent evidence surfaces:
  - `adl/src/runtime_v2/boot_admission.rs`
  - `adl/src/runtime_v2/private_state_lineage.rs`
  - `adl/src/runtime_v2/private_state_witness.rs`
  - `adl/src/runtime_v2/private_state_observatory.rs`
  - `adl/src/obsmem_contract/models.rs`
  - `adl/src/obsmem_indexing.rs`
