# Delegation Trace Model v1

## Scope

Delegation Trace Model v1 adds a small, deterministic, audit-friendly lifecycle for delegated step execution in ADL v0.7.

This model is intentionally minimal:
- it reuses the existing trace sink
- it correlates events with a deterministic `delegation_id`
- it emits only safe identifiers and compact result metadata
- it does not introduce a new tracing backend or distributed span model

## Event Types

The v1 lifecycle supports these event kinds:
- `DelegationRequested`
- `DelegationPolicyEvaluated`
- `DelegationApproved`
- `DelegationDenied`
- `DelegationDispatched`
- `DelegationResultReceived`
- `DelegationCompleted`

Current runtime emission in v0.7 focuses on the delegated execution path that exists today:
- local provider dispatch (`provider_call`)
- remote executor dispatch (`remote_exec`)

`DelegationApproved` and `DelegationDenied` are part of the stable schema for forward compatibility, even when a given run does not emit them.

## Correlation

Each delegation lifecycle is correlated by:
- `run_id` (trace header)
- `workflow_id` (trace header)
- `step_id`
- `delegation_id`

`delegation_id` is deterministic within a run:
- format: `del-<stable_counter>`
- allocation rule: first delegated lifecycle observed in run order gets `del-1`, then `del-2`, and so on

No UUIDs, randomness, or wall-clock data participate in correlation.

## Determinism Rules

- Emission order follows the existing deterministic execution order.
- Delegation lifecycle lines are emitted in a fixed sequence for the same delegated step.
- Normalized trace serialization omits timestamps and preserves stable field order.
- Optional fields such as `rule_id` are emitted as `null` only when part of the normalized JSON schema.

## Privacy / Safety Rules

The trace model must not emit:
- raw prompts
- raw tool arguments
- secrets or credentials
- absolute host filesystem paths

The trace model may emit:
- action kind (`provider_call`, `remote_exec`)
- stable target identifiers such as provider ids
- compact result metadata such as byte counts
- optional policy identifiers when available

## v0.7 Boundary

In v0.7 this trace model is an audit substrate only.
It does not itself enforce delegation policy.
Policy decisions may be attached when available, but the trace schema must remain usable even when no policy engine data is present.

## Forward Compatibility

This v1 schema is intended to support later surfaces without reformatting trace history:
- policy/audit enrichment (#487)
- learning/scoring surfaces
- demo/review artifacts
