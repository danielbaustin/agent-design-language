# Product Report Writer Skill Input Schema

Schema id: `product_report_writer.v1`

This schema describes structured input for the `product-report-writer` skill.
The skill writes customer-grade CodeBuddy report artifacts from existing review
packet evidence and stops before publication or mutation.

## Required Top-Level Fields

- `skill_input_schema`: must be `product_report_writer.v1`.
- `mode`: one of the supported modes below.
- `artifact_root`: review packet root or report artifact source.
- `audience`: intended audience, such as `internal_review`,
  `customer_private`, or `public_candidate`.
- `policy`: explicit report, privacy, and stop-boundary policy.

## Supported Modes

- `write_from_packet`: write from a CodeBuddy packet root.
- `write_from_synthesis`: write from a synthesis artifact plus supporting
  packet context.
- `write_from_specialist_artifacts`: write from specialist review artifacts.
- `refresh_report`: refresh an existing report after source artifacts change.

## Policy Fields

- `privacy_mode`: local_only, customer_private, public_candidate, or another
  explicit project policy value.
- `publication_intent`: none, internal_review, customer_private, or
  public_candidate.
- `write_report_artifact`: whether to write Markdown/JSON report artifacts.
- `require_redaction_status`: whether redaction evidence is required before
  report completion.
- `preserve_specialist_disagreement`: must be true.
- `stop_before_publication`: must be true.
- `stop_before_mutation`: must be true.

## Optional Fields

- `synthesis_artifact`: synthesis report to prioritize for findings.
- `specialist_artifacts`: role-to-path map for specialist artifacts.
- `existing_report_path`: required for `refresh_report`.
- `output_root`: output directory for `codebuddy_product_report.md` and
  `codebuddy_product_report.json`.
- `quality_gate_path`: review-quality evaluator artifact, when available.
- `redaction_report_path`: redaction report artifact, when available.

## Output Contract

The skill emits report-only artifacts:

- `codebuddy_product_report.md`
- `codebuddy_product_report.json`

The skill must not publish reports, create tracker items, open PRs, write tests,
generate diagrams, implement fixes, mutate customer repositories, or claim
approval, compliance, merge-readiness, or remediation completion.
