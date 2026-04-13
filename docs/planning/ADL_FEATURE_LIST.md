# ADL Feature List

## Purpose

This document is the canonical ADL capability overview.

It answers four practical questions:

- what ADL already does today
- what is active in the current milestone
- which major platform bands are planned next
- how the project is expected to converge by `v0.95`

The tone should be strong because ADL has already become a substantial system.
The status language should remain strict: we only call something implemented
when the repo contains a real bounded runtime surface, proof surface, or
reviewable artifact set for it.

## What ADL Is Now

ADL is no longer just a language idea or schema set.

ADL is now a deterministic agent-runtime and orchestration platform with:
- a Rust reference runtime and CLI
- explicit workflow, task, agent, tool, and provider artifacts
- deterministic planning and bounded execution semantics
- trace, artifact, and review surfaces for post-run inspection
- bounded demos and milestone proof packages
- structured authoring and control-plane workflows for repo-scale execution

In short: ADL has become an engineering system for building AI workflows that
can survive code review, ops review, and postmortem analysis.

## Status Legend

- **Implemented**: materially present on `main` with working code, artifacts,
  docs, and/or demo or review surfaces.
- **Implemented baseline**: already real and usable, with later milestones
  deepening, integrating, or hardening it.
- **Active milestone**: materially present and under active closeout/review in
  the current milestone band.
- **Partially implemented**: meaningful enabling surfaces exist, but the
  capability is not yet complete enough to count as a finished platform band.
- **Planned**: primarily a planned milestone/feature band today.
- **Deferred**: recognized, but not currently part of the `v0.95` must-have
  scope unless explicitly promoted later.

## Current Repo Status

The current repo truth is:
- active milestone: `v0.88`
- current crate version on `main`: `0.87.1`
- most recently completed runtime-completion milestone package: `v0.87.1`
- most recently completed substrate milestone package: `v0.87`

That means the feature story should be read this way:
- `v0.7` through `v0.87.1` provide the implemented platform baseline
- `v0.88` is an active, materially landed milestone in review/remediation/closeout
- `v0.89` through `v0.95` are the next planned capability bands

## ADL at a Glance

ADL already provides a serious platform baseline:
- deterministic workflow execution
- a real Rust runtime and CLI
- bounded concurrency, retry, and failure policy
- signing, verification, and trust surfaces
- trace, artifact, and replay-oriented reviewability
- provider and transport substrate
- structured authoring and task-bundle workflow
- review and validation contracts
- bounded Godel-style experimentation
- ObsMem indexing, retrieval, and shared-memory substrate
- bounded cognitive and agency-oriented proof paths
- operational skills and control-plane workflow substrate
- reviewer-facing milestone packages and demo matrices

## Feature Status Matrix

