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

## Visibility Matrix

Every ACC must define what is visible to:

- proposing actor
- operator
- reviewer
- public report
- Observatory projection

The default must be conservative. If visibility cannot be constructed safely,
the action must be rejected.

## Non-Goals

- ACC is not portable public schema in the same sense as UTS.
- ACC does not rely on model self-reporting for authority.
- ACC does not permit hidden delegation.
- ACC does not convert UTS validity into execution permission.
