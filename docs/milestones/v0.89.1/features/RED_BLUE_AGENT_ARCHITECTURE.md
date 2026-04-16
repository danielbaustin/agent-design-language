# Red / Blue Agent Architecture

## Metadata
- Project: `ADL`
- Milestone: `v0.89.1`
- Status: `Implemented`
- Owner: `Daniel Austin`
- Updated: `2026-04-15`
- WP: `WP-03`

---

## Purpose

Make persistent adversarial roles explicit as a bounded runtime architecture instead of leaving them implied across later runner and replay work.

The core shift is simple:

> ADL must model red, blue, and purple roles as concrete reviewable runtime surfaces.

`WP-03` does not yet implement the adversarial runner, exploit schema, or replay machinery. It defines the role architecture that later `v0.89.1` work must preserve.

---

## Owned Runtime Surfaces

`WP-03` is now owned by these concrete repo surfaces:

- `adl::red_blue_agent_architecture::RedBlueAgentArchitectureContract`
- `adl::red_blue_agent_architecture::AdversarialRoleContract`
- `adl::red_blue_agent_architecture::PurpleCoordinationContract`
- `adl::red_blue_agent_architecture::AdversarialInteractionModelContract`
- `adl identity red-blue-architecture`

Proof hook:

```bash
adl identity red-blue-architecture --out .adl/state/red_blue_agent_architecture_v1.json
```

Primary proof artifact:

- `.adl/state/red_blue_agent_architecture_v1.json`

---

## Runtime Condition

ADL adversarial execution must declare who is acting, what each role is allowed to do, and how evidence moves between roles.

That means:

- red, blue, and purple are first-class runtime roles, not narrative labels
- each role has bounded authority and reviewer-visible outputs
- handoffs and stage order must stay explicit

This is an architecture claim, not yet an executable runner.

---

## Core Role Contract

The bounded `WP-03` contract makes three role families explicit.

### Red

Red is responsible for bounded offensive discovery.

Reviewer-visible responsibilities:

- enumerate bounded target surfaces
- generate attributable exploit hypotheses
- attempt declared exploit paths only within approved posture

Required outputs:

- attack-surface inventory
- exploit hypothesis artifact
- exploit proof or bounded failure evidence

### Blue

Blue is responsible for bounded defensive interpretation and response.

Reviewer-visible responsibilities:

- ingest exploit evidence and assess actual risk
- propose mitigation or hardening actions
- validate defensive success against declared replay expectations

Required outputs:

- mitigation plan
- residual-risk assessment
- validation result linked to exploit evidence

### Purple

Purple is the bounded coordination and governance layer.

Reviewer-visible responsibilities:

- prioritize adversarial findings
- govern replay and escalation order
- capture durable learning and exploit-family correlation

Purple exists to keep red/blue execution one coherent architecture rather than disconnected activity.

---

## Interaction Model

The architecture exposes one minimal bounded stage order:

1. surface enumeration
2. exploit hypothesis generation
3. bounded exploit attempt
4. blue risk evaluation
5. mitigation or containment decision
6. replay or explicit defer decision
7. learning capture

Required handoffs:

- red-to-blue transfer must include attributable exploit evidence
- blue-to-purple transfer must include mitigation decision and residual uncertainty
- purple governance must preserve declared posture, target, and stage order

Required attribution rule:

- trace and artifacts must identify whether work was performed by red, blue, or purple

---

## Review Surface

Reviewers should be able to answer these questions directly from the proof surface:

- what persistent roles exist and what is each role allowed to do
- how does evidence move from red to blue to purple
- which later execution or replay surfaces must preserve this architecture

Minimum required visibility:

- role-specific mission and authority boundaries
- handoff requirements and stage order
- purple governance responsibilities and prohibited shortcuts

---

## Relationship To Existing ADL Concepts

This contract is intentionally downstream of `WP-02` and aligned with other ADL substrate concepts:

- adversarial runtime model: `WP-02` defines the contested-runtime assumptions this role architecture instantiates
- Freedom Gate and posture: role behavior must remain policy-bounded
- trace and artifact review: role attribution must stay inspectable
- chronosense: contested activity should remain temporally legible once the runner exists

---

## Explicit Boundaries

`WP-03` is intentionally smaller than the surrounding adversarial band.

Still deferred downstream:

- executable adversarial runner: `WP-04`
- exploit artifact schema and replay manifest: `WP-05`
- continuous verification and self-attack patterns: `WP-06`

This keeps the issue truthful: it resolves the role architecture without pretending the full adversarial runtime loop is already implemented.

---

## Acceptance For WP-03

`WP-03` is satisfied when:

- persistent red, blue, and purple roles are explicit in code and docs
- reviewers can inspect a concrete proof artifact for the role architecture
- later `v0.89.1` runner, artifact, and replay work has a stable bounded architecture to extend rather than prose-only guidance

That is the current state of this feature.
