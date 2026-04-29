# Source Packet: Determinism in the ADL Runtime

## Metadata

- Packet: `determinism_in_the_adl_runtime_source_packet`
- Intended paper: `Determinism in the ADL Runtime`
- Issue: `#2641`
- Status: draft input for initial manuscript review
- Publication state: not submitted; not publication-ready

## Purpose

Provide one bounded source packet for a focused paper on why ADL treats
determinism, replayability, and execution truth as part of the runtime contract
rather than as implementation polish.

## Core Thesis

ADL treats determinism as a trust primitive. The system's value does not come
only from producing outputs. It comes from producing outputs through explicit
plans, bounded scheduler semantics, trace-visible decisions, deterministic
artifact paths, and closeout records that make the run reconstructable enough
for review and debugging.

## Target Reader

- systems and runtime engineers
- people skeptical of agent systems because they are difficult to audit
- reviewers interested in trust, replay, provenance, and bounded operational
  claims

## Primary Source Surfaces

### `docs/adr/0001-determinism.md`

Supported facts:

- deterministic execution semantics are an accepted architecture decision
- ready-step ordering is lexicographic by full step id
- plan generation is deterministic for canonicalized inputs
- determinism is justified by reproducibility, debugging, and trust

### `README.md`

Supported facts:

- ADL is described as a deterministic orchestration system
- explicit contracts, bounded runtime behavior, and repository-visible proof
  are central project claims
- trace events, run manifests, and replay-friendly artifact roots are part of
  the current system story

### `adl/README.md`

Supported facts:

- the Rust runtime resolves documents into deterministic execution plans
- the runtime has explicit semantics for concurrency, retries, failures,
  signing, tracing, and bounded remote execution
- run artifacts are stable and replay-friendly
- deterministic execution is the foundation on which later runtime layers build

### `docs/architecture/ADL_ARCHITECTURE.md`

Supported facts:

- workflows are compiled into explicit execution plans before execution
- duplicate ids, cycles, and invalid saved-state references are rejected before
  runtime
- execution records lifecycle phases, scheduling policy, step events,
  delegation, failures, and completion
- traces and artifacts are treated as truth-bearing runtime evidence
- STP/SIP/SOR and worktree/PR lifecycle are part of the truth model

### `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`

Supported facts:

- trace is execution truth, not after-the-fact logging
- structured trace plus artifacts form the reconstruction surface
- deterministic structural ordering is one of the core guarantees
- review fails if major control transitions must be inferred

### `docs/default_workflow.md`

Supported facts:

- ADL's default workflow couples deterministic lifecycle steps with explicit
  validation and output records
- replay/security/artifact sections are required in the output-card contract
- docs-only and focused validation must be recorded truthfully rather than
  hidden under generic test claims

### `docs/planning/ADL_FEATURE_LIST.md`

Supported facts:

- deterministic workflow execution is an implemented baseline
- run artifacts and replay-oriented inspection are implemented baseline
  capabilities
- control-plane lifecycle, review surfaces, and milestone proof packages are
  part of the current repo truth

### `docs/milestones/v0.87/README.md`, `docs/milestones/v0.87/RELEASE_NOTES_v0.87.md`

Supported facts:

- trace became a first-class substrate surface for reconstruction-oriented
  execution truth
- provider, shared memory, skills, and control-plane work were aligned around
  the trace/review/documentation truth model

### `docs/milestones/v0.87.1/features/LOCAL_RUNTIME_RESILIENCE.md`

Supported facts:

- runtime resilience depends on durable execution truth
- resilience is not independent of artifact and trace discipline

### `docs/milestones/v0.90/README.md` and later milestone docs

Supported facts:

- later runtime layers deepen determinism by extending artifact, continuity,
  observability, and review surfaces
- long-lived runtime work preserves bounded cycle truth rather than replacing
  it with opaque daemon behavior

## Allowed Claims

| Claim | Status | Evidence |
| --- | --- | --- |
| ADL treats determinism as a runtime contract rather than an implementation accident. | SUPPORTED | `docs/adr/0001-determinism.md`; `adl/README.md`; architecture docs |
| Plan compilation, scheduling, trace ordering, and artifact structure are designed to be stable and reviewable. | SUPPORTED | `docs/adr/0001-determinism.md`; `docs/architecture/ADL_ARCHITECTURE.md`; `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md` |
| Closeout truth is part of runtime trust because stale records undermine reconstruction. | SUPPORTED | `docs/default_workflow.md`; `docs/architecture/ADL_ARCHITECTURE.md`; review docs |
| ADL offers perfect replay or formal proof of correctness. | REMOVE_OR_WEAKEN | Not supported by current repo evidence |
| Every provider interaction is fully deterministic at the semantic output level. | REMOVE_OR_WEAKEN | Model/provider stochasticity remains bounded but not eliminated |
| ADL's determinism posture is stronger than all competing systems. | NEEDS_EVIDENCE | No comparative study in this packet |

## Important Framing Constraints

- The paper should emphasize that ADL's determinism is bounded and layered.
- It should distinguish deterministic structure from stochastic model contents.
- It should explain why reviewability and artifact truth matter more than
  inflated claims of absolute replay.
- It should connect runtime determinism to the control plane and closeout truth.

## Citation And Evidence Gaps

- related work on deterministic workflow engines and replayable execution
- provenance and artifact-traceability literature
- systems/debugging literature around reproducibility and operational truth
- comparison to other orchestration or agent frameworks, if desired later

## Recommended Section Order

1. Why determinism matters for agent systems
2. What ADL means by determinism
3. Deterministic planning and bounded execution
4. Trace and artifacts as execution truth
5. Determinism beyond the runner: control-plane and closeout truth
6. Security, trust, and bounded replay claims
7. Limits and future work

## Non-Goals

- formal-methods proof paper
- complete survey of replay systems
- public submission packet
- broader ADL architecture paper replacement

## Boundary

This packet is for internal drafting and review. It is not a submission package
and does not certify publication readiness.
