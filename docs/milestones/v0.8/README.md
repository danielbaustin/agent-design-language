# v0.8 Milestone Docs Index

This directory is the canonical source of truth for v0.8 milestone documentation.

Use this index as the primary navigation surface for v0.8 scope, sequencing, and release readiness.

## Current Status

- Milestone state: active development (not released).
- Runtime base release: v0.7.0.
- Current repository state includes both bounded implemented runtime/demo surfaces and schema/spec/planning surfaces.
- The v0.8 packet is not yet ready for third-party handoff; see `INTERNAL_READINESS_REVIEW_V0.8.md` and `RECOVERY_AUDIT_V0.8.md` for current blockers.

## External Review Quick Start

If you are reviewing v0.8 for the first time, start with this split:

### Runnable demos

Run these commands from repository root:

```bash
cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-d-godel-obsmem-loop --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-e-multi-agent-card-pipeline --run --trace --out ./out
cargo run --manifest-path swarm/Cargo.toml --bin adl -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

Review these docs while inspecting runnable output:
- [DEMOS_V0.8.md](DEMOS_V0.8.md)
- [RUST_TRANSPILER_DEMO.md](RUST_TRANSPILER_DEMO.md)
- [RUST_TRANSPILER_VERIFICATION_V0.8.md](RUST_TRANSPILER_VERIFICATION_V0.8.md)
- [../demos/v0.8-bounded-critical-demos.md](../demos/v0.8-bounded-critical-demos.md)

### Inspect-only review surfaces

These are review surfaces to read or inspect, not commands to run:
- [CANONICAL_EVIDENCE_VIEW_V1.md](CANONICAL_EVIDENCE_VIEW_V1.md)
- [MUTATION_FORMAT_V1.md](MUTATION_FORMAT_V1.md)
- [EVALUATION_PLAN_V1.md](EVALUATION_PLAN_V1.md)
- [EXPERIMENT_RECORD_V1.md](EXPERIMENT_RECORD_V1.md)
- [OBSMEM_INDEXING_SURFACES_V1.md](OBSMEM_INDEXING_SURFACES_V1.md)
- [GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md](GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md)
- [../../tooling/README.md](../../tooling/README.md)

## Packet Status Note

The v0.8 review packet is materially better aligned than the earlier recovery state, but it still has real reviewer-facing blockers:
- version/release-status language is not fully reconciled across all reader-facing docs,
- the final `THIRD_PARTY_REVIEW_V0.8.md` handoff artifact is still absent,
- some milestone docs remain planning/reference surfaces rather than implemented runtime behavior.

## Reading Order

1. [Vision](VISION_0.80.md)
2. [Architecture](ARCHITECTURE_V0.8.md)
3. [Milestone Design](DESIGN_V0.8.md)
4. [Epic Mapping](EPIC_MAPPING_v0.8.md)
5. [Work Breakdown Structure (WBS)](WBS_V0.8.md)
6. [Sprint Plan](SPRINT_V0.8.md)
7. [Milestone Checklist](MILESTONE_CHECKLIST_V0.8.md)
8. [Decisions Log](DECISIONS_V0.8.md)
9. [Release Plan](RELEASE_PLAN_V0.8.md)
10. [Release Notes](RELEASE_NOTES_V0.8.md)

## Review-Tail Packet

Use these docs together when checking current v0.8 truth:
- [RECOVERY_AUDIT_V0.8.md](RECOVERY_AUDIT_V0.8.md)
- [INTERNAL_READINESS_REVIEW_V0.8.md](INTERNAL_READINESS_REVIEW_V0.8.md)
- [DOCS_CONVERGENCE_V0.8.md](DOCS_CONVERGENCE_V0.8.md)
- [DEMOS_V0.8.md](DEMOS_V0.8.md)

## Vision / Overview

- [VISION_0.80.md](VISION_0.80.md)
- [DESIGN_V0.8.md](DESIGN_V0.8.md)

## Architecture

- [ARCHITECTURE_V0.8.md](ARCHITECTURE_V0.8.md)
- [ADAPTIVE_EXECUTION_ENGINE.md](ADAPTIVE_EXECUTION_ENGINE.md)
- [MEMORY_MODEL_FOR_AI.md](MEMORY_MODEL_FOR_AI.md)
- [GODEL_SCIENTIFIC_METHOD.md](GODEL_SCIENTIFIC_METHOD.md)
- [GODEL_LOOP_INTEGRATION_V0.8.md](GODEL_LOOP_INTEGRATION_V0.8.md)
- [GODEL_LOOP_DIAGRAM.md](GODEL_LOOP_DIAGRAM.md)
- [GODEL_AGENT_NOTES.md](GODEL_AGENT_NOTES.md)

## Epics

- [EPIC_MAPPING_v0.8.md](EPIC_MAPPING_v0.8.md)
- [EPIC_AUTHORING_SURFACES_v1.md](EPIC_AUTHORING_SURFACES_v1.md)

## Execution / Planning

- [WBS_V0.8.md](WBS_V0.8.md)
- [SPRINT_V0.8.md](SPRINT_V0.8.md)
- [EXECUTION_ORDER_V0.8.md](EXECUTION_ORDER_V0.8.md)
- [BOUNDED_AEE_V1_SCOPE_V0.8.md](BOUNDED_AEE_V1_SCOPE_V0.8.md)
- [AUTHORING_DELIVERY_ORDER_V0.8.md](AUTHORING_DELIVERY_ORDER_V0.8.md)
- [GODEL_HANDOFF_BOUNDARIES_V0.8.md](GODEL_HANDOFF_BOUNDARIES_V0.8.md)
- [GODEL_SCHEMA_DELIVERY_ORDER_V0.8.md](GODEL_SCHEMA_DELIVERY_ORDER_V0.8.md)
- [MILESTONE_CHECKLIST_V0.8.md](MILESTONE_CHECKLIST_V0.8.md)
- [QUALITY_GATE_V0.8.md](QUALITY_GATE_V0.8.md)
- [DEMOS_V0.8.md](DEMOS_V0.8.md)
- [CARD_TEMPLATE_LOCATION_V0.8.md](CARD_TEMPLATE_LOCATION_V0.8.md)
- [DECISIONS_V0.8.md](DECISIONS_V0.8.md)
- [RELEASE_PLAN_V0.8.md](RELEASE_PLAN_V0.8.md)
- [RELEASE_NOTES_V0.8.md](RELEASE_NOTES_V0.8.md)
- [DOCS_CONVERGENCE_V0.8.md](DOCS_CONVERGENCE_V0.8.md)

## Supporting Design Docs and Demo Surfaces

- [RUST_TRANSPILER_DEMO.md](RUST_TRANSPILER_DEMO.md)
- [RUST_TRANSPILER_VERIFICATION_V0.8.md](RUST_TRANSPILER_VERIFICATION_V0.8.md)
- [../demos/v0.8-bounded-critical-demos.md](../demos/v0.8-bounded-critical-demos.md)
- [STICKTOITTIVENESS.md](STICKTOITTIVENESS.md)
- [incubation/GODEL_AGENT.md](incubation/GODEL_AGENT.md)
- [incubation/OBSMEM_BAYES.md](incubation/OBSMEM_BAYES.md)
- [incubation/STICKTOITTIVENESS.md](incubation/STICKTOITTIVENESS.md)

## Canonical Schema/Spec Artifacts (Design Stage)

- [agent_profile.v1.json](agent_profile.v1.json)
- [mutation.v1.json](mutation.v1.json)
- [mutation.v1.example.json](mutation.v1.example.json)
- [MUTATION_FORMAT_V1.md](MUTATION_FORMAT_V1.md)
- [evaluation_plan.v1.json](evaluation_plan.v1.json)
- [evaluation_plan.v1.example.json](evaluation_plan.v1.example.json)
- [EVALUATION_PLAN_V1.md](EVALUATION_PLAN_V1.md)
- [EXPERIMENT_RECORD_V1.md](EXPERIMENT_RECORD_V1.md)
- [experiment_record.v1.schema.json](experiment_record.v1.schema.json)
- [experiment_record.v1.example.json](experiment_record.v1.example.json)
- [CANONICAL_EVIDENCE_VIEW_V1.md](CANONICAL_EVIDENCE_VIEW_V1.md)
- [canonical_evidence_view.v1.schema.json](canonical_evidence_view.v1.schema.json)
- [canonical_evidence_view.v1.example.json](canonical_evidence_view.v1.example.json)
- [TOOL_RESULT_CONTRACT_V1.md](TOOL_RESULT_CONTRACT_V1.md)
- [tool_result.v1.schema.json](tool_result.v1.schema.json)
- [tool_result.v1.example.json](tool_result.v1.example.json)
- [run_summary.v1.json](run_summary.v1.json)
- [run_summary.v1.example.json](run_summary.v1.example.json)
- [experiment_index_entry.v1.json](experiment_index_entry.v1.json)
- [experiment_index_entry.v1.example.json](experiment_index_entry.v1.example.json)
- [OBSMEM_INDEXING_SURFACES_V1.md](OBSMEM_INDEXING_SURFACES_V1.md)

## Scope Slicing Reference

- v0.75: EPIC-A + EPIC-B (deterministic substrate + ObsMem)
- v0.8: EPIC-C + EPIC-D (Godel + Authoring)
- v0.85+: cluster / distributed execution

## Related Milestones

- [v0.75 milestone docs](../v0.75/)
- [v0.85 milestone docs](../v0.85/)
- [incubation/](incubation/)
