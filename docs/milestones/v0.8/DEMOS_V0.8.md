# v0.8 Demo Matrix and Integration Demos

This document defines the canonical demo matrix for v0.8 milestone review.

It is an integration-planning surface only. It does not implement demos.

## Deterministic Ordering Rules

Demo entries are ordered by:

1. required milestone criticality,
2. demo ID (`D8-01`, `D8-02`, ...),
3. issue number tie-break.

## Required Demos (Pre-Review / Pre-Release)

These demos are required before third-party review (`#707`) and release convergence.

| Demo ID | Workstream | Scope | Primary Issue(s) | Required Evidence Surface |
|---|---|---|---|---|
| D8-01 | Gödel schema spine | ExperimentRecord + Evidence + Mutation + EvaluationPlan schema alignment | `#609`, `#610`, `#611`, `#612`, `#683` | canonical schema/example artifacts under `docs/milestones/v0.8/` |
| D8-02 | Gödel workflow integration | Failure -> hypothesis -> mutation -> experiment -> evaluation -> record loop template alignment | `#613`, `#615`, `#616` | `GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md` + `godel_experiment_workflow.template.v1.json` + demo docs |
| D8-03 | ObsMem indexing integration | Run summary + ExperimentRecord-derived indexing surfaces | `#614` | indexing surface definitions and retrieval linkage notes |
| D8-04 | Runtime/transpiler flagship | Rust transpiler and integration verification path | `#702`, `#703` | `RUST_TRANSPILER_DEMO.md` plus milestone checklist references |
| D8-05 | Authoring/reviewer compatibility | Prompt spec + reviewer checklist/output contracts and ordering | `#633`, `#650`, `#651`, `#649`, `#667`, `#677` | tooling docs/contracts + template references |

## Supporting Demos (Helpful, Not Release-Blocking Alone)

| Demo ID | Workstream | Scope | Primary Issue(s) | Evidence Surface |
|---|---|---|---|---|
| D8-S1 | AEE boundary clarity | Bounded v0.8 adaptive execution scope statement | `#669` | `BOUNDED_AEE_V1_SCOPE_V0.8.md` |
| D8-S2 | Execution sequencing | Milestone dependency/order check surface | `#664`, `#665`, `#666` | `EXECUTION_ORDER_V0.8.md` + related boundary docs |

## Required Validation/Evidence Expectations

Each required demo row should provide:

1. A canonical doc/spec pointer in `docs/milestones/v0.8/`.
2. Deterministic artifact/evidence references (schema/example/template/contract).
3. Clear in-scope vs deferred boundary notes where applicable.
4. No secrets, tool arguments, raw prompts, or absolute host paths in persisted evidence.

## Review Gate Usage

- Use this matrix as the integration-demo checklist for `#706` docs convergence and `#707` third-party review pass.
- A required demo is considered complete when its evidence surface exists, is cross-linked, and matches milestone scope boundaries.

## Out of Scope

- Adding new milestone features solely to satisfy demos.
- Reclassifying deferred v0.9+ autonomy work into v0.8.
- Replacing issue-level acceptance criteria with this matrix.
