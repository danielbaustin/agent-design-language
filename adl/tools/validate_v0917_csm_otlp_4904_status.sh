#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROOF="$ROOT/docs/milestones/v0.91.7/review/runtime/csm_otlp_4904"

python3 - "$PROOF" <<'PY'
import json
import pathlib
import sys

proof = pathlib.Path(sys.argv[1])
required = [
    "README.md",
    "proof_summary.json",
    "collector/collector_status.json",
    "collector/received_otlp_http_json.jsonl",
    "logs/observability.log",
    "logs/otel.jsonl",
    "logs/otel_status.json",
    "logs/csm_stdout.json",
    "state/daemon_status.json",
    "state/continuity_checkpoint.json",
    "state/continuity_replay_manifest.json",
    "state/operator_events.jsonl",
]
missing = [path for path in required if not (proof / path).exists()]
if missing:
    raise SystemExit(f"missing proof artifacts: {missing}")

summary = json.loads((proof / "proof_summary.json").read_text(encoding="utf-8"))
if summary.get("schema") != "adl.csm.otlp_4904_proof_summary.v1":
    raise SystemExit("proof_summary schema drift")
if summary.get("runtime_owner") != "csm":
    raise SystemExit("runtime_owner must remain csm")
if summary.get("proof_classification") != "proving":
    raise SystemExit("proof must remain classified as proving")
if summary.get("collector_mode") != "loopback_otlp_http_json_proto":
    raise SystemExit("collector mode must remain loopback_otlp_http_json_proto")
if summary.get("exported_request_count", 0) < 4:
    raise SystemExit("expected at least four exported OTLP requests")
semantics = summary.get("semantics_claimed", {})
for key in ["traces", "spans", "events", "resources", "trace_and_span_ids"]:
    if semantics.get(key) is not True:
        raise SystemExit(f"missing claimed OTLP semantic: {key}")
if semantics.get("metrics") is not False:
    raise SystemExit("metrics must remain an explicit non-claim")
if semantics.get("service_name") != "csm-runtime-daemon":
    raise SystemExit("unexpected service.name claim")

status = json.loads((proof / "logs/otel_status.json").read_text(encoding="utf-8"))
if status.get("schema") != "adl.otel.monitor_status.v1":
    raise SystemExit("otel status schema drift")
exporter = status.get("exporter") or {}
if exporter.get("schema") != "adl.otel.exporter_status.v1":
    raise SystemExit("missing exporter status schema")
if exporter.get("protocol") != "otlp_http_json":
    raise SystemExit("exporter protocol drift")
if exporter.get("status") != "success":
    raise SystemExit("exporter did not report success")
if exporter.get("http_status") != 200:
    raise SystemExit("exporter did not retain HTTP 200 proof")
if exporter.get("endpoint") != "<configured>":
    raise SystemExit("exporter endpoint must be redacted in status")

collector = json.loads((proof / "collector/collector_status.json").read_text(encoding="utf-8"))
if collector.get("schema") != "adl.csm.otlp_loopback_collector_status.v1":
    raise SystemExit("collector status schema drift")
if collector.get("state") != "completed":
    raise SystemExit("collector did not complete")
if collector.get("received_request_count", 0) != summary.get("exported_request_count"):
    raise SystemExit("collector request count does not match proof summary")

payloads = [
    json.loads(line)
    for line in (proof / "collector/received_otlp_http_json.jsonl").read_text(encoding="utf-8").splitlines()
    if line.strip()
]

def attr_strings(attrs, key):
    found = []
    if not isinstance(attrs, list):
        raise SystemExit("OTLP attributes must be arrays")
    for attr in attrs:
        if attr.get("key") == key:
            value = (attr.get("value") or {}).get("stringValue")
            if isinstance(value, str):
                found.append(value)
    return found

def is_hex(value, length):
    return (
        isinstance(value, str)
        and len(value) == length
        and all(ch in "0123456789abcdef" for ch in value)
    )

names = set()
service_names = set()
trace_ids = set()
for payload in payloads:
    for resource_span in payload.get("resourceSpans", []):
        service_names.update(
            attr_strings((resource_span.get("resource") or {}).get("attributes", []), "service.name")
        )
        for scope_span in resource_span.get("scopeSpans", []):
            for span in scope_span.get("spans", []):
                names.add(span.get("name"))
                trace_id = span.get("traceId")
                span_id = span.get("spanId")
                parent_span_id = span.get("parentSpanId")
                if not is_hex(trace_id, 32):
                    raise SystemExit("span traceId is not 32-char OTLP hex")
                if not is_hex(span_id, 16):
                    raise SystemExit("span spanId is not 16-char OTLP hex")
                if parent_span_id is not None and not is_hex(parent_span_id, 16):
                    raise SystemExit("span parentSpanId is not 16-char OTLP hex")
                if not str(span.get("startTimeUnixNano", "")).isdigit():
                    raise SystemExit("span startTimeUnixNano is not numeric")
                if not str(span.get("endTimeUnixNano", "")).isdigit():
                    raise SystemExit("span endTimeUnixNano is not numeric")
                if not isinstance(span.get("attributes", []), list):
                    raise SystemExit("span attributes must be an OTLP attribute array")
                trace_ids.add(trace_id)
if "csm.daemon_started" not in names or "csm.checkpoint_write" not in names:
    raise SystemExit("missing daemon lifecycle/checkpoint exported spans")
if "csm-runtime-daemon" not in service_names:
    raise SystemExit("missing csm-runtime-daemon resource service.name")
if not trace_ids:
    raise SystemExit("missing CSM daemon trace id")

daemon_status = json.loads((proof / "state/daemon_status.json").read_text(encoding="utf-8"))
if daemon_status.get("schema") != "adl.long_lived_agent_daemon_status.v1":
    raise SystemExit("daemon status schema drift")
if daemon_status.get("runtime_capabilities", {}).get("runtime_owner") != "csm":
    raise SystemExit("daemon runtime owner drift")
if daemon_status.get("state") != "completed":
    raise SystemExit("daemon proof did not complete")

bad_markers = [
    "/Users/",
    "/private/tmp/",
    "/var/folders/",
    "api_key",
    "api-key",
    "secret",
    "token",
]
for path in proof.rglob("*"):
    if not path.is_file():
        continue
    text = path.read_text(encoding="utf-8", errors="ignore")
    if any(marker in text for marker in bad_markers):
        raise SystemExit(f"proof hygiene marker found in {path.relative_to(proof)}")

print("validate_v0917_csm_otlp_4904_status: PASS")
PY
