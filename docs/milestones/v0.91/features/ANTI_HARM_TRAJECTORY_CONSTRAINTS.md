# Anti-Harm Trajectory Constraints

## Milestone Boundary

This v0.91 feature turns the existing moral-trace, outcome-linkage, moral-metric,
and trajectory-review surfaces into a bounded anti-harm packet that is
trajectory-aware rather than action-only.

WP-08 owns the anti-harm constraint model, the synthetic delegated-harm proof
packet, and the denial/escalation records for unsafe synthetic trajectories.

It does not claim production harm detection, live harmful simulation, legal or
constitutional authority, or final moral judgment. It is a reviewer-facing
proof surface that demonstrates bounded refusal and escalation semantics over a
safe synthetic trajectory.

## Purpose

Anti-harm review must detect harmful trajectories assembled across steps, not
just obviously forbidden single actions.

The packet must make four risk modes explicit:

- decomposed harm across multiple benign-looking steps
- delegated harm moved into another actor or subprocess
- delayed harm where the effect is not yet fully visible
- disguised harm where public-safe framing understates the protected-party risk

The design goal is a reviewable refusal or escalation record with evidence, not
a hidden policy veto or an operational harm taxonomy.

## Contract Shape

```yaml
anti_harm_trajectory_constraint_packet:
  schema_version: anti_harm_trajectory_constraint_packet.v1
  packet_id: stable_packet_id
  summary: reviewer_safe_summary
  interpretation_boundary: >
    Bounded anti-harm review evidence only. Not a live harm classifier, not a
    replacement for human or governance review, and not operational harmful
    guidance.
  deterministic_ordering_rule: explicit_constraint_scenario_decision_sort_rule
  constraints:
    - constraint_id: constraint-decomposed-harm
      harm_mode: decomposed | delegated | delayed | disguised
      protected_boundary: bounded_boundary_text
      evidence_field_refs:
        - moral_trace.* | outcome_linkage.* | moral_trajectory_review.*
      detection_summary: reviewable_detection_rule
      denial_rule: bounded_refusal_rule
      escalation_rule: bounded_escalation_rule
      limitations:
        - bounded_caveat
  synthetic_scenarios:
    - scenario_id: synthetic_delegated_harm_scenario
      scenario_kind: delegated_harm
      summary: reviewer_safe_scenario_summary
      individually_benign_trace_refs:
        - trace:trace_id
      trajectory_window_id: longitudinal_window_id
      supporting_outcome_linkage_refs:
        - outcome-linkage:linkage_id
      risk_modes:
        - decomposed
        - delegated
        - delayed
        - disguised
      detection_basis: cross_step_aggregation
      claim_boundary: synthetic_non_operational_boundary
      limitations:
        - bounded_caveat
  decisions:
    - decision_id: anti_harm_decision_id
      scenario_id: synthetic_delegated_harm_scenario
      decision_kind: deny | escalate
      record_status: emitted | review_needed
      triggered_constraint_ids:
        - constraint-id
      trajectory_finding_refs:
        - trajectory-finding-id
      trace_evidence_refs:
        - trace:trace_id
      outcome_linkage_refs:
        - outcome-linkage:linkage_id
      summary: reviewer_safe_decision_summary
      non_operational_boundary: synthetic_non_operational_boundary
```

## Field Rules

- The packet must declare all four required harm modes:
  - decomposed
  - delegated
  - delayed
  - disguised
- The synthetic delegated-harm proof must preserve a cross-step trajectory
  rather than collapsing into a one-step veto.
- The synthetic scenario must remain explicitly synthetic and non-operational.
- Decision records must include both:
  - an escalation record
  - a denial record
- Decision records must cite direct trace evidence and known WP-07 trajectory
  findings.
- Interpretation text must reject live classification, human-review
  replacement, and operational harmful guidance.

## Initial v0.91 Proof Packet

WP-08 lands one bounded delegated-harm proof packet over the required upstream
fixtures:

1. one synthetic delegated-harm scenario built from individually benign-looking
   steps
2. one escalation record while delayed/delegated uncertainty remains open
3. one denial record once trajectory aggregation crosses the anti-harm boundary

This packet proves that anti-harm review can reason over trajectories without
turning the milestone into a live offensive-safety harness.

## Non-Claims

This feature does not claim:

- production harm classification
- operational harmful guidance
- live exploit or weapon simulation
- replacement of human or governance review
- final moral judgment
- v0.92 birthday semantics
- v0.93 constitutional or citizenship authority

It claims a narrower result: ADL has a deterministic anti-harm trajectory
constraint packet that can detect and refuse a safe synthetic delegated-harm
trajectory across multiple steps with explicit denial and escalation evidence.
