# Use Case Writing Playbook

## Source Grounding Checklist

- Name the source brief, issue, feature, demo, or milestone context.
- Extract actors from supplied text or mark them as assumptions.
- Separate user goals from system behavior.
- Convert acceptance criteria into acceptance hooks, not implementation plans.
- Preserve non-goals and constraints.
- Put uncertain product details into unsupported assumptions.

## Useful Packet Shapes

- PRD packet: product actors, goals, constraints, acceptance hooks, and open
  assumptions.
- Issue packet: one implementable scenario family tied to acceptance criteria.
- Demo packet: operator/viewer flow, proof surface, happy path, and failure path.
- Milestone packet: milestone-level actor flows and acceptance hooks without
  claiming implementation completion.

## Unsafe Signals

- no declared source
- invented personas or market claims
- implementation status claimed from a planning doc alone
- acceptance criteria created from unsupported assumptions
- issue or PR creation requested without explicit operator approval
- external publication or business claims requested without review
