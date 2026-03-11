# Design — v0.8

## Metadata
- Milestone: `v0.8`
- Version: `0.8`
- Date: `2026-03-07`
- Owner: `Daniel Austin / Agent Logic`
- Related issues: #517, #559, #609, #610, #611, #612, #613, #614, #615, #616, #618

## Purpose
Define the design for **ADL v0.8**, which moves the project from a deterministic workflow substrate into a **controlled experimentation and authoring milestone**. v0.8 builds on v0.75’s deterministic replay, trace bundles, ObsMem indexing/retrieval, and evidence surfaces in order to introduce:

- the first **Gödel scientific loop** artifacts and workflows
- the first **authoring surfaces** for structured prompt / card driven execution
- a flagship **Rust transpiler / migration demo** that demonstrates adaptive execution, verification, and replayable evidence

The milestone is intended to prove that ADL is not merely a workflow runner, but a framework for **deterministic, auditable, provider-agnostic agent execution and improvement experiments**.

## Problem Statement
v0.75 establishes deterministic traces, replay, coverage gates, ObsMem integration, and demo infrastructure. However, the system still lacks:

- a first-class experimental loop for **hypothesis → mutation → evaluation → decision**
- stable machine-readable artifacts for Gödel-style self-improvement work
- a tooling path from **structured cards → generated prompts → agent execution → review**
- a flagship demo that proves ADL can run a serious engineering workflow with bounded retries and explicit evidence

Without these, ADL risks remaining “just” a deterministic workflow engine rather than becoming a programmable **scientific reasoning and software operations framework**.

## Goals
- Introduce the **Gödel scientific method surface** for ADL using deterministic, replayable artifacts.
- Add the first **machine-readable schemas** for experiments, mutations, and evidence-oriented execution.
- Establish the first **authoring surfaces v1** so structured cards/prompts can drive execution consistently.
- Deliver a flagship **Rust transpiler / migration demo** that showcases adaptive execution, verification, and replayable evidence.
- Keep all new capabilities compatible with v0.75 guarantees: determinism where declared, replayability, security boundaries, and auditability.

## Non-Goals
- Full autonomous online learning or unconstrained self-modification.
- Production-grade distributed or cluster execution.
- Full natural-language-to-ADL compiler with broad language coverage.
- Rich GUI authoring tools or hosted control plane.
- Runtime schema enforcement for every planned Gödel artifact (some remain design-stage in v0.8).

## Scope
### In scope
- Gödel scientific loop design and initial implementation surfaces:
  - ExperimentRecord schema v1
  - Canonical Evidence View
  - Mutation format v1
  - EvaluationPlan v1
  - Gödel experiment workflow template
  - ObsMem indexing for run summaries and experiment records
  - Gödel demo: failure → hypothesis → experiment → adopt/reject
- Authoring surfaces v1:
  - structured cards as first-class execution contracts
  - Prompt Spec block for deterministic prompt generation
  - Verification Summary block for output cards
  - card automation tooling that converts cards to execution prompts
- Rust transpiler / migration demo:
  - fixture crate
  - transformation workflow
  - verification hooks (`fmt`, `clippy`, `test`)
  - bounded retry/recovery hooks using explicit `retry.max_attempts` and `on_error` policy
  - replayable evidence bundle
- Contract hardening required to support the above, including ToolResult metadata and machine-readable validation surfaces.

### Out of scope
- Full semantic / embedding-based retrieval ranking beyond deterministic v0.75 retrieval.
- Unbounded mutation search or unconstrained provider switching.
- Live human approval UI or enterprise console.
- Promotion of all design-stage schemas into runtime-enforced schemas under `swarm/schemas/` if implementation is not yet consuming them.

