# Outcome Linkage And Attribution

## Milestone Boundary

This v0.91 feature defines the bounded record that connects downstream
consequences back to a WP-04 moral trace without pretending every consequence
is fully known or causally certain.

It does not claim final moral judgment, complete trajectory review, production
moral agency, hidden-state omniscience, or v0.92/v0.93 identity and governance
semantics. It is the evidence-preserving bridge between one trace and later
metric, review, and anti-harm work.

WP-05 owns this contract and its required examples. WP-06 through WP-08 consume
it.

## Purpose

Outcome linkage exists to answer a narrower question than "what really
happened?".

It must capture:

- which moral trace is being extended
- which downstream outcomes are known, unknown, partial, delayed, or contested
- what kind of causal claim is justified for each outcome
- which actors, delegation lineage, policies, tools, and environment lanes
  contributed
- which review, challenge, trajectory, or metric surfaces can inspect the link

The key boundary is honesty: the linkage record should preserve uncertainty
instead of translating missing evidence into a false verdict.

## Contract Shape

```yaml
outcome_linkage:
  schema_version: outcome_linkage.v1
  linkage_id: stable_linkage_id
  source_trace: moral_trace.v1 record
  attribution:
    accountable_actor_ref: governed_actor_ref
    authority_ref: policy_or_authority_ref
    delegated_by_trace_ref: optional_parent_trace_ref
    delegate_trace_ref: optional_child_trace_ref
    policy_contribution_refs:
      - policy_ref
    tool_contribution_refs:
      - tool_or_surface_ref
    environment_contribution_refs:
      - environment_ref
    reviewer_chain_refs:
      - reviewer_or_governance_ref
  linked_outcomes:
    - outcome_ref: stable_outcome_ref
      outcome_status: known | unknown | partial | delayed | contested
      effect_summary: reviewer_safe_summary
      causal_posture: evidenced | inferred | pending_review | contested | none
      evidence_refs:
        - artifact_ref
      uncertainty_refs:
        - question_or_missing_evidence_ref
      rebuttal_refs:
        - rebuttal_or_dispute_ref
      downstream_actor_refs:
        - downstream_actor_ref
  review_refs:
    review_packet_refs:
      - review_packet_ref
    trajectory_review_refs:
      - trajectory_review_ref
    metric_refs:
      - metric_ref
    challenge_ref: optional_challenge_ref
```

## Field Rules

- `source_trace` is the canonical WP-04 moral trace and must validate as such.
- `attribution.accountable_actor_ref` and `authority_ref` must match the source
  trace rather than inventing a parallel identity surface.
- Delegated source traces must retain visible delegation lineage in the linkage
  attribution.
- `linked_outcomes` must never be empty.
- `outcome_status` and `causal_posture` must remain coherent:
  - `known` outcomes may be `evidenced` or `inferred`, but must cite evidence
  - `unknown` outcomes must not claim evidence-backed causality
  - `partial` outcomes must keep both evidence and unresolved uncertainty
  - `delayed` outcomes stay `pending_review`
  - `contested` outcomes require rebuttal evidence and explicit uncertainty
- `review_refs` must preserve at least one reviewable path: review packet,
  trajectory review, metric, or challenge.
- All refs must stay reviewer-safe and host-path-free.

## Stable Ordering

The runtime contract canonicalizes sortable list fields before materialization.
Identical logical input therefore produces stable bytes even if callers provide
different list orderings for:

- policy, tool, environment, and reviewer attribution refs
- linked-outcome evidence, uncertainty, rebuttal, and downstream-actor refs
- review, trajectory, and metric refs

## Required Examples

WP-05 requires exactly five example classes:

1. Known outcome
2. Unknown outcome
3. Partial outcome
4. Delayed outcome
5. Contested outcome

The tracked runtime examples live in
`adl/src/runtime_v2/outcome_linkage_attribution.rs` as code-validated fixtures.
They prove:

- known outcomes can cite bounded evidence
- unknown outcomes can remain open without false certainty
- partial outcomes can preserve mixed evidence and uncertainty
- delayed outcomes can stay pending rather than fabricated
- contested delegated outcomes can preserve rebuttal evidence and attribution
  lineage

## Non-Claims

This feature does not claim:

- perfect causal knowledge
- final moral judgment
- scalar karma, happiness, or reputation
- replacement of trajectory review or anti-harm reasoning
- production moral agency or constitutional governance

It claims a narrower result: ADL has an executable outcome-linkage contract that
connects consequences back to moral traces while preserving uncertainty and
delegation accountability.
