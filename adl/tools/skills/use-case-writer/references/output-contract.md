# Output Contract

The use-case-writer skill produces source-grounded use-case packets from a
declared brief, issue, feature, demo, or milestone context.

Default artifact root:

```text
.adl/reviews/use-case-writer/<run_id>/
```

## Required Artifacts

### use_case_packet.md

Required sections:

- Use Case Packet Summary
- Source
- Audience
- Actors
- Use Cases
- Acceptance Hooks
- Assumptions
- Unsupported Assumptions
- Non-goals
- Stop Boundary

### use_case_packet.json

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `source`
- `audience`
- `actors`
- `use_cases`
- `acceptance_hooks`
- `assumptions`
- `unsupported_assumptions`
- `non_goals`
- `stop_boundary`

## Status Values

- `ready`: packet is source-grounded and contains actors, use cases, and
  acceptance hooks.
- `partial`: packet is useful, but some actors, behavior, acceptance hooks, or
  assumptions need human review.
- `not_run`: no declared source was supplied.
- `blocked`: the request requires unsupported product commitments,
  implementation claims, issue creation, PR creation, publication, or repository
  mutation.

## Use Case Shape

Each use case must include:

- stable id
- title
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

## Rules

- Require a declared source before writing.
- Distinguish user goals, system behavior, acceptance hooks, and non-goals.
- Flag unsupported assumptions instead of making them requirements.
- Use repo-relative, issue-relative, or packet-relative paths.
- Do not write absolute host paths into packet artifacts.
- Do not claim implementation status unless source-supported.
- Do not create issues, PRs, implementation plans, release notes, or external
  publication artifacts.
