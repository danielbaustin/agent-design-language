# Runtime And Provider Action Logging Proof (`#3997`)

## Scope

Bounded proof for the runtime/provider action evidence that `v0.91.6` may
consume safely.

## Evidence Surfaces

- `adl/src/instrumentation/action_log.rs`
- `adl/src/cli/run_artifacts/runtime/writer.rs`
- `adl/src/provider_communication.rs`
- `docs/milestones/v0.91.5/RUNTIME_ACTION_LOG_CONTRACT_3556.md`
- `docs/milestones/v0.91.6/features/PROVIDER_MODEL_RELIABILITY_v0.91.6.md`

## Coverage Statement

The current bounded slice provides:

- trace-derived runtime `logs/action_log.jsonl` records
- provider route identity including provider kind, runtime surface,
  `provider_model_id`, and optional endpoint/credential refs
- request correlation through optional `run_id` and `request_id`
- classified attempt and final failure kinds such as
  `provider_auth_missing`, `provider_timeout`, `provider_rate_limited`,
  `local_runtime_busy`, and `provider_error`
- redacted output / excerpt posture instead of raw prompts, raw payloads, or
  secret material

## Why This Is Enough For WP-03

The logging mini-sprint does not need to invent a second telemetry system. The
implemented runtime action log and provider communication schemas already give
`v0.91.6` a bounded correlated slice:

- runtime stage/result/elapsed records
- provider/model identity
- request and run refs when available
- stable failure classification suitable for downstream review and routing

That is enough for the milestone’s provider/model and Observatory consumers to
work from a truthful baseline.

## Focused Validation

- `cargo test --manifest-path adl/Cargo.toml instrumentation::action_log -- --nocapture`

## Non-Claims

- This packet does not claim full end-to-end unification of runtime,
  provider, long-lived-agent, and control-plane telemetry.
- This packet does not claim every provider path writes the same durable
  artifact family.
- This packet does not claim raw provider transcripts are safe to publish.
