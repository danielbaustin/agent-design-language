# Tooling Proof-Loop Reliability

## Metadata

- Feature Name: Tooling Proof-Loop Reliability
- Milestone Target: `v0.91.6`
- Status: WP-03 logging and GitHub/tooling observability baseline completed for `#3995`-`#4001`
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture, artifact
- Proof Modes: tests, review, CI

## Purpose

Define the tooling/logging reliability work required so docs-only and
feature-doc issues can move quickly without weakening proof truth.

## Scope

In scope:

- prompt-card validation latency and diagnostics;
- enum diagnostics for lifecycle states;
- absolute-path leakage false positives;
- octocrab token preflight;
- PR merge false-negative and GitHub checks transient classification;
- logging/Otel consumption for proof loops;
- card-to-GitHub projection reliability for PR publication and closeout truth.

Out of scope:

- broad toolkit simplification;
- runtime feature implementation;
- replacing the C-SDLC lifecycle.

## Required Decisions

- Which validation checks must be local and deterministic?
- Which GitHub/API failures are retryable versus blocking?
- Which observability events are release-gating proof versus diagnostic noise?
- Which docs-only fast paths can be used without bypassing review?
- Which GitHub issue/PR surfaces should be card-owned managed projections
  versus drift-checked mirrors?

## Dependencies

- Existing remediation issues `#3802`-`#3805`, `#3811`, `#3822`, and `#3823`.
- `#3935` for `SOR`-driven PR-body convergence and generalized card-projection
  policy, now consumed by the completed WP-03 tooling lane.
- Logging mini-sprint outputs.
- Toolkit simplification sprint.

## Validation And Review

- Run focused tooling tests for changed validators.
- Record transient failures and retry evidence.
- Verify no token or secret is logged.
- Review SOR truth for local versus CI proof.
- Review the card/GitHub projection policy before implementation claims.

## WP-03 Execution Packet

The logging/tooling mini-sprint execution packet for `#3995`-`#4001` is:

- [`LOGGING_COMPLETION_LEDGER_v0.91.6.md`](../LOGGING_COMPLETION_LEDGER_v0.91.6.md)
- [`CONTROL_PLANE_LOGGING_PROOF_3996.md`](../review/logging_observability/CONTROL_PLANE_LOGGING_PROOF_3996.md)
- [`RUNTIME_PROVIDER_LOGGING_PROOF_3997.md`](../review/logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3997.md)
- [`HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3998.md`](../review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3998.md)
- [`OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md`](../review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md)
- [`LOGGING_VALIDATION_REDACTION_PROOF_4000.md`](../review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md)
- [`GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md`](../review/logging_observability/GITHUB_TOKEN_RELEASE_PROJECTION_PROOF_4001.md)

This packet now includes the bounded `#4001` GitHub/token/release/projection
lane while preserving explicit non-claims for broader credential-manager and
existing-issue metadata-repair work.

Runtime observability completion is also scheduled explicitly in
[`RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md`](../RUNTIME_OBSERVABILITY_COMPLETION_SCHEDULE_v0.91.6.md)
so WP-03 does not overclaim provider-backed runtime or Observatory completion.

## v0.92 Consumption

`v0.92` may consume a bounded proof-loop contract after validator, GitHub,
logging, and card-projection failure modes are classified, with `SOR`-owned PR
publication truth completed for the first tranche.

For the logging-only execution slice, `v0.92` may consume:

- the shared channel-policy boundary
- the bounded runtime/provider action-log slice
- the bounded heartbeat/timeout/progress slice
- the redacted Observatory/Unity example packet

It may not consume a claim that production OpenTelemetry export, full
credential-manager unification, or broad existing-issue metadata repair is
already complete.

It also may not consume a claim that bounded `runtime-v2` logging proof equals
full provider/runtime observability completion. That broader completion band is
scheduled explicitly by `#3922` and deferred to later milestone homes where
required.

## Non-Goals

- No hidden fallback to deprecated `gh` paths.
- No broad rewrite of lifecycle tooling in this feature doc.
- No claim that all tooling debt is gone.
- No collapse of `SIP -> STP -> SPP -> SRP -> SOR` into one GitHub-facing text
  surface.
