---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive-review-prompt"
issue: 3473
task_id: "issue-3473"
version: "v0.91.5"
title: "[v0.91.5][WP-05][docs] Inventory and disposition local ADL state for cleanup and ObsMem archive"
branch: "codex/3473-v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive"
generated_at: "2026-06-04T20:59:18Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3473"
  - kind: "stp"
    ref: ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/spp.md"
  - kind: "sor"
    ref: ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/stp.md"
  - ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/sip.md"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence, targeted validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope."
  - "Do not merge, publish, or close the issue."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
  - "Refuse approving behavior outside the recorded issue scope."
follow_up_routing:
  - "Route actionable defects back to the issue branch before PR publication."
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/stp.md"
  - ".adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/sip.md"
review_results:
  findings_status: "not_run"
  recommended_outcome: "not_run"
notes: "Structured Review Prompt prepared before execution; finalize with actual review findings before PR publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.0/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/stp.md
- .adl/v0.91.5/tasks/issue-3473__v0-91-5-wp-05-docs-inventory-and-disposition-local-adl-state-for-cleanup-and-obsmem-archive/sip.md

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence, targeted validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- Issue-local proofs recorded in the SOR.

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope.
- Do not merge, publish, or close the issue.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.
- Refuse approving behavior outside the recorded issue scope.

## Follow-up Routing

- Route actionable defects back to the issue branch before PR publication.

## Non-Claims

- This prompt does not claim review has already run.
- This prompt does not guarantee review quality by itself.

## Review Results

When finalizing review, record the machine-readable review result in frontmatter:

```yaml
review_results:
  findings_status: "no_findings | findings_present"
  recommended_outcome: "pass | block | needs_followup"
```

### Findings

- Not run yet; implementation has not been bound.

### Dispositions

- Not applicable until review runs.

### Recommended Outcome

- Not run yet.

## Notes

Structured Review Prompt prepared before execution; finalize with actual review findings before PR publication.
