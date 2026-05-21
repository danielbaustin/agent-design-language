# ADL Logistic Split

## Status

Draft strategic thesis; not yet accepted tracked roadmap policy.

This document captures an emerging post-`v0.95` strategy direction. It should be treated as planning input until a tracked follow-on issue and canonical roadmap document define the operational boundary and governance model.

## Purpose

Define the proposed candidate bifurcation of the ADL project after `v0.95`.

This is not a retreat from the larger ADL vision. It is the planned transition from rapid architectural discovery into stable substrate engineering, while preserving a separate high-velocity path for continued cognitive-systems research.

This document explains:
- why ADL should stabilize after `v0.95`
- why a separate CSM-oriented repository should continue rapid cognitive-systems research
- how this transition follows the logistic innovation curve already visible in the roadmap
- why this split improves enterprise trust, investor clarity, and architectural coherence

---

## Core Thesis

As an interpretive planning frame, ADL currently appears to sit in the rapid-expansion phase of a classic logistic innovation curve.

Early milestones focused on establishing:
- deterministic execution
- replayability
- trace
- governance
- bounded orchestration
- execution infrastructure

Beginning around `v0.7`, the project accelerated rapidly as larger cognitive-system ideas emerged:

- cognitive arbitration
- chronosense
- bounded agency
- instinct and affect
- identity continuity
- constitutional governance
- persistent memory
- Cognitive Spacetime concepts
- polis and manifold structures

This acceleration was necessary.

The architecture could not be discovered in advance. It had to emerge through iterative synthesis.

The resulting architecture is significantly larger and more coherent than the original project conception. ADL evolved from a deterministic orchestration framework into a broader cognitive substrate concerned with memory, continuity, identity, governance, bounded agency, and shared cognitive reality.

However, the same conceptual velocity that benefits research eventually becomes difficult for:
- enterprise adopters
- reviewers
- partners
- investors

to follow coherently.

---

## The Problem

The current ADL roadmap contains two increasingly distinct modes of development:

### 1. Stable substrate engineering

This includes:
- deterministic execution
- replay and trace
- policy/governance
- runtime orchestration
- provider abstraction
- execution infrastructure
- identity and trust surfaces

This category benefits from:
- slower change
- stability
- compatibility
- enterprise hardening
- operational maturity

### 2. Frontier cognitive-systems research

This includes:
- Cognitive Spacetime
- polis architecture
- persistent citizens
- chronosense expansion
- societal cognition
- manifold structures
- cognitive world-models
- long-lived identity continuity

This category benefits from:
- rapid iteration
- conceptual freedom
- experimentation
- aggressive architectural evolution

These two modes now have different optimal velocities.

Trying to force both modes into a single repository indefinitely would eventually create several pathologies:

- enterprise instability
- roadmap opacity
- reviewer fatigue
- investor confusion
- excessive architectural churn
- reduced operational trust

At sufficient scale, the platform would begin to appear incoherent from the outside even while becoming more internally sophisticated.

---

## Planned Transition After v0.95

`v0.95` should act as the intentional convergence and stabilization point for ADL as an enterprise-grade substrate.

After `v0.95`:

- the core ADL repository should evolve much more slowly
- new architectural domains should be admitted only rarely
- compatibility and governance become dominant priorities
- replayability, auditability, and operational trust become central
- ADL increasingly functions as stable infrastructure

This transition is intentional.

The purpose of `v0.95` is not merely feature completion. It is architectural convergence:
- the point where the platform shape becomes stable enough for enterprise adoption
- the point where semantics become durable
- the point where operational trust becomes more important than rapid conceptual expansion

This document should not be interpreted as declaring the `v0.95` boundary permanently closed today.

Before the stabilization posture becomes binding, ADL still requires one final explicit intake and normalization pass for architectural work that is already visible but not yet fully reconciled into the tracked roadmap.

Examples include:
- WebSocket/runtime transport work
- remaining provider/runtime integration surfaces
- any remaining substrate-level security or governance gaps discovered before freeze

The purpose of the split is not to prematurely halt discovery.
It is to ensure that discovery eventually converges into a stable substrate.

---

## Intended Division

The exact operational boundary between ADL and CSM is intentionally deferred until the post-`v0.95` planning phase.

This document defines the strategic direction, not the final implementation mechanics.

A future tracked planning document should define:
- repository boundaries
- dependency direction
- release cadence
- API and compatibility contracts
- issue migration rules
- ownership boundaries
- documentation split strategy
- naming and public positioning
- governance coordination between repositories

## ADL

ADL becomes the stable governed execution substrate for cognitive systems.

It should increasingly be viewed as:
- infrastructure
- runtime substrate
- governed execution environment
- enterprise cognitive operating layer

---

## CSM

CSM becomes the high-velocity cognitive-systems research layer built atop ADL.

Primary concerns:
- persistent cognitive spacetime
- polis architecture
- societal cognition
- long-lived agent continuity
- shared memory ecologies
- chronosense expansion
- bounded emergent behavior
- manifold/world-model structures
- cognitive civilization research

The intent is not to move all advanced cognition work out of ADL immediately.

The current roadmap through `v0.95` already contains many CSM-adjacent concepts that still belong inside the convergence phase of the main substrate.

The split becomes operational after the convergence boundary, not before it.

CSM remains intentionally experimental.

