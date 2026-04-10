# Feature Docs - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Date: `2026-04-07`
- Owner: `Daniel Austin`

## Purpose
Provide one canonical index for the promoted `v0.87.1` feature docs so reviewers can map milestone planning, work packages, demos, and runtime architecture to the concrete feature surfaces.

## How To Use
- Start here when you want the promoted runtime feature set for `v0.87.1`.
- Use this index with `README.md`, `WBS_v0.87.1.md`, and `DEMO_MATRIX_v0.87.1.md`.
- Keep filenames, status lines, and scope language aligned with the milestone docs.

## Scope Interpretation

`v0.87.1` is the runtime-completion milestone. It includes the runtime primitives needed for:
- execution environment
- lifecycle control
- execution-boundary enforcement
- trace alignment
- local resilience and Shepherd preservation
- operator and review surfaces

It does not attempt to complete the richer higher-order systems planned for later milestones, including broader chronosense, identity, instinct, and bounded-agency layers beyond the runtime primitives required here.

## Feature Index

| Feature doc | Primary concern | Main WPs |
|---|---|---|
| `features/ADL_RUNTIME_ENVIRONMENT.md` | runtime environment surface and contracts | `WP-02`, `WP-06` |
| `features/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md` | runtime environment architecture and integration shape, including runtime reviewable trace proof paths | `WP-02`, `WP-04`, `WP-08` |
| `features/AGENT_LIFECYCLE.md` | lifecycle states, transitions, and continuity rules | `WP-03`, `WP-04`, `WP-07` |
| `features/EXECUTION_BOUNDARIES.md` | explicit execution-boundary enforcement and control points | `WP-03`, `WP-04`, `WP-06` |
| `features/LOCAL_RUNTIME_RESILIENCE.md` | local failure handling, restartability, and resilience guarantees | `WP-05`, `WP-07` |
| `features/SHEPHERD_RUNTIME_MODEL.md` | preservation, recovery, runtime stewardship model, and reviewer-facing recovery evidence | `WP-05`, `WP-07`, `WP-08` |

## Review Guidance
- Treat the README, WBS, sprint plan, and demo matrix as the milestone-level planning surface.
- Treat the files in `features/` as the promoted runtime architecture and behavior surface.
- Treat `bash adl/tools/demo_v0871_suite.sh` as the current WP-13 demo-suite entrypoint for implemented proof surfaces.
- Treat the WBS Acceptance Mapping as the canonical acceptance contract; feature docs should supply proof surfaces for those criteria, not introduce separate definitions of done.
- Any contradiction between milestone planning docs and these feature docs is a defect and must be resolved before review closeout.

## Exit Criteria
- Every promoted feature doc is linked from this index.
- WBS and demo coverage remain consistent with the promoted feature set.
- Status language in the feature docs matches the actual `v0.87.1` milestone scope.
