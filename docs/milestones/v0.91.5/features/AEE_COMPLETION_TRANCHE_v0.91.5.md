# AEE Completion Tranche v0.91.5

## Metadata

- Feature Name: `AEE Completion Tranche`
- Milestone Target: `v0.91.5`
- Status: `planned`
- Owner: ADL maintainers
- Doc Role: `primary`
- Supporting Docs: `docs/explainers/AEE.md`, `docs/milestones/v0.8/ADAPTIVE_EXECUTION_ENGINE.md`, `docs/milestones/v0.8/BOUNDED_AEE_V1_SCOPE_V0.8.md`, `docs/milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md`
- Feature Types: `runtime`, `artifact`, `policy`, `architecture`
- Proof Modes: `tests`, `schema`, `replay`, `review`, `demo`

## Template Rules

This is a planning feature doc. It defines the completion tranche and routing
requirements for AEE; it does not claim AEE subsystem completion by itself.

## Purpose

Make Adaptive Execution Engine completion explicit and schedulable before
`v0.95` instead of leaving it diffused across broad MVP convergence.

## Context

- Related milestone: `v0.91.5`
- Related issues: `#3526`, `#3528`, `#3534`
- Dependencies: v0.91.5 bridge planning, v0.92 activation readiness, existing
  AEE v1 and convergence docs

AEE has a real baseline: bounded retry, policy hooks, convergence docs, and
runtime-adjacent proof surfaces already exist. The gap is not "nothing exists."
The gap is that subsystem completion is not yet named as its own closure lane
with done criteria, proof, and milestone routing.

## Coverage / Ownership

This document owns the bridge-level definition of what AEE completion means and
where the remaining closure work must land.

- Primary owner doc: `docs/milestones/v0.91.5/features/AEE_COMPLETION_TRANCHE_v0.91.5.md`
- Covered surfaces:
  - AEE closure criteria
  - milestone routing for AEE subsystem proof
  - v0.92 activation-test handoff
  - non-goals that keep AEE bounded
- Related / supporting docs:
  - `docs/explainers/AEE.md`
  - `docs/milestones/v0.8/ADAPTIVE_EXECUTION_ENGINE.md`
  - `docs/milestones/v0.8/BOUNDED_AEE_V1_SCOPE_V0.8.md`
  - `docs/milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md`
  - `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
  - `docs/milestones/v0.92/WBS_v0.92.md`

## Overview

AEE completion means ADL can show a bounded execution-adaptation subsystem with
reviewable state transitions, explicit policy stops, deterministic handoff
semantics, and proof that adaptation is governed rather than improvised.

Key capabilities:
- steering semantics that are explicit and replayable
- queue, wake, and handoff semantics that do not depend on chat memory
- distributed execution boundaries that preserve authority and trace truth
- control-path evidence that reviewers can inspect
- policy, budget, and safety stops that fail closed
- proof surfaces for end-to-end AEE behavior

## Design

### Core Concepts

- `AEE baseline`: existing bounded retry, runtime control, and convergence
  surfaces from earlier milestones.
- `AEE completion tranche`: the named remaining work required to call AEE a
  subsystem rather than a scattered family of runtime features.
- `AEE proof surface`: a reviewable packet that shows steering, queue/wake,
  handoff, policy stops, trace/replay, and demo behavior.
- `AEE non-claim`: explicit limits that prevent AEE from being framed as
  unbounded autonomy, hidden self-modification, or retry-until-plausible output.

### Architecture

- Inputs (explicit sources / triggers):
  - existing AEE baseline docs and runtime control surfaces
  - `#3526` / PR `#3528` feature-completion audit
  - `#3534` AEE completion tranche planning issue
  - v0.91.5 multi-agent, provider/model, and activation-readiness evidence
- Outputs (artifacts / side effects):
  - AEE closure criteria
  - owner issue list or issue-candidate list for remaining work
  - v0.92 activation-test requirements
  - proof/demo expectations
- Interfaces (APIs, CLI, files, schemas):
  - issue-wave and WBS planning docs
  - activation-test map
  - future AEE proof packet, trace/replay packet, and demo packet
- Invariants (must always hold):
  - AEE must remain policy-bounded.
  - AEE must not hide state transitions.
  - AEE must not claim completion without proof.
  - AEE must not be deferred implicitly to `v0.95` MVP polish.

### Data / Artifacts

- AEE completion criteria table in this document.
- v0.91.5 issue-wave routing for `#3534`.
- v0.92 activation-test row for AEE.
- Future v0.92 AEE proof packet if implementation/proof is routed there.

## Execution Flow

1. v0.91.5 records the AEE completion tranche and owner routing through
   `#3534`.
2. v0.91.5 updates the activation map so v0.92 cannot open without seeing the
   AEE closure requirements.
3. v0.92 WP-01 consumes v0.91.5 closeout and decides final issue seeding for
   AEE subsystem implementation/proof.
4. v0.92 executes the required AEE proof or explicitly blocks release until the
   closure lane is complete.
5. v0.95 keeps only broad convergence, polish, dashboard/demo catalog, and MVP
   hardening work; it is not the first place AEE completion is discovered.

