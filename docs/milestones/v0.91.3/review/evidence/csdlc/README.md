# v0.91.3 C-SDLC Evidence

This directory is the durable review-evidence home for v0.91.3 C-SDLC proof records.

It contains tracked copies of issue-local proof packets that need to survive local `.adl/` cache cleanup and worktree removal. These records are milestone evidence, not active runtime state.

## Layout

- `issues/`: issue-scoped proof packets, including copied SIP, STP, SPP, SRP, and SOR cards.
- Future subdirectories may hold traces, artifacts, validation summaries, demo evidence, and review indexes when those proof surfaces become durable tracked records.

## Boundary

- Local `.adl/` paths remain execution/control-plane cache unless a tracked handoff says otherwise.
- This directory is for human-reviewable milestone evidence under `docs/`, not a top-level repository subsystem.
- Evidence here should use repository-relative references and avoid machine-local absolute paths.
