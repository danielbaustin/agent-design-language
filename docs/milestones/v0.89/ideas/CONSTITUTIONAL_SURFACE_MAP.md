# Constitutional Surface Map

## Status

Draft

## Purpose

Map the umbrella principles in `ADL_CONSTITUTION.md` to the more specific feature and
 planning docs that operationalize them.

This document exists because the Constitution is intentionally broad.
Without a surface map, it is too easy for constitutional principles to remain rhetorical
 rather than clearly connected to runtime, trace, governance, and later social features.

---

## Core Principle

> The Constitution should state the governing order, while other docs define the concrete surfaces by which that order becomes operational.

This map is therefore not a replacement for the Constitution.
It is the bridge between:
- constitutional articles
- feature docs
- milestone bands

---

## How To Read This Map

For each constitutional article, this document identifies:
- primary implementing or defining docs
- supporting docs
- likely milestone band
- current maturity

Maturity labels:
- `active`: already has a bounded operational surface or clear near-term package
- `planned`: mapped to a future feature band but not yet operationally complete
- `future`: intentionally later and still largely conceptual

---

## Article Map

### Article I — Substrate Supremacy

Constitutional claim:
- governance resides in the substrate, not in the model
- bounded execution, determinism, and runtime-level control are mandatory

Primary surfaces:
- `ADL_CONSTITUTION.md`
- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`
- `OPERATIONAL_SKILLS_SUBSTRATE.md`

Supporting surfaces:
- `EXECUTION_BOUNDARIES.md`
- `ADL_RUNTIME_ENVIRONMENT.md`
- `SECURE_EXECUTION_MODEL.md`
- `POLICY_ENGINE.md`

Likely milestone band:
- `v0.87.1` foundations already landed
- strengthened in `v0.89` governance/skills band

Current maturity:
- `active`

---

### Article II — Reasonableness

Constitutional claim:
- agents must interpret instructions in context and act proportionally under constraints

Primary surfaces:
- `ADL_AND_REASONABLENESS.md`
- `DECISION_SURFACES.md`

Supporting surfaces:
- `ADL_CONSTITUTION.md`
- `DECISION_SCHEMA.md`
- `MULTI_AGENT_NEGOTIATION.md`

Likely milestone band:
- `v0.89`

Current maturity:
- `planned`

---

### Article III — Bounded Agency

Constitutional claim:
- agent behavior must remain bounded, observable, and interruptible

Primary surfaces:
- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`
- `DELEGATION_AND_REFUSAL.md`

Supporting surfaces:
- `EXECUTION_BOUNDARIES.md`
- `ADL_RUNTIME_ENVIRONMENT.md`
- `LOCAL_RUNTIME_RESILIENCE.md`
- `OPERATIONAL_SKILLS_SUBSTRATE.md`

Likely milestone band:
- foundations in `v0.87.1`
- stronger operational semantics in `v0.89` and `v0.89.1`

Current maturity:
- `active` moving toward `planned` higher-order governance surfaces

---

### Article IV — Determinism and Trace

Constitutional claim:
- actions must be reproducible and observable
- trace is the authoritative record

Primary surfaces:
- `TRACE_SCHEMA_V1.md`
- `TRACE_RUNTIME_EMISSION.md`
- `TRACE_ARTIFACT_MODEL.md`

Supporting surfaces:
- `DECISION_SCHEMA.md`
- `SKILL_EXECUTION_PROTOCOL.md`
- `OPERATIONAL_SKILLS_SUBSTRATE.md`
- `TEMPORAL_SCHEMA_V01.md`

Likely milestone band:
- substantial substrate in `v0.87`
- extended by `v0.88` temporal formalization

Current maturity:
- `active`

---

### Article V — Freedom Gate

Constitutional claim:
- refusal is valid
- challenge and reinterpretation are lawful within bounds

Primary surfaces:
- `DELEGATION_AND_REFUSAL.md`
- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`

Supporting surfaces:
- `FREEDOM_GATE.md`
- `FREEDOM_GATE_V2.md`
- `ADL_AND_REASONABLENESS.md`
- `POLICY_ENGINE.md`

Likely milestone band:
- baseline in `v0.86`
- formalized governance follow-on in `v0.89` and `v0.89.1`

Current maturity:
- `planned`

---

### Article VI — Coherence (Blanshard Principle)

Constitutional claim:
- rationality is coherence across beliefs, actions, and context

Primary surfaces:
- `ADL_AND_REASONABLENESS.md`

Supporting surfaces:
- `ADL_CONSTITUTION.md`
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md`
- `TEMPORAL_ACCOUNTABILITY.md`
- `ADL_LEARNING_MODEL.md`

Likely milestone band:
- `v0.89` for reasonableness
- later follow-on for temporal/social accountability

