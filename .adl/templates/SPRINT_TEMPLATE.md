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
- Track blockers in this file instead of scattered chat notes.

## Sprint Goal
`{{sprint_goal}}`

## Planned Scope
- `{{scope_item_1}}`
- `{{scope_item_2}}`
- `{{scope_item_3}}`

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | `{{work_item_1}}` | `{{issue_1}}` | `{{owner_1}}` | `{{status_1}}` |
| 2 | `{{work_item_2}}` | `{{issue_2}}` | `{{owner_2}}` | `{{status_2}}` |
| 3 | `{{work_item_3}}` | `{{issue_3}}` | `{{owner_3}}` | `{{status_3}}` |

## Cadence Expectations
- Use issue cards (`input`/`output`) for each item.
- Keep changes scoped per issue and use draft PRs until checks pass.
- Run required quality gates (`fmt`, `clippy`, `test`) for code changes.

## Risks / Dependencies
- Dependency: `{{dependency_1}}`
  - Risk: `{{risk_1}}`
  - Mitigation: `{{mitigation_1}}`

## Demo / Review Plan
- Demo artifact: `{{demo_artifact}}`
- Review date: `{{review_date}}`
- Sign-off owners: `{{signoff_owners}}`

## Exit Criteria
- All planned scope items are completed or explicitly deferred with rationale.
- Linked issues and PRs are updated and traceable.
- CI is green for merged work.
- Sprint summary is captured in the milestone report/release notes inputs.
