# ADL Card Lifecycle

## Purpose

This document defines the canonical ADL issue-card lifecycle.

The supported issue lifecycle is:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

This lifecycle is the semantic order of issue work. It is not necessarily the
physical file-creation order. Tooling may create placeholder files early for
path stability, but only one lifecycle stage is authoritative at a time.

## Canonical Cards

| Card | Meaning | Primary question |
| --- | --- | --- |
| `SIP` | Structured Issue Prompt | What problem, context, scope, and acceptance boundary are we addressing? |
| `STP` | Structured Task Prompt | What task or solution are we choosing? |
| `SPP` | Structured Plan Prompt | How will the selected task be executed? |
| `SRP` | Structured Review Prompt | What review instructions apply, what did review find, and how were findings dispositioned? |
| `SOR` | Structured Outcome Record | What changed, what was validated, and what is now true? |

All new issue workflow work should follow this model only. Legacy-compatible
exceptions must be explicit, temporary, and detectable by tooling.

## Creation Order Versus Activation Order

ADL tooling may create multiple card files during issue bootstrap.

That does not make every card active.

The lifecycle distinguishes:

- `file exists`: the card path is present, possibly as a scaffold.
- `stage active`: the card is the authoritative surface for the current
  lifecycle phase.
- `stage complete`: the card contains enough reviewed truth for the next phase.

For example, an `sor.md` scaffold may exist before implementation begins, but
it is not final outcome truth until execution, review, PR publication, merge or
closure, and closeout have updated it.

## Stage Semantics

### SIP: Structured Issue Prompt

The `SIP` is the issue-intent surface.

It records:

- the problem statement
- context and evidence
- in-scope and out-of-scope boundaries
- required outcome
- acceptance criteria
- dependency and issue-graph truth

Example target-state summary:

```text
This issue corrects card-lifecycle drift so new issues follow
SIP -> STP -> SPP -> SRP -> SOR.
```

### STP: Structured Task Prompt

The `STP` is the selected task or solution surface.

It records:

- chosen solution approach
- changed surfaces
- invariants and non-goals
- expected post-change behavior
- proof or demo shape
- rationale for the chosen path over alternatives

Example target-state summary:

```text
Implement a tracked card-lifecycle semantics doc and point existing workflow
docs at it without changing validators in this issue.
```

### SPP: Structured Plan Prompt

The `SPP` is the execution-plan surface.

It records:

- execution sequence
- dependencies
- validation plan
- review handoff
- stop conditions
- risks and fallback path
- worktree, branch, and scope constraints

For issue scope, `SPP` remains issue-local. Sprint-scoped planning is a
separate workflow extension and must use an explicit scope or separate editor
path before becoming mandatory.

Example target-state summary:

```text
Read the source issue, update bounded docs, run focused markdown/path checks,
obtain pre-PR review, then publish through pr finish.
```

### SRP: Structured Review Prompt

The `SRP` is the review-instruction and review-result surface.

`SRP` means Structured Review Prompt.

It records both:

- pre-review instructions and review policy
- post-review findings, dispositions, residual risks, and recommended outcome

This avoids creating another document type for review results. Review is a
cognitive activity, so the artifact remains a prompt, but it must also retain
the result of that cognition.

The final `SRP` should be suitable for `ObsMem` ingestion because findings,
dispositions, and residual risks are durable engineering memory.

Example target-state summary:

```text
Review the changed lifecycle docs for semantic drift. Finding P1 fixed before
publication; no remaining blockers; residual risk recorded.
```

### SOR: Structured Outcome Record

The `SOR` is the final outcome-truth surface.

It records:

- actual changed paths
- validation actually run
- review actually performed
- PR and merge state
- closeout state
- unresolved follow-ups
- final issue truth

The `SOR` should summarize and link to `SIP`, `STP`, `SPP`, and `SRP`; it
should not absorb their full planning or review burden.

The final `SOR` should feed `ObsMem` alongside the final `SRP`.

Example target-state summary:

```text
Changed docs/tooling/card-lifecycle.md, docs/GLOSSARY.md, and related workflow
references; markdown/path checks passed; PR opened; closeout pending merge.
```

## Lifecycle Gates

The normal gate order is:

1. `SIP` complete enough to define the issue.
2. `STP` complete enough to define the selected task or solution.
3. `SPP` complete enough to guide execution.
4. `SRP` complete enough to guide review before PR publication, then updated
   with findings and dispositions.
5. `SOR` complete enough to record final issue truth after execution,
   publication, merge or closure, and closeout.

Validators and doctor output should report both file existence and stage
readiness. Existing file presence is not enough.

`pr doctor` now exposes a bounded lifecycle-readiness summary in both text and
JSON output:

- `CARD_LIFECYCLE_ACTIVE_STAGE` / `card_lifecycle.active_stage`
- `CARD_LIFECYCLE_NEXT_REQUIRED_STAGE` /
  `card_lifecycle.next_required_stage`
- `CARD_LIFECYCLE_PR_RUN_READINESS` /
  `card_lifecycle.pr_run_readiness`
- `CARD_LIFECYCLE_PR_FINISH_READINESS` /
  `card_lifecycle.pr_finish_readiness`
- one per-card stage row/object for `SIP`, `STP`, `SPP`, `SRP`, and `SOR`

The stage classifier distinguishes `scaffold`, `active`, `complete`, `final`,
and `legacy_compatible` states. In particular, legacy
`Structured Review Policy` SRP scaffolds remain compatibility-valid, but they
are not final `Structured Review Prompt` truth until review results or an
explicit policy exception are recorded.

## Memory Handoff

`SRP` and `SOR` are distinct memory surfaces:

- `SRP` preserves review cognition: instructions, findings, dispositions,
  residual risks, and recommended outcome.
- `SOR` preserves outcome truth: changed artifacts, validation, integration,
  PR state, closeout, and follow-ups.

Both should be eligible for `ObsMem` ingestion when the issue is complete.

Together they let future ADL agents learn from intent, plan, review, and
outcome instead of reconstructing truth from raw PR comments or chat history.

See `srp-sor-obsmem-handoff-v0.91.2.md` for the bounded `v0.91.2`
finish, closeout, review-skill, and future `ObsMem` handoff model. That model
keeps `SRP` review cognition and `SOR` outcome truth linked but distinct.

## Non-Goals

This document does not implement validators, templates, editor-skill behavior,
or conductor routing by itself.

Those changes must land through the bounded follow-on issues in the
`v0.91.2` Cognitive SDLC card-lifecycle migration mini-sprint.
