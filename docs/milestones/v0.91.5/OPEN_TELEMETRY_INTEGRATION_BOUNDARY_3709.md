# OpenTelemetry Integration Boundary (#3709)

Issue: #3709  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: reviewed_boundary_decision

## Purpose

Define the truthful OpenTelemetry integration boundary for ADL logging and
observability without making local development, CI, or deterministic proof
paths depend on a collector or hosted telemetry service.

## Current State

ADL already ships the local observability primitives that should remain the
authoritative source of truth:

- terminal `adl_event` control-plane emission for shell and Rust CLI paths;
- runtime durable action-log projection in `logs/action_log.jsonl`;
- provider and review-provider JSONL logs;
- long-lived-agent ledgers, status files, and operator event streams;
- tracked review packets that cite those local artifacts.

`adl/Cargo.toml` already includes:

- `tracing`
- `tracing-subscriber`

The repo does not currently include:

- `opentelemetry`
- `opentelemetry_sdk`
- `tracing-opentelemetry`
- OTLP or hosted exporter crates

That is intentional in the current state. OTEL is still an integration
boundary, not a local runtime requirement.

## Decision

`#3709` closes the OpenTelemetry question as a bounded boundary decision:

1. keep the existing local ADL observability artifacts authoritative;
2. keep OTEL export optional and disabled by default;
3. do not add OTEL exporter crates in this issue;
4. define the exact feature-flag and configuration shape future implementation
   must use when exporter wiring is actually added.

This issue therefore resolves the dependency and policy surface without
pretending OTEL export is already implemented.

## Dependency Decision

### Keep Now

Keep the current always-on local dependencies:

- `tracing`
- `tracing-subscriber`

These are sufficient for:

- local subscriber setup;
- continuing the current local `tracing`/`tracing-subscriber` posture without
  adding collector/export dependencies in this issue;
- future span/event emission convergence;
- a clean later bridge to OTEL without forcing that bridge into every binary
  today.

### Do Not Add In This Issue

Do not add these crates in `#3709`:

- `opentelemetry`
- `opentelemetry_sdk`
- `tracing-opentelemetry`
- `opentelemetry-otlp`
- collector-specific transport crates

Reason:

- no current binary requires collector-backed export to satisfy the logging
  sprint acceptance surface;
- adding those crates now would widen compile and dependency surface before the
  Observatory consumption contract is finalized in `#3710`;
- local and CI proof for this sprint only requires a truthful OTEL-ready
  boundary, not exporter implementation.

## Required Feature-Flag Shape For Future Implementation

When OTEL exporter wiring is introduced later, it must remain behind explicit
opt-in features. The required shape is:

- base behavior with no OTEL features:
  - local stderr or compatibility-log observability only;
  - no collector dependency;
  - no network export attempt;
  - deterministic local and CI behavior preserved.
- optional feature family:
  - one bounded ADL feature gate such as `otel-export`
  - optional exporter-specific subfeature(s) only if needed later
  - no hidden activation from environment variables alone

Required rule:

- environment variables may configure an enabled exporter, but must not enable
  OTEL export when the feature gate is absent.

## Exporter Configuration Policy

### Local Development

- default: no OTEL exporter
- allowed proving modes:
  - normal local stderr observability
  - compatibility mirror file via `ADL_OBSERVABILITY_LOG`
  - explicit local stdout/no-op subscriber proof when needed
- forbidden default:
  - automatic collector dialing
  - hidden hosted telemetry dependency

### CI

- CI must pass with no collector present
- CI may validate OTEL-disabled code paths and configuration guards
- OTEL-enabled tests, if added later, must be bounded, opt-in, and clearly
  separated from ordinary default CI

### Runtime And Long-Lived Processes

- long-running runtime or Observatory processes may later enable richer spans
  and optional exporter wiring
- those processes must still preserve local durable ADL artifacts
- exporter failure must not become the sole runtime truth surface

### Observatory Context

- OTEL, if later enabled, is for export and integration
- Observatory should continue to consume ADL-governed local artifacts and
  mapped shared-vocabulary events rather than depending exclusively on OTEL
  transport

## Required Mapping Boundary

The existing shared contract in
`docs/milestones/v0.91.5/SHARED_OBSERVABILITY_AND_OTEL_CONTRACT_3705.md`
remains the mapping authority.

Future OTEL export must derive from those shared fields, including:

- `command` / `component`
- `stage` / `operation`
- `result`
- `reason_code`
- `issue_ref`, `pr_ref`, `run_id`, `request_id`, `cycle_id`
- `elapsed_ms` / `duration_ms`
- `artifact_ref`
- `provider_model_id`
- `runtime_surface`

OTEL output must not redefine those meanings or become a competing truth model.

## Redaction And Privacy Boundary

Any future OTEL export must inherit the same redaction rules already required
for ADL observability:

- no raw prompts
- no raw provider payloads
- no credentials or secret markers
- no private tool arguments
- no host-local absolute paths
- no unbounded stderr/stdout blobs

If a field cannot satisfy those rules safely, it must stay out of OTEL export
and remain in a separately governed local artifact if needed.

## Validation Requirements For This Decision

This issue is complete when all of the following are true:

- `adl/Cargo.toml` truthfully shows only `tracing` and `tracing-subscriber`
  today
- the shared observability contract remains the OTEL mapping authority
- this issue no longer overclaims OTEL implementation or exporter readiness
- the future feature-flag/export policy is explicit enough that later issues do
  not need to guess

## Non-Claims

This issue does not claim:

- OTEL exporter implementation is complete
- a collector is required or configured
- every binary already emits spans through `tracing`
- OTEL output is the canonical ADL observability surface
- hosted observability services are required for ADL operation

## Follow-On Boundary

- `#3710` defines how Observatory consumes ADL logging and observability
  surfaces using the shared contract.
- Any future exporter implementation must open a new bounded issue rather than
  silently widening `#3709`.
