# Next Milestone Handoff - v0.90.5

## Purpose

Record the clean next-milestone handoff for `v0.90.5` after Governed Tools
v1.0, the first landed ACIP tranche, and the review tail all completed without
creating a fake leftover backlog.

`v0.90.5` was difficult operationally, but the handoff should preserve what
actually landed and route the remaining planned work into the right future
milestones.

## What v0.90.5 Actually Landed

`v0.90.5` does not hand off an empty thesis. It landed:

- Universal Tool Schema (`UTS`) as a governed portable tool-description layer
- ADL Capability Contracts (`ACC`) as the execution-authority layer
- deterministic `UTS` to `ACC` compilation
- policy injection and Freedom Gate mediation
- governed execution and refusal behavior
- trace, replay, redaction, and review evidence surfaces
- dangerous negative proofs that fail closed
- bounded local/Gemma proposal evaluation
- the Governed Tools v1.0 flagship proof packet
- the first landed Agent Communication and Invocation Protocol tranche

## Zero-Leftover Result

- There is no vague `v0.90.5` “tool debt” bucket.
- There is no fake backlog item claiming Governed Tools still needs to be
  invented from scratch.
- There is no hidden carry-forward issue wave being parked as “we will figure
  it out later.”

What remains is deliberately planned later-band work, not unresolved milestone
truth.

## v0.91 Placement

`v0.91` is the immediate follow-on milestone.

It should fully implement the core moral-governance, wellbeing, cognitive-being,
and secure intra-polis communication substrate:

- moral event, validation, trace, attribution, metrics, and trajectory review
- anti-harm trajectory constraints
- moral resources
- wellbeing diagnostics
- kindness, humor/absurdity, affect reasoning-control, and cultivated
  intelligence
- secure local Agent Communication and Invocation Protocol substrate work
- durable structured planning (`SPP`) and plan-review workflow
- durable Structured Review Policy (`SRP`) and issue-local review-policy
  discipline

`v0.91` should consume `v0.90.5` governed-tools evidence where later moral
review, refusal, trace, or communication work needs it. It should not reopen
the governed-tools implementation thesis itself.

## v0.91.1 Placement

`v0.91.1` is the adjacent-systems completion lane.

It should carry the larger follow-on systems that are already planned but do
not belong inside the `v0.91` core milestone:

- capability and aptitude testing
- intelligence metric architecture
- ANRM / Gemma and broader local-model evidence architecture
- Theory of Mind foundation work
- memory and identity alignment for `v0.92`
- runtime-v2 and polis documentation / architecture alignment
- ACIP conformance, redaction, replay, encryption hardening, and additional
  specializations
- the full Gemma/local/remote comparison report that `v0.90.5` deliberately
  did not absorb
- A2A external-agent adapter implementation and hardening over the comms
  substrate

## A2A Placement

The A2A adapter is not a separate giant milestone.

It should be treated as one governed external-agent adapter layered on top of
the broader Agent Communication and Invocation Protocol work:

- `v0.91`: promote and anchor the feature in tracked planning docs, identity
  mapping, capability translation, and invocation-boundary rules
- `v0.91.1`: implement the bounded runnable adapter slice and hardening

This keeps A2A from inventing a parallel communication model while still making
external-agent interoperability a real planned feature.

## Workflow Upgrades That Need To Land Early

The next milestone should also start with workflow improvements that reduce the
kind of process cost `v0.90.5` suffered:

- durable structured planning artifact (`SPP`)
- plan review before tracked execution where policy requires it
- planning skill support (`pr-plan`)
- durable structured review policy (`SRP`)
- review-readiness checks that understand `SRP`

These are not decoration. They are intended to make implementation, delegation,
and review more reliable in the next milestone.

## v0.92 Placement

`v0.92` remains the identity, continuity, and first true birthday milestone.

It should consume landed `v0.91` evidence, not absorb unfinished `v0.90.5`
governed-tools work or `v0.91.1` adjacent-system spillover prematurely.

## v0.93 Placement

`v0.93` remains the constitutional citizenship, social-cognition, reputation
boundary, and polis-governance milestone.

That work is downstream of the moral and communication evidence built in
`v0.91` and `v0.91.1`; it is not a leftover repair lane for `v0.90.5`.

## Non-Reduction Rule

This handoff does not reduce any downstream milestone:

- `v0.91` remains a full implementation milestone for the core moral and
  cognitive-being substrate
- `v0.91.1` remains the adjacent-systems completion lane
- `v0.92` remains identity, continuity, and birthday
- `v0.93` remains constitutional citizenship and polis governance

The handoff is meant to sharpen those boundaries, not blur them.

## Future WP-25 Rule

Future next-milestone handoffs should:

- promote all already-accepted feature docs into tracked milestone surfaces
  before ceremony
- separate “actually landed” from “bounded later-band completion”
- avoid carrying key planning surfaces only in TBD or side worktrees when the
  next milestone is about to start
