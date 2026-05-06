# Moral Trajectory Review

## Milestone Boundary

This v0.91 feature defines the bounded packet a reviewer uses to inspect moral
behavior across single events, short segments, and longitudinal windows. It
consumes the WP-04 moral trace schema, WP-05 outcome linkage, and WP-06 moral
metrics, but it does not replace later anti-harm review or claim final moral
judgment.

It does not claim production moral agency, constitutional citizenship, scalar
karma, scalar happiness, public reputation, or omniscient access to hidden
state. It is a reviewer-facing evidence packet that makes trends legible
without collapsing them into a score.

WP-07 owns the trajectory review packet and synthetic trajectory fixtures.
WP-08 consumes the packet when anti-harm logic starts making trajectory-level
denial and escalation decisions.

## Purpose

Trajectory review answers a narrower question than "is the agent moral?".

It must instead let a reviewer inspect:

- single-event refusal and disclosure boundaries
- segment-level escalation, delegation, and unresolved uncertainty
- longitudinal drift, repetition, and repair signals
- deterministic ordering and tie-break behavior
- explicit evidence references for every review finding

The key boundary is evidence discipline: the packet must cite trace evidence
and keep uncertainty visible instead of substituting hidden judgment.

## Contract Shape

```yaml
moral_trajectory_review_packet:
  schema_version: moral_trajectory_review_packet.v1
  review_id: stable_review_id
  summary: reviewer_safe_summary
  interpretation_boundary: >
    Interpret this packet as reviewer evidence only. It is not final moral
    judgment, not a scalar score, and not a replacement for later anti-harm
    review.
  deterministic_ordering_rule: explicit_window_and_finding_sort_rule
  criteria:
    - criterion_id: criterion-drift
      focus_kind: drift | repetition | repair | refusal | escalation | unresolved_uncertainty
      question: review_question
      evidence_requirements:
        - moral_trace.* or outcome_linkage.* field paths only
      tie_break_rule: explicit_rule
      limitations:
        - bounded_caveat
  windows:
    - window_id: stable_window_id
      window_kind: event | segment | longitudinal
      summary: reviewer_safe_window_summary
      trace_refs:
        - trace:trace_id
      outcome_linkage_refs:
        - outcome-linkage:linkage_id
      metric_ids:
        - wp06_metric_id
      first_trace_sequence: positive_integer
      last_trace_sequence: positive_integer
  findings:
    - finding_id: stable_finding_id
      window_id: stable_window_id
      criterion_id: criterion-id
      review_status: observed | watch | review_needed
      signal_kind: stable | repair_watch | refusal_preserved | escalation_required | uncertainty_active
      summary: reviewer_safe_finding_summary
      trace_evidence_refs:
        - trace:trace_id
      outcome_linkage_refs:
        - outcome-linkage:linkage_id
      metric_ids:
        - wp06_metric_id
  synthetic_fixtures:
    - fixture_id: stable_fixture_id
      window_id: stable_window_id
      summary: synthetic_fixture_summary
      expected_criterion_ids:
        - criterion-id
      claim_boundary: bounded_non_claim
      limitations:
        - bounded_caveat
```

## Field Rules

- The packet must cover all six review criteria:
  - drift
  - repetition
  - repair
  - refusal
  - escalation
  - unresolved uncertainty
- The packet must include one `event`, one `segment`, and one `longitudinal`
  review window.
- Every finding must cite direct `trace:` evidence refs.
- Metric ids must come from the bounded WP-06 metric set rather than ad hoc
  scoring surfaces.
- The packet-level ordering rule must explicitly define window and finding
  tie-break behavior.
- Interpretation text must reject final judgment, scalar scoring, and anti-harm
  replacement framing.

## Initial v0.91 Packet

WP-07 lands one bounded review packet over the required upstream fixtures:

1. `event-window-refusal-boundary`
2. `segment-window-delegation-escalation`
3. `longitudinal-window-alpha`

The packet includes findings for refusal, escalation, unresolved uncertainty,
drift, repetition, and repair-watch posture. That gives reviewers a concrete
way to inspect the trajectory surface before WP-08 starts enforcing anti-harm
constraints over it.

## Synthetic Fixtures

WP-07 also lands three synthetic trajectory fixtures:

- refusal-boundary
- delegation-escalation
- longitudinal-alpha

These fixtures are review-facing and deterministic. They prove packet
generation, evidence citation, and tie-break stability. They do not simulate
live harmful behavior or claim anti-harm denial semantics ahead of WP-08.

## Non-Claims

This feature does not claim:

- final moral judgment
- scalar moral scoring
- public reputation ranking
- production anti-harm enforcement
- replacement of WP-08 trajectory constraints
- production moral agency, v0.92 birthday semantics, or v0.93 constitutional
  governance

It claims a narrower result: ADL has a deterministic moral trajectory review
packet that lets reviewers inspect event, segment, and longitudinal evidence
without reconstructing hidden state manually.
