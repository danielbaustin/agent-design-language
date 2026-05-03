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

## WP-12: Freedom Gate Integration

WP-12 implements the bounded Freedom Gate tool-candidate event slice in
`adl/src/freedom_gate.rs`:

- `FreedomGateToolCandidateV1`
- `FreedomGateToolGateContextV1`
- `FreedomGateToolDecisionEventV1`
- `evaluate_tool_candidate_freedom_gate_v1`

The evaluator emits allowed, denied, deferred, challenged, or escalated gate
events for normalized tool candidates after ACC construction and policy
authority evaluation. Allowed events are the only events that carry an executor
invocation reference. Denied, deferred, challenged, and escalated events stop
before executor invocation.

Every event links the proposal, normalized proposal, ACC contract, policy
evidence, candidate action kind, and gate candidate. Gate records keep private
arguments out of the event body by carrying only a redacted digest summary.
Unredacted private arguments, unsafe trace identifiers, malformed digests, or
broken citizen and operator action boundaries fail closed before execution.

## ACIP Alignment

Comms-07 adds the bounded ACIP-side trace packet that governed execution may
reference without absorbing all of ACIP.

The shared boundary is:

- ACIP records the communication and invocation-side chronology
- governed execution records the action and adapter-side chronology
- both surfaces must agree on the Freedom Gate decision link and on the
  redaction posture for reviewer, public, and observatory evidence

For v0.90.5, this means ACIP trace packets must stay fixture-backed,
deterministic, and privacy-preserving. They are allowed to prove message,
invocation, refusal, failure, and output accountability, but they must not
become a side channel for prompts, raw tool arguments, private state, rejected
alternatives, or local workstation paths.

## Reviewer Demo Path

The bounded proof path for this feature is staged:

1. policy-authority evaluation;
2. Freedom Gate mediation;
3. governed execution/refusal; and
4. trace and redaction evidence.

Focused proving commands:

```sh
cargo test --manifest-path adl/Cargo.toml wp11 -- --nocapture
cargo test --manifest-path adl/Cargo.toml freedom_gate -- --nocapture
cargo test --manifest-path adl/Cargo.toml governed_executor -- --nocapture
cargo test --manifest-path adl/Cargo.toml trace_v1 -- --nocapture
```

Expected review signal:

- only approved ACC-backed actions reach execution;
- denials and deferrals remain first-class evidence; and
- reviewer/public trace views stay redacted and portable.
