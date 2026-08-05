# Structured Intent Prompt

Template: 1.0.0

Issue: 5821

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Pass a reviewed distributed Guardian/polis architecture and threat-model gate, execute the exact 16-child program with disjoint ownership, and integrate real multi-node membership, fencing, migration, rollback, certificate, and recovery behavior.

## Required Outcome

One reviewed distributed-runtime contract and threat model governs 16 concrete terminal child issues whose production paths converge at one exact revision with single-authority, mTLS membership, partition, fencing, relocation, rollback, rotation, and recovery proof.

## Scope

- Distributed architecture, threat model, schemas, COTS decisions, and 16-child ownership ledger
- Narrow child-owned surfaces under adl-runtime and adl-runtime-kernel for Guardian identity, networking, topology, state, control, observability, and resource context
- Integrated multi-node, partition, fencing, migration, rollback, certificate, and recovery proof
- .csdlc/evidence/5821 program and integration evidence

## Authority

- Issue 5821 owns the architecture/security gate, exact 16-child denominator, integration, and final reconciliation
- Each child retains its own implementation, proof, review, PR, and closeout authority
- Guardian remains process 0; network transport never becomes polis, cognition, governance, or identity authority
- Issue 5832 waits for the integrated substrate and owns protocol reconciliation
- No Runtime v2 or v0.93 governance authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
