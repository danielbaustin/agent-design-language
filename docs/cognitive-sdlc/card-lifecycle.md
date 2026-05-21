# C-SDLC Card Lifecycle

## Status

Tracked C-SDLC card-role summary.

The detailed tooling contract remains in
[`docs/tooling/card-lifecycle.md`](../tooling/card-lifecycle.md) and
[`docs/tooling/structured-prompt-contracts.md`](../tooling/structured-prompt-contracts.md).

## Lifecycle

```text
SIP -> STP -> SPP -> SRP -> SOR
```

This is the issue-local truth order. Tooling may create stubs early, but each
card becomes authoritative only when its lifecycle role is truthfully populated.

## Card Roles

### SIP: Structured Issue Prompt

`SIP` records the issue intent, scope, dependencies, acceptance boundary, and
source context.

It answers: what problem are we solving?

### STP: Structured Task Prompt

`STP` records the selected task or solution approach.

It answers: what are we going to do?

### SPP: Structured Plan Prompt

`SPP` records the issue-local operative execution plan.

The first `SPP` should be generated during issue/card bootstrap as a
design-time plan. It should be concrete enough for review before execution:
steps, proof gates, dependency assumptions, stop conditions, out-of-scope
boundaries, and validation strategy.

Runtime execution must treat that plan as live truth rather than a historical
memo. If execution diverges, update `SPP` first.

It answers:

- what step is active now
- what step comes next
- what proof is required before proceeding
- what condition forces a replan
- what is explicitly out of bounds

`SPP` is not sprint orchestration, review-result truth, or output truth.

### SRP: Structured Review Prompt

`SRP` records review instructions, review results, findings, dispositions, and
residual risk.

It answers: how should the result be reviewed, and what did review find?

`SRP` must feed ObsMem as review-learning evidence when the issue closes.

### SOR: Structured Outcome Record

`SOR` records actual changes, validation, integration state, closeout state,
and final issue truth.

It answers: what happened and what is now true?

`SOR` must not pretend work is merged, validated, or closed when GitHub and repo
evidence do not support that claim.

## Review And Closeout

C-SDLC depends on the cards staying separate:

- `SPP` guides execution.
- `SRP` preserves review learning.
- `SOR` preserves outcome truth.

Collapsing those roles recreates the drift C-SDLC is designed to remove.
