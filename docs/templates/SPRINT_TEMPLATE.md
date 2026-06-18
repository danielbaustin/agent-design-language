# Sprint Template

## Metadata
- Sprint: `{{sprint_id}}`
- Milestone: `{{milestone}}`
- Start date: `{{start_date}}`
- End date: `{{end_date}}`
- Owner: `{{owner}}`

## How To Use
- Keep scope small enough to finish with green CI and merged PRs.
- List work items in planned execution order.
- Track blockers here (not scattered chat notes).
- For a sprint umbrella or mini-sprint, include or link a Sprint Execution
  Packet from `docs/templates/SPRINT_EXECUTION_PACKET_TEMPLATE.md` so
  dependency order, safe parallel lanes, PVF notes, and closeout bars are
  durable rather than chat-only.

## Sprint Goal
{{sprint_goal}}

## Planned Scope
- {{scope_item_1}}
- {{scope_item_2}}
- {{scope_item_3}}

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | {{work_item_1}} | {{issue_1}} | {{owner_1}} | {{status_1}} |
| 2 | {{work_item_2}} | {{issue_2}} | {{owner_2}} | {{status_2}} |
| 3 | {{work_item_3}} | {{issue_3}} | {{owner_3}} | {{status_3}} |

## Cadence Expectations
- Use issue cards (`SIP -> STP -> SPP -> SRP -> SOR`) for each tracked item.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run the smallest truthful validation for each touched surface.
- Record whether the sprint runs `sequential`, `parallel`, or `hybrid`, and
  name any serial gates before work begins.

## Sprint Execution Packet

- Execution mode: `{{execution_mode}}`
- SEP artifact or issue-body section: `{{sep_artifact_or_section}}`
- Safe parallel lanes: `{{safe_parallel_lanes}}`
- Serial gates: `{{serial_gates}}`
- PVF / validation-tail notes: `{{pvf_notes}}`

## Risks / Dependencies
- Dependency: {{dependency_1}}
  - Risk: {{risk_1}}
  - Mitigation: {{mitigation_1}}

## Demo / Review Plan
- Demo artifact: {{demo_artifact}}
- Review date: {{review_date}}
- Sign-off owners: {{signoff_owners}}

## Exit Criteria
- All planned scope items completed or explicitly deferred with rationale.
- Linked issues/PRs updated and traceable.
- CI is green for merged work.
- Sprint summary captured in milestone docs.
- Sprint closeout records child issue state, PR URLs, proof surfaces, review
  findings, worktree pruning state, and residual routing.
