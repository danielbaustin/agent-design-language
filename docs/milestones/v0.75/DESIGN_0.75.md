# ADL v0.75 — Design (Deterministic Substrate + ObsMem v1)

## Metadata
- Milestone: v0.75
- Version: 0.75
- Date: 2026-03-02
- Owner: Daniel / Agent Logic team
- Related issues: (TBD — will be created during v0.75 issue breakdown)

## Purpose
Define what we are building in v0.75, why it matters, and the design constraints that keep ADL deterministic and reviewable.

v0.75 is an interstitial release between v0.7 and v0.8 that (1) **freezes the deterministic execution contract** and (2) ships **ObsMem v1** so ADL can query and cite evidence from its own runs without sacrificing replayability.

## Problem Statement
ADL’s value proposition depends on **determinism + inspectability**. In v0.7 we made large foundational changes (naming, security envelope/remote signing, demos). The next step is to stabilize the substrate so it can be safely extended.

At the same time, users need ADL to “remember” what happened across runs (failures, fixes, patterns) in a way that is reproducible and evidence-backed.

Without a frozen execution/replay contract and a deterministic memory layer:
- Reviews become high-risk because behavior can drift.
- Debugging becomes expensive because runs are not comparable.
- Higher layers (Gödel, authoring surfaces) cannot be trusted.

## Goals
- Freeze the **deterministic execution contract** (activation log + replay) so identical inputs and captured tool-boundary events produce identical outputs/artifacts (excluding run-id/timestamps).
- Ship **ObsMem v1**: ingest trace bundles and answer similarity queries with deterministic ordering, explanations, and citations.

## Non-Goals
- Distributed / cluster execution (defer to v0.85/v0.9).
- Gödel self-improvement layer (v0.8).
- Authoring surfaces / NL→ADL compiler (v0.8).
- Major provider/tool refactors or a new security sandbox model beyond preserving existing guarantees.

## Scope
### In scope
- EPIC-A: Deterministic Substrate hardening
  - Activation log schema and replay runner
  - Trace bundle export v2 (versioned, replay-sufficient)
  - Failure taxonomy (stable machine-readable codes)
  - Deterministic artifact layout rules
- EPIC-B: ObsMem v1
  - Trace bundle ingestion
  - Structured index for runs/activations/evidence
  - Retrieval/query surfaces (structured + optional semantic)
  - Deterministic ranking + tie-break rules
  - Retrieval explanations + citations

### Out of scope
- Cluster orchestration, worker coordination, leases, multi-host execution
- Autonomous learning, fine-tuning, or hidden state updates
- Large CLI redesign

## Requirements
### Functional
- **Replay:** Given a run’s captured boundary events, replay produces the same step outputs and artifact layout.
- **Trace bundles v2:** Export a versioned bundle manifest that is sufficient for replay, inspection, and memory ingestion.
- **ObsMem ingest:** Ingest 2–N trace bundles into an index.
- **ObsMem query:** Answer “similar failures / similar runs” with a ranked list, citations, and an explanation.
- **Operational report:** Provide deterministic summaries (counts, cost/latency aggregates, failure classes) from the index.

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- No secrets persisted in artifacts/bundles.
- No absolute host paths persisted.
- Stable ordering rules for plans, activations, evidence, and retrieval results.

## Proposed Design
### Overview
v0.75 is two layers:

1) **Deterministic Substrate (EPIC-A)**
- Execution produces an **activation log** and **artifact tree**.
- A **replay runner** consumes the activation log (and captured tool-boundary events) to reproduce outputs deterministically.
- A **trace bundle v2** export packages the run for portability and ingestion.

2) **ObsMem v1 (EPIC-B)**
- ObsMem ingests trace bundles into a versioned **structured index**.
- Queries operate over structured fields (run id, workflow id, failure codes, tool names, timestamps, etc.) and may optionally use embeddings.
- Ranking is deterministic: stable scoring + deterministic tie-break.
- Results include citations back into the underlying trace bundle artifacts.

### Interfaces / Data contracts
- Activation Log (schema frozen in v0.75):
  - Append-only activations with stable identifiers
  - Captured boundary events sufficient for replay
- Trace Bundle v2:
  - Versioned manifest
  - Canonical serialization for hashed components
  - Stable relative paths within the bundle
- Failure Taxonomy:
  - Stable machine-readable classification codes
  - Deterministic mapping from observed failures to codes
- ObsMem Index:
  - Versioned schema
  - Deterministic query and ordering semantics

### Execution semantics
- Determinism definition:
  - If workflow version + inputs + captured boundary events are identical, then replay produces identical outputs and artifact layout (excluding run-id/timestamps).
- Tool boundary capture is the “seal” between deterministic interpretation and nondeterministic world.
- Retrieval determinism definition:
  - Given the same index state + query + retrieval config, results return in the same order.

## Risks and Mitigations
- Risk: Hidden nondeterminism at tool boundaries (time, env, ordering)
  - Mitigation: boundary capture + replay gating; add regression tests; enforce stable ordering.
- Risk: Embedding instability or model drift
  - Mitigation: embeddings optional; record embedding model id + retrieval config; deterministic tie-break and fallback to structured ranking.
- Risk: Index schema churn breaks compatibility
  - Mitigation: versioned manifests and additive migrations; keep read paths compatible.
- Risk: Ceremony overload slows velocity
  - Mitigation: keep v0.75 limited to EPIC-A/B with measurable demos; defer cluster/Gödel/authoring.

## Alternatives Considered
- Option: Ship Gödel in v0.75
  - Tradeoff: too many axes of change; increases review risk before substrate and memory are stable.
- Option: Ship cluster execution in v0.8
  - Tradeoff: expands infrastructure surface area and nondeterminism risks; distracts from cohesive product story.
- Option: ObsMem as “just embeddings”
  - Tradeoff: weak provenance and reproducibility; harder to keep deterministic and explainable.

## Validation Plan
- Checks/tests:
  - Determinism/replay tests (byte/structure equivalence of artifacts where applicable)
  - Trace bundle v2 export/import round-trip tests
  - Failure taxonomy stability tests (stable codes; deterministic mapping)
  - ObsMem ingest/query tests (deterministic ordering + citations)
  - CI gates for “no secrets / no host paths”
- Success metrics:
  - Demo A/B/C (from VISION_0.75.md) run from docs on a fresh checkout
  - Replay produces identical outputs for at least N representative workflows
  - ObsMem returns deterministic ranked results for a fixed corpus
- Rollback/fallback:
  - Feature flags or “v1/v2” bundle readers where needed
  - Keep prior trace export path available until v0.75 proves stable

## Exit Criteria
- Scope boundaries (A/B only) are explicit and reflected in milestone checklist.
- Contracts to freeze are documented and referenced by tests.
- Demo matrix is runnable and reproducible.
- Major open questions are captured in DECISIONS_0.75.md.
