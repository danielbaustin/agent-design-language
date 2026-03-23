# ADL Design — v0.85

> **Planning note**
> `docs/milestones/v0.85/` is the canonical location for the tracked v0.85 milestone documentation set.
> Related planning artifacts in temporary or local workspaces should be treated as source material only after their content is reconciled into this directory.

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-16`
- Owner: `Daniel Austin / Agent Logic`
- Related issues: `#886, #674, #716, #743, #748, #749, #750, #751, #752, #866-#882`

## Purpose
Define the major design directions for v0.85, with emphasis on bounded operational maturity, dependable execution, verifiable inference, stronger authoring and review surfaces, Adaptive Execution Engine progress, Gödel runtime progress, and a minimal working affective-reasoning substrate. This document anchors the revised four-sprint, twenty-five-work-package milestone structure into one coherent design statement.

## Problem Statement
v0.8 established a strong conceptual and architectural substrate for ADL, but the system still lacks several capabilities required for practical, trustworthy use by serious teams.

The main gaps are:

- execution remains stronger conceptually than operationally, especially for queueing, checkpointing, and distributed work
- review and authoring surfaces are still too manual and do not yet provide first-class editors
- dependable execution and verifiable inference need stronger first-class representation in tooling and documentation
- the Adaptive Execution Engine (AEE) has been conceptually deferred for several releases and must advance materially in v0.85
- the emerging bounded affect model is not yet represented as a disciplined working surface even though it may become important to bounded cognition, evaluation signals, and agent priority handling
- the Gödel issue set (`#748` through `#752`) exists, but needs to become a first-class milestone track rather than floating beside the work-package structure

v0.85 therefore exists to convert ADL from a compelling substrate into a more usable, scalable, and trustworthy platform.

A secondary problem is documentation fragmentation itself: milestone intent is currently distributed across multiple directories and partially overlapping planning documents. That fragmentation increases ambiguity around source-of-truth, acceptance criteria, and sequencing. v0.85 should therefore improve not only runtime and authoring maturity, but also milestone-document coherence.

For the cognitive substrate specifically, tracked v0.85 docs now rely on:

- `COGNITIVE_LOOP_MODEL_v0.85.md` as the canonical loop/flow model
- `COGNITIVE_STACK_v0.85.md` as the canonical internal stack/layer model

## Goals
- Advance ADL toward a practical execution substrate with deterministic queueing, checkpointing, resumability, and cluster-oriented work distribution.
- Make dependable execution and verifiable inference explicit design principles across runtime, artifacts, and positioning.
- Improve authoring and review ergonomics without weakening structure, including first real editor surfaces and editing/review GPT assets.
- Move the Adaptive Execution Engine forward as a major milestone theme rather than continuing to defer it.
- Elevate Gödel issues `#748` through `#752` into a central milestone track for deterministic hypothesis generation, bounded adaptation, prioritization, cross-workflow learning, and evaluation-report artifacts.
- Establish a bounded affective / emotion-model surface that is minimal, working, demoable, and able to influence evaluation, priority handling, and adaptive behavior.
- Reduce milestone-planning ambiguity by making design intent, scope, and validation language consistent across the canonical v0.85 doc set.
- Align the live issue graph to the revised twenty-five-work-package milestone structure under the `#886` umbrella.
- Finish the milestone with an explicit active-surface `swarm` -> `adl` cutover once the rest of the code and review work has stabilized.

## Non-Goals
- Deliver unrestricted autonomy or unconstrained online self-modification.
- Replace the entire existing runtime in one milestone.
- Build a full synthetic psychology system.
- Solve every enterprise or UI concern before v0.9.