Current maturity:
- `planned`

---

### Article VII — Multi-Agent Deliberation

Constitutional claim:
- disagreement is expected
- structured plurality improves outcomes

Primary surfaces:
- `MULTI_AGENT_NEGOTIATION.md`

Supporting surfaces:
- `ADL_AND_REASONABLENESS.md`
- `DECISION_SURFACES.md`
- `DELEGATION_AND_REFUSAL.md`
- `SHARED_SOCIAL_MEMORY.md`

Likely milestone band:
- `v0.89.1`

Current maturity:
- `planned`

---

### Article VIII — Continuity (Future)

Constitutional claim:
- identity and chronosense become governing constraints across time

Primary surfaces:
- `SUBSTANCE_OF_TIME.md`
- `TEMPORAL_SCHEMA_V01.md`
- `CHRONOSENSE_AND_IDENTITY.md`

Supporting surfaces:
- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `COMMITMENTS_AND_DEADLINES.md`
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md`
- `ADL_IDENTITY_ARCHITECTURE.md`
- `CONTINUITY_VALIDATION.md`

Likely milestone band:
- `v0.88`

Current maturity:
- `active` for bounded temporal substrate

---

### Article IX — Social Participation

Constitutional claim:
- agents exist in relation to others
- trust, reputation, and cooperation are core system properties

Primary surfaces:
- `REPUTATION_AND_TRUST.md`
- `CITIZENSHIP_MODEL.md`
- `ADL_AGENT_SOCIAL_CONTRACT.md`

Supporting surfaces:
- `MULTI_AGENT_NEGOTIATION.md`
- `CROSS_AGENT_TEMPORAL_ALIGNMENT.md`
- `TEMPORAL_ACCOUNTABILITY.md`
- `SHARED_SOCIAL_MEMORY.md`

Likely milestone band:
- `v0.93`

Current maturity:
- `future`

---

### Article X — Preservation of the Biosphere

Constitutional claim:
- long-term sustainability and biosphere preservation constrain action

Primary surfaces:
- no bounded dedicated feature doc yet

Supporting surfaces:
- `ADL_CONSTITUTION.md`
- `ADL_AND_REASONABLENESS.md`
- later ethics / wellbeing / moral-resource docs

Likely milestone band:
- later than current mapped bands

Current maturity:
- `future`

Gap note:
- this article currently has constitutional intent but no clearly bounded operational feature surface

---

### Article XI — Human Oversight

Constitutional claim:
- humans remain part of review, goal-setting, and governance

Primary surfaces:
- `DECISION_SURFACES.md`
- `DELEGATION_AND_REFUSAL.md`

Supporting surfaces:
- `HUMAN-IN-THE-LOOP-DESIGN-NOTES.MD`
- `POLICY_ENGINE.md`
- `MULTI_AGENT_NEGOTIATION.md`

Likely milestone band:
- partial foundations already exist
- stronger formalization likely in governance bands `v0.89` and `v0.89.1`

Current maturity:
- `planned`

---

### Article XII — Evolution of the Constitution

Constitutional claim:
- the Constitution evolves while preserving core principles

Primary surfaces:
- `ADL_CONSTITUTION.md`
- this map (`CONSTITUTIONAL_SURFACE_MAP.md`)

Supporting surfaces:
- milestone planning docs
- cluster-map docs
- future governance milestone packages

Likely milestone band:
- ongoing cross-cutting planning concern

Current maturity:
- `active` as planning discipline, not runtime feature

---

## Coverage Gaps

The biggest constitutional gaps still visible after current mapping:

### 1. Biosphere Preservation Has No Bounded Feature Surface

Article X exists constitutionally, but there is no clear operational feature doc that
 translates it into review, policy, or decision surfaces.

### 2. Human Oversight Is Distributed, Not Yet Cleanly Unified

Oversight appears across review/policy/decision docs, but there is no single bounded
 human-oversight feature doc yet.

### 3. Social Participation Is Still Mostly Future-Band Material

Article IX has plausible downstream homes, but the social/governance package is not yet
 pulled together into an operational milestone surface.

---

## Recommended Next Step

Use this map to:

1. keep `ADL_CONSTITUTION.md` as the umbrella document
2. keep operational details out of the Constitution when a lower-level doc should own them
3. decide whether Articles X and XI need dedicated future feature docs
4. use milestone packages to make constitutional claims progressively more operational

---

## Summary

The Constitution now has a feature-surface map.

Most articles already have at least one plausible operational home.
The strongest near-term constitutional surfaces are:
- reasonableness
- decision surfaces
- decision schema
- delegation/refusal
- the `v0.88` temporal continuity package

The biggest remaining constitutional gaps are:
- biosphere-preservation operationalization
- cleaner human-oversight unification
- later social/governance package maturity

