# Cognitive SDLC Architecture

## Status

Tracked architecture summary for the C-SDLC planning canon.

## Core Claim

The future ADL software-development lifecycle is centered on governed cognitive
state transitions.

Pull requests remain important, but they are too small to be the only semantic
unit for AI-native software work. A C-SDLC transition wraps ordinary GitHub
workflow with structured issue intent, selected task, operative plan, review,
evidence, outcome truth, trace, and memory handoff.

## Implementation Boundary

The general C-SDLC model is portable, but ADL is building it first on top of
Git, GitHub issues, worktrees, pull requests, structured prompts, and review
packets. These surfaces already expose the state transitions ADL needs to
govern.

A different implementation can use different storage or coordination mechanics
if it preserves the same properties:

- durable state
- bounded role-specific work
- explicit transition instructions
- typed artifacts
- review gates
- mergeability
- truthful closeout

## Substrate Boundary

C-SDLC amplifies the existing repository workflow. It does not replace it.

| Layer | Role |
| --- | --- |
| Git repository | Canonical tracked source and durable docs. |
| GitHub issue | External issue intent and coordination surface. |
| Branch/worktree | Bounded execution context. |
| Pull request | Review, CI, publication, and merge transport. |
| C-SDLC records | Structured lifecycle, evidence, trace, and memory truth. |
| Human review | Authority boundary for merge and governance judgment. |

In ADL's implementation, Git is a state substrate rather than the whole
lifecycle. Structured prompts are transition instructions over that substrate,
and typed work packets turn hidden coordination into reviewable artifacts.

## Software Development Polis

C-SDLC treats software development as a polis: a structured society of human
and AI actors with scoped standing, explicit responsibility, bounded authority,
and reviewable outcomes.

The polis model matters because parallel agents can generate abundant code.
The hard problem becomes coordination, trust, convergence, and governance.

## Safety Principles

- Preserve GitHub issue, PR, CI, branch, and closeout truth.
- Make planning, review, and outcome records inspectable.
- Keep authority boundaries explicit.
- Prefer bounded shards over unbounded autonomous work.
- Treat evidence as part of the product, not as after-the-fact decoration.
- Never trade governance integrity for speed.

## C-SDLC Transition

A Cognitive State Transition is the semantic and governance unit for one
bounded change.

It should contain:

- transition identity
- issue and branch references
- card lifecycle records
- transition DAG or shard plan
- evidence bundle
- review synthesis
- merge-readiness gate
- outcome record
- signed trace or trace-ready proof references
- ObsMem handoff boundary

## Relationship To CSM And Governed Cognition

C-SDLC applies ADL's governed-cognition ideas to software development. It
inherits the same basic posture:

- cognition is useful only when bounded and inspectable
- memory should be evidence-backed
- plans and outcomes should not collapse into one mutable blob
- governance must be part of the workflow, not an external apology
