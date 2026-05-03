# Moral Resources

## Milestone Boundary

This v0.91 feature makes moral resources a tracked subsystem rather than a
scattered philosophical note. It sits downstream of moral trace and anti-harm
foundations and upstream of later identity, birthday, and constitutional
citizenship work.

The feature is about durable design resources such as care, refusal,
anti-dehumanization, and moral attention. It is not a claim that morality is
fully solved or that one scalar score can summarize moral worth.

## Purpose

Moral resources are the internal structures that let an agent:

- evaluate actions beyond instrumental success
- resist harmful or dehumanizing directives
- maintain continuity of moral identity
- learn from consequences over time
- treat other agents as morally real entities

Within ADL, these are engineering surfaces that must become inspectable,
trace-linked, and reviewable.

## Core Thesis

If the system only optimizes for task completion, speed, or local success, it
will not reliably preserve care, dignity, refusal, or other-recognition under
pressure. Moral resources must therefore be represented as durable resources in
the architecture rather than expected to appear accidentally.

## Key Resource Families

The initial v0.91 moral resources surface should cover:

- care
- refusal
- anti-dehumanization
- moral attention
- consequence memory
- other-recognition

These should remain compatible with trace, review, and constitutional
constraint.

## Architectural Placement

Moral resources should interact with:

- moral event and trace records
- Freedom Gate and refusal surfaces
- affect-like salience or weighting
- memory of consequences and lessons
- identity continuity and commitments

The goal is a durable moral-cognition band, not a one-off policy hook.

## Example Record

```yaml
moral_resources_record:
  care: unknown
  refusal_capacity: unknown
  anti_dehumanization: unknown
  moral_attention: unknown
  consequence_memory: unknown
  other_recognition: unknown
  evidence_links:
    - moral_trace_ref
    - review_packet_ref
  outcome: sufficient | strained | degraded | unclear
```

## Design Commitments

The first implementation should preserve these commitments:

- moral evaluation is not bypassable
- moral state persists across time and task boundaries
- the system can evaluate and revise its own moral conclusions
- affected agents are treated as morally real, not merely as graph objects
- moral consequence memory remains reviewable

## Implementation Placement

v0.91 should land:

- a moral-resources contract or record
- fixtures showing care, refusal, anti-dehumanization, and moral attention
- trace linkage to decisions and outcomes
- review criteria for degradation or strain under pressure

## Evidence Expectations

The proof surface should show that moral resources remain visible when the
system is under conflict, pressure, or convenience temptations, rather than
appearing only in ideal cases.

## Non-Claims

This feature does not claim a final moral psychology, consciousness, or
objective moral scoring system. It claims a narrower result:

ADL should have an explicit and reviewable substrate for care, refusal,
anti-dehumanization, and moral attention as durable engineering resources.
