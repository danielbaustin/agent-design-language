# Work Breakdown Structure (WBS) Template

## Metadata
- Milestone: `<milestone>`
- Version: `<version>`
- Date: `<date>`
- Owner: `<owner>`

## How To Use
- Break work into independently-mergeable issues.
- Keep each item measurable and testable.
- Include deliverables + dependencies + issue links.
- `WP-01` is the milestone design pass (canonical docs + WBS + decisions + sprint plan + checklist).
- Use as many middle WPs as the milestone needs; current milestones often need more than 16 total WPs.
- Reserve the final WPs for the release tail in this order: demos/proof, quality gate, docs/review convergence, release ceremony.
- Do not hard-code exact tail WP numbers unless the milestone WBS has already fixed them.

## WBS Summary
<wbs_summary>

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | <description_01> | <deliverable_01> | <deps_01> | <issue_01> |
| WP-02 | <package_02> | <description_02> | <deliverable_02> | <deps_02> | <issue_02> |
| WP-03 | <package_03> | <description_03> | <deliverable_03> | <deps_03> | <issue_03> |
| WP-04 | <package_04> | <description_04> | <deliverable_04> | <deps_04> | <issue_04> |
| WP-05 | <package_05> | <description_05> | <deliverable_05> | <deps_05> | <issue_05> |
| WP-06 | <package_06> | <description_06> | <deliverable_06> | <deps_06> | <issue_06> |
| WP-07 | <package_07> | <description_07> | <deliverable_07> | <deps_07> | <issue_07> |
| WP-08 | <package_08> | <description_08> | <deliverable_08> | <deps_08> | <issue_08> |
| WP-09 | <package_09> | <description_09> | <deliverable_09> | <deps_09> | <issue_09> |
| WP-10 | <package_10> | <description_10> | <deliverable_10> | <deps_10> | <issue_10> |
| WP-11 | <package_11> | <description_11> | <deliverable_11> | <deps_11> | <issue_11> |
| WP-12 | <package_12> | <description_12> | <deliverable_12> | <deps_12> | <issue_12> |
| WP-N-3 | Demo matrix + integration demos | <description_demo_tail> | <deliverable_demo_tail> | <deps_demo_tail> | <issue_demo_tail> |
| WP-N-2 | Quality gate (focused proof + exceptions) | <description_quality_tail> | <deliverable_quality_tail> | <deps_quality_tail> | <issue_quality_tail> |
| WP-N-1 | Docs + review pass (repo-wide alignment) | <description_docs_tail> | <deliverable_docs_tail> | <deps_docs_tail> | <issue_docs_tail> |
| WP-N | Release ceremony (final validation + tag + notes + cleanup) | <description_release_tail> | <deliverable_release_tail> | <deps_release_tail> | <issue_release_tail> |

## Sequencing
- Phase 1: <phase_1>
- Phase 2: <phase_2>
- Phase 3: <phase_3>

## Acceptance Mapping
- WP-01 (Design pass) -> <acceptance_criteria_01>
- WP-02 -> <acceptance_criteria_02>
- WP-03 -> <acceptance_criteria_03>
- WP-04 -> <acceptance_criteria_04>
- WP-05 -> <acceptance_criteria_05>
- WP-06 -> <acceptance_criteria_06>
- WP-07 -> <acceptance_criteria_07>
- WP-08 -> <acceptance_criteria_08>
- WP-09 -> <acceptance_criteria_09>
- WP-10 -> <acceptance_criteria_10>
- WP-11 -> <acceptance_criteria_11>
- WP-12 -> <acceptance_criteria_12>
- WP-N-3 (Demos/proof) -> <acceptance_criteria_demo_tail>
- WP-N-2 (Quality gate) -> <acceptance_criteria_quality_tail>
- WP-N-1 (Docs/review) -> <acceptance_criteria_docs_tail>
- WP-N (Release ceremony) -> <acceptance_criteria_release_tail>

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
