# Gap Analysis Skill Input Schema

Schema id: `gap_analysis.v1`

## Purpose

Compare an explicit expected baseline against observed implementation, docs,
tests, review, PR, milestone, or closeout evidence and produce a bounded
findings-first gap report.

## Required Top-Level Fields

- `skill_input_schema`: must be `gap_analysis.v1`.
- `mode`: one of `compare_issue_to_implementation`,
  `compare_milestone_to_evidence`, `compare_spec_to_docs`,
  `compare_review_to_closeout`, or `compare_packet_to_report`.
- `expected_baseline`: issue, milestone, spec, PR, review packet, or closeout
  source of intended truth.
- `observed_evidence`: implementation, docs, tests, review, PR, report, or
  closeout evidence to compare.
- `policy`: comparison and stop-boundary policy.

## Optional Fields

- `artifact_root`: report destination.
- `issue_ref`
- `milestone_plan`
- `spec_path`
- `review_packet_path`
- `report_path`
- `closeout_record`

## Policy Fields

- `severity_floor`
- `required_gap_types`
- `uncertainty_policy`
- `issue_creation_allowed`
- `write_gap_artifact`
- `stop_before_fix`
- `stop_before_mutation`

## Example

```yaml
skill_input_schema: gap_analysis.v1
mode: compare_issue_to_implementation
expected_baseline:
  issue_ref: "#2044"
  acceptance_criteria_path: .adl/v0.90/tasks/issue-2044__backlog-skills-add-gap-analysis-skill/stp.md
observed_evidence:
  changed_paths:
    - adl/tools/skills/gap-analysis/SKILL.md
    - adl/tools/skills/gap-analysis/adl-skill.yaml
    - adl/tools/test_gap_analysis_skill_contracts.sh
  validation_commands:
    - bash adl/tools/test_gap_analysis_skill_contracts.sh
policy:
  severity_floor: P3
  required_gap_types:
    - missing_evidence
    - implementation_gap
    - docs_drift
    - test_gap
    - closeout_drift
  uncertainty_policy: record_explicitly
  issue_creation_allowed: false
  write_gap_artifact: true
  stop_before_fix: true
  stop_before_mutation: true
```

## Output Contract

Default artifact root:

```text
.adl/reviews/gap-analysis/<run_id>/
```

Required artifacts:

- `gap_analysis_report.md`
- `gap_analysis_report.json`

Statuses:

- `pass`: no gaps found.
- `partial`: gaps or missing evidence exist.
- `fail`: severe gap should block the requested closeout or release decision.
- `not_run`: explicit baseline missing.
- `blocked`: requested action violates stop boundary.

## Stop Boundary

The skill must not fix implementation, docs, tests, cards, or reports; create
issues or PRs; approve closeout, publication, or release readiness; or mutate
repositories.
