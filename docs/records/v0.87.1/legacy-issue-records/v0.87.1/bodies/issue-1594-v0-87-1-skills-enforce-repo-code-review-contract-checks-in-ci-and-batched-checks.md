---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-87-1-skills-enforce-repo-code-review-contract-checks-in-ci-and-batched-checks"
title: "[v0.87.1][skills] Enforce repo-code-review contract checks in CI and batched checks"
labels:
  - "area:tools"
  - "type:bug"
  - "severity:medium"
  - "version:v0.87.1"
issue_number: 1594
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Mirrored from the authored GitHub issue body during bootstrap/init."
pr_start:
  enabled: false
  slug: "v0-87-1-skills-enforce-repo-code-review-contract-checks-in-ci-and-batched-checks"
---

## Summary

Add the new repo-review skill contract test to the repository’s normal enforcement surfaces so manifest/schema/guide drift cannot merge silently.

## Goal

Make the repo-review skill self-protecting by ensuring its dedicated contract test runs in CI and in the repo’s local batched-check path or another clearly documented equivalent preflight surface.

## Required Outcome

- wire the repo-review contract test into CI
- wire the same contract test into the local batched-check path or a clearly intentional equivalent
- keep the added coverage repo-local, deterministic, and low-noise
- ensure failures point clearly at manifest/schema/guide drift

## Deliverables

- CI update to execute the repo-review contract test
- local batched-check or equivalent preflight update to execute the same test
- any concise docs/readme update needed so operators know it is enforced

## Acceptance Criteria

- the repo-review contract test runs in CI
- the repo-review contract test runs in local batched checks or another documented default preflight surface
- the added coverage does not materially destabilize the normal check path
- failures clearly indicate repo-review contract drift rather than a vague shell failure

## Repo Inputs

- `.github/workflows/ci.yaml`
- `adl/tools/batched_checks.sh`
- `adl/tools/test_repo_code_review_skill_contracts.sh`
- `adl/tools/README.md`

## Dependencies

- the repo-review skill contract test should already exist and pass locally before this issue is executed

## Demo Expectations

- no demo required; targeted shell/CI validation is sufficient

## Non-goals

- broad CI redesign
- adding every skill contract test to every check surface
- changing the repo-review skill contract itself beyond what is needed for clear enforcement

## Issue-Graph Notes

- this issue hardens the repo-review skill after its schema normalization work
- it should remain narrow and enforcement-focused

## Notes

- the current gap is enforcement coverage, not missing contract-test content

## Tooling Notes

- use the ADL PR lifecycle
- prefer the same shell-test invocation in both CI and local batched checks where practical
