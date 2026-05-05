# Structured Plan Prompt Template

## Purpose

Use this template for ADL `SPP` artifacts. An `SPP` is an issue-local,
read-only planning artifact created after `STP`, `SIP`, and `SOR` exist and
before execution is bound.

This template is compatible with Codex plan mode by carrying a simple
`codex_plan` list. Each item has:

- `step`: one concise execution step
- `status`: `pending`, `in_progress`, or `completed`

For pre-execution plans, all implementation steps should normally remain
`pending`. Do not mark work `completed` unless it has actually happened.

## File Location

```text
.adl/<version>/tasks/issue-<n>__<slug>/spp.md
```

Live `.adl/` issue records remain local workflow artifacts. Tracked milestone
docs may record SPP readiness evidence without publishing the local SPP files.

## Template

```markdown
---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<short plan name>"
issue: <issue number>
task_id: "issue-<n>"
run_id: "issue-<n>"
version: "<version>"
title: "<issue title>"
branch: "not bound yet"
status: "draft"
plan_revision: 1
source_refs:
  - kind: "issue"
    ref: "<issue URL or number>"
  - kind: "stp"
    ref: ".adl/<version>/tasks/issue-<n>__<slug>/stp.md"
scope:
  files:
    - "<path or glob>"
  components:
    - "<component>"
  out_of_scope:
    - "<explicit non-goal>"
constraints:
  - "read_only"
  - "no_mutation"
  - "no_side_effects"
confidence: "medium"
plan_summary: "<one paragraph summary>"
assumptions:
  - "<assumption>"
proposed_steps:
  - id: "step-1"
    description: "<execution step>"
    expected_output: "<artifact or result>"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "<same concise execution step used by Codex plan mode>"
    status: "pending"
affected_areas:
  - "<path, module, doc, demo, or subsystem>"
invariants_to_preserve:
  - "<invariant>"
risks_and_edge_cases:
  - "<risk>"
test_strategy:
  - "<validation command or review check>"
execution_handoff: "<how execution should use this plan>"
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "<when to stop and re-plan>"
alternatives_considered:
  - description: "<alternative>"
    reason_not_chosen: "<reason>"
review_hooks:
  - "<review emphasis>"
notes: "<optional note>"
---

# Structured Plan Prompt

## Plan Summary

<Human-readable summary.>

## Codex Plan

1. [pending] <step>

## Assumptions

- <assumption>

## Proposed Steps

1. <step>

## Affected Areas

- <area>

## Invariants To Preserve

- <invariant>

## Risks And Edge Cases

- <risk>

## Test Strategy

- <validation>

## Execution Handoff

<Instructions for the execution agent.>

## Stop Conditions

- <stop condition>

## Notes

<Optional notes.>
```

