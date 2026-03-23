# v0.85 Milestone Docs

This directory is the canonical location for the tracked v0.85 milestone documentation set.

Use this directory for:

- milestone design and planning
- architecture and substrate notes
- release planning and checklist artifacts
- rationale, schema, and execution-model docs that define the public shape of v0.85

Tracked public task-record history for v0.85 should live under `docs/records/v0.85/tasks/`, not inside `.adl/` or only inside tracker-specific systems. Temporary draft artifacts may still exist in `.adl/`, but they should be reconciled into tracked task bundles before authoritative lifecycle transitions.

## Canonical Milestone Shape

The canonical v0.85 plan is the revised four-sprint, twenty-five-work-package structure being reconciled under issue `#886`.

The most important tracker rules are:

- `#886` is the umbrella milestone-reorganization issue until the issue graph is fully aligned.
- `#674` is the canonical queue/checkpoint/steering issue.
- `#867` is placeholder/duplicate queueing material to absorb, supersede, or close in favor of `#674`.
- Gödel issues `#748` through `#752` are first-class milestone work packages, not side issues.
- The provisional generated issue set `#866` through `#882` must be absorbed or remapped to fit the canonical twenty-five-work-package structure.

## Required Proof Surfaces

v0.85 is not a docs-only milestone. The canonical plan requires:

- real queue/checkpoint/steering progress with replay-compatible proof surfaces
- first practical authoring/editor surfaces
- stronger editing/review tooling
- first meaningful Gödel hypothesis-engine progress
- a minimal working affect engine integrated with reasoning/Gödel behavior
- multiple runnable bounded demos, including:
  - steering/queueing
  - HITL/editor/review workflow
  - affect influencing reasoning or hypothesis behavior

## Canonical Doc Set

The current tracked v0.85 set includes:

- `COGNITIVE_LOOP_MODEL_v0.85.md`
- `AFFECTIVE_REASONING_MODEL.md`
- `AFFECT_MODEL_v0.85.md`
- `CLUSTER_EXECUTION.md`
- `COGNITIVE_STACK_v0.85.md`
- `DECISIONS_v0.85.md`
- `DESIGN_v0.85.md`
- `EDITING_ARCHITECTURE.md`
- `BOUNDED_AFFECT_MODEL.md` (legacy rename from `EMOTION_MODEL.md`; bounded-affect framing)
- `HTA_EDITOR_PLANNING.md`
- `HUMAN-IN-THE-LOOP-DESIGN-NOTES.MD`
- `LAYER_8_IMPLEMENTATION.md`
- `MILESTONE_CHECKLIST_v0.85.md`
- `MILESTONE_ISSUE_RECONCILIATION_v0.85.md`
- `QUALITY_GATE_v0.85.md`
- `REASONING_GRAPH_SCHEMA_V0.85.md`
- `RELEASE_NOTES_v0.85.md`
- `RELEASE_PLAN_v0.85.md`
- `SPRINT_v0.85.md`
- `STRUCTURED_PROMPT_ARCHITECTURE.md`
- `SWARM_REMOVAL_PLANNING.md`
- `VISION_v0.85.md`
- `WBS_v0.85.md`

If v0.85 planning material exists elsewhere in a temporary workspace or local draft area, reconcile it into this directory rather than treating that alternate location as authoritative.

Retired historical docs may remain in this directory for continuity, but they
are not part of the active canonical milestone set. `WHY_RUST_FOR_ADL.md` is
currently retained in that historical-only status.

## Cognitive Authority

Tracked v0.85 cognitive authority is split into two complementary docs:

- `COGNITIVE_LOOP_MODEL_v0.85.md` is the authoritative loop/flow model.
- `COGNITIVE_STACK_v0.85.md` is the authoritative internal stack/layer model.

Other tracked v0.85 docs should not define a competing authoritative loop or fractional layer numbering.
