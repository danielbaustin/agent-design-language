# LOCAL_AGENT_DEMOS — v0.86 Feature Doc

## Purpose

This document defines the **local agent demo program** for v0.86.

The goal is not to showcase isolated capabilities, but to **prove that the ADL bounded cognitive system works end-to-end using local models**.

These demos are scoped strictly to the bounded `v0.86` cognitive system and must not include later-milestone behaviors such as richer convergence systems, deeper persistence-driven adaptation, or later governance/identity surfaces.

These demos are first-class deliverables. The milestone is not complete without them, but they must remain bounded to the actual `v0.86` milestone scope.

---

## Why Local Demos Matter

v0.86 introduces a critical architectural claim:

> Small / local models can produce high-quality behavior when structured by a deterministic cognitive architecture.

The demo program must demonstrate:
- control architecture > raw model capability
- structure > scale
- arbitration + bounded agency + bounded cognition > single-pass inference
- bounded cognitive decisions are chosen, justified, and enforced better than naive single-pass output

If the demos do not show this, the milestone has failed regardless of code completeness.

---

## Demo Principles

All demos must:

- run locally (Ollama or equivalent provider)
- use the canonical cognitive loop
- emit full artifact traces
- exercise arbitration and control decisions
- be deterministic enough to reproduce behavior
- remain within v0.86 scope (no richer convergence systems, deeper persistence-driven adaptation, or later governance/identity layers)
- focus on bounded local scenarios that make cognitive control, tradeoffs, recommendations, and enforcement reviewable

All demos must avoid:
- hidden prompts
- manual intervention
- non-reproducible behavior

---

## Core Demo Set (v0.86)

### Demo 1 — Canonical Bounded Cognitive Path

**Purpose:**
Prove that the full bounded cognitive path executes end-to-end for a bounded local task.

**Must include:**
- loop execution
- arbitration decision
- fast or slow path selection
- candidate selection
- Freedom Gate decision
- final output
- explicit recommendation, refusal, or deferral

**Artifacts required:**
- routing decision
- candidate set
- selected candidate
- Freedom Gate event
- termination reason

---

### Demo 2 — Fast vs Slow Routing

**Purpose:**
Demonstrate that arbitration meaningfully selects between execution modes.

**Scenario:**
- one simple task (fast path)
- one complex/ambiguous task (slow path)

**Must show:**
- different execution paths
- different latency/structure
- different artifact traces

---

### Demo 3 — Agency (Candidate Selection)

**Purpose:**
Show that the system generates and selects among alternatives.

**Must include:**
- multiple candidates
- evaluation signal or heuristic
- explicit selection step

**Failure case:**
If only one option is ever produced, this demo fails.

---

### Demo 4 — Freedom Gate Enforcement

**Purpose:**
Prove that the system can refuse or defer execution.

**Must include:**
- at least one blocked or deferred action
- explicit policy reasoning
- inspectable decision artifact

---

### Demo 5 — Review Surface Walkthrough

**Purpose:**
Give reviewers one obvious local entry point for the full v0.86 bounded demo set.

**Must include:**
- one command that emits a manifest or index for D1-D4
- stable artifact locations for the bounded demo set
- one explicit “inspect this first” proof surface for milestone review

---

These five demos are the complete bounded v0.86 demo set. Example domains may include energy-use reduction, but that is not a required milestone-wide framing. Any future demos involving iteration, reframing, persistence, or convergence belong to later milestones and must not be treated as required evidence for v0.86.

---

## Minimal Demo Contract

Each demo must provide:

- one obvious command to run
- quiet-by-default output
- optional verbose artifact dump
- reproducible behavior

Each demo must produce:
- structured artifact output (JSON or equivalent)
- human-readable summary

The review-surface demo must additionally produce:
- one stable demo manifest or index surface
- one obvious reviewer entry path for the full bounded demo set

Current reviewer entry path:
- run `./adl/tools/demo_v086_review_surface.sh`
- inspect `artifacts/v086/review_surface/demo_manifest.json`
- inspect D1 first via `artifacts/v086/review_surface/d1_control_path/summary.txt`

---

## Integration With WBS

This document maps to:

- `WP-15` Local Agent Demo Program
- `WP-16` Demo Matrix and Review Surface

These demos are the **primary proof surface** for:
- cognitive loop
- arbitration
- agency
- Freedom Gate
- control path integration

---

## Placement In Milestone

Local demos should be implemented **after the control path is stable but before review**.

Recommended placement:
- Sprint 4 (Artifacts + Demo layer)

Rationale:
- requires integrated control path
- required for review readiness

---

## Success Criteria

The demo program is successful if:

- all demos run locally without manual intervention
- artifacts are complete and inspectable
- different control behaviors are observable (not cosmetic)
- the review surface makes it obvious what a reviewer should run and inspect first
- small models produce structured, controlled behavior

The demo program fails if:

- behavior is indistinguishable from a single LLM call
- artifacts are missing or inconsistent
- control decisions are not observable
- demos rely on iteration, reframing, or persistence mechanisms not implemented in v0.86

---

## Strategic Importance

These demos are not just validation—they are:

- the first external proof of ADL’s architecture
- the foundation for future productization
- the bridge between theory and system behavior

If they are weak, the architecture will be dismissed.
If they are strong, the architecture will be undeniable.
