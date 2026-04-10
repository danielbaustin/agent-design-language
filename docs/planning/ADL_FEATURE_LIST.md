# ADL Feature List, Status, and Completion Schedule

## Purpose

This document is the durable feature inventory for ADL.

It answers three planning questions:

- which essential ADL features are already implemented
- which currently implemented features were missing from the earlier feature
  narrative
- when the remaining features are scheduled to become complete enough for the
  `v0.95` MVP

The tone of the feature descriptions is intentionally strong, but the status
language is conservative. A feature is only marked fully implemented when the
repo already contains a material working surface for it; bounded demos, early
surfaces, and planning-only work are called out as such.

## Status Legend

- **Implemented**: materially present in the repo with working code, artifacts,
  docs, and/or demo/proof surfaces.
- **Implemented baseline**: a real bounded version exists, but later milestones
  are scheduled to deepen, harden, or integrate it.
- **Partially implemented**: some enabling surfaces exist, but the feature is
  not yet complete enough to count as a finished platform capability.
- **Planned**: currently primarily planning/specification work.
- **Deferred**: recognized, but not required for the `v0.95` MVP unless later
  explicitly promoted.

## Current Repo Status

As of the current planning state, the repo describes `v0.87` as the most
recently completed milestone and `v0.87.1` as the next planned milestone. The
feature list below therefore treats `v0.7` through `v0.87` as the implemented
baseline and uses the `ROAD_TO_v0.95` milestone sequence for remaining work.

## Feature Status Matrix

| Feature | Status now | Current proof surface | Completion target |
| --- | --- | --- | --- |
| Deterministic workflow execution | Implemented | Runtime/CLI, examples, v0.7+ docs | Complete baseline |
| ExecutionPlan runtime | Implemented | Rust runtime and plan execution | Complete baseline |
| Sequential and fork/join coordination | Implemented | Examples, tests, demo docs | Complete baseline |
| Bounded concurrency and retry/failure controls | Implemented | Runtime semantics, tests, v0.7 docs | Complete baseline |
| Run artifacts and replay-oriented inspection | Implemented baseline | Run artifacts, traces, demo/review docs | Deepen through v0.90 |
| Signing, verification, and trust policy | Implemented baseline | Signing/verification surfaces, trust docs | Deepen through v0.90 |
| Remote/provider execution baseline | Implemented baseline | Remote execution MVP, provider docs | Deepen through v0.87 and v0.92 |
| HITL pause/resume | Implemented baseline | v0.7/v0.85 proof surfaces | Integrate through v0.95 |
| Structured authoring model | Implemented baseline | STP/SIP/SOR contracts, prompt tooling | Deepen through v0.95 |
| Control-plane lifecycle | Implemented baseline | `pr init/create/start/run/finish` surfaces | Harden through v0.87.1 and v0.95 |
| HTML/editor command surfaces | Implemented baseline | Editor docs, adapter/demo surfaces | Deepen through v0.95 |
| Review and validation surfaces | Implemented baseline | Reviewer contracts, validation tools | Deepen through v0.95 |
| Task-bundle workflow | Implemented baseline | Public task records and editor docs | Deepen through v0.95 |
| Bounded Godel loop | Implemented baseline | v0.8 runtime artifacts and demos | Expand through v0.89 |
| ObsMem indexing/retrieval | Implemented baseline | v0.8/v0.87 demos and shared-memory docs | Deepen through v0.87+ |
| Bounded cognitive path | Implemented baseline | v0.86 cognitive demo/artifacts | Deepen through v0.88+ |
| Trace v1 substrate | Implemented baseline | v0.87 trace docs and proof surfaces | Signed/query completion in v0.90 |
| Provider/transport substrate | Implemented baseline | v0.87 provider portability docs | Capability maturation in v0.92 |
| Operational skills substrate | Implemented baseline | v0.87 skills/control-plane docs | Harden through v0.87.1 |
| Shared ObsMem foundation | Implemented baseline | v0.87 shared-memory docs | Deepen with persistence/identity |
| Runtime environment and lifecycle completion | Partially implemented | v0.87.1 planning and runtime docs | v0.87.1 |
| Local runtime resilience / Shepherd preservation | Partially implemented | v0.87.1 planning docs | v0.87.1 |
| Persistence, instinct, aptitudes, bounded agency | Planned | v0.88 planning docs | v0.88 |
| AEE 1.0 convergence | Partially implemented | AEE recovery/demo surfaces and planning | v0.89 |
| Freedom Gate v2 | Implemented baseline | v0.86 Freedom Gate proof surfaces | v0.89 |
| Reasoning graph baseline | Partially implemented | planning/schema/proof surfaces | v0.90 |
| Signed trace and trace query | Planned | roadmap/planning docs | v0.90 |
| Affect, moral cognition, kindness, humor | Planned | v0.91 planning docs | v0.91 |
| Identity, capability, names, chronosense | Planned | v0.92 planning docs | v0.92 |
| Governance, delegation, IAM, social contract | Planned | v0.93 planning docs | v0.93 |
| Distributed execution substrate | Partially implemented | cluster groundwork/planning | v0.94/v0.95 integration |
| Demo catalog and MVP walkthrough | Partially implemented | milestone demo matrices | v0.95 |
| Tooling Rust migration | Partially implemented | shell-heavy control plane plus Rust runtime | v0.95 |
| Zed integration | Deferred | planning docs only | Post-v0.95 unless promoted |

