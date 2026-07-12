# Runtime v3 Weather, GPU, And CloudWatch Proof

Issue: #5252
Target: v0.91.7

## Result

#5252 retains the Runtime v3 control/weather proof as an accepted intentional
divergence from Runtime v2 internals. Runtime v3 keeps host-resource weather in
`adl-runtime-kernel/src/weather.rs`, uses `sysinfo` for portable CPU, memory,
disk, network, and temperature observations, and leaves remote export behavior
to Vector.

The machine-readable retained packet is:

```text
docs/architecture/runtime_v3_weather_cloudwatch_5252.v1.json
```

## Runtime Contract

`WeatherSample` records every observation with a source and an availability
state. GPU telemetry is not faked: when no approved GPU adapter or host is
available, Runtime v3 records `unavailable_not_pass`.

`WeatherHealthReport` records:

- `resource_state`
- `shutdown_decision`
- `gpu_proof_state`
- `cloudwatch_route`
- the retained `WeatherSample`

When resource pressure reaches a stop threshold, the shutdown decision is
`serialize_state_then_stop`. This keeps the local runtime behavior aligned with
the graceful-stop policy: serialize state while the process is still coherent,
then stop instead of crashing under local resource pressure.

## CloudWatch Boundary

Runtime v3 does not implement its own CloudWatch client or OpenTelemetry
collector. The checked-in Vector config in
`adl-runtime-kernel/vector/runtime-v3.yaml` parses stderr `adl_event` records and
shapes them into an EMF-compatible stream named
`vector.runtime_v3_cloudwatch_emf`. Deployment overlays own the managed
CloudWatch sink, credentials, buffering, retry, and retention policy.

## GPU Disposition

No approved GPU host was available in this issue lane. GPU telemetry therefore
remains a non-cutover proof surface for observed-GPU claims. The v0.91.7
cutover packet may rely on CPU, memory, disk, graceful-stop, and Vector boundary
proof from this issue, but it must not claim observed GPU telemetry until a
later approved GPU run replaces the deferred record.
