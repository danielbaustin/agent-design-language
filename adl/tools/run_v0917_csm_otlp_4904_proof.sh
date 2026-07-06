#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${1:-$ROOT/docs/milestones/v0.91.7/review/runtime/csm_otlp_4904}"
CSM_BIN="${CSM_BIN:-$ROOT/adl/target/debug/csm}"

if [ ! -x "$CSM_BIN" ]; then
  printf 'missing executable csm binary: %s\n' "$CSM_BIN" >&2
  exit 1
fi

rm -rf "$OUT"
mkdir -p "$OUT"/collector "$OUT"/service "$OUT"/state "$OUT"/logs

PORT_FILE="$OUT/collector/port"
RECEIVED="$OUT/collector/received_otlp_http_json.jsonl"
COLLECTOR_STATUS="$OUT/collector/collector_status.json"

python3 - "$PORT_FILE" "$RECEIVED" "$COLLECTOR_STATUS" <<'PY' &
import http.server
import json
import sys
import time

port_file, received_path, status_path = sys.argv[1:4]

class Handler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("content-length", "0"))
        body = self.rfile.read(length).decode("utf-8")
        try:
            validate_otlp_json_proto(json.loads(body))
        except Exception as exc:
            self.send_response(400)
            self.send_header("content-length", str(len(str(exc))))
            self.end_headers()
            self.wfile.write(str(exc).encode("utf-8"))
            return
        with open(received_path, "a", encoding="utf-8") as f:
            f.write(body.replace("\n", " ") + "\n")
        self.send_response(200)
        self.send_header("content-length", "2")
        self.end_headers()
        self.wfile.write(b"OK")

    def log_message(self, *_args):
        return

def validate_otlp_json_proto(payload):
    resource_spans = payload.get("resourceSpans")
    if not isinstance(resource_spans, list) or not resource_spans:
        raise ValueError("missing resourceSpans")
    for resource_span in resource_spans:
        resource = resource_span.get("resource", {})
        attrs = resource.get("attributes", [])
        if not isinstance(attrs, list):
            raise ValueError("resource.attributes must be an array")
        scope_spans = resource_span.get("scopeSpans")
        if not isinstance(scope_spans, list) or not scope_spans:
            raise ValueError("missing scopeSpans")
        for scope_span in scope_spans:
            spans = scope_span.get("spans")
            if not isinstance(spans, list) or not spans:
                raise ValueError("missing spans")
            for span in spans:
                if not isinstance(span.get("name"), str):
                    raise ValueError("span.name must be a string")
                if not is_hex(span.get("traceId"), 32):
                    raise ValueError("span.traceId must be 32 lowercase hex chars")
                if not is_hex(span.get("spanId"), 16):
                    raise ValueError("span.spanId must be 16 lowercase hex chars")
                if "parentSpanId" in span and not is_hex(span.get("parentSpanId"), 16):
                    raise ValueError("span.parentSpanId must be 16 lowercase hex chars")
                if not str(span.get("startTimeUnixNano", "")).isdigit():
                    raise ValueError("span.startTimeUnixNano must be numeric")
                if not str(span.get("endTimeUnixNano", "")).isdigit():
                    raise ValueError("span.endTimeUnixNano must be numeric")
                if not isinstance(span.get("attributes", []), list):
                    raise ValueError("span.attributes must be an array")

def is_hex(value, length):
    return isinstance(value, str) and len(value) == length and all(ch in "0123456789abcdef" for ch in value)

server = http.server.HTTPServer(("127.0.0.1", 0), Handler)
endpoint = f"http://127.0.0.1:{server.server_port}/v1/traces"
with open(port_file, "w", encoding="utf-8") as f:
    f.write(endpoint)
with open(status_path, "w", encoding="utf-8") as f:
    json.dump({
        "schema": "adl.csm.otlp_loopback_collector_status.v1",
        "collector": "python_stdlib_loopback_http",
        "endpoint_ref": "<loopback>",
        "state": "listening"
    }, f, indent=2)
    f.write("\n")
deadline = time.time() + 10
last_request = None
server.timeout = 0.25
while time.time() < deadline:
    server.handle_request()
    try:
        with open(received_path, "r", encoding="utf-8") as f:
            lines = [line for line in f if line.strip()]
        if lines:
            last_request = time.time()
    except FileNotFoundError:
        pass
    if last_request and time.time() - last_request > 0.75:
        break
