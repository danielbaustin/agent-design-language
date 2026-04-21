# v0.90.4 Economics Handoff

## Purpose

Explain why full economics and contract-market work is not part of the v0.90.3
safety floor.

## Handoff

v0.90.4 is the likely home for:

- contract schema
- bid schema
- evaluation and selection model
- transition authority model
- delegation and subcontracting
- external counterparty model
- contract lifecycle state
- fixture set
- runner
- reviewer-facing summary
- bounded contract-market proof

## v0.90.3 Responsibility

v0.90.3 should decide whether citizen-state safety needs a narrow
resource-stewardship bridge.

Examples:

- preserving state when compute is constrained
- making resource-sensitive preservation explicit
- documenting why resource decisions affect continuity and citizen rights

## Non-Goals For v0.90.3

- no market simulation
- no contract execution
- no bid evaluation
- no subcontracting
- no payment rails
- no inter-polis trade
