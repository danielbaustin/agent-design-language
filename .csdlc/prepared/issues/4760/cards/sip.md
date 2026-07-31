# Prepared SIP Draft: #4760 Memory Palace

Status: ready_for_typed_application_after_execution_claim

## Goal

Implement and prove one deterministic Memory Palace context handoff consumed by
the long-lived-agent runtime path, so #5007 can evaluate ADR 0051 from evidence
rather than planning intent.

## Required Outcome

At one exact revision, declared ObsMem-shaped records and Chronosense-compatible
anchors produce a bounded, redaction-safe Memory Palace packet that is written
by a real long-lived-agent cycle and referenced by
`decision_request.memory_refs`. Missing implementation, consumer integration,
negative/replay proof, or review evidence leaves #4760 open and #5007 deferred.

## Declared Scope

- Add one deterministic Memory Palace packet builder.
- Consume existing ObsMem record/citation and temporal-anchor semantics.
- Use existing `AgentSpec.memory` as the declared input boundary.
- Connect the emitted packet to the existing long-lived-agent decision request.
- Retain exact-revision implementation, runtime, validation, and review proof.

## Authority Boundary

- GitHub issue #4760 owns implementation/proof.
- #5007 owns later ADR 0051 acceptance and must consume #4760 evidence.
- ADR 0007 remains authoritative for the external ObsMem boundary.
- ADR 0010 remains authoritative for Chronosense temporal semantics.
- Preparation drafts are non-authoritative until applied through typed v2 after
  execution claim acquisition.

## Initial Assumptions

- `AgentSpec.memory` remains the narrow declared configuration surface.
- The existing ObsMem contract types remain sufficient for the MVP handoff.
- No new COTS dependency, network backend, or persistent database is required.

## Operator Constraints

- Exactly one primary concern: `memory-palace`.
- No sibling WP-21 work, shared milestone rewrite, or v0.92 implementation.
- No planning-only close, mock-only proof, or isolated unconsumed artifact.
- Required local proof lanes cannot be deferred.
- Product edits remain within the intended paths in `design.md` unless SPP is
  explicitly replanned before widening.
- Execution must acquire a live issue-local typed claim; preparation does not.
