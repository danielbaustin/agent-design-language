# <milestone> Sprint Plan

## Metadata
- Sprint: `<sprint_id>`
- Milestone: `<milestone>`
- Start date: `<start_date>`
- End date: `<end_date>`
- Owner: `<owner>`
- Status: `<status>`

## Status

`<status>`

## How To Use

- List work in planned execution order.
- Track blockers here rather than scattered chat notes.
- Keep sidecar or support work visible and bounded.
- Record closeout expectations before execution begins.
- For sprint umbrellas and mini-sprints, fill a Sprint Execution Packet using
  `docs/templates/SPRINT_EXECUTION_PACKET_TEMPLATE.md` or embed the same
  sections here so order, parallel lanes, PVF notes, and closeout truth are
  reviewable.

## Sprint Overview

`<sprint_goal>`

Planned scope:

- `<scope_item_1>`
- `<scope_item_2>`
- `<scope_item_3>`

## <sidecar_sprint_heading>

- Scope: `<sidecar_scope>`
- Boundary: `<sidecar_boundary>`
- Proof surface: `<sidecar_proof_surface>`

## Sprint Goals

- `<work_item_1>`
- `<work_item_2>`
- `<work_item_3>`

## Sprint Goal

`<sprint_goal>`

## Planned Scope

- `<scope_item_1>`
- `<scope_item_2>`
- `<scope_item_3>`

## Work Plan

| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | `<work_item_1>` | `<issue_1>` | `<owner_1>` | `<status_1>` |
| 2 | `<work_item_2>` | `<issue_2>` | `<owner_2>` | `<status_2>` |
| 3 | `<work_item_3>` | `<issue_3>` | `<owner_3>` | `<status_3>` |

## Execution Policy

- Each tracked issue follows `SIP -> STP -> SPP -> SRP -> SOR`.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run the smallest meaningful validation for each touched surface.
- Record proof truthfully in issue-local output records or review docs.
- Declare execution mode as `sequential`, `parallel`, or `hybrid`.
- Parallel child issue execution is allowed only when the Sprint Execution
  Packet names the safe lanes, write-set boundaries, proof lanes, and serial
  gates.

## Sprint Execution Packet

- Execution mode: `<execution_mode>`
- SEP artifact or issue-body section: `<sep_artifact_or_section>`
- Recommended execution order: `<recommended_execution_order>`
- Safe parallel lanes: `<safe_parallel_lanes>`
- Serial gates: `<serial_gates>`
- PVF / validation-tail notes: `<pvf_notes>`
- Residual routing policy: `<residual_routing_policy>`

## Cadence Expectations

- Preserve ordered execution where dependencies matter.
- Do not merge hidden sidecar work into unrelated issues.
- Escalate blockers as findings or follow-on issues.

## Risks / Dependencies

- Dependency: `<dependency_1>`
- Risk: `<risk_1>`
- Mitigation: `<mitigation_1>`

## Demo / Review Plan

- Demo artifact: `<demo_artifact>`
- Review date: `<review_date>`
- Sign-off owners: `<signoff_owners>`

## Closeout Bar

- All planned scope items are completed or explicitly deferred with rationale.
- Linked issues and PRs are updated and traceable.
- Focused validation is recorded for every touched surface.
- Sprint summary is captured in milestone docs.
- Child issue state, PR URLs, proof surfaces, review findings, worktree pruning
  state, and follow-up routing are recorded in the sprint closeout artifact.

## Exit Criteria

- All planned scope items completed or explicitly deferred with rationale.
- Linked issues/PRs updated and traceable.
- CI is green for merged work, or exceptions are documented.
- Sprint summary captured in milestone docs.
