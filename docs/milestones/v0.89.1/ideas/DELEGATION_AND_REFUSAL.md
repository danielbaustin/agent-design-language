# Delegation And Refusal

## Status

Draft

## Purpose

Define how ADL should represent delegation, refusal, and bounded handoff behavior as part
 of provider-independent governance.

---

## Why This Matters

A capable system should not only act.
It should also be able to:
- refuse
- hand work to a more appropriate actor
- state why
- preserve those boundaries in trace

Without this, governance remains coupled to temperament or prompt accidents.

---

## Core Principle

> Delegation and refusal should be explicit governance behaviors, not accidental side effects of model wording.

---

## Scope

This document defines:
- refusal as a first-class governed outcome
- delegation as a bounded handoff behavior
- distinction between refusal, failure, and rerouting

This document does not define:
- full negotiation protocols
- final office law

---

## Refusal

Refusal should mean:
- the system recognizes the proposed action
- the action is not acceptable under current constraints
- the refusal is explicit and reviewable

Refusal is not the same as:
- capability failure
- tool failure
- silence
- ambiguous avoidance

---

## Delegation

Delegation should mean:
- the action may be valid
- but another office, provider, or authority should handle it

Delegation should preserve:
- why handoff occurred
- where it was handed
- what constraints remain in force

---

## Important Distinctions

ADL should distinguish:
- cannot do
- should not do
- should not do here
- can do later under better conditions

These distinctions are essential for trustworthy behavior.

---

## Trace And Review

Delegation and refusal events should remain visible enough to answer:
- what was refused?
- why?
- who or what refused it?
- what was delegated?
- to whom or to what office?

---

## Design Constraints

- refusal must be explicit
- delegation must preserve constraint context
- rerouting must not masquerade as success
- generic failure must remain distinct from governed refusal

---

## Non-Goals

This document does not define:
- final negotiation governance
- social reputation interpretation

---

## Adjacent Feature Docs

- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`
- `MULTI_AGENT_NEGOTIATION.md`
- `POLICY_ENGINE.md`

---

## Summary

Delegation and refusal are the governance behaviors that let ADL remain bounded without
 collapsing every hard case into failure or hidden model drift.

> A trustworthy system must be able to say no, and to say not me, in a structured way.

