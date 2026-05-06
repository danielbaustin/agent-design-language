# Structured Review Policy Template

## Purpose

Use this template for ADL `SRP` artifacts. An `SRP` is an issue-local,
review-policy artifact that governs independent pre-PR review without acting as
the transport protocol or the review findings output.

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

## Template

```markdown
---
schema_version: "0.1"
artifact_type: "structured_review_policy"
name: "<short review-policy name>"
issue: <issue number>
task_id: "issue-<n>"
version: "<version>"
title: "<issue title>"
branch: "not bound yet"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "<issue URL or number>"
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
reviewer_constraints:
  - "<prohibited reviewer action>"
refusal_policy:
  - "<unsupported-claim refusal rule>"
follow_up_routing:
  - "<how findings route back into execution>"
non_claims:
  - "<what this policy does not claim>"
policy_refs:
  - ".adl/<version>/tasks/issue-<n>__<slug>/stp.md"
notes: "<optional note>"
---

# Structured Review Policy

## Review Summary

<Human-readable summary.>

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

## Non-Claims

- <non-claim>

## Notes

<Optional notes.>
```
