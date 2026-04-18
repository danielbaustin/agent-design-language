---
name: finding-to-issue-planner
description: Convert CodeBuddy review findings into grouped, reviewable issue candidates with titles, bodies, evidence, acceptance criteria, non-goals, validation plans, and dependency links while stopping before tracker creation or repository mutation.
---

# Finding To Issue Planner

Convert source-grounded CodeBuddy review findings into human-reviewable issue
candidates. This skill is a planning and approval-boundary skill, not a tracker
mutation skill and not a remediation workflow.

Use this skill after specialist review, synthesis, redaction, or quality-gate
artifacts identify findings that may need follow-up issues.

## Quick Start

1. Confirm the finding source:
   - specialist review artifact
   - synthesis report
   - review packet root
   - existing finding list
2. Confirm the target tracker style and approval policy.
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/plan_review_issues.py <finding-file-or-root> --out <artifact-root>`
4. Review the emitted issue candidates for severity, evidence, grouping, and
   acceptance criteria.
5. Stop before issue creation. Hand approved candidates to the operator or a
   separate issue-creation workflow.

## Required Inputs

At minimum, gather:

- `finding_source`
- `mode`
- `policy`

Supported modes:

- `plan_from_review`
- `plan_from_synthesis`
- `plan_from_packet`
- `refresh_issue_plan`

Useful policy fields:

- `approval_required`
- `tracker_creation_allowed`
- `grouping_policy`
- `severity_floor`
- `preserve_specialist_disagreement`
- `stop_before_mutation`

If there is no concrete finding source, stop and report `not_run`.

## Workflow

### 1. Establish Scope

Record:

- source artifact or packet root
- review roles included
- non-reviewed surfaces
- target tracker, if known
- approval boundary
- severity floor
- grouping policy

Do not infer customer approval. If approval is absent, emit candidates only.

### 2. Extract Findings

For each finding, preserve:

- stable finding id, if present
- severity
- confidence
- source role or specialist
- title
- affected path or artifact
- trigger scenario
- evidence
- impact
- recommended action
- validation or proof gap
- related findings

Do not create issues from unsupported claims. Mark weak or incomplete findings
as `needs_human_review` or `deferred` rather than making them look ready.

### 3. Group Without Hiding Severity

Group exact or near-duplicate findings only when the evidence supports the same
remediation target.

When grouping:

- preserve every source finding id
- preserve the highest severity
- preserve specialist disagreement
- list secondary affected paths
- avoid collapsing distinct root causes into one issue

### 4. Draft Issue Candidates

Each candidate must include:

- title
- severity
- source finding ids
- source roles
- evidence summary
- affected paths or artifacts
- problem statement
- acceptance criteria
- validation plan
- non-goals
- dependency or ordering notes
- approval status

Titles should be specific enough for a tracker list and boring enough to be
searchable. Avoid dramatic language and avoid claiming confirmed impact beyond
the evidence.

### 5. Stop Before Mutation

This skill must not:

- create GitHub, Linear, Jira, or other tracker items
- open pull requests
- edit customer repositories
- generate fixes
- generate tests directly
- synthesize a final customer report

Handoff candidates to:

- `review-to-test-planner` for test task planning
- `test-generator` for approved bounded test generation
- `pr-init` or the repo workflow conductor only after explicit operator approval
- `product-report-writer` for customer report language

## Output

Write a Markdown issue plan and a JSON issue-candidate list when an artifact
root is available.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/issue-planning/
```

Required artifacts:

- `issue_candidates.md`
- `issue_candidates.json`

Use the detailed contract in `references/output-contract.md`.

## Blocked States

Return `not_run` when the source is missing or unreadable.

Return `blocked` when:

- tracker creation was requested without explicit approval
- findings lack evidence
- the requested grouping would hide severity or disagreement
- the source artifact includes private data that has not passed the configured
  redaction gate

Return `partial` when some findings can be planned and others must be deferred.
