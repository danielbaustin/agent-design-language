# Governance Cluster Map

## Status

Draft

## Purpose

Define the intended document boundaries for the governance / constitution /
decision cluster so these docs can evolve without becoming a stack of partially
overlapping philosophical and operational drafts.

---

## Core Package

### `ADL_CONSTITUTION.md`

Owns:
- the umbrella constitutional framework
- top-level governing principles
- substrate-level supremacy and broad normative constraints

Should not own:
- the detailed runtime shape of every decision event
- the full semantics of every delegated interaction
- specialized multi-agent coordination patterns in detail

### `ADL_AND_REASONABLENESS.md`

Owns:
- reasonableness as a cross-cutting governance principle
- how context-sensitive judgment differs from literal obedience
- the philosophical and architectural role of reasonableness

Should not own:
- the entire constitutional framework
- the detailed schema of decisions
- the full execution semantics of delegation

### `DECISION_SURFACES.md`

Owns:
- explicit workflow points where choice is exercised
- accept / reject / defer / escalate / reroute as operational states
- visibility of decision points in workflow design and trace

Should not own:
- the full record schema for decisions
- the whole constitutional philosophy
- full negotiation mechanics

### `DECISION_SCHEMA.md`

Owns:
- the structured record shape for decision events
- required semantic fields for decision capture
- linkage to trace, policy, rationale, and downstream effects

Should not own:
- the broader philosophical meaning of governance
- the full set of workflow decision surfaces

### `ACTION_MEDIATION_LAYER.md`

Owns:
- the authority boundary between model intent and runtime execution
- validation, approval, rejection, revision, and defer flow over proposed actions
- the bridge between governance surfaces and actual tool/skill invocation

Should not own:
- the full constitutional umbrella
- the full canonical skill model
- every provider-specific interaction shape

### `ACTION_PROPOSAL_SCHEMA.md`

Owns:
- the canonical structured proposal contract emitted by model cognition
- proposal kinds, fields, and validation expectations before execution
- the non-authoritative interface consumed by the Action Mediation Layer

Should not own:
- final decision-event records
- the full execution protocol after approval
- broad governance philosophy

### `DELEGATION_AND_REFUSAL.md`

Owns:
- refusal as a first-class governed outcome
- delegation as bounded handoff behavior
- distinctions among refusal, failure, rerouting, and deferred action

Should not own:
- the entire constitutional system
- full social coordination semantics

### `MULTI_AGENT_NEGOTIATION.md`

Owns:
- bounded disagreement handling
- negotiation as an explicit coordination surface
- congressional-principle style coordination
- visible consensus, dissent, escalation, and unresolved disagreement

Should not own:
- all delegation semantics
- the full constitutional umbrella
- citizenship, trust, or social standing in full

---

## Package Boundary

The intended governance cluster is:

- `ADL_CONSTITUTION.md`
- `ADL_AND_REASONABLENESS.md`
- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`
- `ACTION_MEDIATION_LAYER.md`
- `ACTION_PROPOSAL_SCHEMA.md`
- `DELEGATION_AND_REFUSAL.md`
- `MULTI_AGENT_NEGOTIATION.md`

---

## Editing Rule

When editing this cluster:

- put top-level constitutional principles in `ADL_CONSTITUTION.md`
- put reasonableness and context-sensitive judgment in `ADL_AND_REASONABLENESS.md`
- put named workflow decision points in `DECISION_SURFACES.md`
- put structured decision records in `DECISION_SCHEMA.md`
- put the governed intent-to-execution bridge in `ACTION_MEDIATION_LAYER.md`
- put model-emitted action intent structure in `ACTION_PROPOSAL_SCHEMA.md`
- put refusal/handoff semantics in `DELEGATION_AND_REFUSAL.md`
- put structured disagreement and coordination in `MULTI_AGENT_NEGOTIATION.md`

If a concept appears in multiple docs, one doc should own it and the others should only
 reference it.

---

## Overlap Notes

### Constitution vs Reasonableness

- `ADL_CONSTITUTION.md` should say what the governing order is
- `ADL_AND_REASONABLENESS.md` should explain one of its key operating principles

### Decision Surfaces vs Decision Schema

- `DECISION_SURFACES.md` should define where decisions happen
- `DECISION_SCHEMA.md` should define how those decisions are recorded

### Action Proposal vs Decision Record

- `ACTION_PROPOSAL_SCHEMA.md` should define the proposed intent entering runtime authority
- `DECISION_SCHEMA.md` should define the governed outcome recorded after review/approval logic

### Decision Records vs Action Mediation

- `DECISION_SCHEMA.md` should define the record shape of governed choice
- `ACTION_MEDIATION_LAYER.md` should define the runtime control boundary that consumes proposals and routes execution

### Delegation vs Negotiation

- `DELEGATION_AND_REFUSAL.md` should own bounded handoff and refusal semantics
- `MULTI_AGENT_NEGOTIATION.md` should own explicit disagreement and coordination patterns

---

## Likely Future Cleanup

Likely later actions:

- place this cluster into its eventual governance / constitutional milestone band
- decide whether `DELEGATION_AND_REFUSAL.md` and `MULTI_AGENT_NEGOTIATION.md` stay separate or fold into broader v0.93 governance docs
- add a future constitutional implementation/coverage map if the umbrella constitution becomes the root doc for multiple feature surfaces

---

## Summary

This cluster already contains the pieces of a coherent governance package.

The main cleanup need is to preserve these roles:
- constitution owns the umbrella
- reasonableness owns the key normative operating principle
- decision surfaces own points of choice
- decision schema owns record shape
- action proposal schema owns the non-authoritative intent contract
- action mediation owns the runtime authority boundary over proposed action
- delegation/refusal own bounded handoff semantics
- negotiation owns structured disagreement
