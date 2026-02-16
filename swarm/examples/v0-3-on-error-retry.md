# v0.3 On-Error and Retry Example

File:
- `swarm/examples/v0-3-on-error-retry.adl.yaml`

## What it demonstrates

- `on_error: "continue"` allows a failed step to be recorded while execution proceeds.
- `retry.max_attempts` retries a step deterministically with no backoff.

## Run

From repo root:

```bash
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-on-error-retry.adl.yaml --print-plan
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-on-error-retry.adl.yaml --run
```

## Expected behavior

- Default remains fail-fast when `on_error` is not set.
- `step_continue` may fail and execution continues to `step_retry`.
- `step_retry` can attempt up to `max_attempts` and reports final status in run summary.
