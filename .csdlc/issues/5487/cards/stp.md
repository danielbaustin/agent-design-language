# Structured Task Prompt

Template: 1.0.0

Issue: 5487

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add a typed receipt-aware terminal design repair and reapproval transaction with rollback and materialization proof.

## Deliverables

- Typed atomic repair request and operation
- Identity, authority, generation, digest, and artifact-hash guards
- Rollback and reconcile-terminal tests
- Repair of #5467 retained design and diagram

## Acceptance

1. Only closed-out targets with explicit authority can repair
2. Design and diagram hashes are verified before mutation
3. Receipt, SPP/VPP references, projections, and audit update atomically
4. Failure leaves all artifacts and receipt unchanged
5. Reconcile-terminal materializes the repaired retained artifacts

## Dependencies

- Terminal receipt schema and Store::reconcile_terminal
- Closed-out #5467 receipt and retained artifacts
- markdown.rs AST validation

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/model.rs
- csdlc-v2/tests
- .csdlc/issues/5467/retained/design.md
- .csdlc/issues/5467/retained/diagram.mmd

## Non Goals

- AWS execution or inspection
- Manual receipt or rendered-card edits
- Reopening or changing #5467 implementation
