---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "example-srp-repair-review-prompt"
issue: 4004
task_id: "issue-4004"
version: "v0.91.4"
title: "[example][SRP] Repaired review truth"
branch: "codex/4004-example-srp-repair"
generated_at: "2026-05-26T12:00:00Z"
card_status: "ready"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/4004"
  - kind: "stp"
    ref: ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/spp.md"
  - kind: "sor"
    ref: ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/stp.md"
  - ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/sip.md"
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
  - ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/stp.md"
  - ".adl/v0.91.4/tasks/issue-4004__example-srp-repair/sip.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "pass"
notes: "Example repaired SRP showing truthful findings and disposition recording after bounded review."
---

Canonical Template Source: `docs/templates/prompts/1.0.0/srp.md`

# Structured Review Prompt

## Review Summary

Bounded pre-PR review completed for the example card-repair slice. Review
surfaced one medium documentation-truth issue, which was fixed before
publication.

## Scope Basis

- .adl/v0.91.4/tasks/issue-4004__example-srp-repair/stp.md
- .adl/v0.91.4/tasks/issue-4004__example-srp-repair/sip.md

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence, targeted validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- `bash adl/tools/test_card_editor_repair_examples.sh`
- `git diff --check`

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

- `Medium` The original example text implied the browser prompt editor could replace editor-skill repair. Fixed by clarifying the human-editor boundary and adding durable repair examples.

### Dispositions

- Fixed the actionable finding in scope before publication and tied proof to the focused repair-example test lane.

### Recommended Outcome

- PASS

## Notes

This example shows truthful review-result recording without inventing review coverage beyond the bounded evidence listed above.
