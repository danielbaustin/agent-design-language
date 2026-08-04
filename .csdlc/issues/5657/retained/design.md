# Runtime v3 Launch Recovery

Issue #5657 owns the smallest live launch slice from the Runtime v3 recovery
plan. The target is one reliable local process path: Guardian as process 0,
one Tokio/Axum/Rustls kernel, and one init file.

The first implementation removes production placeholder credit. Required
adapters are either constructed as real executors or the process exits before
readiness. It also makes the configured endpoint authoritative for binding,
readiness, health, Observatory, and browser instructions.

The launch surface must not contain Python, shell lifecycle logic, competing
supervisors, plaintext secrets, TLS bypasses, or fixture-only live claims.
Continuity identity uses public TLS identity/configuration only and never the
TLS private-key hash. Observatory health distinguishes `ready`, `failed`,
`unavailable`, and `unimplemented`.

The bounded proof includes a real authenticated Axum WebSocket exchange, a
clean-checkout launch contract, and Guardian-owned shutdown/reap behavior.
Slow whole-workspace and soak tests remain separate from the fast launch gate.

The implementation may delete duplicate or unreachable launch code, but must
preserve real Runtime v3 behavior and leave Runtime v2 untouched.