## Requirements
### Functional
- The system must support a first-class **experiment artifact model** for baseline/variant evaluation.
- The system must support deterministic **canonical evidence comparison** that excludes volatile fields.
- The system must support **bounded, policy-gated mutations** expressed in a stable machine-readable format.
- The system must support **evaluation plans** that run deterministic checks and emit reproducible results.
- The system must support a **card → prompt generation** path that is deterministic for identical cards.
- The system must provide a **Rust transpiler / migration workflow** that can generate a patch, run verification, retry boundedly, and emit evidence artifacts.
- The system must allow ObsMem to index **run summaries** and **experiment records** in a deterministic, privacy-safe way.

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- Replay compatibility for all declared deterministic paths.
- Explicit security/privacy guarantees: no prompts, secrets, tool arguments, or host paths leaked into evidence artifacts.
- Provider-agnostic execution surfaces wherever possible.
- Minimal scope creep: v0.8 should remain a focused milestone, not an attempt to solve every future autonomy problem.

## Proposed Design
### Overview
v0.8 introduces a layered architecture above the v0.75 substrate:

Runtime Execution (ADL)
        ↓
Deterministic Trace + Replay
        ↓
ObsMem Operational Memory (index + retrieval)
        ↓
Gödel Scientific Loop
        ↓
Authoring Surfaces / Prompt Automation

The key idea is that **learning and improvement are implemented as ordinary ADL workflows operating over deterministic artifacts**. Instead of inventing a separate “learning runtime,” ADL uses:

- trace bundles for evidence
- ObsMem for indexed operational memory
- experiment schemas for self-improvement structure
- structured cards/prompts for reproducible authoring

### Interfaces / Data contracts
- **ExperimentRecord v1** — stable record of hypothesis, mutation, evaluation plan, evidence, and decision.
- **Canonical Evidence View** — deterministic representation of a run’s evidence (activation log, failure codes, artifact hashes, verification results) excluding volatile fields.
- **Mutation format v1** — bounded, policy-gated mutation descriptor with explicit scope and patch representation.
- **EvaluationPlan v1** — deterministic check plan (`fmt`, `clippy`, `test`, schema checks, replay checks, demo matrix checks).
- **Prompt Spec block** — machine-readable mapping from card sections to generated prompt structure.
- **Verification Summary block** — machine-readable output summary for automated review and CI validation.
- **ToolResult hardening** — richer metadata and clearer success/error semantics to support experiment evidence and repair loops.

### Execution semantics
#### 1. Gödel experiment semantics
A Gödel experiment is executed as an ADL workflow with the following phases:

1. Retrieve baseline evidence.
2. Select or receive a bounded mutation.
3. Execute variant workflow under deterministic conditions.
4. Run deterministic verification hooks.
5. Compare canonical evidence.
6. Emit ExperimentRecord and adopt/reject decision.

All mutation application must be policy-gated. All comparisons must operate on canonicalized artifacts.

#### 2. Authoring semantics
Input cards become structured execution contracts. A prompt generator deterministically maps card sections into prompt blocks:

- Task
- Success Criteria
- Context
- Execution Constraints
- Architectural Invariants
- Validation Expectations

This enables provider-agnostic, reproducible execution prompt generation and makes future reviewer automation feasible.

#### 3. Adaptive execution semantics
The Rust transpiler demo and related workflows use the Adaptive Execution Engine pattern:

Attempt -> Failure -> Bounded Retry Decision -> Retry -> Verification -> Convergence or Exhaustion

Retry behavior must remain bounded, explicit, and traceable.

#### 4. Demo semantics
The flagship Rust transpiler / migration demo is an ADL workflow that:

- analyzes a fixture crate
- generates a transformation plan and patch
- applies the patch in sandbox
- runs `cargo fmt`, `cargo clippy`, and `cargo test`
- retries using bounded runtime retry/on_error hooks if verification fails
- emits deterministic evidence artifacts and a replayable trace bundle

This demo also serves as the first concrete Gödel substrate because its structure mirrors:

Observe → Hypothesize → Evaluate → Accept/Reject

