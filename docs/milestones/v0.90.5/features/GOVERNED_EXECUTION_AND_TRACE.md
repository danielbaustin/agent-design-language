# Governed Execution And Trace

## Purpose

The governed executor runs tool actions only after ACC construction, policy
evaluation, and Freedom Gate mediation.

This feature inherits the WP-02 proposal/action boundary from
`TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`: direct model-output execution is
forbidden, and dangerous categories require denial evidence unless a later
bounded fixture proves a safer dry-run path.

## Execution Contract

The executor must:

- reject direct model-output execution
- require an approved ACC
- preserve refusal and deferral behavior
- execute only registered adapters
- support dry-run and fixture-backed adapters for the first milestone
- emit selected and rejected action records
- preserve replay posture
- avoid leaking protected tool arguments, private state, or secret values

## Trace Contract

Trace must record:

- proposal
- normalized proposal
- constructed ACC
- policy injection
- visibility policy
- Freedom Gate decision
- selected action
- rejected alternatives where allowed
- execution result or refusal
- redaction decisions

Trace must be useful for accountability without becoming a privacy leak.

Denied proposals are first-class trace evidence. They must identify the
boundary that stopped the proposal without leaking protected arguments,
private state, prompts, credentials, or secret-like values.

## WP-11: Policy Injection And Authority Evaluation

WP-11 implements the bounded policy-authority slice in
`adl/src/policy_authority.rs`:

- `PolicyAuthorityContextV1`
- `PolicyAuthorityConstraintsV1`
- `PolicyAuthorityEvaluationInputV1`
- `PolicyAuthorityEvaluationV1`
- `PolicyAuthorityEvidenceRecordV1`
- `evaluate_policy_authority_v1`
- `wp11_policy_context_fixture`
- `wp11_policy_constraints_fixture`
- `wp11_policy_input_fixture`

The evaluator consumes explicit actor role, standing, grant status, delegation
depth, execution environment, data sensitivity, resource scope, adapter id, and
bounded constraints. It emits allowed, denied, deferred, challenged, or revoked
decisions with policy evidence records. Missing context fails closed, and model
confidence is accepted only as an ignored input field so tests can prove that
authority evaluation does not depend on model self-report.
