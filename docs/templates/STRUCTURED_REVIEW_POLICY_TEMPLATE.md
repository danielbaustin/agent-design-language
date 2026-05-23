# Structured Review Prompt Template

## Purpose

Compatibility note: the canonical copy-and-fill `SRP` template now lives at
`docs/templates/prompts/1.0.0/srp.md`. This legacy filename remains as a
descriptive compatibility reference for older links.

Use the canonical template for ADL `SRP` artifacts. An `SRP` is an issue-local
Structured Review Prompt. It carries both the review instructions/policy and
the review results: findings, dispositions, residual risks, and recommended
outcome.

The canonical card lifecycle is:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

Tooling may create `srp.md` as an early scaffold for path stability. That file
is not review-complete until the review instructions have been applied and the
review results have been recorded.

## File Location

```text
.adl/<version>/tasks/issue-<n>__<slug>/srp.md
```

Compatibility surface:

```text
.adl/cards/<issue>/srp_<issue>.md
```

Live `.adl/` issue records remain local workflow artifacts. Tracked milestone
docs may record SRP readiness evidence without publishing the local SRP files.

Filename compatibility note: this template currently remains at
`STRUCTURED_REVIEW_POLICY_TEMPLATE.md` so older docs and references continue to
resolve. Its semantic role is now Structured Review Prompt.

## Template

```markdown
---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<short review-prompt name>"
issue: <issue number>
task_id: "issue-<n>"
version: "<version>"
title: "<issue title>"
branch: "not bound yet"
lifecycle_stage: "SRP"
status: "draft"
activation_state: "scaffold | draft | active | reviewed | legacy_compatible"
source_refs:
  - kind: "issue"
    ref: "<issue URL or number>"
  - kind: "sip"
    ref: ".adl/<version>/tasks/issue-<n>__<slug>/sip.md"
  - kind: "stp"
    ref: ".adl/<version>/tasks/issue-<n>__<slug>/stp.md"
  - kind: "spp"
    ref: ".adl/<version>/tasks/issue-<n>__<slug>/spp.md"
  - kind: "sor"
    ref: ".adl/<version>/tasks/issue-<n>__<slug>/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".adl/<version>/tasks/issue-<n>__<slug>/stp.md"
  - ".adl/<version>/tasks/issue-<n>__<slug>/sip.md"
in_scope_surfaces:
  - "<path or bounded surface>"
evidence_policy:
  - "<what evidence the reviewer may inspect>"
validation_inputs:
  - "<what proof inputs already exist>"
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
review_results:
  findings_status: "not_run | findings_present | no_findings"
  recommended_outcome: "pass | block | needs_followup | not_run"
  findings:
    - severity: "P0 | P1 | P2 | P3"
      summary: "<finding summary>"
      evidence: "<repo-relative evidence reference>"
      disposition: "open | fixed | deferred | not_applicable"
  residual_risks:
    - "<risk or 'none'>"
reviewer_constraints:
  - "<prohibited reviewer action>"
refusal_policy:
  - "<unsupported-claim refusal rule>"
follow_up_routing:
  - "<how findings route back into execution>"
non_claims:
  - "<what this prompt does not claim>"
policy_refs:
  - ".adl/<version>/tasks/issue-<n>__<slug>/stp.md"
notes: "<optional note>"
---

# Structured Review Prompt

## Review Summary

<Human-readable review instruction and result summary.>

## Scope Basis

- <basis>

## In-Scope Surfaces

- <surface>

## Evidence Rules

- <rule>

## Validation Inputs

- <input>

## Allowed Dispositions

- PASS

## Reviewer Constraints

- <constraint>

## Refusal Policy

- <refusal rule>

## Follow-up Routing

- <route>

## Review Results

### Findings

- <finding, severity, evidence, and disposition; use `none` when no findings remain>

### Residual Risks

- <risk or `none`>

### Recommended Outcome

- <pass, block, needs_followup, or not_run>

## Non-Claims

- <non-claim>

## Notes

<Optional notes.>
```

## Compatibility Notes

- New SRP content should use `artifact_type: "structured_review_prompt"` and
  the Structured Review Prompt sections above.
- The validator remains backward-compatible with historical
  `structured_review_policy` cards while new scaffolds use the prompt type.
- Historical policy-only SRPs are legacy-compatible scaffolds, not final
  review-result truth.