## Scope
### In scope
- Deterministic queue and checkpoint design/implementation work.
- Queue/checkpoint/steering planning must explicitly treat `#674` as canonical and absorb or supersede placeholder issue `#867`.
- Cluster / distributed execution planning and initial execution substrate improvements.
- Prompt Spec completeness and stronger authoring surfaces.
- First editor surfaces for issue prompts and input/output cards.
- Editing/review GPT stabilization and reusable review assets.
- Bounded AEE progress: retry policy, adaptation surfaces, strategy loop integration, experiment lifecycle support.
- Trust surfaces: dependable execution, verifiable inference, artifact provenance, replayability.
- Gödel runtime progress through issues `#748` through `#752`.
- A bounded affect model that fits the Gödel–Hadamard–Bayes architecture and is concrete enough to demo.
- Reasoning-graph integration with affect and hypothesis behavior.
- A milestone demo program with multiple runnable proof surfaces, including steering/queueing, HITL/editor/review flow, and affect-plus-Gödel behavior.
- Positioning and philosophy docs that clarify why ADL chooses stronger guarantees and explicit artifacts.
- Milestone-document alignment across the canonical v0.85 planning artifacts, especially design/scope/validation language.
- Explicit split of the milestone endgame into docs consistency, internal review, external review, remediation, release ceremony, and next-milestone planning.
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
- ADL must provide at least initial working editor surfaces for issue prompts and input/output cards.
- ADL must make review and artifact validation more reliable.
- ADL must advance bounded adaptive execution so it is materially closer to operational use.
- ADL must advance Gödel runtime behavior through the canonical issue set `#748` through `#752`.
- ADL must define a minimal working bounded affect representation that is compatible with later cognitive work.
- ADL must provide a bounded affect/reasoning/Gödel demo slice that emits legible artifacts.
- ADL milestone planning documents for v0.85 must present materially consistent scope, trust, AEE, and affect-model intent within the canonical `docs/milestones/v0.85/` directory.
- ADL milestone planning documents must present one coherent four-sprint, twenty-five-work-package structure with explicit issue-graph alignment.

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- Explicit artifact contracts and provenance.
- Verifiable inference surfaces suitable for business-facing trust arguments.
- Bounded scope: every new adaptive or affective mechanism must remain inspectable and reviewable.

## Proposed Design
### Overview
v0.85 is organized as a four-sprint, twenty-five-work-package program:

1. **Sprint 1: execution substrate and milestone alignment** — milestone reorganization, deterministic queueing/checkpointing/steering, cluster groundwork, and process cleanup.
2. **Sprint 2: authoring surfaces and review tooling** — Prompt Spec improvements, first editor surfaces, and stronger editing/review GPT workflows.
3. **Sprint 3: Gödel, affect, reasoning graphs, and AEE progress** — deterministic hypothesis generation, bounded adaptive Gödel behavior, experiment prioritization, cross-workflow learning, promotion/eval artifacts, affect engine work, reasoning-graph integration, and a runnable affect/Gödel vertical slice.
4. **Sprint 4: demos, quality, review, release, and next-step planning** — demo program, quality gate, docs consistency, internal review, external review, remediation, release ceremony, and next-milestone planning.

The design principle across all tracks is that ADL should move toward a system whose behavior is increasingly:

- explicit before execution
- reproducible after execution
- reviewable by humans and tools
- bounded in adaptation and failure modes
- documented with less ambiguity at the milestone-planning level

The authoritative loop model for tracked v0.85 docs is intentionally separated into [COGNITIVE_LOOP_MODEL_v0.85.md](/tmp/adl-wp-929/docs/milestones/v0.85/COGNITIVE_LOOP_MODEL_v0.85.md) so milestone design docs do not need to compete over loop authority.

The `swarm` -> `adl` repository cutover is intentionally sequenced at the end of the milestone rather than treated as parallel background churn. The repo still carries many path-sensitive `adl/...` assumptions, so executing the cutover late reduces merge-conflict pressure while substantive runtime, trust, and authoring work is still landing.

The issue-graph rule for this milestone is:

- every work package must map to one canonical issue
- every canonical issue should belong to one work package
- the provisional generated issue set `#866` through `#882` is useful scaffolding, but the canonical milestone structure is the revised twenty-five-work-package model under `#886`

