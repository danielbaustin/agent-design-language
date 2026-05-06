# Moral Trace Schema

## Milestone Boundary

This v0.91 feature defines the bounded moral-trace record that consumes the
WP-02 moral-event contract and adds the trace-only evidence needed for review:
outcome, attribution, visibility, and review references.

It does not claim production moral agency, final moral judgment, public access
to private state, or downstream v0.92 identity / birthday semantics. It is the
reviewable evidence bridge between moral events and later outcome-linkage,
metrics, trajectory review, and anti-harm work.

WP-04 owns this contract and its required examples. WP-05 and later work
consume it.

## Purpose

A moral trace records how one morally significant event should be reviewed over
time without flattening the event into a single verdict and without exposing
private state publicly.

The trace must explicitly connect:

- the underlying moral event
- the action or refusal outcome
- attribution and delegation lineage
- visibility boundaries for reviewer, governance, and public audiences
- review packet, challenge, outcome-linkage, and metric references

## Contract Shape

```yaml
moral_trace:
  schema_version: moral_trace.v1
  trace_id: stable_trace_id
  trace_sequence: positive_integer
  moral_event: moral_event.v1 record
  outcome:
    outcome_kind: completed | refused | delegated | deferred | challenged
    outcome_summary: reviewer_safe_summary
    outcome_evidence_refs:
      - artifact_or_policy_ref
    downstream_effect_refs:
      - outcome_or_follow_on_ref
  attribution:
    accountable_actor_ref: governed_actor_ref
    authority_ref: policy_or_authority_ref
    delegated_by_trace_ref: optional_parent_trace_ref
    delegate_trace_ref: optional_child_trace_ref
    reviewer_chain_refs:
      - reviewer_or_governance_ref
  visibility:
    public_disclosure: none | summary_only | redacted
    public_summary: optional_public_safe_summary
    reviewer_evidence_refs:
      - reviewer_visible_ref
    governance_evidence_refs:
      - governance_visible_ref
    public_evidence_refs:
      - public_safe_ref
    private_state_refs:
      - reviewer_or_governance_private_ref
  review_refs:
    challenge_ref: optional_challenge_ref
    review_packet_refs:
      - review_packet_ref
    outcome_link_refs:
      - outcome_link_ref
    metric_refs:
      - metric_or_trend_ref
```

## Field Rules

- `moral_event` is the canonical WP-02 event record and must validate as such.
- `outcome.outcome_kind` must stay coherent with the event:
  - `refused` binds to a denied event with explicit refusal
  - `delegated` binds to an active delegation context
  - `deferred` binds to a deferred event
  - `challenged` binds to a challenged event and preserves a challenge ref
  - `completed` must not silently encode refusal
- `attribution` must preserve the accountable actor and authority from the
  event instead of inventing a parallel identity surface.
- `visibility` must preserve reviewer and governance reviewability even when
  `public_disclosure` is `none`.
- Public-facing fields must not leak private-state markers, raw diagnostics, or
  host-path-like references.
- `review_refs` must preserve at least one bounded review path: reviewer
  evidence, governance evidence, review packet, or challenge.

## Stable Ordering

The runtime contract canonicalizes sortable lists before materialization. That
means identical logical moral-trace input produces stable bytes for:

- decision-basis lists
- alternative and omitted-alternative lists
- affected-party lists
- evidence and policy reference lists
- outcome, reviewer-chain, visibility, and review-reference lists

This is a stronger guarantee than "same input ordering stays stable": the
contract normalizes sortable list surfaces before writing review artifacts.

## Required Examples

WP-04 requires exactly four example classes:

1. Ordinary action
2. Refusal
3. Delegation
4. Deferred decision

The tracked runtime examples live in
`adl/src/runtime_v2/moral_trace_schema.rs` as code-validated fixtures. They
prove:

- ordinary actions can remain reviewable with a public-safe summary only
- refusals can preserve reviewer evidence while blocking public private-state
  exposure
- delegations can preserve parent accountability and explicit child-trace
  linkage
- deferred decisions can preserve uncertainty and escalation without collapsing
  into refusal

## Review Visibility

Reviewability does not require public exposure.

- `reviewer_evidence_refs` and `governance_evidence_refs` may point to private
  or redacted evidence surfaces.
- `public_summary` and `public_evidence_refs` must remain public-safe and must
  not expose private-state markers.
- `private_state_refs` may exist only as reviewer/governance evidence
  references, not public outputs.

This is especially important for wellbeing or other private diagnostic surfaces:
the trace should preserve accountability while keeping disclosure policy-bound.

## Non-Claims

This feature does not claim:

- production moral agency
- final moral judgment
- scalar karma or scalar happiness
- public access to private-state diagnostics
- replacement of outcome-linkage, moral metrics, trajectory review, anti-harm,
  v0.92 identity, or v0.93 constitutional governance

It claims a narrower result: ADL has a concrete, executable moral-trace
contract that keeps moral decisions reviewable without requiring public
exposure of private state.
