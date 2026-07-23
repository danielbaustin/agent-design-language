# Parity-D Security And Acceptance Matrix

| ID | Capability | Positive proof | Required negative proof | Authority / evidence owner |
|---|---|---|---|---|
| AC-1 | One init model | Guardian reads one init file and launches the canonical kernel at the configured listener/public base | Unknown fields, invalid listener, non-HTTPS public base, invalid TLS paths, and duplicate origins fail before readiness | `config.rs`, guardian launch contract |
| AC-2 | Secure local/remote API | Local and gateway-shaped clients use the same rustls Axum router and configured endpoint | Plain HTTP, missing/invalid credentials, capability escalation, and unlisted origins fail closed | control adapter plus transport-independent authority |
| AC-3 | HTTP/WebSocket Observatory | Browser consumes live admitted-agent and Runtime state over authenticated HTTP and WebSocket | Missing bearer, bad origin, oversized/malformed frame, token in URL, and stale session fail closed | Runtime feed and HTML Observatory |
| AC-4 | Discovery correctness | Readiness/feed expose the actual bound/configured address and public HTTPS base | Default-port substitution, hard-coded IP, conflicting public base, and ephemeral-port misreporting are rejected | listener-owned discovery projection |
| AC-5 | Guardian resilience | Child start, graceful signal, pressure serialization, checkpoint restore, and bounded restart execute | Invalid config does not loop, intentional stop does not restart, restart budget exhausts, child is reaped | external guardian and canonical child contract |
| AC-6 | Vector telemetry | Redacted `adl_event` reaches Vector route while kernel health remains independent | Vector unavailable degrades truthfully; secrets, raw credentials, key material, unsafe errors, and absolute host paths never emit | tracing stderr plus Vector configuration |
| AC-7 | Operational rollback | Executable selector transitions candidate Runtime v3 to the prior approved Runtime v3 process/configuration and authenticated HTTPS health passes before and after | Report-only selector, metadata assertion, environment echo, in-memory facade, automatic default switch, Runtime v2 source edit, sidecar, or AWS operation receives no credit | operational selector receipt plus service-health evidence |
| AC-8 | Quality and budget | Exact-revision tests, strict lint, COTS inventory, LoC/module/test report, and bounded soak are green | Fixture-only, skipped, deferred, prose-only, stale-revision, or over-budget proof receives no credit | #5336 budget and #5361 acceptance |

## Security invariants

1. TLS is mandatory for local and remote Runtime access.
2. Network location never grants authority; every command or feed session is authenticated.
3. Configuration is data, not executable policy, and rejects unknown or secret-bearing fields.
4. Discovery reports listener truth rather than a default constant.
5. The guardian is an external process owner, not a Runtime sidecar or second control plane.
6. Vector owns export mechanics; kernel liveness does not depend on a collector.
7. Retained evidence is relative, bounded, redacted, deterministic, and exact-revision bound.
8. Runtime v2 code, AWS execution, hard-coded IPs, and automatic cutover are outside scope.
9. Exact preparation base `6d0f6115632a06619544b8ad4792792e741f1f31` and reviewed head `2f26da4455efd4dfc7ab6c65df5d19327fe765c8` remain in retained validation.
