# Runtime v3 Vector OTEL Logging Design

## Boundary

Runtime v3 emits structured events through the existing Rust `tracing` facade.
Production startup installs one JSON subscriber that writes to Vector ingress.
Vector is the only general-purpose log writer and router: it owns durable
master-log output, bounded buffering, redaction transforms, OTLP export,
CloudWatch-compatible routing, retry, and drain.

The observatory HTML app is out of scope. Runtime v3 only exposes API/status
truth for the observability pipeline on the existing Axum/Tokio control API.

## Parity Source

Runtime v2 parity is the managed Vector component in
`adl-runtime/src/observability.rs`: pinned `.adl/bin/vector`, generated config,
OTLP/HTTP endpoint configuration, resource attributes, exporter status,
supervised child lifecycle, durable files, and redaction.

Runtime v3 must implement the same behavior in the kernel process without
Python, wrappers, duplicate JSONL writers, or degraded production placeholders.

## Implementation Shape

1. Add a Runtime v3 observability module that:
   - resolves and verifies the pinned Vector binary;
   - renders a Vector config under an explicit absolute state root;
   - creates a tracing JSON ingress consumed by Vector;
   - supervises one long-lived Vector child;
   - exposes health, config, endpoint, drain, and failure status.
2. Install the subscriber once during production kernel startup.
3. Route existing Runtime v3 events through `tracing` fields with stable
   runtime, guardian, process, lifecycle, component, operation, reason, error,
   revision, trace, span, and parent identifiers.
4. Extend Runtime v3 API/status surfaces so WP-12 and the observatory API can
   consume log health without custom harness logging.
5. Add a Rust clean-log auditor that validates the Vector durable master log.

## Validation

Validation must run in the #5691 worktree and use the repo-pinned Vector binary.
The proof must cover config rendering, vector validation, real Vector startup,
real master-log output, OTLP receiver exchange, redaction, export failure
recording, shutdown drain, status/API exposure, and clean-log rejection cases.
