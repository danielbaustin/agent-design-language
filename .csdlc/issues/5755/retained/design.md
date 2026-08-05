# #5755 Runtime v3 Protocol Security Closeout Repair

## Scope

Resolve the two accepted #5664 terminal-closeout blockers:

- protocol-adapter network security must prove client identity or a bounded equivalent instead of relying on no-client-auth TLS;
- Runtime control must reject oversized request bodies at the route boundary before unbounded JSON parsing.

## Approach

Keep the change bounded to `adl-runtime-kernel` protocol/control surfaces and focused tests. Prefer the smallest production-facing API change that makes the security invariant explicit and testable.

## Validation

Run focused protocol adapter and control tests, diff hygiene, and exact-head review before publication.
