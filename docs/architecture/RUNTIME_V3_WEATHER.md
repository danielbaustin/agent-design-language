# Runtime V3 Weather

Runtime v3 now has a small host-resource weather report in
`adl-runtime/src/weather.rs`.

The weather surface uses the `sysinfo` crate for CPU, memory, and disk readings.
It does not implement a custom monitoring backend. CloudWatch delivery remains
owned by the Vector observability component; the weather module only emits a
bounded CloudWatch EMF-compatible JSON event for that pipeline.

The proof surface is the runtime module and its focused contract tests. There
is no standalone daemon or separate monitoring process.

## Stop Policy

The default graceful-stop thresholds are:

- CPU usage at or above 99 percent.
- Memory usage at or above 95 percent.
- Disk usage at or above 97 percent.

When a threshold is crossed, the report health becomes
`graceful_stop_required` and the shutdown decision is
`serialize_state_then_stop`. This preserves the runtime state before local
resource pressure can turn into a crash.

## GPU Proof

Local GPU telemetry is reported as `deferred` when no approved GPU host is
available. That is an explicit proof state, not a pass. A later GPU-host run can
replace the deferred state with observed GPU telemetry without changing the CPU,
memory, disk, CloudWatch-shape, or serialization contracts.
