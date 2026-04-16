# v0.85 Milestone Docs

This directory is the canonical tracked documentation set for the **v0.85** milestone.

v0.85 is a **stabilization and maturity milestone**. Its purpose is not to land the larger cognitive architecture planned for later milestones, but to make the repository, workflow, proof surfaces, and release discipline substantially more dependable.

Use this directory as the authoritative source for:

- milestone design and planning
- release planning, release notes, and checklist surfaces
- quality-gate and demo verification material
- architecture and execution-model notes that define the public shape of v0.85

Tracked public task-record history for v0.85 should live under `docs/records/v0.85/tasks/`, not only in `.adl/` or tracker-specific systems. Temporary draft artifacts may still exist in `.adl/`, but authoritative lifecycle transitions should be reflected in tracked milestone and task-bundle surfaces.

## What v0.85 Delivers

v0.85 delivers a stronger and more reviewable ADL repository state, including:

- major code-organization improvements and cleaner module boundaries
- the `swarm/` to `adl/` repository transition
- stronger milestone, review, and release artifacts
- formalized quality-gate expectations
- clearer bounded Gödel architectural documentation through ADR 0008 and related milestone docs
- verified demo and proof surfaces for milestone closeout

v0.85 does **not** claim to ship the broader later-milestone systems such as full AEE, reasoning-graph runtime, affect-driven cognition, identity substrate, or governance layers. Those remain explicitly deferred to later milestones.

## Canonical Milestone Shape

The canonical v0.85 milestone shape is the revised four-sprint, twenty-five-work-package structure reconciled during milestone closeout.

Key tracker truths for this milestone:

- `#886` is the milestone-reorganization and issue-alignment issue used to reconcile the canonical shape.
- `#674` is the canonical queue/checkpoint/steering issue.
- Gödel issues `#748` through `#752` are treated as first-class milestone work packages.
- The provisional generated issue set `#866` through `#882` was reconciled, absorbed, narrowed, or remapped as part of milestone closeout rather than treated as a competing canonical structure.

## Required Proof Surfaces

v0.85 is not a docs-only milestone. The canonical proof surfaces for release closeout include:

- queue/checkpoint/steering progress with replay-compatible proof surfaces
- practical HITL/editor/review flow surfaces
- stronger editing/review tooling and output-record rigor
- bounded Gödel hypothesis-engine progress
- affect-linked reasoning / adaptation proof surfaces
- canonical runnable milestone demos tracked in `DEMO_MATRIX_v0.85.md`

For final release readiness, the canonical demo set and proof surfaces should be treated as **verified**, not merely documented.

## Canonical Doc Set

The current tracked v0.85 set includes:

- `AFFECTIVE_REASONING_MODEL.md`
- `AFFECT_MODEL_v0.85.md`
- `BOUNDED_AFFECT_MODEL.md`
- `CLUSTER_EXECUTION.md`
- `COGNITIVE_LOOP_MODEL_v0.85.md`
- `COGNITIVE_STACK_v0.85.md`
- `DECISIONS_v0.85.md`
- `DESIGN_v0.85.md`
- `DEMO_MATRIX_v0.85.md`
- `EDITING_ARCHITECTURE.md`
- `HTA_EDITOR_PLANNING.md`
- `HUMAN-IN-THE-LOOP-DESIGN-NOTES.MD`
- `ideas/README.md`
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

If v0.85 material exists elsewhere in temporary workspace or local draft areas, reconcile it into this directory rather than treating alternate locations as authoritative.

Retired historical docs may remain here for continuity, but they are not part of the active canonical milestone set.

## Cognitive Authority

Tracked v0.85 cognitive authority is split into two complementary docs:

- `COGNITIVE_LOOP_MODEL_v0.85.md` is the authoritative loop / flow model.
- `COGNITIVE_STACK_v0.85.md` is the authoritative stack / layer model.

Other v0.85 docs should not define a competing authoritative loop or conflicting layer structure.

## Reader Guidance

Readers new to the milestone should start with:

1. `VISION_v0.85.md`
2. `DESIGN_v0.85.md`
3. `DEMO_MATRIX_v0.85.md`
4. `ideas/README.md`
5. `QUALITY_GATE_v0.85.md`
6. `RELEASE_NOTES_v0.85.md`

These documents provide the clearest path through the milestone’s intent, implementation shape, proof surfaces, quality expectations, and release summary.
