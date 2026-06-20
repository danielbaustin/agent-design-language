# Runtime ACIP-to-SNS Bridge Proof For `#4296`

## Status

- Issue: `#4296`
- Milestone: `v0.91.6`
- Status: `local_mock_proof`
- Scope: bounded ACIP projection and SNS mock-publisher proof against the
  `#4294` runtime AWS signal bridge contract
- Proof mode: focused Rust validation plus local/mock event inspection; no live
  AWS mutation

## What Landed

`#4296` extends the shared runtime AWS signal seam with the ACIP projection
lane described by `#4294`.

The landed slice is intentionally narrow:

- builds `adl.runtime.aws_signal.v1` envelopes with
  `signal_kind = acip_projection`
- keeps heartbeat and ACIP publication as separate lanes on the shared signal
  contract
- supports `disabled`, `mock`, and approval-gated `live` mode through the
  existing `ADL_AWS_SIGNAL_MODE` control surface
- writes reviewer-readable mock SNS projection envelopes to a local JSONL
  artifact
- refuses local-only ACIP routes instead of silently widening them into
  external delivery
- keeps live SNS delivery fail-closed until a later issue lands the real SNS
  transport and approved caller wiring

## Contract Alignment With `#4294`

The landed projection envelope preserves the `#4294` contract:

- `schema_version = adl.runtime.aws_signal.v1`
- `signal_kind = acip_projection`
- `heartbeat_seq = null`
- includes `runtime_id`, `agent_id`, `cycle_id`, `status`, `timestamp`,
  `capabilities`, `failure_class`, `correlation_id`, `projection_level`,
  `transport`, and `payload`
- uses `transport.target_kind = sns`
- preserves bounded ACIP projection payload fields only:
  - `message_kind`
  - `route_class`
  - `sender_class`
  - `recipient_class`
  - `delivery_outcome`
  - `message_ref`
  - `trace_ref`
- includes `summary` and `content_sha256` only for
  `projection_level = content_summary`
- keeps `projection_level = delivery_metadata` free of content-derived fields

Non-claims preserved:

- no live SNS mutation
- no runtime-wide ACIP event fanout claim
- no raw ACIP message body publication
- no credentials, topic ARNs, account ids, or private endpoints in proof
- no claim that SNS becomes ACIP authority

## Mode Behavior

### `mock`

- no AWS credentials required
- no live AWS mutation
- reviewer-readable JSONL artifact written at:
  `aws_acip_sns_projection_mock.jsonl`
- emits `stage=aws_acip_sns_projection result=completed`

### explicit `disabled`

- emits observable skip classification
- writes no mock artifact
- does not imply external delivery happened

### `live`

- requires explicit operator approval through `ADL_AWS_SIGNAL_APPROVED`
- requires explicit region and `ADL_AWS_SNS_TOPIC_ARN`
- currently fails closed with bounded observability instead of inventing an SNS
  client path
- does not widen the issue into live AWS bootstrapping or target creation

## Focused Validation

- `cargo fmt --manifest-path adl/Cargo.toml --all`
- `cargo test --manifest-path adl/Cargo.toml runtime_aws_signal -- --nocapture`
- `git diff --check`

## Tests Added / Covered

Focused proof now covers:

- mock ACIP projection writes a reviewer-readable SNS envelope artifact
- the emitted envelope carries the shared `#4294` schema/version and SNS target
- `correlation_id` is stable and `heartbeat_seq` stays `null` for ACIP
  projection
- `delivery_metadata` stays metadata-only
- `content_summary` stays bounded and does not leak raw ACIP message content
- local-only ACIP routes are rejected with `projection_denied`
- disabled mode stays quiet
- live mode without complete approved configuration stays fail-closed with
  `config_missing`

## Reviewer Read Path

1. Inspect `adl/src/runtime_aws_signal.rs` for the ACIP projection request,
   envelope builder, and SNS mock/live gating.
2. Run the focused validation commands above.
3. Inspect the mock artifact expectations in the new unit tests.

## Residuals / Follow-on Work

- A real SNS transport remains deferred.
- Runtime call-site wiring for approved ACIP projection events remains deferred;
  this issue lands the projection contract and mock publisher seam without
  inventing a broader event source.
- If live SNS publishing is later added, it must preserve:
  - fail-closed behavior
  - no secret or private-target leakage
  - tail-friendly observability
  - no widening from local-only ACIP routes into external delivery
