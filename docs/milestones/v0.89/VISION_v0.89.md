# Vision - v0.89

## Metadata
- Project: `ADL`
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-13`
- Owner: `Daniel Austin`
- Related issues: `#1662` and the official opened `v0.89` implementation issue wave (`#1789` - `#1807`)

## Purpose

Define the milestone-level vision for `v0.89`: what changes at this stage, why it matters, and which strategic pillars it advances.

## Overview

Version `v0.89` is the milestone where ADL evolves from bounded cognition, persistence, and instinct-shaped behavior into governed adaptive behavior with explicit runtime authority boundaries.

This release should strengthen the foundation for:
- AEE 1.0 convergence
- richer runtime judgment through Freedom Gate v2
- explicit decision, action, skill, and security contracts

`v0.89` focuses on **governed adaptive execution**.

The goal is to make ADL more useful to:
- engineers building serious agent runtimes
- reviewers trying to inspect and trust bounded adaptive behavior
- future milestone work that depends on explicit convergence, security, and execution contracts

This milestone should strengthen the architectural pillars of:
- bounded agency
- governed execution
- replayable evidence
- trustworthy adaptive control

This is the milestone where ADL should stop speaking about convergence and governed action as primarily future ideas and start packaging them as explicit platform surfaces.

---

# Core Goals

`v0.89` advances ADL in five major areas:

1. bounded convergence and adaptive stop conditions
2. richer runtime judgment and decision surfaces
3. explicit authority boundaries between cognition and action
4. skill execution as a first-class governed substrate
5. security, trust, and evidence strong enough to support later adversarial and governance work

---

# 1. Bounded Convergence and Adaptive Stop Conditions

`v0.89` improves bounded convergence so the project can treat adaptation as a governed subsystem rather than a retry habit.

Key objectives:
- define explicit convergence and stall states
- require bounded justification for continuing an adaptive loop
- expose progress signals and stop conditions
- make convergence reviewer-legible

These capabilities move the project toward **AEE 1.0 as a real runtime substrate**.

The system should guarantee:
- convergence claims are evidence-bearing
- bounded-out and policy-stop outcomes are explicit
- continuation is justified rather than assumed

---

# 2. Richer Runtime Judgment and Decision Surfaces

Runtime judgment must improve without sacrificing boundedness or replayability.

`v0.89` introduces or improves:
- Freedom Gate v2
- explicit decision surfaces
- explicit decision records
- clearer refusal, defer, escalate, and reroute semantics

The goal is to move from a minimal gate toward **a bounded, inspectable judgment layer**.

These changes should help users:
- understand where the system is actually exercising choice
- review why an action was permitted, blocked, deferred, or rerouted

---

# 3. Explicit Authority Boundaries Between Cognition and Action

A central principle of ADL is **model output is not execution authority**.

The project must not merely serialize tool calls. It must preserve an architectural boundary between proposed action and authorized action.

`v0.89` strengthens this pillar with:
- Action Proposal Schema
- Action Mediation Layer
- decision-policy bindings
- trace-visible authorization outcomes

This work supports the broader principle of **governance in the substrate rather than the model**.

The result should make the project more:
- trustworthy
- inspectable
- portable across provider styles

---

# 4. Skill Execution as a First-Class Governed Substrate

`v0.89` continues development of the operational skills substrate.

The focus remains on **canonical skill definition and invocation protocol**, not on every future composition or learning layer.

Key capabilities:
- stable skill identity and contract language
- explicit invocation lifecycle
- bounded input/output behavior
- trace and review semantics for execution

This milestone should help the project better support:
- deterministic workflow execution over skills
- future action mediation and delegation
- later capability / aptitude / identity work

These improvements should guide the system toward skills as explicit system intelligence rather than workflow folklore.

---

# 5. Security, Trust, and Evidence for Later Bands

To support real-world use, `v0.89` must improve trust and security planning quality.

Important targets include:
- explicit threat boundaries
- declared security posture
- explicit trust assumptions under adversary
- carry-forward clarity into `v0.89.2`

This milestone is not the entire adversarial runtime band. It is the point where the system gets a serious enough security and trust contract that later adversarial proof work has a coherent foundation.

This work should strengthen the development and operating workflow by making it easier to:
- justify bounded adaptive behavior
- inspect authorization and decision boundaries
- package later adversarial runtime work on top of a serious trust contract

---

# Milestone Context

`v0.88` gave ADL stronger persistence, chronosense, bounded agency, and review discipline.

From `v0.90` onward the project will likely shift toward:
- reasoning graph and signed trace
- stronger query/inspection over reasoning and trace artifacts
- later identity, capability, and governance integration

The goal is to have a coherent, inspectable, and governable agent substrate by **`v0.95`**.

`v0.89` therefore focuses on governed adaptive execution before those later bands deepen reasoning and social/governance layers.

---

# Long-Term Direction

ADL is being designed as a deterministic, reviewable, governance-capable agent substrate rather than a loose bundle of prompting tricks.

Its long-term goals include:
- explicit runtime authority boundaries
- durable evidence and provenance
- identity-bearing and governance-capable agents
- trustworthy adaptive behavior under bounded policy

These principles aim to move the project beyond ad hoc agent orchestration toward a serious platform substrate.

---

# Summary

`v0.89` is the milestone where ADL becomes:
- more explicit about governed adaptation
- more legible at points of runtime choice
- stronger in the boundary between cognition and action
- more serious about security and trust as runtime contracts

It strengthens bounded convergence, advances runtime judgment, improves execution authority boundaries, and stabilizes the security/trust story needed for later bands.

These improvements prepare the project for the reasoning, identity, capability, and governance work that follows.

## Exit Criteria
- The milestone's strategic priorities are explicit and internally consistent.
- The five core goal areas are filled with milestone-specific content.
- The vision can be read without requiring implementation details from the design document.
- The long-term direction clearly connects this milestone to the next phase of the roadmap.