with open(status_path, "w", encoding="utf-8") as f:
    json.dump({
        "schema": "adl.csm.otlp_loopback_collector_status.v1",
        "collector": "python_stdlib_loopback_http",
        "endpoint_ref": "<loopback>",
        "state": "completed",
        "received_request_count": sum(1 for _ in open(received_path, encoding="utf-8")) if __import__("os").path.exists(received_path) else 0
    }, f, indent=2)
    f.write("\n")
PY
COLLECTOR_PID=$!

for _ in $(seq 1 80); do
  if [ -s "$PORT_FILE" ]; then
    break
  fi
  sleep 0.05
done

if [ ! -s "$PORT_FILE" ]; then
  kill "$COLLECTOR_PID" 2>/dev/null || true
  printf 'collector did not publish endpoint\n' >&2
  exit 1
fi

ENDPOINT="$(cat "$PORT_FILE")"
SPEC="$OUT/agent.yaml"
cat > "$SPEC" <<'YAML'
schema: adl.long_lived_agent_spec.v1
agent_instance_id: csm-otlp-4904
display_name: CSM OTLP 4904 Proof Agent
state_root: state
workflow:
  kind: demo_adapter
  name: csm_otlp_4904_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/csm-otlp-4904
  write_policy: append_only
YAML

(
  cd "$OUT"
  ADL_OBSERVABILITY_STDERR=0 \
  ADL_OBSERVABILITY_LOG="$OUT/logs/observability.log" \
  ADL_OBSERVABILITY_REPO_ROOT="$ROOT" \
  ADL_OTEL_LOG="$OUT/logs/otel.jsonl" \
  ADL_OTEL_STATUS="$OUT/logs/otel_status.json" \
  ADL_OTEL_EXPORTER_OTLP_ENDPOINT="$ENDPOINT" \
  ADL_OTEL_EXPORTER_TIMEOUT_MS=2000 \
  "$CSM_BIN" daemon --spec "$SPEC" --max-restarts 1 --checkpoint-interval-secs 1 --no-sleep --json \
    > "$OUT/logs/csm_stdout.json" \
    2> "$OUT/logs/csm_stderr.log"
)

wait "$COLLECTOR_PID"

python3 - "$ROOT" "$OUT" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
out = pathlib.Path(sys.argv[2])
received = out / "collector" / "received_otlp_http_json.jsonl"
events = [json.loads(line) for line in received.read_text(encoding="utf-8").splitlines() if line.strip()]
status = json.loads((out / "logs" / "otel_status.json").read_text(encoding="utf-8"))
summary = {
    "schema": "adl.csm.otlp_4904_proof_summary.v1",
    "runtime_owner": "csm",
    "proof_classification": "proving",
    "collector_mode": "loopback_otlp_http_json_proto",
    "exported_request_count": len(events),
    "exporter_status": status.get("exporter", {}),
    "local_fallbacks_retained": ["logs/otel.jsonl", "logs/otel_status.json", "logs/observability.log"],
    "semantics_claimed": {
        "traces": True,
        "spans": True,
        "events": True,
        "metrics": False,
        "resources": True,
        "trace_and_span_ids": True,
        "service_name": "csm-runtime-daemon"
    },
    "negative_cases_retained": {
        "malformed_endpoint": "covered by cli::observability::tests::otlp_exporter_malformed_endpoint_fails_closed_with_durable_status",
        "collector_unavailable": "covered by exporter failure status path; no hosted backend claimed",
        "redaction_sensitive_fields": "covered by loopback exporter unit test and retained ADL_OBSERVABILITY_REPO_ROOT path sanitization"
    },
    "non_claims": [
        "does not claim hosted SaaS telemetry",
        "does not claim protobuf OTLP/gRPC",
        "does not claim metrics export beyond explicit false metric semantics"
    ]
}
(out / "proof_summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
readme = """# CSM OTLP Export Proof (#4904)

This packet retains a bounded CSM runtime proof for `ADL_OTEL_EXPORTER_OTLP_ENDPOINT`.

Evidence:
- `collector/received_otlp_http_json.jsonl` contains loopback collector POST payloads emitted by `csm daemon`; the collector validates OTLP HTTP JSON mapping shape before returning HTTP 200.
- `logs/otel.jsonl` and `logs/otel_status.json` retain the local fallback/status path.
- `logs/observability.log` retains human `adl_event` compatibility evidence outside stdout.
- `state/` contains daemon status, continuity checkpoint, replay, and operator events from the same run.

Truth boundary: this proves local OTLP HTTP JSON-proto collector export for CSM daemon runtime events. It does not claim hosted telemetry, protobuf OTLP/gRPC, or metrics export.
"""
(out / "README.md").write_text(readme, encoding="utf-8")
PY

printf 'wrote CSM OTLP proof packet: %s\n' "$OUT"
