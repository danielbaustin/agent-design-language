# Work Breakdown Structure (WBS) Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Date: `{{date}}`
- Owner: `{{owner}}`

## How To Use
- Break work into independently-mergeable issues.
- Keep each item measurable and testable.
- Include deliverables + dependencies + issue links.

## WBS Summary
{{wbs_summary}}

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | {{package_1}} | {{description_1}} | {{deliverable_1}} | {{deps_1}} | {{issue_1}} |
| WP-02 | {{package_2}} | {{description_2}} | {{deliverable_2}} | {{deps_2}} | {{issue_2}} |
| WP-03 | {{package_3}} | {{description_3}} | {{deliverable_3}} | {{deps_3}} | {{issue_3}} |

## Sequencing
- Phase 1: {{phase_1}}
- Phase 2: {{phase_2}}
- Phase 3: {{phase_3}}

## Acceptance Mapping
- {{package_1}} -> {{acceptance_criteria_1}}
- {{package_2}} -> {{acceptance_criteria_2}}
- {{package_3}} -> {{acceptance_criteria_3}}

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
