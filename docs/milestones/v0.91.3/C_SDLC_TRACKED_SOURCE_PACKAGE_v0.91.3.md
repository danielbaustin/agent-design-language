# v0.91.3 C-SDLC Tracked Source Package

## Status

Tracked source package for the first Cognitive SDLC implementation slice.

This document promotes the durable planning truth that originally lived in
local `.adl/docs/TBD/cognitive-sdlc/` notes into the reviewable milestone
package for `v0.91.3`.

The source notes remain useful as drafting history, but they are not the
canonical public/auditable planning surface for the milestone. Reviewers should
use this tracked package, the v0.91.3 milestone docs, and the tracked tooling
docs as the branch-verifiable source set.

## Source Notes Promoted

The v0.91.3 first slice is grounded in these source topics:

| Source topic | Tracked v0.91.3 role |
| --- | --- |
| ADL Cognitive SDLC architecture | Defines C-SDLC as governed cognitive state transition, not a PR shortcut. |
| Card lifecycle migration plan | Establishes `SIP -> STP -> SPP -> SRP -> SOR` as the canonical issue flow. |
| Cognitive SDLC v1 plan | Bounds the implementation slice to transition schema, evidence, review, gate, and memory handoff. |
| Cognitive Transition schema | Defines transition identity, manifest, DAG/shard model, evidence bundle, merge gate, and memory boundary. |
| Five-minute sprint demo | Defines the flagship first proof while preserving review, replay, merge, and closeout discipline. |
| C-SDLC metrics | Defines coordination, review, replay, throughput, and governance metrics. |
| Issue card sequence note | Explains why each card has one job and feeds later review/recovery/memory. |
| Sprint and issue SPP note | Keeps issue-local `SPP` authoritative while treating sprint-scoped planning as future extension. |
| Process mini-sprint issue bodies | Provide migration lessons for templates, validators, editors, conductor routing, active-bundle readiness, and ObsMem handoff. |

## Canonical First-Slice Contract

`v0.91.3` proves one bounded Cognitive State Transition.

The transition must preserve:

- GitHub issue and PR truth
- branch/worktree truth
- CI and validation truth
- human review truth
- the canonical card lifecycle
- evidence and proof boundaries
- review-result and outcome-memory handoff

The lifecycle is:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

The cards mean:

| Card | Role |
| --- | --- |
| `SIP` | Structured Issue Prompt: issue intent, scope, acceptance boundary, and dependencies. |
| `STP` | Structured Task Prompt: selected task or solution. |
| `SPP` | Structured Plan Prompt: issue-local execution plan. |
| `SRP` | Structured Review Prompt: review instructions plus findings, dispositions, and residual risk. |
| `SOR` | Structured Outcome Record: actual changes, validation, integration state, and final issue truth. |

## Tracked Evidence Requirement

C-SDLC evidence must be public and inspectable in Git.

For `v0.91.3`, the first slice should produce or define tracked surfaces for:

- transition manifest
- transition DAG and shard plan
- evidence bundle
- review synthesis
- merge-readiness gate
- final SRP and SOR truth
- ObsMem handoff boundary
- timing and coordination metrics
- trace/proof references that can become signed trace bundles in `v0.91.4`

The first slice does not need to complete signed trace verification, but it must
not design itself into a corner where durable C-SDLC proof remains local-only or
unsigned forever.

## Relationship To Tooling Docs

The first slice must remain aligned with:

- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`
- `docs/tooling/srp-sor-obsmem-handoff-v0.91.2.md`
- `AGENTS.md`

If those docs and this source package disagree, v0.91.3 must route the mismatch
to review or remediation before claiming the first slice is complete.

## Non-Claims

This tracked source package does not claim:

- C-SDLC is already default operation
- sprint-scoped `SPP` is mandatory today
- signed trace verification is complete in v0.91.3
- trace query/TQL is required before first-slice proof
- any external collaboration workspace is required infrastructure

`v0.91.4` owns default operation, tracked durable workflow records, signed trace
proof, repeatability, and active-issue migration.
