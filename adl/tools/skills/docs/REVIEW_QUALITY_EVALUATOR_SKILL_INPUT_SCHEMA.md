# Review Quality Evaluator Skill Input Schema

Schema id: `review_quality_evaluator.v1`

## Purpose

Evaluate a CodeBuddy review packet, product report, synthesis artifact, or
specialist artifact bundle against the review quality bar before customer-facing
use.

## Required Top-Level Fields

- `skill_input_schema`: must be `review_quality_evaluator.v1`.
- `mode`: one of `evaluate_packet`, `evaluate_report`, `evaluate_synthesis`, or
  `pre_publication_gate`.
- `artifact_root`: packet or report artifact root.
- `publication_intent`: `none`, `internal_review`, `customer_private`, or
  `public_candidate`.
- `policy`: quality-gate and stop-boundary policy.

## Optional Fields

- `report_path`: explicit report artifact for report-only evaluation.
- `synthesis_artifact`: explicit synthesis artifact.
- `specialist_artifacts`: map of specialist artifact paths.
- `output_root`: quality evaluation artifact destination.
- `redaction_report_path`: explicit redaction report path.
- `product_report_path`: explicit product report path.

## Policy Fields

- `publication_intent`
- `required_roles`
- `severity_floor`
- `require_redaction_status`
- `require_template_sections`
- `reject_unsupported_claims`
- `write_evaluation_artifact`
- `stop_before_publication`
- `stop_before_mutation`

## Example

```yaml
skill_input_schema: review_quality_evaluator.v1
mode: pre_publication_gate
artifact_root: .adl/reviews/codebuddy/run-2026-04-18
publication_intent: customer_private
policy:
  required_roles:
    - code
    - security
    - tests
    - docs
    - architecture
  severity_floor: P3
  require_redaction_status: true
  require_template_sections: true
  reject_unsupported_claims: true
  write_evaluation_artifact: true
  stop_before_publication: true
  stop_before_mutation: true
```

## Output Contract

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/quality-evaluation/
```

Required artifacts:

- `review_quality_evaluation.md`
- `review_quality_evaluation.json`

Statuses:

- `pass`: no blockers or warnings.
- `partial`: warnings need human review before customer-facing use.
- `fail`: blockers should prevent publication.
- `not_run`: source missing or unreadable.

## Stop Boundary

The skill must not publish reports, claim approval or compliance, create issues
or PRs, generate tests or diagrams, run specialist reviews, rewrite reports, or
mutate customer repositories.
