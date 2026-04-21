# Review Readiness Cleanup Skill Input Schema

Schema id: `review_readiness_cleanup.v1`

Use this schema when invoking `review-readiness-cleanup` with structured input.

## Required Top-Level Fields

- `skill_input_schema`: must be `review_readiness_cleanup.v1`
- `mode`: one of `inspect_review_packet`, `inspect_milestone_review`, or
  `refresh_readiness_cleanup`
- `review_root`: root path for the review packet or milestone review surface
- `target`: review-cycle identity, milestone, or previous report details
- `policy`: stop-boundary and artifact-writing policy

## Target Object

Recommended fields:

- `milestone`: milestone identifier when applicable
- `review_type`: internal, external, CodeBuddy, product, or release-tail review
- `required_surfaces`: expected file or section names
- `previous_report_path`: previous cleanup report for refresh mode

## Policy Object

Recommended fields:

- `write_cleanup_artifact`: boolean
- `allow_safe_mechanical_cleanup`: boolean
- `require_finding_register`: boolean
- `require_demo_or_proof_register`: boolean
- `stop_before_remediation`: must be true
- `stop_before_publication`: must be true
- `stop_before_review_approval`: must be true

## Example

```yaml
skill_input_schema: review_readiness_cleanup.v1
mode: inspect_milestone_review
review_root: .adl/reviews/v0.90.2/internal-review
target:
  milestone: v0.90.2
  review_type: internal
  required_surfaces:
    - review plan
    - finding register
    - demo matrix
policy:
  write_cleanup_artifact: true
  allow_safe_mechanical_cleanup: false
  require_finding_register: true
  require_demo_or_proof_register: true
  stop_before_remediation: true
  stop_before_publication: true
  stop_before_review_approval: true
```

## Output

Default output root:

```text
.adl/reviews/review-readiness-cleanup/<run_id>/
```

Required artifacts:

- `review_readiness_cleanup_report.md`
- `review_readiness_cleanup_report.json`

The report must preserve non-claims and safety flags. It must not remediate
findings, rewrite severity, publish reports, create tracker items, or approve
review readiness.

