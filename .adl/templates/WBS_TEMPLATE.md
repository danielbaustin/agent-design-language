# Work Breakdown Structure (WBS) Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Date: `{{date}}`
- Owner: `{{owner}}`

## How To Use
- Break work into independently mergeable issues.
- Keep each task measurable and testable.
- Reference issue IDs and expected deliverables for every task.

## WBS Summary
`{{wbs_summary}}`

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | `{{package_1}}` | `{{description_1}}` | `{{deliverable_1}}` | `{{deps_1}}` | `{{issue_1}}` |
| WP-02 | `{{package_2}}` | `{{description_2}}` | `{{deliverable_2}}` | `{{deps_2}}` | `{{issue_2}}` |
| WP-03 | `{{package_3}}` | `{{description_3}}` | `{{deliverable_3}}` | `{{deps_3}}` | `{{issue_3}}` |

## Sequencing
- Phase 1: `{{phase_1}}`
- Phase 2: `{{phase_2}}`
- Phase 3: `{{phase_3}}`

## Acceptance Mapping
- `{{work_package_1}}` -> `{{acceptance_criteria_1}}`
- `{{work_package_2}}` -> `{{acceptance_criteria_2}}`
- `{{work_package_3}}` -> `{{acceptance_criteria_3}}`

## Exit Criteria
- Every in-scope milestone requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
