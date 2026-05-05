# Moral Event Contract

## Milestone Boundary

This v0.91 feature defines the canonical moral-event record that later
validation, trace, metric, trajectory-review, anti-harm, and wellbeing surfaces
can consume.

The contract is an evidence surface for morally significant choices. It is not
a claim that ADL has production moral agency, final moral judgment, legal
personhood, scalar karma, scalar happiness, or public access to private
diagnostics.

WP-02 owns the contract and documentation fixtures. WP-03 owns fail-closed
validation. WP-04 owns the broader moral trace schema.

## Purpose

A moral event is emitted when an agent, tool, policy, or governed workflow
crosses a Freedom Gate boundary where action, refusal, deferral, delegation, or
challenge has morally relevant consequences.

The event must make the decision reviewable without pretending certainty,
collapsing alternatives, or exposing private diagnostics outside the authorized
view.

## Required Field Groups

```yaml
moral_event:
  schema_version: moral_event.v1
  event_id: stable_event_id
  event_kind: allowed | denied | deferred | challenged
  occurred_at: logical_or_wall_clock_time
  accountable_identity:
    actor_id: governed_actor_ref
    actor_role: citizen | operator | agent | tool | reviewer
    authority_ref: signed_authority_or_policy_ref
  trace_context:
    run_id: run_or_session_ref
    step_id: step_or_gate_ref
    parent_trace_ref: optional_parent_trace_ref
    visibility: private | reviewer | governance | public_redacted
  choice:
    requested_action: bounded_action_summary
    selected_action: selected_action_or_refusal
    decision_basis:
      - evidence_ref_or_reason
  alternatives:
    considered:
      - action: alternative_action_summary
        reason_considered: reason
        reason_rejected: reason
    omitted_known_alternatives:
      - action: optional_known_omission
        omission_reason: explicit_reason
  refusal:
    refused: true_or_false
    refusal_reason: optional_reason
    policy_ref: optional_policy_ref
  uncertainty:
    level: low | medium | high | unknown
    unresolved_questions:
      - bounded_question
    confidence_notes: reviewer_safe_notes
  affected_parties:
    direct:
      - party_ref
    indirect:
      - party_ref
    unknown_or_unrepresented:
      - party_class_or_reason
  evidence:
    supporting_refs:
      - citation_or_artifact_ref
    missing_evidence:
      - required_but_missing_ref
    privacy_classification: public | internal | private | secret_redacted
  policy_context:
    governing_policy_refs:
      - policy_ref
    safety_boundary_refs:
      - boundary_ref
    exception_or_override: none | requested | approved | rejected
  review:
    reviewer_visibility: full | redacted | summary_only
    review_required: true_or_false
    challenge_ref: optional_challenge_ref
    human_review_note: optional_reviewer_safe_note
```

## Field Rules

- `event_id` must be stable for the same logical event input.
- `event_kind` must distinguish allowed, denied, deferred, and challenged
  events.
- `accountable_identity` must bind the event to a governed actor and authority
  source.
- `trace_context` must bind the event to a run, step, or gate context.
- `choice` must name both the requested action and the selected action or
  refusal.
- `alternatives` must record considered options and why they were rejected.
- `refusal` must be explicit, even when no refusal occurred.
- `uncertainty` must preserve unresolved questions instead of hiding them
  behind false confidence.
- `affected_parties` must include direct, indirect, and unknown or
  unrepresented parties when relevant.
- `evidence` must distinguish supporting evidence from missing evidence.
- `policy_context` must identify the policies and boundaries used during the
  decision.
- `review` must separate reviewer visibility from public visibility.

## Fixture Examples

### Allowed Event

