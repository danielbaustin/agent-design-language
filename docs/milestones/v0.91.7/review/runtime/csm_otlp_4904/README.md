# CSM OTLP Export Proof (#4904)

This packet retains a bounded CSM runtime proof for `ADL_OTEL_EXPORTER_OTLP_ENDPOINT`.

Evidence:
- `collector/received_otlp_http_json.jsonl` contains loopback collector POST payloads emitted by `csm daemon`; the collector validates OTLP HTTP JSON mapping shape before returning HTTP 200.
- `logs/otel.jsonl` and `logs/otel_status.json` retain the local fallback/status path.
- `logs/observability.log` retains human `adl_event` compatibility evidence outside stdout.
- `state/` contains daemon status, continuity checkpoint, replay, and operator events from the same run.

Truth boundary: this proves local OTLP HTTP JSON-proto collector export for CSM daemon runtime events. It does not claim hosted telemetry, protobuf OTLP/gRPC, or metrics export.