| Feature band | Status now | Current proof surface | Completion target |
| --- | --- | --- | --- |
| Deterministic workflow execution | Implemented | runtime/CLI, examples, milestone docs | Complete baseline |
| ExecutionPlan runtime | Implemented | Rust runtime and plan execution | Complete baseline |
| Sequential + fork/join coordination | Implemented | examples, tests, demo docs | Complete baseline |
| Bounded concurrency and retry/failure controls | Implemented | runtime semantics, tests, v0.7 docs | Complete baseline |
| Run artifacts and replay-oriented inspection | Implemented baseline | run artifacts, trace/review docs, milestone demos | Deepen through `v0.90` |
| Signing, verification, and trust policy | Implemented baseline | signing/verification surfaces, trust docs | Deepen through `v0.90` |
| Provider and transport substrate | Implemented baseline | provider docs, HTTP/local provider surfaces, reviewer package | Deepen through `v0.92` |
| Remote execution baseline | Implemented baseline | bounded remote execution surfaces and docs | Deepen through `v0.92+` |
| Human-in-the-loop pause/resume | Implemented baseline | runtime/control surfaces and review docs | Integrate through `v0.95` |
| Structured authoring model | Implemented baseline | STP/SIP/SOR contracts and prompt tooling | Deepen through `v0.95` |
| Control-plane lifecycle | Implemented baseline | `pr init/create/start/run/finish`, doctor, janitor, closeout surfaces | Harden through `v0.95` |
| Editor and command-adapter surfaces | Implemented baseline | editor docs, demos, bounded command adapters | Deepen through `v0.95` |
| Review and validation surfaces | Implemented baseline | reviewer contracts, validation tools, review packages | Deepen through `v0.95` |
| Task-bundle workflow | Implemented baseline | issue/task bundles and public execution records | Deepen through `v0.95` |
| Bounded Godel loop | Implemented baseline | `v0.8` runtime artifacts, demos, experiment surfaces | Deepen through `v0.89` |
| ObsMem indexing and retrieval | Implemented baseline | `v0.8` / `v0.87` proof surfaces and shared memory docs | Deepen through `v0.88+` |
| Shared ObsMem foundation | Implemented baseline | `v0.87` shared-memory docs and proof surfaces | Deepen with identity/continuity |
| Bounded cognitive path | Implemented baseline | `v0.86` cognitive demo/artifact package | Deepen through `v0.88+` |
| Freedom Gate baseline | Implemented baseline | `v0.86` bounded cognitive proof path | Deepen through `v0.89` |
| Trace substrate | Implemented baseline | `v0.87` trace docs and reviewer-facing proof surfaces | Signed/query completion in `v0.90` |
| Operational skills substrate | Implemented baseline | `v0.87` skills/control-plane docs and operational demos | Harden through `v0.95` |
| Runtime environment and lifecycle completion | Implemented baseline | `v0.87.1` runtime docs, demos, and review package | Deepen through later hardening |
| Local runtime resilience and Shepherd preservation | Implemented baseline | `v0.87.1` resilience and preservation docs/demos | Deepen through later runtime work |
| Chronosense / temporal substrate | Active milestone | `v0.88` feature package and review surfaces | `v0.88` |
| Commitments, deadlines, and bounded temporal causality | Active milestone | `v0.88` feature docs and reviewer package | `v0.88` |
| PHI-style integration metrics | Active milestone | `v0.88` feature docs and review surfaces | `v0.88` |
| Instinct and bounded agency | Active milestone | `v0.88` feature docs, instinct review surface, Paper Sonata | `v0.88` |
| Paper Sonata public-facing proof surface | Active milestone | `demo_v088_paper_sonata.sh` and milestone docs | `v0.88` |
| Deep-agents comparative proof | Active milestone | `demo_v088_deep_agents_comparative_proof.sh` and feature doc | `v0.88` |
| AEE 1.0 convergence | Partially implemented | bounded recovery/AEE proof surfaces and planning docs | `v0.89` |
| Freedom Gate v2 | Planned | baseline Freedom Gate plus `v0.89` planning docs | `v0.89` |
| Decision, action, and skill-governance surfaces | Planned | `v0.89` planning package | `v0.89` |
| Security, posture, and trust-under-adversary package | Planned | `v0.89` planning package | `v0.89` / `v0.89.2` |
| Reasoning graph baseline | Planned | planning/schema/proof surfaces | `v0.90` |
| Signed trace and trace query | Planned | roadmap and planning docs | `v0.90` |
| Affect, kindness, moral cognition, humor | Planned | `v0.91` planning docs | `v0.91` |
| Identity, capability, names, and continuity substrate | Planned | `v0.92` planning docs | `v0.92` |
| Governance, delegation, IAM, social contract | Planned | `v0.93` planning docs | `v0.93` |
| Economics, accounting, and payment substrate | Planned | economics planning corpus and roadmap docs | `v0.93` / `v0.94` |
| Distributed execution integration | Partially implemented | cluster groundwork plus planning docs | `v0.94` / `v0.95` |
| Demo catalog and polished MVP walkthrough | Partially implemented | milestone demo matrices and reviewer packages | `v0.95` |
| Control-plane Rust migration / tooling hardening | Partially implemented | mixed Rust/shell control plane and active tooling hardening | `v0.95` |
| Zed integration | Deferred | planning docs only | Post-`v0.95` unless promoted |

## Implemented Platform Highlights

### Deterministic Runtime and Execution Semantics

ADL already executes workflows as explicit, deterministic plans rather than
fragile prompt chains. That gives the project its core credibility: readers can
inspect what will run, reason about ordering and failure behavior, and trust
that the runtime is not improvising hidden orchestration logic.

### Real Rust Runtime and CLI

The Rust runtime is not a placeholder. ADL already has a reference runtime and
CLI capable of plan printing, execution, tracing, signing, verification,
artifact emission, and bounded remote/provider interaction. That is the
difference between “a language idea” and “a platform you can actually run.”

### Bounded Workflow Coordination

ADL already supports the coordination patterns serious orchestration needs:
sequential execution, fork/join structure, bounded concurrency, retries, and
failure policy. This makes the system useful for real engineering workflows,
not just single-prompt demos.

### Reviewable Artifacts and Proof Surfaces

Every important ADL milestone has pushed toward one principle: execution should
leave behind durable proof surfaces. Trace artifacts, run records, milestone
demo matrices, review handoffs, and local review packages make the platform
inspectable after the fact rather than dependent on oral reconstruction.

### Signing, Verification, and Trust Boundaries

ADL already includes signing and verification surfaces and treats trust as part
of the runtime story. That baseline becomes richer later, but it is already a
real part of the system, not an aspirational security note.

### Provider, Remote, and Transport Substrate

ADL already has real provider/transport structure, including bounded remote
execution and local/provider proof paths. This matters because it establishes
the platform boundary between orchestration logic and execution backends.

### Structured Authoring and Control Plane

