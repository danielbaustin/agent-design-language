# ADL Adversarial Runtime Model

## Metadata
- Project: `ADL`
- Milestone: `v0.89.1`
- Status: `Implemented`
- Owner: `Daniel Austin`
- Updated: `2026-04-15`
- WP: `WP-02`

---

## Purpose

Make the carry-forward adversarial band explicit as a bounded runtime contract instead of leaving it as planning rhetoric.

The core shift is simple:

> ADL must assume continuous intelligent opposition as a first-class operating condition.

`WP-02` does not yet implement the role architecture, runner, exploit artifacts, or replay loop. It defines the contested-runtime assumptions and reviewer-facing guarantees that later `v0.89.1` work must preserve.

---

## Owned Runtime Surfaces

`WP-02` is now owned by these concrete repo surfaces:

- `adl::adversarial_runtime::AdversarialRuntimeModelContract`
- `adl::adversarial_runtime::AdversarialPressureContract`
- `adl::adversarial_runtime::DynamicAttackSurfaceContract`
- `adl::adversarial_runtime::AdversarialRuntimeGuaranteeContract`
- `adl identity adversarial-runtime`

Proof hook:

```bash
adl identity adversarial-runtime --out .adl/state/adversarial_runtime_model_v1.json
```

Primary proof artifact:

- `.adl/state/adversarial_runtime_model_v1.json`

---

## Runtime Condition

ADL treats adversarial pressure as a normal runtime condition, not an exceptional afterthought.

That means:

- valuable systems should be assumed to operate under contest
- attack discovery is expected to be sustained, automated, and reasoning-assisted
- static “secure until reviewed again” assumptions are no longer sufficient

This is a runtime-model claim, not yet a full execution pipeline.

---

## Core Adversarial Pressure Contract

The bounded `WP-02` contract makes three claims explicit:

1. Meaningful weaknesses should be treated as eventually discoverable under sustained automated reasoning pressure.
2. Security review must remain legible under contested conditions rather than relying on hidden hope, obscurity, or schedule gaps.
3. Later adversarial execution surfaces must preserve traceability, attribution, posture visibility, and policy-bounded operation.

Legacy assumptions explicitly displaced by this model:

- obscurity is sufficient protection
- manual pentest cadence is the dominant discovery path
- adversarial behavior is exceptional rather than an always-possible runtime condition

---

## Dynamic Attack Surface Model

The attack surface is modeled as a dynamic graph rather than a static checklist.

Reviewer-visible dimensions:

- current system state
- available actions and interfaces
- temporal conditions and recurrence
- policy and posture constraints

This is the main structural claim of `WP-02`: adversarial reasoning happens against changing state, changing posture, and bounded authority, not against a frozen list of endpoints.

---

## Runtime Guarantees

Any later adversarial runtime lane in ADL must preserve these guarantees:

- adversarial activity is traceable
- security-relevant behavior is attributable to declared configuration and role context
- adversarial work remains policy-bounded
- replay and review surfaces are explicit rather than implied

Required evidence posture for this band:

- posture and target scope must be reviewer-visible
- contested execution must link to trace or artifact references
- replay expectations must be declared as strict, bounded, or deferred

Prohibited shortcuts:

- unobserved adversarial execution
- non-attributable mitigation claims
- hidden escalation from review surface into exploit automation

---

## Relationship To Existing ADL Concepts

This contract is intentionally aligned with already-settled `v0.89` substrate concepts:

- `Chronosense`: contested behavior unfolds over time and should remain temporally legible
- execution posture and Freedom Gate: adversarial pressure must stay governed, not free-roaming
- trace and artifact review: runtime claims must remain inspectable
- instinct and routing pressure: later work may bias prioritization under threat, but only visibly and within policy

---

## Review Surface

Reviewers should be able to answer these questions directly from the proof surface:

- what contested-runtime assumption is ADL making
- how is the attack surface modeled conceptually
- which guarantees must later runner, artifact, and replay work preserve

Minimum required visibility:

- continuous adversarial pressure assumption
- dynamic attack-surface graph model
- boundedness and evidence requirements

---

## Explicit Boundaries

`WP-02` is intentionally smaller than the surrounding adversarial band.

Still deferred downstream:

- persistent red / blue / purple role architecture: `WP-03`
- adversarial execution runner: `WP-04`
- exploit artifact schema and replay manifest: `WP-05`
- continuous verification and self-attack loops: `WP-06`

This keeps the current issue truthful: it resolves the runtime-model contract without pretending that the whole adversarial subsystem already exists.

---

## Acceptance For WP-02

`WP-02` is satisfied when:

- the contested-runtime model is explicit in code and docs
- reviewers can inspect a concrete proof artifact for the runtime contract
- later `v0.89.1` work has a stable bounded contract to extend rather than a prose-only carry-forward note

That is the current state of this feature.
