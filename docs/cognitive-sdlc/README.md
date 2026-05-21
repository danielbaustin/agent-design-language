# Cognitive SDLC

## Status

Tracked canonical documentation home for the Cognitive SDLC planning and
workflow-state decisions that support `v0.91.3` and `v0.91.4`.

These docs promote the C-SDLC material that previously lived only in local
`.adl/docs/TBD/cognitive-sdlc/` notes into public, inspectable repository
truth.

## Purpose

The Cognitive SDLC models software development as governed cognitive state
transition rather than as only a pull-request exchange.

Git, GitHub issues, branches, worktrees, pull requests, CI, protected branches,
and human review remain the transport and enforcement substrate. C-SDLC adds
structured intent, planning, review, evidence, trace, closeout, and memory
around that substrate.

The goal is not faster unchecked code generation. The goal is higher software
throughput while preserving reviewability, auditability, replay, and
governance.

## Canonical Lifecycle

Issue-local C-SDLC work uses this lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

| Card | Meaning | Question |
| --- | --- | --- |
| `SIP` | Structured Issue Prompt | What issue, scope, context, and acceptance boundary are we addressing? |
| `STP` | Structured Task Prompt | What selected task or solution will resolve it? |
| `SPP` | Structured Plan Prompt | How will the selected solution be executed? |
| `SRP` | Structured Review Prompt | What review applies, what did review find, and how were findings handled? |
| `SOR` | Structured Outcome Record | What changed, what was validated, and what is now true? |

`SPP` is issue-local operative plan truth. It must not become sprint
orchestration, review-result truth, or output truth.

## Durable Workflow Truth

C-SDLC requires durable, replayable workflow state when records are needed for
governance, review, closeout, release evidence, signed trace proof, or ObsMem
ingestion. The general theory does not require a specific storage backend.

ADL's current implementation uses tracked Git state as the clearest substrate
for observable workflow transitions because it is public to the repo,
versioned, reviewable, and tied to the code and docs it governs.

Local `.adl/` state may remain for execution cache, staging, and machine-local
helper files. It is not sufficient as the only authoritative home for durable
C-SDLC truth.

The target namespace for durable workflow records is:

```text
workflow/c-sdlc/<version>/
```

`v0.91.3` proves the namespace shape for the first slice.
`v0.91.4` makes it the default durable workflow-record home.

## Document Map

- [Architecture](architecture.md)
- [Card Lifecycle](card-lifecycle.md)
- [Transition Schema](transition-schema.md)
- [Metrics](metrics.md)
- [Five-Minute Sprint Demo](five-minute-sprint-demo.md)
- [Tracked Workflow State](tracked-workflow-state.md)
- [Source Map And Supersession](source-map.md)

## Milestone Relationship

- `v0.91.3` proves one bounded C-SDLC transition slice.
- `v0.91.4` hardens C-SDLC into ADL's default software-development path.

These docs are source material for both milestones. Milestone-specific plans
remain under `docs/milestones/v0.91.3/` and `docs/milestones/v0.91.4/`.

## Non-Claims

This tracked docs home does not claim:

- C-SDLC is already implemented as default operation.
- Signed trace verification is complete.
- ObsMem ingestion is fully wired.
- Sprint-scoped `SPP` is part of the live contract.
