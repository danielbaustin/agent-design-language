# Heartbeat, Timeout, And Progress Proof (`#3998`)

## Scope

Bounded proof for the long-running paths explicitly claimed by `v0.91.6`.

## Evidence Surfaces

- `adl/src/cli/observability.rs`
- `adl/src/cli/agent_cmd.rs`
- `adl/src/cli/pr_cmd/finish_support.rs`
- `adl/src/execute/support.rs`
- `adl/src/remote_exec/errors.rs`
- `docs/milestones/v0.91.5/review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3708.md`

## Covered Paths

| Path | Signal | Notes |
| --- | --- | --- |
| `agent` long-running CLI operations | `started` / `heartbeat` / terminal `completed|failed` | Implemented through `ProgressHeartbeat`. |
| `pr finish` validation subprocess wait | `started` / `heartbeat` / terminal classification | Uses `ProgressHeartbeat` and explicit validation-wait status. |
| runtime step execution | `STEP start` / `STEP done` stderr progress plus trace/action-log timing | Real operator-visible progress exists, but it is not yet normalized to `adl_event`. |
| remote/provider timeout classification | stable failure kinds such as `timeout` / `provider_error` | Distinguishes timeout from generic provider failure on the covered remote paths. |

## Focused Validation

- `bash adl/tools/test_control_plane_observability.sh`
- `cargo test --manifest-path adl/Cargo.toml cli::observability -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml finish_validation_emits_subprocess_heartbeat_and_classification -- --nocapture`

## Claimed Result

- Slow covered control-plane paths emit heartbeat before terminal completion.
- Fast covered operations remain quiet rather than spamming heartbeat lines.
- Timeout reasoning on covered provider/remote paths is stable enough to
  distinguish timeout from other provider failures.
- Runtime execution exposes bounded progress, even though not all of it is yet
  mapped into the shared `adl_event` vocabulary.

## Non-Claims

- This packet does not claim exhaustive heartbeat coverage for every command in
  the repository.
- This packet does not claim runtime step stderr progress is already fully
  converged with the shared control-plane event vocabulary.
