# Prepared STP Draft: #4760 Memory Palace

Status: ready_for_typed_application_after_execution_claim

## Task Boundary

Implement the smallest real Memory Palace MVP: transform one declared,
validated set of ObsMem-shaped records into deterministic topology/working-set
artifacts and make a long-lived-agent cycle consume the resulting packet
reference.

## Deliverables

- `adl/src/memory_palace.rs` with bounded contracts, normalization, validation,
  stale-context disposition, and deterministic packet construction.
- `adl/src/lib.rs` module export.
- `adl/src/long_lived_agent.rs` declared-input and `memory_refs` consumer hook.
- `adl/tests/memory_palace_tests.rs` plus one deterministic fixture under
  `adl/tests/fixtures/memory_palace/`.
- Exact command/runtime/review evidence under `.csdlc/evidence/4760/`.

## Acceptance Criteria

- AC-1: Identical declared inputs and bounds produce byte-identical normalized
  packets and ordering.
- AC-2: Every selected context item names a relative citation/hash,
  inclusion reason, and compatible temporal/continuity anchor.
- AC-3: Stale, missing, malformed, private/host-path, provenance-mismatched, or
  over-budget context fails closed or is explicitly excluded with reason.
- AC-4: A real long-lived-agent cycle writes
  `memory_palace_context.json` and references it in
  `decision_request.memory_refs`.
- AC-5: Unconfigured agents retain the current empty `memory_refs` behavior.
- AC-6: Proof distinguishes Memory Palace topology/working-set ownership from
  ObsMem storage/retrieval and Chronosense time semantics.
- AC-7: All required VPP lanes pass at one exact revision and bounded review
  has no unresolved actionable findings.
- AC-8: #5007 remains deferred unless the complete #4760 proof packet exists.

## Exact Dependencies

- Product evidence authority: GitHub #4760.
- ADR consumer: #5007 / ADR 0051, hard dependent on #4760 implementation proof.
- Parent handoff owner: #5362 / WP-21.
- Milestone ordering predecessor: #5363 / WP-20; verify current disposition
  before execution. GitHub merge/ancestry may satisfy ordering, while unrelated
  typed closeout receipts do not gate preparation or prove Memory Palace.
- Temporal compatibility inputs: #4765, #4768, and #4771.
- Accepted boundaries: ADR 0007 and ADR 0010.
- Source contracts: `adl/src/obsmem_contract/models.rs`,
  `adl/src/long_lived_agent/types.rs`, and `adl/src/long_lived_agent.rs`.
- Activation truth: `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`.

## Non-Goals

- Do not implement or replace ObsMem storage/ranking.
- Do not introduce a new clock or alter Chronosense semantics.
- Do not add a graph/vector database, embedding provider, service, or binary.
- Do not implement broad v0.92 activation, identity, birthday, or capability
  envelope work.
- Do not edit ADR 0051 or claim ADR acceptance in #4760.
