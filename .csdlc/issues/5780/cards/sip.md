# Structured Intent Prompt

Template: 1.0.0

Issue: 5780

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Finish the closeout redesign by deleting supported tracked post-merge terminal mutation, receipt authority, reconciliation, repair, transport, and coupled-prune machinery.

## Required Outcome

The supported operator surface uses csdlc-finish for terminal delivery truth, csdlc-pr-state for live status, and csdlc-clean for independent cleanup; legacy terminal records and receipts remain read-only compatible.

## Scope

- Delete the csdlc-closeout binary, skill, installation entry, and supported commands
- Delete current terminal projection and receipt writers, repairs, transports, and reconciliation APIs
- Retain read-only legacy phase, terminal evidence, receipt deserialization, and compatibility indexing
- Replace obsolete behavior tests with compatibility and non-reintroduction guards
- Update active operator documentation and record reduction metrics

## Authority

- csdlc-finish is the sole terminal mutation authority
- csdlc-clean never changes delivery truth
- Legacy records and receipts are immutable read-only evidence
- Historical architecture and tracked evidence are not rewritten

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- No tracked edits on main
- No AWS
- No compatibility wrapper that preserves a deleted writer
- Exact-head independent review before publication
