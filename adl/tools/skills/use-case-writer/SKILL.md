---
name: use-case-writer
description: Turn one declared source brief, issue, feature, demo, or milestone context into grounded use-case packets with actors, goals, scenarios, success and failure flows, acceptance hooks, assumptions, unsupported assumptions, and non-goals without inventing product commitments or implementation status.
---

# Use Case Writer

Use this skill when a product, runtime, demo, feature, or milestone idea needs a
reviewable use-case packet. The skill writes source-grounded use cases; it does
not invent roadmap commitments, implementation status, or requirements that are
not supported by the supplied brief.

## Quick Start

1. Confirm the declared source:
   - issue body
   - feature brief
   - product note
   - demo plan
   - milestone planning doc
2. Identify the actors, user goals, system behavior, acceptance hooks, and
   non-goals that are explicitly supported.
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/write_use_cases.py <use-case-root> --out <artifact-root>`
4. Review unsupported assumptions before treating the packet as product truth.
5. Stop before creating issues, PRs, implementation plans, or external claims.

## Required Inputs

At minimum, gather:

- `mode`
- `source`
- `audience`
- `policy`

Supported modes:

- `write_prd_use_cases`
- `write_issue_use_cases`
- `write_demo_use_cases`
- `write_milestone_use_cases`

Useful policy fields:

- `source_required`
- `unsupported_assumptions_policy`
- `implementation_status_claims_allowed`
- `issue_creation_allowed`
- `write_use_case_artifact`
- `stop_before_commitment`

If there is no declared source brief or issue context, stop and report
`not_run`. Do not backfill missing product intent from plausible-sounding
requirements.

## Use Case Rules

Each use case should distinguish:

- actor
- user goal
- system behavior
- trigger
- preconditions
- success flow
- failure or edge flow
- acceptance hooks
- evidence source
- non-goals
- unsupported assumptions

When source evidence is incomplete, record the unsupported assumption instead of
silently turning it into a requirement.

## Output

Write Markdown and JSON artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/use-case-writer/<run_id>/
```

Required artifacts:

- `use_case_packet.md`
- `use_case_packet.json`

Use the detailed contract in `references/output-contract.md`.

## Stop Boundary

This skill must not:

- create GitHub issues, PRs, commits, release notes, or implementation plans
- claim implementation status unless the source explicitly proves it
- invent product requirements, user personas, business commitments, timelines,
  success metrics, or demos
- convert unsupported assumptions into acceptance criteria
- replace implementation planning, product approval, or milestone approval
- publish externally without explicit approval

Handoff candidates:

- `finding-to-issue-planner` when human-approved issue candidates are needed.
- `documentation-specialist` or docs authoring workflows when a packet should
  become published documentation.
- `gap-analysis` when a use-case packet must be checked against implementation
  or milestone evidence.
- `refactoring-helper` only when use cases expose code-structure follow-up that
  needs bounded refactor planning.

## Blocked States

Return `not_run` when the source brief or issue context is missing.

Return `blocked` when the requested output requires unsupported product
commitments, implementation claims, issue creation, PR creation, publication, or
repository mutation.

Return `partial` when a packet can be produced but important actor, behavior, or
acceptance evidence is missing.
