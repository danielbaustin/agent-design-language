# Use Case Writer Skill Input Schema

Schema id: `use_case_writer.v1`

## Purpose

Turn one declared source brief, issue, feature, demo, or milestone context into
grounded use cases, actor flows, concrete scenarios, and acceptance-oriented
examples without inventing product commitments or implementation status.

## Required Top-Level Fields

- `skill_input_schema`: must be `use_case_writer.v1`.
- `mode`: one of `write_prd_use_cases`, `write_issue_use_cases`,
  `write_demo_use_cases`, or `write_milestone_use_cases`.
- `source`: declared source brief, issue, demo plan, or milestone plan.
- `audience`: target reader for the packet.
- `policy`: source grounding and stop-boundary policy.

## Optional Fields

- `artifact_root`: use-case packet destination.
- `actors`
- `acceptance_hooks`
- `non_goals`
- `unsupported_assumptions`

## Policy Fields

- `source_required`
- `unsupported_assumptions_policy`
- `implementation_status_claims_allowed`
- `issue_creation_allowed`
- `write_use_case_artifact`
- `stop_before_commitment`

## Example

```yaml
skill_input_schema: use_case_writer.v1
mode: write_demo_use_cases
source:
  demo_plan_path: docs/milestones/v0.90/features/STOCK_LEAGUE_DEMO.md
audience: milestone reviewer
policy:
  source_required: true
  unsupported_assumptions_policy: record_explicitly
  implementation_status_claims_allowed: false
  issue_creation_allowed: false
  write_use_case_artifact: true
  stop_before_commitment: true
```

## Output Contract

Default artifact root:

```text
.adl/reviews/use-case-writer/<run_id>/
```

Required artifacts:

- `use_case_packet.md`
- `use_case_packet.json`

Statuses:

- `ready`: source-grounded actors, use cases, and acceptance hooks are present.
- `partial`: packet is useful but needs human review for missing evidence.
- `not_run`: declared source missing.
- `blocked`: request violates product-commitment, implementation-claim,
  issue-creation, PR-creation, publication, or mutation boundary.

## Stop Boundary

The skill must not create issues, create PRs, publish externally, claim
implementation completion, replace implementation planning, or promote
unsupported assumptions into requirements.
