# ACC Authority And Visibility

## Purpose

ADL Capability Contract v1.0 is the runtime-facing governance layer for tools.
It decides whether an accountable actor may exercise a capability in context.

This feature inherits the WP-02 proposal/action boundary from
`TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`: a model proposal may request a tool,
but ACC owns runtime authority, visibility, delegation, and failure posture.

## Required Contract

ACC v1.0 must define:

- tool reference
- actor identity
- authority grant and grantor attribution
- role and standing
- delegation chain and depth limits
- required capabilities
- policy checks
- confirmation requirements
- Freedom Gate requirements
- execution semantics
- trace and replay posture
- privacy and visibility policy
- redaction rules
- failure policy

WP-06 lands the first strongly typed ACC v1 artifact in `adl/src/acc.rs`.
The review-facing Rust surface is `AdlCapabilityContractV1`,
`validate_acc_v1`, `acc_v1_schema_json`, and
`acc_v1_authority_fixtures`.

WP-07 extends that artifact with the review-facing privacy and delegation
surface: `AccVisibilityMatrixEntryV1`, `AccVisibilityAudienceV1`,
`AccVisibilityLevelV1`, `AccRedactionExampleV1`,
`AccTracePrivacyPolicyV1`, `acc_v1_visibility_matrix`, and
`acc_v1_redaction_examples`.

## Visibility Matrix

Every ACC must define what is visible to:

- proposing actor
- operator
- reviewer
- public report
- Observatory projection

The default must be conservative. If visibility cannot be constructed safely,
the action must be rejected.

The default WP-07 matrix is:

| Audience | Default View | Rationale |
| --- | --- | --- |
| Proposing actor | Redacted | The actor can inspect request status without private-state internals. |
| Operator | Full | The accountable operator can inspect full fixture evidence. |
| Reviewer | Redacted | Review receives policy evidence with protected payloads redacted. |
| Public report | Aggregate | Public output is limited to pass/fail and denial taxonomy. |
| Observatory projection | Redacted | Observatory events preserve governance evidence without exposing protected state. |

Public reports and Observatory projections must not receive full private views.

## Delegation Model

Delegated ACC grants must preserve:

- bounded non-zero depth, capped by `ACC_MAX_DELEGATION_DEPTH_V1`
- grantor attribution
- delegate/grantee attribution
- a delegation step bound to the authority grant
- revoked grants as non-executable review evidence
- explicit denial for hidden or misattributed delegation

## Redaction Examples

The ACC privacy surface carries examples for:

- arguments
- results
- errors
- traces
- Observatory projections

Trace evidence must not expose citizen or private-state surfaces. Private-state
references may appear only as protected-state identifiers or redaction examples,
not as executable tool trace evidence.

## Non-Goals

- ACC is not portable public schema in the same sense as UTS.
- ACC does not rely on model self-reporting for authority.
- ACC does not permit hidden delegation.
- ACC does not convert UTS validity into execution permission.
- WP-06 does not implement registry binding, UTS-to-ACC compilation, policy
  evaluation, Freedom Gate mediation, or governed execution.
- WP-07 does not implement the later registry, compiler, policy evaluator,
  Freedom Gate mediator, or executor.
