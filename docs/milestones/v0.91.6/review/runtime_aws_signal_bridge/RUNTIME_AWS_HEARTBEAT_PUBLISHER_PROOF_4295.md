# Runtime AWS Heartbeat Publisher Proof For `#4295`

## Status

- Issue: `#4295`
- Milestone: `v0.91.6`
- Status: `local_mock_proof`
- Scope: bounded runtime heartbeat publication proof against the `#4294`
  runtime AWS signal bridge contract
- Proof mode: focused Rust validation plus local/mock event inspection; no live
  AWS mutation

## What Landed

`#4295` adds the first runtime heartbeat publication slice for the long-lived
agent runtime surface.

The landed behavior is intentionally narrow:

- emits `adl.runtime.aws_signal.v1` heartbeat envelopes from long-lived-agent
  status writes
- supports three operator-facing modes through `ADL_AWS_SIGNAL_MODE`:
  - `mock`
  - `live`
  - explicit `disabled`
- keeps the unconfigured default quiet instead of emitting synthetic AWS noise
- writes mock heartbeat envelopes to a local reviewer-readable JSONL artifact
- allocates monotonic heartbeat sequence numbers from a local cursor artifact so
  later heartbeats advance identity instead of collapsing into retry lookalikes
- keeps `live` fail-closed and observable until a later issue lands the real
  CloudWatch transport

## Contract Alignment With `#4294`

The emitted heartbeat envelope preserves the `#4294` contract:

- `schema_version = adl.runtime.aws_signal.v1`
- `signal_kind = heartbeat`
- includes `runtime_id`, `agent_id`, `cycle_id`, `heartbeat_seq`, `status`,
  `timestamp`, `capabilities`, `failure_class`, `correlation_id`,
  `projection_level`, `transport`, and `payload`
- uses `projection_level = operations_safe`
- keeps the payload to bounded operational fields only:
  - `state`
  - `elapsed_ms`
  - `next_cycle_hint`
  - `stop_requested`
  - `lease_state`

Non-claims preserved:

- no ACIP-to-SNS implementation in this issue
- no raw private state publication
- no provider prompts/responses
- no AWS account ids, private ARNs, or credentials in committed proof artifacts
- no live CloudWatch mutation claim

## Mode Behavior

### Unconfigured default

- no AWS heartbeat publication path is activated
- no mock artifact is written
- no live configuration is inferred

### `mock`

- no AWS credentials required
- no live AWS mutation
- reviewer-readable JSONL artifact written at:
  `state/aws_runtime_heartbeat_mock.jsonl`
- monotonic sequence cursor written at:
  `state/aws_runtime_heartbeat_cursor.json`
- observability emits `stage=aws_runtime_heartbeat result=completed`

### `live`

- requires explicit operator approval through `ADL_AWS_SIGNAL_APPROVED`
- requires explicit region/target/log-group/log-stream configuration
- currently fails closed with observable `adl_event` output instead of
  inventing a CloudWatch client path
- does not make the underlying runtime cycle pretend publication succeeded

### explicit `disabled`

- emits observable skip classification
- does not require AWS credentials or write the mock artifact

## Focused Validation

- `cargo fmt --manifest-path adl/Cargo.toml --all`
  - verified formatting for the bounded Rust changes
- `cargo test --manifest-path adl/Cargo.toml long_lived_agent -- --nocapture`
  - verified the long-lived-agent lane plus the new runtime AWS heartbeat
    publisher tests

## Tests Added / Covered

Focused proof now covers:

- mock mode writes reviewer-readable runtime AWS heartbeat envelopes
- emitted envelopes carry the `#4294` schema/version and bounded heartbeat
  payload fields
- live mode without explicit approval stays fail-closed
- unsupported heartbeat targets are rejected instead of being written into mock
  proof artifacts
- live-blocked mode remains observable through `adl_event`
- live-blocked mode does not leak private ARN/account strings into the
  observability log
- repeated heartbeats for the same cycle advance `heartbeat_seq` and
  `correlation_id`
- live-blocked mode does not downgrade the underlying runtime cycle into a fake
  external-delivery success claim

## Reviewer Read Path

1. Inspect `adl/src/runtime_aws_signal.rs` for mode parsing, envelope shaping,
   and fail-closed live gating.
2. Inspect `adl/src/long_lived_agent/storage.rs` to confirm publication is
   hooked to status writes rather than a separate synthetic timer.
3. Inspect `adl/src/long_lived_agent/tests.rs` for the mock and live-blocked
   proof cases.
4. Run the focused validation commands above.

## Residuals / Follow-on Work

- Real CloudWatch Logs transport remains deferred; this issue only stages the
  contract seam and fail-closed live gate.
- Downstream SNS projection remains owned by `#4296`.
- If live transport is later added, it must preserve:
  - no secret/account/ARN leakage
  - fail-closed behavior
  - tail-friendly observability
  - non-authoritative AWS posture