### Interfaces / Data contracts
- **Queue / checkpoint contracts**: deterministic run identity, queue state, retry state, checkpoint state, and resumability invariants.
- **Prompt Spec contracts**: explicit authoring fields for actor, model, inputs, outputs, constraints, and review surfaces.
- **Review artifacts**: machine-readable review outputs and consistent card/output structures.
- **AEE interfaces**: bounded strategy-loop hooks, retry/adaptation policy surfaces, and experiment lifecycle integration points.
- **Emotion / affect surfaces**: explicit state or signal representation for priorities, tensions, and evaluation guidance.
- **Gödel interfaces**: deterministic hypothesis artifacts, promotion/eval artifacts, prioritization traces, and bounded adaptive-loop state.
- **Trust surfaces**: replay bundles, provenance markers, validation artifacts, and evidence-linked output structures.
- **Planning contracts**: consistent terminology and scope boundaries across design, WBS, decisions, checklist, and release artifacts.
- **Demo contracts**: bounded demos for major runtime, editor, and cognitive surfaces, especially steering/queueing, HITL/editor/review flow, and affect-plus-Gödel behavior.

### Execution semantics
The execution semantics for v0.85 remain centered on deterministic workflow behavior.

New or strengthened behavior should obey the following principles:

- queue and checkpoint behavior must not weaken deterministic replay guarantees
- steering behavior must remain replay-compatible and explicitly tied to canonical issue `#674`
- retry and adaptation must remain bounded, explicit, and policy-driven
- bounded affect signals must not become hidden nondeterministic state; they must appear as inspectable inputs to evaluation or prioritization
- Gödel runtime progress must emit inspectable artifacts rather than remain at the level of conceptual narrative
- distributed execution must preserve explicit ownership, claims, and resumability semantics
- authoring improvements must generate more reliable artifacts, not more ambiguous ones
- milestone-planning updates must reduce source-of-truth ambiguity rather than create additional overlapping statements
- cognitive-stack updates must preserve stable layer numbering and avoid fractional-layer semantics

## Risks and Mitigations
- Risk: v0.85 becomes too large by combining runtime, authoring, trust, and cognition work.
  - Mitigation: keep the milestone focused on bounded maturity gains and push broader autonomy or productization to later releases.
- Risk: AEE scope again slips into future milestones without concrete progress.
  - Mitigation: make AEE a named centerpiece of v0.85 and require concrete deliverables, not only planning language.
- Risk: the bounded affect model is treated as either too speculative or too vague to matter.
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
- Option: push the bounded affect model entirely to v0.9+.
  - Tradeoff: safer in the short term, but loses the chance to shape the cognitive substrate while foundational interfaces are still being designed.
- Option: prioritize only authoring/UI improvements and defer AEE again.
  - Tradeoff: would improve usability, but would continue a pattern of postponing one of ADL’s most strategically important runtime themes.
- Option: postpone document alignment until after runtime work stabilizes.
  - Tradeoff: saves time in the short term, but increases confusion about milestone intent and makes later cleanup harder.

## Validation Plan
- Checks/tests: milestone issue tree, implementation cards, deterministic validation commands, replay and provenance checks where applicable, reviewer artifact checks, editor/demo verification, bounded design review for the affect model, and cross-checking of v0.85 planning language across split document locations.
- Success metrics: v0.85 materially improves execution maturity, reviewer reliability, authoring usability, Gödel concreteness, and AEE progress; the affect model is represented as a bounded working substrate rather than an untracked idea.
- Rollback/fallback: if a sub-track becomes too ambitious, reduce it to explicit design docs and bounded artifacts rather than forcing partial uncontrolled implementation.

## Exit Criteria
- Goals/non-goals and scope boundaries are explicit.
- Validation plan is actionable and referenced by the milestone checklist.
- Major open questions are resolved or tracked in the decision log.
- v0.85 clearly advances AEE, reviewer reliability, authoring maturity, and Gödel runtime capability.
- The bounded affect model has a minimal working artifact, a bounded role in the architecture, and a demoable proof path.
- The major v0.85 planning documents no longer materially contradict one another on scope, trust, AEE, Gödel emphasis, editor expectations, demo expectations, or affect-model intent.
- The milestone positions ADL to enter v0.9 with nearly all base features in place.
