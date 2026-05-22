# v0.91.3 Vision

## Metadata

- Project: `Agent Design Language`
- Milestone: `v0.91.3`
- Version: `v0.91.3`
- Date: `2026-05-22`
- Owner: ADL maintainers
- Related issues: `#3099`, `#3199` through `#3214`, `#3225` through `#3231`

## Purpose

Define the milestone-level vision for the first C-SDLC implementation slice.
This document explains what should be true by the end of `v0.91.3` without
claiming the full C-SDLC rollout is complete.

## Overview

`v0.91.3` is the milestone where ADL moves from corrected issue-card semantics
to one bounded, evidence-backed Cognitive State Transition.

This release strengthens:

- public, tracked C-SDLC planning truth
- issue-local operative planning through `SPP`
- transition identity, evidence, review, and outcome records
- governed merge-readiness without bypassing GitHub, CI, or human review
- an ObsMem handoff boundary for review results and outcome truth

The milestone is for a first proof, not default adoption. `v0.91.4` owns
repeatability, enforcement, migration policy, and default operation.

## Core Goals

`v0.91.3` advances ADL in five areas:

1. Cognitive Transition identity.
2. Public tracked issue-card lifecycle.
3. Transition DAG and shard coordination.
4. Evidence, review synthesis, merge readiness, and memory handoff.
5. First five-minute-sprint proof surface.

## Cognitive Transition Identity

The milestone should make one transition concrete enough to inspect:

- a manifest names the issue, participants, states, cards, evidence, gate, and
  memory handoff
- actor roles are visible without overclaiming full Software Development Polis
  enforcement
- transition state changes are tied to tracked artifacts
- skipped or blocked states remain truthful execution outcomes

## Public Tracked Lifecycle

The corrected card lifecycle must be preserved as the operating grammar:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

For this milestone, `SPP` is the public, issue-local operative execution plan.
It records the current step, next step, required proof, replan/stop conditions,
and explicit bounds. It is not sprint orchestration, review truth, or output
truth.

## Transition DAG And Shard Coordination

The first slice should show how a transition can describe:

- serial work
- bounded parallel shards
- shard ownership
- interface-freeze points
- synchronization barriers
- review and merge gates

The purpose is not to maximize parallelism. The purpose is to make coordination
auditable and safe.

## Evidence, Review, Merge, And Memory

The transition must produce a proof surface that reviewers can inspect:

- evidence bundle
- SRP review results
- SOR outcome truth
- merge-readiness gate preserving issue, PR, branch, CI, and human review truth
- ObsMem handoff boundary

The review surface should make uncertainty, residual risk, and skipped evidence
visible rather than compressing them away.

## First Proof

The milestone should end with one bounded first proof of the C-SDLC shape. The
proof can be narrow, but it must be real enough to guide `v0.91.4`.

Success means the team can say:

- the first Cognitive State Transition is represented
- the evidence is tracked and reviewable
- the process lessons are known
- the hardening backlog for `v0.91.4` is concrete

## Milestone Context

`v0.91.2` repaired process drift and prepared the C-SDLC runway. `v0.91.3`
proves one transition. `v0.91.4` should turn that proof into the default
development path for future ADL software work.

## Long-Term Direction

ADL is moving toward a software-development process that is explicit,
inspectable, bounded, humane, and governed. The C-SDLC is one of the core
mechanisms for making AI-assisted software work auditable instead of mystical.
