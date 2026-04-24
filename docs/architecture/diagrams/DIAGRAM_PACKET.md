# ADL Architecture Diagram Packet

## Validation

Run source validation:

```bash
python3 adl/tools/validate_architecture_docs.py
```

Optional Mermaid rendering, when Mermaid CLI is installed:

```bash
mmdc -i docs/architecture/diagrams/system_context.mmd -o artifacts/diagrams/adl-system-context.svg
```

## Diagram A1: System Context

- Source: `docs/architecture/diagrams/system_context.mmd`
- Purpose: show ADL's operator, GitHub, skill, runtime, provider, tool, artifact,
  and documentation boundaries.
- Evidence: `docs/architecture/ADL_ARCHITECTURE.md`,
  `adl/src/adl/types.rs`, `docs/default_workflow.md`.
- Assumptions: external providers and GitHub are boundaries, not ADL internals.
- Unsupported claims excluded: no claim that GitHub merge is automatic.

## Diagram A2: Runtime Lifecycle

- Source: `docs/architecture/diagrams/runtime_lifecycle.mmd`
- Purpose: show parse, resolve, plan, schedule, execute, trace, artifact, and
  finish flow.
- Evidence: `adl/src/execution_plan.rs`, `adl/src/execute/mod.rs`,
  `adl/src/trace.rs`, `adl/src/artifacts.rs`.
- Assumptions: labels summarize code paths rather than all internal functions.
- Unsupported claims excluded: no claim that every provider/tool path has equal
  maturity.

## Diagram A3: Control Plane Lifecycle

- Source: `docs/architecture/diagrams/control_plane_lifecycle.mmd`
- Purpose: show issue-to-closeout lifecycle and worktree-first execution.
- Evidence: `docs/default_workflow.md`, `adl/src/control_plane.rs`, `pr-*`
  skills.
- Assumptions: merge remains a human/repository event.
- Unsupported claims excluded: no silent merge or silent closeout behavior.

## Diagram A4: Task Bundle State

- Source: `docs/architecture/diagrams/task_bundle_state.mmd`
- Purpose: show STP/SIP/SOR bundle state transitions.
- Evidence: `adl/src/control_plane.rs`, `docs/default_workflow.md`.
- Assumptions: state labels describe lifecycle truth, not a single Rust enum.
- Unsupported claims excluded: no guarantee that every historic bundle is clean.

## Diagram A5: Skill Orchestration

- Source: `docs/architecture/diagrams/skill_orchestration.mmd`
- Purpose: show conductor, lifecycle, editor, specialist, diagram, redaction,
  fitness, and report skill boundaries.
- Evidence:
  `docs/milestones/v0.89.1/features/SKILL_COMPOSITION_MODEL.md`,
  `docs/milestones/v0.89.1/features/OPERATIONAL_SKILLS_SUBSTRATE.md`.
- Assumptions: specialist skill names are representative of the current
  CodeBuddy review stack.
- Unsupported claims excluded: no claim that conductor owns downstream work.

## Diagram A6: Artifact Data Flow

- Source: `docs/architecture/diagrams/artifact_data_flow.mmd`
- Purpose: show deterministic run artifact families used for reviewer
  reconstruction.
- Evidence: `adl/src/artifacts.rs`, `adl/src/trace.rs`,
  `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`.
- Assumptions: artifact groups summarize deterministic path families.
- Unsupported claims excluded: no claim that prose reports are the runtime truth.

## Diagram A7: Trust Boundaries

- Source: `docs/architecture/diagrams/trust_boundaries.mmd`
- Purpose: show document, runtime, sandbox, signing, remote execution, provider,
  and publication boundaries.
- Evidence: `adl/src/signing.rs`, `adl/src/sandbox.rs`,
  `adl/src/remote_exec/security.rs`, `adl/src/provider/mod.rs`.
- Assumptions: publication is a review-process boundary, not a Rust module
  boundary.
- Unsupported claims excluded: no claim that all workflows are safe by default.

## Diagram A8: Task Bundle and PR Lifecycle

- Source: `docs/architecture/diagrams/task_bundle_and_pr_lifecycle.mmd`
- Purpose: show issue binding, worktree execution, PR draft, janitor review, and
  closeout checkpoints.
- Evidence: `docs/default_workflow.md`, `adl/src/control_plane.rs`,
  `docs/architecture/ADL_ARCHITECTURE.md`.
- Assumptions: merge and closeout are external governance events; state names in
  the diagram capture lifecycle checkpoints used by issue processing.
- Unsupported claims excluded: no claim that merge timing or conflict resolution is
  always fast, or that checks do not require reruns.

## Diagram A9: Runtime v2 Subsystem Structure

- Source: `docs/architecture/diagrams/runtime_v2_subsystem_structure.mmd`
- Purpose: show deterministic relationships among plan/scheduler/trace/artifacts,
  control-plane issue binding, and long-lived continuity artifacts.
- Evidence: `adl/src/execution_plan.rs`, `adl/src/execute/mod.rs`,
  `adl/src/trace.rs`, `adl/src/artifacts.rs`,
  `adl/src/long_lived_agent.rs`, `adl/src/control_plane.rs`.
- Assumptions: component labels are bounded slices of the v0.90/0.90.4 runtime
  surface and omit non-canonical UI/demo-only modules.
- Unsupported claims excluded: no claim that the diagram is the complete runtime
  source tree or a full threat model.

## Diagram A10: CodeBuddy Review Flow

- Source: `docs/architecture/diagrams/codebuddy_review_flow.mmd`
- Purpose: show the bounded CodeBuddy/ADL review specialist lane for architecture
  and proof surfaces.
- Evidence: `docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md`,
  `docs/architecture/ADL_ARCHITECTURE.md`,
  `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md`.
- Assumptions: not every specialist runs for every issue; this is the canonical
  review-sequence template.
- Unsupported claims excluded: no claim that all reviewers agree on every finding
  or that every issue follows the same full specialist breadth.
