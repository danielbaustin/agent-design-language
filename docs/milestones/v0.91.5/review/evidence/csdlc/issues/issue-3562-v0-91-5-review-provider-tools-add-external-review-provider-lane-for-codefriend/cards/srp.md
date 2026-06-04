---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend-review-prompt"
issue: 3562
task_id: "issue-3562"
version: "v0.91.5"
title: "[v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend"
branch: "codex/3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend"
generated_at: "2026-06-01T03:18:59Z"
card_status: "ready"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3562"
  - kind: "stp"
    ref: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/spp.md"
  - kind: "sor"
    ref: ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/stp.md"
  - ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sip.md"
in_scope_surfaces:
  - "adl/src/provider_communication.rs"
  - "docs/milestones/v0.91.5/review/review_provider/REVIEW_PROVIDER_V1_CONTRACT_3562.md"
evidence_policy:
  - "Use repository evidence, targeted validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "cargo test --manifest-path adl/Cargo.toml review_provider --lib"
  - "cargo test --manifest-path adl/Cargo.toml provider_communication --lib"
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
  - "This review does not claim the review-provider CLI executor is implemented."
  - "This review does not claim external provider output is authoritative."
policy_refs:
  - ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/stp.md"
  - ".adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sip.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "pass"
---

Canonical Template Source: `docs/templates/prompts/1.0.0/srp.md`

# Structured Review Prompt

## Review Summary

Bounded pre-PR review ran against the #3562 ReviewProviderV1 contract slice.
The first review found four actionable issues. All were fixed and re-reviewed.
The second review reported no remaining blockers.

## Scope Basis

- `.adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/stp.md`
- `.adl/v0.91.5/tasks/issue-3562__v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend/sip.md`

## In-Scope Surfaces

- `adl/src/provider_communication.rs`
- `docs/milestones/v0.91.5/review/review_provider/REVIEW_PROVIDER_V1_CONTRACT_3562.md`

## Evidence Rules

- Use repository evidence, targeted validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- `cargo test --manifest-path adl/Cargo.toml review_provider --lib`
- `cargo test --manifest-path adl/Cargo.toml provider_communication --lib`
- `cargo fmt --manifest-path adl/Cargo.toml`
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

- Route future implementation of `adl review-provider run` as a follow-on issue.
- Route CodeFriend ingestion of `ReviewProviderResultV1` as a follow-on issue.

## Non-Claims

- This review does not claim the review-provider CLI executor is implemented.
- This review does not claim external provider output is authoritative.
- This review does not claim all hosted/local providers have been live-tested as review providers.

## Review Results

### Findings

- `P1`: Blank scope refs could satisfy the bounded review target requirement.
- `P1`: Failed provider results could carry scored findings despite provider failure.
- `P2`: Authority boundary accepted arbitrary non-empty strings.
- `P2`: Review-provider schema-version fields were not validated.

### Dispositions

- `P1` blank scope refs: fixed by trimmed non-empty scope validation for issue, PR, diff, and file refs.
- `P1` failed provider findings: fixed by `validate_review_provider_result`, which rejects findings on failed, blocked, or skipped statuses.
- `P2` authority boundary: fixed with `REVIEW_PROVIDER_AUTHORITY_BOUNDARY_V1` exact-match validation.
- `P2` schema versions: fixed with schema-version validation for the request and embedded provider envelope.

### Recommended Outcome

PASS. Reviewer re-check reported no blockers remain in the reviewed slice.

## Notes

Reviewer subagent: Popper. The review was bounded to changed #3562 surfaces and did not edit files.
