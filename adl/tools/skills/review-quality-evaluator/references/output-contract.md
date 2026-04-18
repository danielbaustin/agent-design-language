# Output Contract

The review-quality-evaluator skill produces CodeBuddy quality-gate artifacts
from existing review packet or report evidence.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/quality-evaluation/
```

## Required Artifacts

### review_quality_evaluation.md

Required sections:

- Quality Gate Summary
- Scope And Source
- Scorecard
- Blocking Issues
- Warnings
- Specialist Coverage
- Template Compliance
- Unsupported Claims Check
- Residual Risk Clarity
- Publication Boundary
- Recommended Handoffs

### review_quality_evaluation.json

Required top-level fields:

- `schema`
- `run_id`
- `repo_name`
- `repo_ref`
- `status`
- `publication_intent`
- `score`
- `scorecard`
- `blocking_issues`
- `warnings`
- `specialist_coverage`
- `template_compliance`
- `unsupported_claims`
- `residual_risk_clarity`
- `publication_boundary`
- `recommended_handoffs`

## Status Values

- `pass`: no blockers or warnings were found for the requested gate.
- `partial`: no blockers were found, but warnings need human review.
- `fail`: one or more blockers should prevent customer-facing publication.
- `not_run`: no readable packet or report source was available.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into quality artifacts.
- Treat unsupported approval, compliance, publication, merge-readiness, or
  remediation-complete claims as blockers.
- Treat missing finding evidence as a blocker.
- Treat missing redaction status as a blocker for customer-private or public
  publication intent.
- Preserve missing specialist roles as coverage warnings unless policy marks
  them required blockers.
- Do not publish or transmit reports.
- Do not create issues, PRs, tests, diagrams, ADRs, fixes, or repository edits.
- Surface caveats, non-reviewed surfaces, disagreements, and residual risk
  instead of smoothing them over.