## Determinism and Constraints

- Determinism guarantees:
  - AEE state transitions must be traceable through explicit artifacts.
  - Queue, wake, handoff, retry, and stop decisions must be reviewable.
  - Proof commands and demo artifacts must not depend on private chat context.
- Constraints:
  - No unbounded autonomy claim.
  - No hidden retry loop.
  - No policy bypass through model confidence or tool availability.
  - No deletion or cleanup of historical evidence without review.

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| Runtime / AEE | observe | Existing baseline and convergence surfaces supply source evidence. |
| C-SDLC control plane | read/write | Issue wave, cards, and SOR truth route AEE completion work. |
| Multi-agent workcell | observe | Multi-agent proof supplies queue, handoff, and distributed-boundary pressure. |
| Provider/model matrix | observe | Provider breadth informs AEE routing, latency, and capability constraints. |
| Trace / replay | write | Future AEE proof must show deterministic trace/replay evidence. |
| v0.92 birthday readiness | read/write | AEE activation requirements feed `#3377` and v0.92 WP-01. |

## Validation

### Demo

- Demo script(s): future AEE proof demo or bounded multi-agent/AEE bridge demo.
- Expected behavior: show a governed adaptation path with explicit steering,
  queue/wake/handoff, policy stop, and trace/replay evidence.

### Deterministic / Replay

- Replay requirements: proof packet must replay or inspect AEE decisions without
  relying on transient chat state.
- Determinism guarantees: repeated validation over the same artifacts must
  produce the same pass/fail classification.

### Schema / Artifact Validation

- Schemas involved: AEE proof packet or existing trace/control-path schemas, as
  selected by the implementation issue.
- Artifact checks: no host-path leakage, no hidden state, no missing owner issue,
  no unclassified deferred AEE component.

### Tests

- Test surfaces: focused runtime/control-path tests for steering, queue/wake,
  handoff, policy stop, trace/replay, and distributed-boundary behavior.

### Review / Proof Surface

- Review method: internal review plus release-tail quality gate.
- Evidence location: v0.91.5 closeout, `#3534` output, v0.92 WP-01 issue wave,
  and future AEE proof packet.

## Acceptance Criteria

- Functional correctness: AEE closure criteria are explicit and routed to
  v0.91.5/v0.92 work, not hidden in `v0.95`.
- Determinism / replay correctness: closure requires inspectable artifacts for
  steering, queue/wake/handoff, policy stops, and trace/replay.
- Validation completeness: each AEE closure component has a proof expectation,
  owner routing, and non-goal boundary.

## AEE Completion Criteria

| Closure component | Done means | Soonest routing | Proof expectation |
| --- | --- | --- | --- |
| Steering semantics | Runtime can describe why adaptation selected, retried, deferred, or stopped. | `v0.92` implementation/proof, seeded by `#3534` and `#3377`. | Focused control-path tests plus reviewable decision artifacts. |
| Queue / wake / handoff semantics | Work can pause, resume, wake, and hand off without chat-only state. | `v0.92` implementation/proof; multi-agent evidence from v0.91.5 informs boundaries. | Trace/replay or fixture packet showing deterministic state transitions. |
| Distributed execution boundary | AEE can interact with delegated/remote work without losing authority or evidence truth. | `v0.91.5` defines constraints; `v0.92` proves required subset if birthday depends on it. | Provider/model and multi-agent proof packet with explicit non-claims. |
| Control-path truth | Reviewers can inspect the control decisions and their inputs. | `v0.92`. | Control-path artifact, validation command, and review packet. |
| Policy and budget stops | AEE fails closed when policy, budget, safety, or authority constraints block continuation. | `v0.92`. | Negative tests or fixtures proving blocked/deferred/refused states. |
| Trace / replay proof | AEE decisions are replayable or inspectable from durable artifacts. | `v0.92`; release-tail validation before `v0.92` closeout. | Replay/inspection command and no host-path/private-state leakage. |
| End-to-end proof/demo | A bounded scenario shows AEE behavior without overclaiming autonomy. | `v0.92`; demo polish can continue through `v0.95`. | Runnable demo or proof packet classified as proving/non-proving/skipped. |

## Risks

- Primary risks:
  - AEE remains scattered across runtime, multi-agent, and demo work.
  - AEE proof gets postponed until broad MVP convergence.
  - Distributed execution or provider variability hides control-path truth.
- Mitigations:
  - Keep `#3534` as the named AEE closure-planning issue.
  - Add AEE to v0.91.5 proof coverage and v0.92 activation inputs.
  - Require explicit blocker/routing if any closure component cannot land in
    `v0.92`.

## Future Work

- v0.92 should create concrete AEE implementation/proof issues from this
  tranche during WP-01 if they are not already opened.
- v0.95 should consume AEE as a completed subsystem for MVP convergence, not
  discover the subsystem definition for the first time.

## Notes

This document honors the existing AEE baseline while correcting the roadmap
truth: AEE completion is a first-class subsystem closure lane, not a side effect
of the whole MVP eventually feeling coherent.