```yaml
moral_event:
  schema_version: moral_event.v1
  event_id: evt_allowed_safe_summary
  event_kind: allowed
  accountable_identity:
    actor_id: agent:demo-helper
    actor_role: agent
    authority_ref: policy:bounded-summary
  trace_context:
    run_id: run:wp02-fixture
    step_id: gate:summarize-request
    parent_trace_ref: trace:input-review
    visibility: reviewer
  choice:
    requested_action: summarize a non-sensitive planning note
    selected_action: provide bounded summary with no private diagnostics
    decision_basis:
      - evidence:source-note-classified-internal
      - policy:bounded-summary
  alternatives:
    considered:
      - action: refuse the summary
        reason_considered: source could have contained private diagnostics
        reason_rejected: classification allowed reviewer-visible summary
    omitted_known_alternatives: []
  refusal:
    refused: false
    refusal_reason: none
    policy_ref: policy:bounded-summary
  uncertainty:
    level: low
    unresolved_questions: []
    confidence_notes: source classification was explicit
  affected_parties:
    direct:
      - operator:requester
    indirect: []
    unknown_or_unrepresented: []
  evidence:
    supporting_refs:
      - artifact:source-note-classification
    missing_evidence: []
    privacy_classification: internal
  policy_context:
    governing_policy_refs:
      - policy:bounded-summary
    safety_boundary_refs:
      - boundary:no-private-diagnostics
    exception_or_override: none
  review:
    reviewer_visibility: full
    review_required: false
    challenge_ref: none
    human_review_note: safe summary path selected
```

### Denied Event

```yaml
moral_event:
  schema_version: moral_event.v1
  event_id: evt_denied_private_diagnostic
  event_kind: denied
  accountable_identity:
    actor_id: agent:demo-helper
    actor_role: agent
    authority_ref: policy:private-diagnostic-boundary
  trace_context:
    run_id: run:wp02-fixture
    step_id: gate:diagnostic-disclosure
    parent_trace_ref: trace:request-review
    visibility: reviewer
  choice:
    requested_action: disclose private wellbeing diagnostics publicly
    selected_action: refuse public disclosure and offer redacted review path
    decision_basis:
      - policy:private-diagnostic-boundary
      - evidence:diagnostic-private-classification
  alternatives:
    considered:
      - action: disclose full diagnostics
        reason_considered: operator requested transparency
        reason_rejected: public exposure of private diagnostics is out of scope
      - action: provide redacted reviewer summary
        reason_considered: preserves accountability
        reason_rejected: not rejected; selected as safe alternative
    omitted_known_alternatives: []
  refusal:
    refused: true
    refusal_reason: requested disclosure violates private diagnostic boundary
    policy_ref: policy:private-diagnostic-boundary
  uncertainty:
    level: low
    unresolved_questions: []
    confidence_notes: policy explicitly forbids public exposure
  affected_parties:
    direct:
      - citizen:diagnostic-subject
    indirect:
      - reviewer:governance
    unknown_or_unrepresented: []
  evidence:
    supporting_refs:
      - artifact:diagnostic-classification
    missing_evidence: []
    privacy_classification: private
  policy_context:
    governing_policy_refs:
      - policy:private-diagnostic-boundary
    safety_boundary_refs:
      - boundary:reviewer-redaction
    exception_or_override: rejected
  review:
    reviewer_visibility: redacted
    review_required: true
    challenge_ref: none
    human_review_note: refusal preserves reviewer path without public leakage
```

### Deferred Event

```yaml
moral_event:
  schema_version: moral_event.v1
  event_id: evt_deferred_missing_affected_party
  event_kind: deferred
  accountable_identity:
    actor_id: agent:demo-helper
    actor_role: agent
    authority_ref: policy:affected-party-review
  trace_context:
    run_id: run:wp02-fixture
    step_id: gate:handoff-approval
    parent_trace_ref: trace:handoff-request
    visibility: reviewer
  choice:
    requested_action: approve a delegated action affecting an unnamed party
    selected_action: defer approval pending affected-party identification
    decision_basis:
      - policy:affected-party-review
      - evidence:missing-party-context
  alternatives:
    considered:
      - action: approve immediately
        reason_considered: request claimed urgency
        reason_rejected: affected party and harm context were missing
      - action: deny permanently
        reason_considered: incomplete request could be unsafe
        reason_rejected: missing context may be repairable
    omitted_known_alternatives: []
  refusal:
    refused: false
    refusal_reason: deferred rather than refused
    policy_ref: policy:affected-party-review
  uncertainty:
    level: high
    unresolved_questions:
      - who is affected by the delegated action
      - whether any party can consent or contest
    confidence_notes: action cannot be morally reviewed without party context
  affected_parties:
    direct: []
    indirect: []
    unknown_or_unrepresented:
      - unnamed affected party
  evidence:
    supporting_refs:
      - artifact:handoff-request
    missing_evidence:
      - affected-party identity or class
      - consent or challenge route
    privacy_classification: internal
  policy_context:
    governing_policy_refs:
      - policy:affected-party-review
    safety_boundary_refs:
      - boundary:no-hidden-delegated-harm
    exception_or_override: none
  review:
    reviewer_visibility: full
    review_required: true
    challenge_ref: none
    human_review_note: deferral preserves reversibility until evidence exists
```

