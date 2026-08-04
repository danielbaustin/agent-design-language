# Design: Make HTML Observatory Fully Functional

Issue #5789 makes the checked-in HTML Observatory a real Runtime v3 operator
surface. The page must prefer the active local Runtime v3 API when available,
render live readiness/feed truth without stale retained placeholders, and expose
safe operator-to-agent communication through the governed write path.

The implementation should keep retained v0.91.7 evidence available, but only as
historical fallback. Runtime v3 GET feed truth remains authoritative whenever
`/v1/observatory` is reachable, even if WebSocket streaming is unavailable.

The operator messaging flow is part of the feature, not a later garnish:
operators must be able to choose/target agents, compose a message, submit via
authenticated Runtime v3 write authority, and see receipt/event-tail evidence or
a fail-closed rejection.

No AWS operation is in scope.