The repo now has a real control-plane lifecycle around issue creation,
bootstrap, run binding, validation, and closeout. STP/SIP/SOR records, doctor
checks, janitoring, and bounded PR tooling give ADL a strong authoring and
execution spine instead of relying on vague contributor habit.

### Operational Skills as System Intelligence

Operational skills are now part of ADL’s real platform story. They reduce
error, improve determinism, and turn repeated repo operations into bounded,
reviewable execution surfaces rather than free-form prompting.

### Bounded Godel, ObsMem, and Cognitive Substrate

ADL already has real bounded reflective execution, memory participation, and
cognitive proof surfaces:
- `v0.8` established bounded Godel-style experimentation and canonical artifacts
- `v0.86` established the first working bounded cognitive-system proof package
- `v0.87` strengthened trace/provider/shared-memory/skills substrate

These are not disconnected demos. Together they form the core of ADL’s claim
that bounded adaptive systems can be both powerful and reviewable.

## Current Active Milestone: v0.88

`v0.88` is not just planning work. It is an active, materially landed milestone
whose implementation wave is complete through `WP-13`, with docs/review/
remediation/release-tail work still in progress.

This milestone adds two major bounded bands:
- temporal / chronosense substrate
- instinct / bounded agency substrate

High-signal `v0.88` achievements include:
- promoted temporal schema, continuity/identity semantics, temporal retrieval,
  commitments/deadlines, bounded temporal causality, PHI metrics, instinct, and
  instinct runtime influence into one canonical feature package
- reviewer-facing proof surfaces for temporal review, PHI review, instinct
  review, and the integrated `v0.88` review surface
- `Paper Sonata` as the flagship public-facing bounded demo
- deep-agents comparative proof as a supporting reviewer-facing row
- a full internal repo-code-review pass completed before 3rd-party review

So the truthful `v0.88` story is:
- core implementation: materially landed
- review/remediation/closeout: still active
- milestone value: already very real

## Major Capability Bands Still to Come

### v0.89 - Governed Adaptive Execution

`v0.89` is the band where ADL should turn governed adaptation into a first-class
platform package:
- AEE 1.0 convergence
- Freedom Gate v2
- explicit decision and action mediation surfaces
- skill execution contracts
- security, trust, and posture planning serious enough to support adversarial work

### v0.90 - Reasoning Graph, Signed Trace, and Query

`v0.90` is expected to deepen reasoning and provenance into a much stronger
inspection stack:
- reasoning-graph baseline
- signed trace completion
- query and inspection over reasoning and trace artifacts

### v0.91 - Affect and Moral Cognition

`v0.91` is where ADL’s cognitive architecture becomes more emotionally and
normatively legible:
- affect
- kindness
- humor/absurdity
- moral cognition and related evaluation surfaces

### v0.92 - Identity, Capability, and Continuity

`v0.92` is the bridge from bounded cognitive behavior to identity-bearing
agents:
- first-class identity
- provider/model capability contracts
- stable names
- continuity hooks across runs

### v0.93 - Governance, Delegation, IAM, and Social Contract

`v0.93` is expected to turn identity substrate into accountable governance:
- IAM
- delegation
- policy and constitutional surfaces
- rights/duties and social contract surfaces

### v0.93 - v0.94 Economics and Payment Substrate

The planning corpus already points toward a serious economics band:
- accounting schema
- economic agency
- governance rules
- payment adapters
- Lightning / x402 experiments
- market and settlement surfaces

This is an important future platform direction, even though it is not part of
the current `v0.88` closure band.

### v0.94 - Integration and Dependency Closure

`v0.94` should close the remaining cross-cutting dependency gaps:
- distributed-substrate integration
- cross-band convergence
- MVP dependency cleanup

### v0.95 - MVP Convergence and Feature Freeze

`v0.95` is the planned convergence point:
- polished demo catalog
- coherent MVP walkthrough
- control-plane/tooling hardening
- feature freeze and `1.0` scope boundary

## Deferred Feature

### Zed Integration

Zed integration is recognized as useful, but it is not currently required for
the `v0.95` MVP. It should remain explicitly deferred unless a later milestone
promotes it into must-have scope.

## Summary

ADL already has a substantial platform:
- deterministic execution
- a real Rust runtime
- bounded orchestration semantics
- trust and verification surfaces
- reviewable traces and artifacts
- provider and transport substrate
- structured authoring and control-plane workflow
- operational skills
- bounded Godel, ObsMem, and cognitive proof paths
- active temporal and bounded-agency milestone work

What remains through `v0.95` is not random feature accumulation. It is a
deliberate convergence path:
- finish `v0.88` cleanly
- make governed adaptation explicit in `v0.89`
- deepen reasoning and provenance in `v0.90`
- add affect, identity, governance, and economics in bounded later bands
- close the MVP as a serious, reviewable agent-runtime platform