## Features Fully Implemented Today

### Deterministic Workflow Execution

ADL already turns agent workflows into explicit, deterministic execution plans
rather than fragile prompt chains. This is the foundation that makes the rest
of the platform credible: users can inspect what will run, reason about the
order of execution, and trust that the runtime is not improvising hidden
behavior.

### ExecutionPlan Runtime

The runtime already compiles structured workflow definitions into an
ExecutionPlan and executes against that plan. This gives ADL the feel of a real
language/runtime pair rather than a loose collection of scripts.

### Sequential and Fork/Join Coordination

ADL already supports the essential workflow coordination patterns needed for
serious agent orchestration. Sequential work, fork/join execution, deterministic
ready ordering, and bounded coordination allow agent systems to scale beyond a
single prompt while still remaining legible.

### Bounded Concurrency and Failure Controls

The runtime already includes bounded concurrency, retry, and failure-policy
surfaces. These controls are crucial because they keep automation powerful
without letting it become operationally reckless.

## Implemented Baselines That Continue to Deepen

### Replay-Oriented Runs and Inspectable Artifacts

ADL already emits stable run artifacts and review surfaces that make execution
inspectable after the fact. This is implemented as a strong baseline, while the
later signed-trace and query work will make the inspection story even more
authoritative by `v0.90`.

### Signing, Verification, and Trust Policy

ADL already has signing and verification surfaces that make authenticity part
of the execution model. The current implementation is real, and later signed
trace work is scheduled to deepen this from workflow-level trust into a richer
provenance stack.

### Remote and Provider Execution Baseline

ADL already has remote/provider execution surfaces and early trust-policy
guardrails. Provider and transport correctness are now being treated as a
platform substrate, with v0.87 and v0.92 carrying the deeper portability and
capability-contract work.

### Human-in-the-Loop Control

ADL already supports bounded human pause/resume and review-oriented control
surfaces. The baseline is present; the long-term goal is to integrate HITL
cleanly across the full authoring, execution, review, and governance story.

### Structured Authoring Model

ADL already has a real structured authoring model around artifacts such as STP,
SIP, and SOR. This gives the platform a professional foundation for prompt
engineering, issue work, implementation handoff, and execution records.

### Control-Plane Lifecycle

The `pr init`, `pr create`, `pr start`, `pr run`, and `pr finish` lifecycle is
now a real ADL control-plane surface. It is implemented enough to support the
authoring workflow, while `v0.87.1` and `v0.95` are scheduled to harden the
runtime environment and migrate the highest-risk shell-heavy tooling.

### HTML Editor and Editor-Adapter Surfaces

ADL already has bounded editor surfaces and command-adapter proof paths. This
is not yet the final product-quality editor, but it is a real baseline that
proves the authoring model can become usable rather than remaining a raw-file
discipline.

### Review and Validation Surfaces

ADL already includes reviewer contracts, prompt/card validation, provenance
surfaces, and structured review artifacts. This is a powerful baseline because
it makes review part of the system rather than a purely manual afterthought.

### Task-Bundle Workflow

ADL already groups task work into structured bundles and public records. The
current form is an early slice, but it establishes the essential unit of work:
design intent, implementation prompt, and output record stay linked rather than
drifting apart.

### Bounded Godel Loop

ADL already includes bounded Godel-style scientific loop surfaces with
canonical artifacts and runnable proof paths. The baseline exists today, while
the roadmap schedules stronger AEE/Godel convergence through `v0.89`.

### ObsMem Indexing and Retrieval

ADL already has ObsMem indexing and retrieval demo surfaces, plus the first
shared-memory direction from v0.87. This is the memory foundation that later
persistence, identity, and continuity features will build on.

### Bounded Cognitive Path

ADL already has a bounded cognitive-system proof path from `v0.86`. The current
implemented baseline covers signals, candidate selection, arbitration,
reasoning, bounded execution, evaluation, reframing, memory participation, and
Freedom Gate behavior.

