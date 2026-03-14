# ADL Design — v0.85

> **Planning note**
> `docs/milestones/v0.85/` is the canonical location for the tracked v0.85 milestone documentation set.
> Related planning artifacts in temporary or local workspaces should be treated as source material only after their content is reconciled into this directory.

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-10`
- Owner: `Daniel Austin / Agent Logic`
- Related issues: `#674, #716, #730, #735, #559, #681`

## Purpose
Define the major design directions for v0.85, with emphasis on bounded operational maturity, dependable execution, verifiable inference, stronger authoring and review surfaces, Adaptive Execution Engine progress, and an initial affective / emotion-model substrate. This document is intended to anchor scattered milestone planning material into one coherent design statement.

## Problem Statement
v0.8 established a strong conceptual and architectural substrate for ADL, but the system still lacks several capabilities required for practical, trustworthy use by serious teams.

The main gaps are:

- execution remains stronger conceptually than operationally, especially for queueing, checkpointing, and distributed work
- review and authoring surfaces are still too manual
- dependable execution and verifiable inference need stronger first-class representation in tooling and documentation
- the Adaptive Execution Engine (AEE) has been conceptually deferred for several releases and must advance materially in v0.85
- the emerging emotion / affect model is not yet represented as a disciplined design surface even though it may become important to bounded cognition, evaluation signals, and agent priority handling

v0.85 therefore exists to convert ADL from a compelling substrate into a more usable, scalable, and trustworthy platform.

A secondary problem is documentation fragmentation itself: milestone intent is currently distributed across multiple directories and partially overlapping planning documents. That fragmentation increases ambiguity around source-of-truth, acceptance criteria, and sequencing. v0.85 should therefore improve not only runtime and authoring maturity, but also milestone-document coherence.

## Goals
- Advance ADL toward a practical execution substrate with deterministic queueing, checkpointing, resumability, and cluster-oriented work distribution.
- Make dependable execution and verifiable inference explicit design principles across runtime, artifacts, and positioning.
- Improve authoring and review ergonomics without weakening structure.
- Move the Adaptive Execution Engine forward as a major milestone theme rather than continuing to defer it.
- Establish a bounded affective / emotion-model design surface that can later influence evaluation, priority handling, and adaptive behavior.
- Reduce milestone-planning ambiguity by making design intent, scope, and validation language consistent across the canonical v0.85 doc set.
- Finish the milestone with an explicit active-surface `swarm` -> `adl` cutover once the rest of the code and review work has stabilized.

## Non-Goals
- Deliver unrestricted autonomy or unconstrained online self-modification.
- Replace the entire existing runtime in one milestone.
- Build a full synthetic psychology system.
- Solve every enterprise or UI concern before v0.9.

## Scope
### In scope
- Deterministic queue and checkpoint design/implementation work.
- Cluster / distributed execution planning and initial execution substrate improvements.
- Prompt Spec completeness and stronger authoring surfaces.
- Card Reviewer GPT stabilization.
- Bounded AEE progress: retry policy, adaptation surfaces, strategy loop integration, experiment lifecycle support.
- Trust surfaces: dependable execution, verifiable inference, artifact provenance, replayability.
- A bounded emotion / affect model design that fits the Gödel–Hadamard–Bayes architecture.
- Positioning and philosophy docs that clarify why ADL chooses stronger guarantees and explicit artifacts.
- Milestone-document alignment across the canonical v0.85 planning artifacts, especially design/scope/validation language.
- Final active-surface identity cutover as documented in `SWARM_REMOVAL_PLANNING.md`, executed only after the major code and review changes for v0.85 are otherwise complete.

### Out of scope
- Full production-grade autonomous self-improvement.
- Fully generalized distributed runtime for arbitrary scale.
- Broad website launch or complete marketing package.
- Final public launch; v0.85 is still a strengthening milestone before the v0.9 base-feature target.
- A full repository-wide information-architecture cleanup for all milestone documents; v0.85 only needs bounded alignment and reduced ambiguity.

## Requirements
### Functional
- ADL must support stronger deterministic execution management, including queue/checkpoint/resume surfaces.
- ADL must improve authoring and validation surfaces for prompts, cards, and workflows.
- ADL must make review and artifact validation more reliable.
- ADL must advance bounded adaptive execution so it is materially closer to operational use.
- ADL must define an initial emotion / affect representation that is compatible with later cognitive work.
- ADL milestone planning documents for v0.85 must present materially consistent scope, trust, AEE, and affect-model intent within the canonical `docs/milestones/v0.85/` directory.

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- Explicit artifact contracts and provenance.
- Verifiable inference surfaces suitable for business-facing trust arguments.
- Bounded scope: every new adaptive or affective mechanism must remain inspectable and reviewable.

## Proposed Design
### Overview
v0.85 is organized around six mutually reinforcing tracks:

1. **Execution substrate** — deterministic queueing, checkpointing, resumability, and cluster execution.
2. **Authoring surfaces** — Prompt Spec completeness, HTML/card-based authoring, validation and linting.
3. **Trust and verification** — dependable execution, verifiable inference, replay bundles, and review surfaces.
4. **Cognitive substrate** — bounded AEE progress, hypothesis/experiment support, and a first affective model.
5. **Operational maturity** — Card Reviewer GPT stabilization, clearer issue/card workflows, and cleaner milestone documentation.
6. **Planning coherence** — tighter alignment of design, WBS, decisions, release, and milestone-checklist language across split documentation locations.