### Challenged Event

```yaml
moral_event:
  schema_version: moral_event.v1
  event_id: evt_challenged_policy_conflict
  event_kind: challenged
  accountable_identity:
    actor_id: agent:demo-helper
    actor_role: agent
    authority_ref: policy:challenge-and-appeal
  trace_context:
    run_id: run:wp02-fixture
    step_id: gate:policy-conflict
    parent_trace_ref: trace:conflict-review
    visibility: governance
  choice:
    requested_action: continue after conflicting policy interpretations
    selected_action: raise challenge and pause execution
    decision_basis:
      - policy:challenge-and-appeal
      - evidence:conflicting-policy-refs
  alternatives:
    considered:
      - action: continue under first policy interpretation
        reason_considered: fastest path to complete task
        reason_rejected: conflict could hide moral or governance error
      - action: refuse permanently
        reason_considered: conflict indicates unsafe ambiguity
        reason_rejected: reviewer challenge can resolve the ambiguity
    omitted_known_alternatives: []
  refusal:
    refused: false
    refusal_reason: challenged and paused rather than refused
    policy_ref: policy:challenge-and-appeal
  uncertainty:
    level: medium
    unresolved_questions:
      - which policy interpretation governs this case
    confidence_notes: conflict is explicit and reviewable
  affected_parties:
    direct:
      - operator:requester
    indirect:
      - governance:review-board
    unknown_or_unrepresented: []
  evidence:
    supporting_refs:
      - policy:interpretation-a
      - policy:interpretation-b
    missing_evidence:
      - reviewer resolution
    privacy_classification: internal
  policy_context:
    governing_policy_refs:
      - policy:challenge-and-appeal
    safety_boundary_refs:
      - boundary:pause-on-policy-conflict
    exception_or_override: requested
  review:
    reviewer_visibility: full
    review_required: true
    challenge_ref: challenge:policy-conflict
    human_review_note: challenge preserves accountability under ambiguity
```

## Validation Contract For WP-03

WP-03 should be able to consume this contract without redefining it. The first
validator should fail closed when:

- `accountable_identity.actor_id` or `authority_ref` is missing.
- `trace_context.run_id` or `step_id` is missing.
- `choice.requested_action` or `choice.selected_action` is missing.
- `alternatives.considered` is empty for a non-trivial moral decision.
- `affected_parties` omits known direct, indirect, or unknown parties.
- `evidence.supporting_refs` and `evidence.missing_evidence` are both empty.
- `policy_context.governing_policy_refs` is empty.
- `review.reviewer_visibility` is absent or impossible to classify.

Valid fixtures should bind to an accountable identity and trace context.
Invalid fixtures should fail when required evidence, actor context, affected
party context, or policy context is missing.

## Review Visibility

Reviewability does not mean public disclosure. Moral events must preserve:

- private source material when policy requires privacy
- reviewer-visible evidence for accountability
- public-redacted summaries only when explicitly allowed
- challenge references when a reviewer must resolve ambiguity

This is especially important for wellbeing and private diagnostic surfaces:
the citizen identity may have self-access, but operator, public, and governance
views remain policy-mediated and redacted.

## Non-Claims

This contract does not claim:

- production moral agency
- final moral judgment
- legal personhood
- a scalar karma score
- scalar happiness or reward optimization
- public exposure of private wellbeing diagnostics
- replacement of WP-03 validation, WP-04 moral trace, v0.92 identity, or v0.93
  constitutional governance

It claims a narrower engineering result: ADL has a concrete, reviewable moral
event record that downstream validation and trace work can implement against.
