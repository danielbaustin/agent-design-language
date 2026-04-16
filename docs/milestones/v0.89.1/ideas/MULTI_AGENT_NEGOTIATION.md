# Multi-Agent Negotiation

## Status

Draft

## Purpose

Define how ADL should support bounded negotiation, disagreement handling, and
 multi-agent coordination without collapsing into opaque social theater.

---

## Why This Matters

If ADL grows beyond single-agent workflows, it will need explicit patterns for:
- disagreement
- escalation
- consensus
- dissent
- structured coordination across offices or agents

Without a bounded negotiation surface, multi-agent behavior risks becoming difficult to
 audit and difficult to govern.

---

## Core Principle

> Multi-agent coordination should be structured enough to be inspectable, but flexible enough to preserve meaningful disagreement.

---

## Scope

This document defines:
- negotiation as an explicit coordination surface
- bounded disagreement handling
- congressional-principle style coordination

This document does not define:
- final constitutional society mechanics
- full voting law

---

## Negotiation Surface

Negotiation may be needed when:
- agents disagree about action
- multiple offices have standing
- policy and execution goals conflict
- escalation does not immediately resolve the issue

Negotiation should not be treated as unstructured chat.

---

## Congressional Principle

The congressional principle is a useful shorthand for bounded coordination in which:
- multiple parties can state positions
- disagreement is visible
- constraints remain in force
- outcomes are not produced by silent dominance alone

This does not require a literal legislature.
It requires explicit treatment of structured disagreement.

---

## Desired Outcomes

Negotiation may produce:
- consensus
- bounded dissent
- escalation
- refusal
- delegated resolution

The system should preserve which outcome occurred.

---

## Traceability

Negotiation should remain traceable enough to answer:
- who participated?
- what positions were presented?
- what constraints mattered?
- how was the outcome reached?
- what dissent remained unresolved?

---

## Design Constraints

- negotiation must remain bounded
- disagreement must remain visible
- outcomes must bind back to explicit decision records
- negotiation should not erase responsibility boundaries

---

## Non-Goals

This document does not define:
- final parliamentary procedure
- social ontology of citizenship

---

## Adjacent Feature Docs

- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`
- `DELEGATION_AND_REFUSAL.md`
- `REPUTATION_AND_TRUST.md`

---

## Summary

Multi-agent negotiation gives ADL a structured way to represent disagreement and
 coordination rather than hiding it inside unreviewable interaction.

> If multiple agents matter, their disagreements and agreements must become part of the architecture.