## Risks and Mitigations
- Risk: v0.8 scope expands too far by mixing Gödel, authoring, demos, and runtime cleanup.
  - Mitigation: keep focus on a narrow “scientific loop + authoring surface + flagship demo” spine; defer non-essential extensions.
- Risk: machine-readable schemas are documented but not actually used by code, creating drift.
  - Mitigation: whenever possible, pair each schema with at least one parser/serializer, example artifact, or consuming workflow.
- Risk: the Rust transpiler demo becomes nondeterministic or too dependent on local environment.
  - Mitigation: use a small fixture crate, pinned toolchain assumptions, bounded strategies, and explicit evidence capture.
- Risk: card automation becomes brittle due to markdown parsing heuristics.
  - Mitigation: use Prompt Spec and Verification Summary blocks as machine-readable anchors.
- Risk: privacy/security regressions through richer artifacts and evidence bundles.
  - Mitigation: enforce redaction and path hygiene in evidence-producing workflows and validate via reviewer checklist / CI.

## Alternatives Considered
- Option: defer all Gödel work until after richer authoring tools exist.
  - Tradeoff: would delay the core differentiator of ADL and postpone the experiment artifact model that other features need.
- Option: implement a separate learning runtime instead of using ordinary ADL workflows.
  - Tradeoff: increases architectural complexity and weakens the “ADL speaks ADL” design principle.
- Option: make the Rust transpiler a v0.75 flagship demo.
  - Tradeoff: would destabilize the v0.75 release and blur the milestone narrative; better fit for v0.8.
- Option: jump directly to GUI authoring surfaces.
  - Tradeoff: creates presentation value but not the deterministic automation backbone needed for long-term reliability.

## Validation Plan
- Checks/tests:
  - Schema round-trip tests for experiment-related artifacts where implemented.
  - Determinism tests for canonical evidence generation, prompt generation, and retrieval ordering.
  - End-to-end tests for the Rust transpiler / migration workflow on a fixture crate.
  - Replay verification for deterministic Gödel and transpiler experiment runs.
  - Card automation tests for Input Card → Prompt generation and Output Card → Verification Summary parsing.
- Success metrics:
  - Experiment artifacts are generated deterministically for identical inputs.
  - Card automation can generate prompts without markdown heuristics beyond the documented schema.
  - Rust transpiler demo completes successfully on the fixture crate and emits replayable evidence.
  - No secrets, prompts, tool arguments, or host paths leak into trace/evidence artifacts.
- Rollback/fallback:
  - Keep design-stage schemas and docs canonical under milestone docs if runtime promotion proves premature.
  - Gate advanced experiment flows and demo features behind explicit milestone/demo configuration until stable.
  - Defer non-essential authoring or automation polish to v0.85/v0.9 if needed.

## Follow-on Backlog Item: Hypothesis Engine for Gödel Agent
A dedicated **Hypothesis Engine for the Gödel agent** remains a tracked future work item. This component will sit between:

Evidence retrieval → hypothesis generation → mutation candidate → evaluation plan → experiment workflow

The hypothesis engine is distinct from:

- ExperimentRecord schema
- Mutation format
- EvaluationPlan
- experiment workflow template

Its role is to decide **what to try next** based on prior evidence, failure patterns, and experiment history.

v0.8 establishes the deterministic experiment, evidence, ObsMem, and authoring substrate that this future hypothesis engine will require.

Treat this as an explicit backlog/follow-on item, not an unbounded v0.8 requirement.

## Exit Criteria
- v0.8 goals, non-goals, and scope boundaries are explicit and consistent with the WBS and sprint plan.
- The Gödel scientific loop has a first concrete artifact/workflow surface, not just prose design.
- The card/prompt automation path is documented and minimally implemented.
- The Rust transpiler / migration demo is specified as a flagship v0.8 deliverable and tied to deterministic evidence/replay.
- Major open questions are either resolved in milestone docs or tracked as explicit v0.8 issues.