Its purpose is to explore the frontier territory that would be too unstable, too speculative, or too high-velocity to merge continuously into the enterprise substrate.

The goal is:
- rapid discovery
- architectural experimentation
- cognitive frontier exploration

---

## This Is Not A Fork

This transition should not be described as:
- abandoning ADL
- replacing ADL
- splitting the company

Instead:

- ADL matures into infrastructure
- CSM becomes the frontier research layer built atop that infrastructure

The relationship is analogous to:
- stable runtime infrastructure
versus
- advanced cognitive experimentation

The projects remain related and interoperable.

CSM depends on ADL.

ADL does not depend on the rapid evolution of CSM.

That asymmetry is intentional and healthy.

---

## Logistic-Curve Interpretation

The current roadmap can be interpreted as following a logistic innovation curve. This should currently be treated as a strategic and architectural framing rather than a formally measured quantitative model:

### Early Phase
Slow substrate formation:
- execution
- orchestration
- trace
- governance
- replay

### Expansion Phase
Rapid architectural acceleration beginning around `v0.7`:
- cognition
- identity
- chronosense
- bounded agency
- manifold concepts
- Cognitive Spacetime

### Convergence Phase
Planned stabilization approaching `v0.95`:
- integration
- demos
- hardening
- operational coherence
- launch readiness

Evidence supporting this interpretation includes:
- rapid growth in milestone scope after `v0.7`
- increasing absorption of TBD/planning documents into tracked roadmap structure
- the emergence of multiple cross-cutting cognitive substrate domains
- explicit convergence and freeze language already present in the `v0.95` planning package
- the growing distinction between stable substrate work and frontier cognitive-systems experimentation

A future tracked planning phase should determine whether this framing can be supported with stronger quantitative evidence such as milestone growth metrics, feature-count expansion, roadmap convergence analysis, or planning-surface compression over time.

The transition after `v0.95` is therefore not:
- stagnation
- slowdown due to exhaustion
- loss of innovation

It is:
- intentional convergence
- stabilization
- platform maturation

The important point is that maturation should not be mistaken for stagnation.

Some of the most valuable software systems in history became slower-moving precisely because they succeeded in becoming trusted infrastructure.

---

## Why This Matters

### Investor Clarity

Investors can understand ADL as:
- stable infrastructure
- governed execution substrate
- enterprise runtime platform

rather than:
- an indefinitely mutating research project

That distinction materially improves:
- strategic legibility
- valuation clarity
- partnership confidence
- enterprise procurement confidence

### Enterprise Trust

Customers need:
- stable semantics
- predictable upgrade paths
- operational continuity
- replayability
- governance guarantees

Rapid conceptual churn weakens confidence.

### Research Freedom

CSM can continue evolving aggressively without destabilizing:
- enterprise contracts
- compatibility
- operational guarantees

### Risks And Mitigations

The split also introduces real risks that must be managed explicitly.

Potential risks include:
- brand fragmentation
- duplicated governance or architectural documents
- API drift between ADL and CSM
- unclear ownership boundaries
- research gravity pulling attention away from substrate hardening
- excessive conceptual bleed-back from CSM into the stabilized substrate

These risks are manageable, but only if the split is treated as a governed architectural transition rather than an informal repo fork.

The intended mitigation strategy is:
- ADL remains the authoritative substrate
- CSM depends on ADL rather than the reverse
- compatibility boundaries become explicit
- governance and replay semantics remain centralized in ADL
- stabilization work receives dedicated prioritization rather than competing continuously with frontier experimentation

### Architectural Coherence

The roadmap already implicitly separates:
- execution substrate concerns
from:
- cognitive civilization concerns

This transition simply makes that separation explicit.

The split therefore clarifies the architectural stack:

- ADL = governed substrate
- CSM = cognitive civilization layer

---

## Long-Term Structure

The intended long-term structure is:

### ADL
Stable runtime substrate

### CSM
Advanced cognitive civilization layer built atop ADL

Together they form:
- a governed execution foundation
plus
- a frontier cognitive architecture research environment

Operationally, the intended dependency direction is:

CSM -> ADL

not:

ADL -> CSM

This preserves substrate stability while still allowing aggressive experimentation above the substrate layer.

---

## Summary

The ADL → CSM transition is a planned maturation step.

After `v0.95`:

- ADL stabilizes
- CSM accelerates

This allows:
- enterprise trust
- investor clarity
- operational maturity
while still preserving:
- rapid cognitive-systems innovation

The split is therefore not fragmentation.

It is convergence.

ADL becomes the durable substrate.

CSM becomes the frontier.

Together they allow the project to mature without losing its capacity for discovery.

---

## Required Follow-On Planning

This document establishes the strategic thesis.

A future tracked planning issue should define the operational implementation of the split.

This follow-on issue is required before the thesis should be treated as canonical roadmap policy.

Suggested future planning artifact:

`POST_V095_ADL_CSM_LOGISTIC_SPLIT_PLAN.md`

Recommended scope:
- repository boundary definition
- dependency contract
- migration and extraction rules
- roadmap ownership after `v0.95`
- naming and positioning strategy
- governance coordination
- compatibility policy
- final pre-freeze intake review
- release and cadence policy
- documentation ownership model

The purpose of that planning phase is not to accelerate the split prematurely.

Its purpose is to ensure that the eventual transition remains coherent, governable, and strategically legible.
