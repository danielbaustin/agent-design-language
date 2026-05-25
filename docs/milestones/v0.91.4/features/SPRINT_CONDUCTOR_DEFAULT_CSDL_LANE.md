# Sprint Conductor Default C-SDLC Lane

## Status

Planned `v0.91.4` feature.

## Purpose

Make sprint-conductor operation safe for C-SDLC by default. The conductor lane
must not advance over stale child truth, skip review results, or call a sprint
clean when its evidence packet is incomplete.

## Scope

This feature covers:

- ordered child issue execution
- child closeout gates
- sprint state artifacts
- umbrella review and closeout truth
- combined-lane validation requirements
- blocked-state reporting when a child or umbrella is not truthful
- demo evidence for repeated sprint execution

## Acceptance Criteria

- The sprint conductor cannot advance to the next child while the current
  child is only waiting for review or missing closeout truth.
- Sprint orchestration stays in sprint-conductor state, child issue state, and
  closeout artifacts; it does not redefine `SPP` as a sprint-level control card.
- Umbrella closeout requires current sprint state, child issue truth, review
  truth, and evidence paths.
- Durable sprint records are written under
  `docs/milestones/v0.91.4/review/evidence/csdlc/sprints/`, not only ignored local state.
- Combined-lane validation is required where process state crosses issue
  boundaries.
- The lane records when a sprint is useful with repairs rather than clean.
- The repeatability demo uses this lane without special manual exceptions.

## Non-Goals

- This feature does not remove operator authority.
- This feature does not permit silent issue creation, merge, or closeout.
- This feature does not make optional external workspaces canonical state.
