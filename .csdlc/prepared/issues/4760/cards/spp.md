# Prepared SPP Draft: #4760 Memory Palace

Status: ready_for_typed_application_after_execution_claim
Plan revision: 2

## Summary

Execute one additive Memory Palace producer/consumer path and prove it without
absorbing adjacent memory, temporal, runtime, or v0.92 scope.

## Plan Steps

1. `S1` pending: acquire a live typed v2 issue-local claim, bind execution,
   verify current #4760/#5007/#5362, WP-20/#5363 ordering, and Chronosense
   dependency truth, and apply these prepared cards through typed operations.
2. `S2` pending: add `adl/src/memory_palace.rs` using existing ObsMem contract
   types and existing COTS only; implement canonical ordering, bounds,
   provenance, redaction, temporal compatibility, and stale-context decisions.
3. `S3` pending: connect configured `AgentSpec.memory.memory_palace.input_ref`
   to long-lived cycle packet materialization and
   `decision_request.memory_refs`, preserving unconfigured behavior.
4. `S4` pending: add deterministic, negative, replay, and integrated consumer
   tests plus one declared fixture.
5. `S5` pending: run every VPP lane, retain exact runtime artifacts under
   `.csdlc/evidence/4760/`, and fix one bounded implementation review.
6. `S6` pending: record exact SOR proof. Keep #4760 open and #5007 deferred if
   any acceptance item is absent.

No implementation step is complete in preparation.

## Affected Areas

- `adl/src/memory_palace.rs`
- `adl/src/lib.rs`
- `adl/src/long_lived_agent.rs`
- `adl/tests/memory_palace_tests.rs`
- `adl/tests/fixtures/memory_palace/long_running_context.json`
- `.csdlc/evidence/4760/`

## Invariants

- Memory Palace is topology and bounded handoff, not a storage backend.
- Declared inputs and canonical ordering preserve replay determinism.
- No raw chat transcript or absolute/private host path is authoritative input.
- A failed validation cannot yield a runtime-consumed packet reference.
- ADR 0051 remains deferred until #5007 consumes complete implementation proof.

## COTS

- Reuse: `serde`, `serde_json`, `chrono`, `sha2`, existing ObsMem contracts.
- Add: none.
- Prohibited without replan: database, cache/vector store, embedding/provider
  client, network service, or new crate.

## LoC And Time Budgets

- Production Rust: <= 500 net new lines.
- Tests/fixtures: <= 500 net new lines.
- Issue-local evidence prose: <= 200 lines excluding logs.
- Total reviewable delta: <= 1,200 net new lines.
- Implementation plus focused proof: 6-10 hours.
- Focused local validation: <= 45 minutes warm-cache elapsed.
- Review/fixes: <= 90 minutes.

## Risks

- A metaphor-only topology could pass serialization without runtime value.
- Summaries could hide provenance or leak private context.
- Stale context could be treated as current if temporal anchors are optional.
- A broad long-lived-agent refactor could bury the single concern.

## Stop, Replan, And Rollback

- Replan before exceeding any LoC/time budget or intended product path.
- Stop if a new backend/crate, ObsMem rewrite, Chronosense rewrite, or shared
  milestone edit appears necessary.
- Roll back before merge on nondeterminism, privacy/path leakage, silent stale
  acceptance, invalid packet consumption, or unconfigured runtime regression.
- Rollback removes the module/export/hook/test/fixture and proves the prior
  empty `memory_refs` behavior; retained evidence remains non-runtime history.

## Handoff

Execution-time claim acquisition is deferred and non-blocking for this
preparation packet. Product/card mutation starts only after typed acquisition.