The design principle across all tracks is that ADL should move toward a system whose behavior is increasingly:

- explicit before execution
- reproducible after execution
- reviewable by humans and tools
- bounded in adaptation and failure modes
- documented with less ambiguity at the milestone-planning level

The `swarm` -> `adl` repository cutover is intentionally sequenced at the end of the milestone rather than treated as parallel background churn. The repo still carries many path-sensitive `swarm/...` assumptions, so executing the cutover late reduces merge-conflict pressure while substantive runtime, trust, and authoring work is still landing.

### Interfaces / Data contracts
- **Queue / checkpoint contracts**: deterministic run identity, queue state, retry state, checkpoint state, and resumability invariants.
- **Prompt Spec contracts**: explicit authoring fields for actor, model, inputs, outputs, constraints, and review surfaces.
- **Review artifacts**: machine-readable review outputs and consistent card/output structures.
- **AEE interfaces**: bounded strategy-loop hooks, retry/adaptation policy surfaces, and experiment lifecycle integration points.
- **Emotion / affect surfaces**: explicit state or signal representation for priorities, tensions, and evaluation guidance.
- **Trust surfaces**: replay bundles, provenance markers, validation artifacts, and evidence-linked output structures.
- **Planning contracts**: consistent terminology and scope boundaries across design, WBS, decisions, checklist, and release artifacts.

### Execution semantics
The execution semantics for v0.85 remain centered on deterministic workflow behavior.

New or strengthened behavior should obey the following principles:

- queue and checkpoint behavior must not weaken deterministic replay guarantees
- retry and adaptation must remain bounded, explicit, and policy-driven
- affective or emotional signals must not become hidden nondeterministic state; they must appear as inspectable inputs to evaluation or prioritization
- distributed execution must preserve explicit ownership, claims, and resumability semantics
- authoring improvements must generate more reliable artifacts, not more ambiguous ones
- milestone-planning updates must reduce source-of-truth ambiguity rather than create additional overlapping statements

## Risks and Mitigations
- Risk: v0.85 becomes too large by combining runtime, authoring, trust, and cognition work.
  - Mitigation: keep the milestone focused on bounded maturity gains and push broader autonomy or productization to later releases.
- Risk: AEE scope again slips into future milestones without concrete progress.
  - Mitigation: make AEE a named centerpiece of v0.85 and require concrete deliverables, not only planning language.
- Risk: the emotion model is treated as either too speculative or too vague to matter.
  - Mitigation: define it as a bounded operational substrate for priorities, tensions, and evaluation signals rather than synthetic psychology.
- Risk: authoring and reviewer improvements remain secondary and continue to bottleneck throughput.
  - Mitigation: explicitly prioritize Card Reviewer GPT stabilization and Prompt Spec / authoring improvements as milestone work.
- Risk: trust language becomes rhetorical rather than implemented.
  - Mitigation: tie dependable execution and verifiable inference to concrete artifacts, replay surfaces, and validation workflows.
- Risk: milestone documents drift if updates land outside `docs/milestones/v0.85/` instead of being reconciled into the canonical milestone directory.
  - Mitigation: treat this design doc as the alignment anchor for scope, validation, AEE, trust, and affect-model language until a cleanup pass consolidates file locations.
- Risk: executing the final `swarm` -> `adl` cutover too early creates avoidable branch drift and path-conflict churn across active work.
  - Mitigation: keep cutover work explicitly late in the milestone and gate it on prior code/review stabilization.

## Alternatives Considered
- Option: keep v0.85 narrowly runtime-focused and defer affective/cognitive work.
  - Tradeoff: would simplify scope, but risks missing an important architectural opportunity and continuing to postpone a promising design area.
- Option: push the emotion model entirely to v0.9+.
  - Tradeoff: safer in the short term, but loses the chance to shape the cognitive substrate while foundational interfaces are still being designed.
- Option: prioritize only authoring/UI improvements and defer AEE again.
  - Tradeoff: would improve usability, but would continue a pattern of postponing one of ADL’s most strategically important runtime themes.
- Option: postpone document alignment until after runtime work stabilizes.
  - Tradeoff: saves time in the short term, but increases confusion about milestone intent and makes later cleanup harder.

## Validation Plan
- Checks/tests: milestone issue tree, implementation cards, deterministic validation commands, replay and provenance checks where applicable, reviewer artifact checks, bounded design doc review for the emotion model, and cross-checking of v0.85 planning language across split document locations.
- Success metrics: v0.85 materially improves execution maturity, reviewer reliability, authoring usability, and AEE concreteness; the emotion model is represented as a disciplined design surface rather than an untracked idea.
- Rollback/fallback: if a sub-track becomes too ambitious, reduce it to explicit design docs and bounded artifacts rather than forcing partial uncontrolled implementation.

## Exit Criteria
- Goals/non-goals and scope boundaries are explicit.
- Validation plan is actionable and referenced by the milestone checklist.
- Major open questions are resolved or tracked in the decision log.
- v0.85 clearly advances AEE, reviewer reliability, and authoring maturity.
- The emotion / affect model has a disciplined design artifact and an agreed bounded role in the architecture.
- The major v0.85 planning documents no longer materially contradict one another on scope, trust, AEE, or affect-model intent.
- The milestone positions ADL to enter v0.9 with nearly all base features in place.
