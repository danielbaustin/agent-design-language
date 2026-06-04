---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter-review-prompt"
issue: 3472
task_id: "issue-3472"
version: "v0.91.5"
title: "[v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter"
branch: "codex/3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter"
generated_at: "2026-06-04T19:39:09Z"
card_status: "approved"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3472"
  - kind: "stp"
    ref: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/spp.md"
  - kind: "sor"
    ref: ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/stp.md"
  - ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sip.md"
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
  - ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/stp.md"
  - ".adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sip.md"
review_results:
  findings_status: "no_open_findings"
  recommended_outcome: "pass"
notes: "Pre-PR review was run over the exporter, tests, docs, and generated packet. One review observation about stale packet replacement was fixed before publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.0/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review ran after implementation and before PR publication.

## Scope Basis

- .adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/stp.md
- .adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sip.md

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

Machine-readable review result is recorded in frontmatter:

```yaml
review_results:
  findings_status: "no_open_findings"
  recommended_outcome: "pass"
```

### Findings

- No open findings remain.
- Review observation fixed during review: the exporter now validates all source cards before replacing the packet directory, removes stale packet output on repeat export, and has a regression assertion for stale-file removal.

### Dispositions

- Stale-output replacement observation: fixed in `adl/src/cli/tooling_cmd/public_prompt_packet.rs` and covered by `public_prompt_packet_export_writes_manifest_readme_and_cards`.

### Recommended Outcome

- PASS. Proceed to `pr finish` / PR publication after final focused validation.

## Notes

Pre-PR review completed with no open findings. Residual risk: the first exporter performs refuse-not-rewrite hygiene checks; richer redaction policy remains owned by later public prompt validation work.
