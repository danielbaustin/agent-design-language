# v0.94 Feature: Secure Execution and Trust Convergence

## Status

Forward-planning feature contract for `v0.94`.

## Purpose

Converge the later security and trust architecture into one bounded execution
story: policy, identity/auth, provider trust, sandboxing, secrets/data
governance, and secure execution should read as one coherent substrate instead
of as disconnected planning fragments.

## Source Inputs

- `docs/milestones/v0.94/README.md`
- `docs/milestones/v0.94/features/README.md`
- `docs/milestones/v0.94/features/SIGNED_TRACE_AND_TRACE_QUERY_v0.94.md`
- `docs/milestones/v0.94/features/REASONING_GRAPH_BASELINE_v0.94.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the bounded `v0.94` secure-execution architecture ADL will actually converge
  on for MVP
- one coherent relationship among identity/auth, policy evaluation, capability,
  sandbox/runtime isolation, provider trust, and secrets/data governance
- explicit integration boundaries with signed trace/query, reasoning graphs,
  and later distributed execution rather than leaving security as a sidecar
- convergence of binary ACIP, WebSocket transport, public schema catalogs,
  per-message authorization, cryptographic trust, and signed/queryable trace
  into one auditable secure-execution path
- a reviewer-facing truth boundary for what `v0.94` does and does not claim

## Enterprise-Security Input Boundary

The v0.94 secure-execution convergence story should consume the v0.91.5
enterprise-security organization packet:

- `docs/milestones/v0.91.5/features/ENTERPRISE_SECURITY_ORGANIZATION_BOUNDARY_v0.91.5.md`

This keeps secure execution connected to the v0.93 enterprise-security bands
without turning enterprise-only audit, isolation, deployment, or compliance
assumptions into hidden prerequisites for normal local ADL development. v0.94
should converge the proven v0.93 bands with signed trace/query and reasoning
graph provenance rather than inventing a second authority model.

## Non-goals

- external certification claims
- silently introducing a second runtime authority model
- treating secure execution as only infrastructure hardening with no policy or
  trace consequences
- pushing core trust-boundary decisions back into undocumented environment
  folklore
- treating provider WebSocket sessions or transport connectivity as execution
  authority without ADL schema, policy, crypto, trace, and replay evidence

## WebSocket/ACIP Convergence

By `v0.94`, ADL's secure-execution story should be able to explain a
WebSocket-carried ACIP message end to end: public schema lookup, binary decode,
identity and policy evaluation, cryptographic acceptance, replay/sequence
handling, execution admission or denial, signed trace emission, and queryable
review evidence.

This does not require WebSocket to be the only transport. It means WebSocket is
no longer a sidecar experiment if used: it must obey the same public schema,
authority, trace, and review rules as every other ADL communication path.

## Completion Target

`v0.94`
