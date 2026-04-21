# Release Evidence Skill Input Schema

Schema id: `release_evidence.v1`

Use this schema when invoking `release-evidence` with structured input.

## Required Top-Level Fields

- `skill_input_schema`: must be `release_evidence.v1`
- `mode`: one of `assemble_milestone_evidence`, `refresh_release_evidence`, or
  `inspect_release_evidence`
- `milestone`: explicit milestone identifier such as `v0.90.2`
- `evidence`: source evidence locations and optional previously generated
  reports
- `policy`: stop-boundary and artifact-writing policy

## Evidence Object

Recommended fields:

- `milestone_root`: milestone documentation or closeout root
- `issue_records_root`: local issue/card root when available
- `review_roots`: internal, external, or gap review roots
- `demo_roots`: demo or proof artifact roots
- `previous_report_path`: previous release evidence report for refresh mode
- `report_path`: existing report to inspect

## Policy Object

Recommended fields:

- `write_evidence_artifact`: boolean
- `include_open_issues`: boolean
- `require_review_records`: boolean
- `require_validation_commands`: boolean
- `stop_before_release_approval`: must be true
- `stop_before_mutation`: must be true

## Example

```yaml
skill_input_schema: release_evidence.v1
mode: assemble_milestone_evidence
milestone: v0.90.2
evidence:
  milestone_root: docs/milestones/v0.90.2
  review_roots:
    - .adl/reviews/v0.90.2/internal-review
  demo_roots:
    - artifacts/v0.90.2
policy:
  write_evidence_artifact: true
  include_open_issues: true
  require_review_records: true
  require_validation_commands: true
  stop_before_release_approval: true
  stop_before_mutation: true
```

## Output

Default output root:

```text
.adl/reviews/release-evidence/<run_id>/
```

Required artifacts:

- `release_evidence_report.md`
- `release_evidence_report.json`

The report must preserve non-claims and safety flags. It must not approve,
publish, tag, merge, close, or mutate anything.

