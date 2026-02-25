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
- `WP-01` is **always** the milestone **design pass** (canonical docs + WBS + decisions + sprint plan + checklist).
- Reserve the final WPs for the release tail in this order: `WP-13` demos, `WP-14` quality/coverage gate, `WP-15` docs/review convergence, `WP-16` release ceremony.
- The numeric IDs (WP-02..WP-N) may shift depending on milestone scope; numbering is structural, not semantic.
- `WP-13`..`WP-16` represent the canonical release tail positions when the milestone has ~12 implementation WPs.
- 16 WPs is a practical upper bound; smaller milestones may end earlier but must preserve the release-tail ordering.

## WBS Summary
{{wbs_summary}}

> Note: If the milestone contains fewer or more implementation WPs, adjust numbering accordingly,
> but always keep the final four WPs reserved for:
> demos → quality gate → docs/review → release ceremony (in that order).

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | {{description_01}} | {{deliverable_01}} | {{deps_01}} | {{issue_01}} |
| WP-02 | {{package_02}} | {{description_02}} | {{deliverable_02}} | {{deps_02}} | {{issue_02}} |
| WP-03 | {{package_03}} | {{description_03}} | {{deliverable_03}} | {{deps_03}} | {{issue_03}} |
| WP-04 | {{package_04}} | {{description_04}} | {{deliverable_04}} | {{deps_04}} | {{issue_04}} |
| WP-05 | {{package_05}} | {{description_05}} | {{deliverable_05}} | {{deps_05}} | {{issue_05}} |
| WP-06 | {{package_06}} | {{description_06}} | {{deliverable_06}} | {{deps_06}} | {{issue_06}} |
| WP-07 | {{package_07}} | {{description_07}} | {{deliverable_07}} | {{deps_07}} | {{issue_07}} |
| WP-08 | {{package_08}} | {{description_08}} | {{deliverable_08}} | {{deps_08}} | {{issue_08}} |
| WP-09 | {{package_09}} | {{description_09}} | {{deliverable_09}} | {{deps_09}} | {{issue_09}} |
| WP-10 | {{package_10}} | {{description_10}} | {{deliverable_10}} | {{deps_10}} | {{issue_10}} |
| WP-11 | {{package_11}} | {{description_11}} | {{deliverable_11}} | {{deps_11}} | {{issue_11}} |
| WP-12 | {{package_12}} | {{description_12}} | {{deliverable_12}} | {{deps_12}} | {{issue_12}} |
| WP-13 | Demo matrix + integration demos | {{description_13}} | {{deliverable_13}} | {{deps_13}} | {{issue_13}} |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | {{description_14}} | {{deliverable_14}} | {{deps_14}} | {{issue_14}} |
| WP-15 | Docs + review pass (repo-wide alignment) | {{description_15}} | {{deliverable_15}} | {{deps_15}} | {{issue_15}} |
| WP-16 | Release ceremony (final validation + tag + notes + cleanup) | {{description_16}} | {{deliverable_16}} | {{deps_16}} | {{issue_16}} |

## Sequencing
- Phase 1: {{phase_1}}
- Phase 2: {{phase_2}}
- Phase 3: {{phase_3}}

## Acceptance Mapping
- WP-01 (Design pass) -> {{acceptance_criteria_01}}
- WP-02 -> {{acceptance_criteria_02}}
- WP-03 -> {{acceptance_criteria_03}}
- WP-04 -> {{acceptance_criteria_04}}
- WP-05 -> {{acceptance_criteria_05}}
- WP-06 -> {{acceptance_criteria_06}}
- WP-07 -> {{acceptance_criteria_07}}
- WP-08 -> {{acceptance_criteria_08}}
- WP-09 -> {{acceptance_criteria_09}}
- WP-10 -> {{acceptance_criteria_10}}
- WP-11 -> {{acceptance_criteria_11}}
- WP-12 -> {{acceptance_criteria_12}}
- WP-13 (Demos) -> {{acceptance_criteria_13}}
- WP-14 (Quality gate) -> {{acceptance_criteria_14}}
- WP-15 (Docs/review) -> {{acceptance_criteria_15}}
- WP-16 (Release ceremony) -> {{acceptance_criteria_16}}

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.
