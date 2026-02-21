# ADR 0003: Remote Execution MVP Boundary

## Status
Accepted (v0.5)

## Decision
Remote execution MVP is intentionally scoped to a narrow boundary:
- local scheduler owns planning, dependency resolution, and ordering
- remote endpoint executes one fully resolved step via `/v1/execute`
- request payloads are capped at 5 MiB

## Rationale
This separation allows distributed execution experiments without moving scheduler
correctness into a network service in v0.5.

## Alternatives Considered
- Full remote orchestration service in v0.5
- Remote planner/scheduler ownership
- Unbounded request payloads

## Consequences
- Deployment guidance must treat this as trusted-network/localhost MVP.
- Security envelope items (authn/authz, request signing, richer policy) are
  deferred to subsequent milestone work.
- Integration tests focus on deterministic mixed local/remote step behavior.
