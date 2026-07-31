# ADR 0054: Runtime v3 Guardian-Owned Kernel And API Boundary

- Status: Accepted
- Date: 2026-07-30
- Accepted in: v0.91.8
- Related issues: #5336, #5341, #5361, #5590, #5591, #5592, #5589
- Related ADRs: ADR 0011, ADR 0012, ADR 0013, ADR 0017, ADR 0048, ADR 0049
- Supersedes: ADR 0012 and ADR 0013 for selected runtime implementation authority
- Source evidence:
  - `infra/runtime-v3/runtime-init.toml`
  - `docs/milestones/v0.91.8/review/runtime_v3_acceptance_5361.v1.json`
  - `docs/milestones/v0.91.8/features/RUNTIME_V3_ADAPTER_v0.91.8.md`
  - `docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md`
  - merge commit `f7258b07e`

## Context

Runtime ownership had been split across compatibility binaries, sidecar
launchers, supervisors, and presentation code. That made startup authority,
health, restart, transport, and state ownership difficult to establish.

v0.91.8 accepted Runtime v3 after cross-platform lifecycle, secure API,
guardian, continuity, rollback, and consumer proof.

## Decision

Runtime v3 has one production authority boundary:

- the guardian is process zero for the runtime lifecycle;
- one Tokio/Axum/Rustls kernel owns execution, HTTPS, authenticated writes,
  WSS, ACIP ingress, health, readiness, shutdown, and recovery;
- one init file supplies operational configuration;
- the Runtime API is the integration boundary for external clients;
- the HTML and Unity Observatories are clients, not embedded runtime servers;
- protobuf is an internal typed carrier and does not create a second runtime
  authority.

Read-only API surfaces may be public. Mutating operations require authenticated
and authorized identity.

## Consequences

- Startup, shutdown, recovery, and health have one accountable owner.
- External clients can evolve independently against versioned API contracts.
- Sidecar launchers and degraded production placeholders cannot receive
  runtime-completion credit.
- Platform-specific process containment may differ internally, but API and
  lifecycle behavior must remain portable.

## Alternatives Considered

### Multiple interchangeable production launchers

Rejected. Split ownership obscures failure and restart authority.

### Embed the Observatory in the runtime

Rejected. Presentation is an external client concern.

## Validation Notes

Validate native guardian launch/shutdown/restart, HTTPS and WSS exchange,
authenticated writes, public reads, canonical ACIP ingress, readiness,
continuity restore, pressure shutdown, rollback, and macOS/Linux/Windows
lifecycle behavior.

## Non-Claims

- This ADR does not claim distributed multi-node relocation.
- This ADR does not make the Observatory an execution authority.

