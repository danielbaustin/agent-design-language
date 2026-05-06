# Structured Review Policy And SRP

## Milestone Boundary

This `v0.91` feature makes `SRP`, the Structured Review Prompt, a first-class
workflow artifact rather than a planning-only concept or a loose protocol
reference.

It sits between issue execution and independent review. It is closely related
to Agent Comms, reviewer-agent invocation, and the new structured planning
workflow, but it has a narrower role: durable review policy.

## Purpose

ADL already has:

- structured task intent (`STP`)
- structured execution context (`SIP`)
- structured output truth (`SOR`)
- reviewer-agent and ACIP specialization work that can carry `srp_ref`

The missing durable surface is the saved review-policy artifact itself.

The purpose of this feature is to:

- encode pre-PR review policy in the issue bundle
- make reviewer scope and allowed claims explicit
- improve review readiness and review consistency
- bind reviewer invocation to a durable policy artifact rather than only to
  session instructions

## Core Thesis

Reviewer invocation should not depend only on packets, chat prompts, or tool
arguments.

ADL should therefore introduce `SRP` as a durable review-policy artifact that
works beside:

- the review invocation contract
- the review evidence packet
- the reviewer result artifact

## Intended Artifact

The canonical review-policy artifact should be:

- `SRP`: Structured Review Prompt

Primary location:

- `.adl/<version>/tasks/issue-<n>__<slug>/srp.md`

Compatibility surface:

- `.adl/cards/<issue>/srp_<issue>.md`

## What The SRP Should Capture

The first `v0.91` implementation should make `SRP` capture:

- review mode and lifecycle timing
- review scope basis
- files and surfaces in scope
- evidence classes the reviewer may inspect
- what validations already exist
- allowed reviewer dispositions
- reviewer constraints and prohibited actions
- follow-up routing rules

## Architectural Placement

`SRP` should remain separate from:

- the runtime review invocation contract
- the review evidence packet
- the reviewer output record

Its job is policy, not transport, packet payload, or findings output.

## Implementation Placement

The landed `v0.91` implementation slice is:

- an `SRP` template
- bootstrap support so issue bundles can carry `srp.md`
- structured-prompt validation support for `SRP`
- compatibility-link support through `.adl/cards/<issue>/srp_<issue>.md`
- durable policy structure that can be referenced by reviewer invocation

Deferred follow-on work includes:

- review-readiness checks that block on missing or stale `SRP`
- end-to-end reviewer-agent tooling that consumes the policy automatically

## Relationship To Agent Comms

`SRP` is not the same thing as Agent Comms.

Agent Comms is the general substrate for messages and invocation.
`SRP` is the durable review-policy artifact that certain review-specialized
invocations should reference.

That means `SRP` should land early in `v0.91`, because it strengthens review
discipline and gives the communication substrate a clearer policy target.

## Evidence Expectations

The proof surface for the landed slice should show that:

- review policy is saved with the issue bundle
- malformed `SRP` surfaces are detectable
- reviewer invocation can reference `srp_ref`
- reviewers have explicit scope, evidence, refusal, and disposition policy to
  bind against even before full readiness gating is added

## Non-Claims

This feature does not claim that review becomes fully automatic or that an
`SRP` alone guarantees review quality.

It claims a narrower result:

ADL should have a durable, reviewable, issue-local review-policy artifact that
anchors independent reviewer behavior before PR publication.
