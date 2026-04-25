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
