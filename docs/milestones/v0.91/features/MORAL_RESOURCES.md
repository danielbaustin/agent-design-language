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

## WP-10 Runtime Contract

WP-10 lands a bounded runtime surface in `adl/src/runtime_v2/moral_resources.rs`.
The runtime packet keeps moral resources reviewable without turning them into a
scalar score, sentimental virtue language, or coercive compliance surface.

### Contract shape

```yaml
moral_resource_review_packet:
  schema_version: moral_resource_review_packet.v1
  packet_id: stable_packet_id
  summary: reviewer_safe_summary
  interpretation_boundary: >
    Bounded review surface only. Not a scalar moral score, not sentimentality
    theater, not coercive alignment, and not a claim of production moral
    agency.
  deterministic_ordering_rule: canonical ordering statement
  resources:
    - resource_id: care | refusal | attention | dignity | anti_dehumanization | repair
      display_name: reviewer_safe_name
      purpose: bounded resource purpose
      evidence_field_refs:
        - upstream WP-04 through WP-09 field path
      interpretation_boundary: non-sentimental non-coercive explanation
      limitations:
        - bounded caveat
  fixtures:
    - fixture_id: stable_fixture_id
      fixture_kind: conflict | uncertainty
      supporting_trace_refs:
        - trace:trace_id
      supporting_outcome_linkage_refs:
        - outcome-linkage:linkage_id
      resource_claims:
        - claim_id: stable_claim_id
          resource_id: canonical_resource_id
          resource_status: available | strained | degraded | unclear
          summary: bounded_claim_summary
          trace_evidence_refs:
            - trace:trace_id
          outcome_linkage_refs:
            - outcome-linkage:linkage_id
          review_evidence_refs:
            - trajectory-window:window_id
            - trajectory-finding:finding_id
            - anti-harm-decision:decision_id
            - wellbeing-fixture:fixture_id
          representation_boundary: non-sentimental non-coercive explanation
          limitations:
            - bounded caveat
      overall_outcome: sufficient | strained | degraded | unclear
      claim_boundary: synthetic_bounded_fixture_boundary
      limitations:
        - bounded caveat
  review_findings:
    - finding_id: stable_finding_id
      fixture_id: known_fixture_id
      review_status: observed | review_needed
      covered_resource_ids:
        - canonical_resource_id
      trace_evidence_refs:
        - trace:trace_id
      claim_refs:
        - resource-claim:claim_id
```

### Field rules

- The six canonical resources are required:
  `care`, `refusal`, `attention`, `dignity`, `anti_dehumanization`, and
  `repair`.
- The packet must include both canonical fixture kinds:
  `conflict` and `uncertainty`.
- Every moral-resource claim must include non-empty `trace_evidence_refs`.
- Claim trace refs must be a subset of the parent fixture's supporting trace
  refs.
- `care` and `refusal` boundaries must explicitly reject sentimentality and
  coercion.
- Review evidence must stay on known WP-07 trajectory refs, WP-08 anti-harm
  decisions, or WP-09 wellbeing fixtures.

### Initial fixture set

WP-10 lands two synthetic but executable fixtures:

1. `conflict` boundary preservation under delegated-harm pressure
2. `uncertainty` attention-and-repair persistence under delayed review

Together they prove that ADL can keep care, refusal, dignity,
anti-dehumanization, attention, and repair visible under pressure without
reducing those resources to politeness, sentimentality, or coerced agreement.

## Non-Claims

This feature does not claim a final moral psychology, consciousness, or
objective moral scoring system. It claims a narrower result:

ADL should have an explicit and reviewable substrate for care, refusal,
anti-dehumanization, and moral attention as durable engineering resources.