### Trace, Provider, Shared Memory, and Skills Substrate

ADL now includes v0.87 substrate work around trace, provider portability,
shared ObsMem, operational skills, and reviewer-facing proof surfaces. These
were missing from the older feature-list narrative, but they are essential
implemented platform features because later AEE, identity, governance, and
runtime-completion work depend on them.

## Implemented Features Missing From the Earlier Feature List

The earlier feature narrative was strong, but it underrepresented several
currently implemented or materially present surfaces:

- **Trace v1 substrate**: v0.87 makes trace and artifact inspection a core
  platform surface rather than a generic replay note.
- **Provider/transport portability**: provider/model/transport separation is a
  distinct platform capability, not just remote execution.
- **Operational skills substrate**: ADL now has skill/control-plane planning and
  proof surfaces that matter for repeatable operator workflows.
- **Five-command control plane**: `pr init/create/start/run/finish` deserves
  explicit feature-list treatment because it is the spine of the authoring and
  execution lifecycle.
- **Runtime environment and lifecycle work**: v0.87.1 makes runtime
  environment, lifecycle, resilience, and operator surfaces a first-class next
  feature band.
- **Freedom Gate baseline**: this is more specific than generic cognitive
  control and should be named because it is one of ADL's clearest bounded-agency
  mechanisms.
- **Demo catalog discipline**: ADL treats demos as a feature-legibility system,
  not only as release evidence.

## Planned Features and Completion Schedule

### v0.87.1 - Runtime Completion

The next milestone completes the local runtime environment, lifecycle,
resilience, operator, state, and review surfaces. This is where ADL turns the
recent substrate work into a stronger runtime completion story that can support
larger cognitive and governance layers.

### v0.88 - Persistence, Instinct, Aptitudes, and Bounded Agency

`v0.88` is scheduled to make persistence over time, chronosense, instinct,
aptitudes, and bounded agency more concrete. This is the point where ADL should
start to feel less like isolated execution and more like an agent substrate
with continuity and shaped priorities.

### v0.89 - AEE 1.0 and Freedom Gate v2

`v0.89` is scheduled to deliver AEE 1.0 convergence under explicit security and
threat constraints. It should also strengthen Freedom Gate behavior so bounded
adaptation has a clear action boundary.

### v0.90 - Reasoning Graph, Signed Trace, and Trace Query

`v0.90` is scheduled to turn reasoning and provenance into a stronger
inspectable stack. The key outcomes are reasoning-graph baseline, signed trace
completion, and query/inspection surfaces over reasoning and trace artifacts.

### v0.91 - Affect and Moral Cognition

`v0.91` is scheduled for affect, moral cognition, kindness, humor/absurdity,
and related evaluation surfaces. This is where ADL's cognitive architecture
becomes more emotionally legible and normatively serious without pretending to
be unconstrained synthetic psychology.

### v0.92 - Identity, Capability, Names, and Chronosense

`v0.92` is scheduled to introduce first-class identity, model/provider
capability contracts, stable names, and continuity hooks. This is the bridge
from bounded cognitive behavior to identity-bearing agents that can maintain a
coherent history across runs.

### v0.93 - Governance, Delegation, IAM, and Social Contract

`v0.93` is scheduled to turn identity substrate into accountable governance.
The feature band includes IAM, policy surfaces, constitutional delegation,
rights/duties, social contract surfaces, and governed autonomy boundaries.

### v0.94 - Integration and Dependency-Gap Closure

`v0.94` is the final gap-closure band before MVP freeze. It should be used for
integration, distributed-substrate closure, cross-cutting dependency cleanup,
and already-implied MVP work rather than new architectural domains.

### v0.95 - MVP Convergence and Feature Freeze

`v0.95` is the MVP convergence point. It is scheduled for polished demos,
platform convergence, tooling migration of the highest-risk surfaces, optional
Zed carry-in if explicitly promoted, and the `1.0` scope freeze.

## Deferred Feature

### Zed Integration

Zed integration is recognized as valuable, but it is not currently required for
the `v0.95` MVP. HTML/editor surfaces remain the required editor path unless a
later explicit decision promotes Zed into the must-have set.

## Summary

ADL already has a serious implemented foundation: deterministic execution,
structured coordination, trust surfaces, authoring/control-plane workflows,
bounded Godel loops, ObsMem, bounded cognition, trace/provider/shared-memory
substrate, and reviewer-facing proof surfaces.

The remaining work through `v0.95` is not random feature expansion. It is a
planned convergence path: complete the runtime, deepen persistence and agency,
finish AEE, make reasoning and trace inspectable, add affect and moral
cognition, establish identity and governance, close integration gaps, and land
the MVP with polished demos and a disciplined feature freeze.
