#!/usr/bin/env python3
"""Validate retained #5039 governed CSM API Gateway bridge proof."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0917_csm_api_gateway_bridge_proof.py <api_gateway_bridge_summary.json>")
    path = Path(sys.argv[1])
    summary = json.loads(path.read_text())

    expected = {
        "schema": "adl.csm.api_gateway_bridge_proof.v1",
        "issue": 5039,
        "status": "bounded_smoke",
        "aws_profile": "agent-logic-admin",
        "aws_region": "us-west-2",
    }
    for key, value in expected.items():
        if summary.get(key) != value:
            fail(f"{key} mismatch: expected {value!r}, got {summary.get(key)!r}")

    account_hash = summary.get("aws_account_hash")
    if not isinstance(account_hash, str) or len(account_hash) != 16 or account_hash.isdigit():
        fail("aws_account_hash must be a 16-character non-numeric redacted hash")

    polis_ingress = summary.get("polis_ingress", {})
    polis_hash = polis_ingress.get("polis_id_hash")
    if not isinstance(polis_hash, str) or len(polis_hash) != 16:
        fail("polis_ingress.polis_id_hash must be a 16-character hash")
    if polis_ingress.get("ingress_model") != "one_api_gateway_api_per_polis":
        fail("polis_ingress.ingress_model must be one_api_gateway_api_per_polis")
    if polis_ingress.get("route_target") != "authorized_api_gateway_to_csm_loopback_runtime_api":
        fail("polis_ingress.route_target must route to the governed CSM loopback runtime API")
    if polis_ingress.get("per_polis_api") is not True:
        fail("polis_ingress.per_polis_api must be true")
    if polis_ingress.get("runtime_identity_verified") is not True:
        fail("polis_ingress.runtime_identity_verified must be true")

    api = summary.get("api_gateway", {})
    if int(api.get("api_count", 0)) < 1:
        fail("api_gateway.api_count must prove at least one API")
    for key in ["selected_api_id_hash", "selected_api_name_hash", "selected_stage_name_hash"]:
        value = api.get(key)
        if not isinstance(value, str) or len(value) != 16:
            fail(f"api_gateway.{key} must be a 16-character hash")
    if api.get("selected_protocol_type") not in {"HTTP", "REST", "WEBSOCKET"}:
        fail("api_gateway.selected_protocol_type must be a known API Gateway protocol")
    required_routes = [
        "GET /status",
        "GET /health",
        "GET /ready",
        "GET /metrics",
        "GET /events",
        "GET /chronosense",
        "GET /weather",
        "GET /shepherd",
        "GET /cav",
        "GET /curiosity",
        "GET /acip",
        "GET /freedom-gate",
        "GET /reasoning",
        "GET /api-gateway-bridge",
        "GET /constructability",
        "GET /persistence",
    ]
    for route in required_routes:
        if route not in api.get("supported_route_keys", []):
            fail(f"api_gateway.supported_route_keys missing {route}")
    if "$default" in api.get("supported_route_keys", []):
        fail("api_gateway.supported_route_keys must not substitute $default for named routes")
    if "GET /acip" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /acip route truth")
    if "GET /acip/ws" in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must not claim inactive /acip/ws Gateway routing")
    if "GET /persistence" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /persistence route truth")
    if "GET /chronosense" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /chronosense route truth")
    if "GET /weather" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /weather route truth")
    if "GET /freedom-gate" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /freedom-gate route truth")
    if "GET /reasoning" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /reasoning route truth")
    if "GET /cav" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /cav route truth")
    if "GET /constructability" not in api.get("planned_route_keys", []):
        fail("api_gateway.planned_route_keys must retain planned /constructability route truth")
    if int(api.get("route_target_count", 0)) < 1:
        fail("api_gateway.route_target_count must prove API Gateway route targets")
    if int(api.get("integration_count", 0)) < 1:
        fail("api_gateway.integration_count must prove API Gateway integrations")
    integration_hashes = api.get("integration_target_hashes", [])
    if not isinstance(integration_hashes, list) or not integration_hashes:
        fail("api_gateway.integration_target_hashes must retain redacted integration targets")
    for value in integration_hashes:
        if not isinstance(value, str) or len(value) != 16:
            fail("api_gateway.integration_target_hashes entries must be 16-character hashes")

    bridge = summary.get("bridge", {})
    if not re.fullmatch(r"csm-5039-[0-9a-f]{16}", str(bridge.get("correlation_id", ""))):
        fail("bridge.correlation_id must be a csm-5039 scoped redacted correlation id")
    if bridge.get("endpoint") != "/api-gateway-bridge":
        fail("bridge.endpoint must be /api-gateway-bridge")
    if bridge.get("http_status") != 200:
        fail("bridge.http_status must prove HTTP 200")
    if bridge.get("response_schema") != "adl.csm.runtime_api.api_gateway_bridge.v1":
        fail("bridge.response_schema must be the CSM runtime API Gateway bridge schema")
    if bridge.get("runtime_owner") != "csm":
        fail("bridge.runtime_owner must be csm")
    if bridge.get("redacted_payload_ref") != "redacted_api_gateway_bridge_payload.json":
        fail("bridge.redacted_payload_ref mismatch")

    observability = summary.get("observability", {})
    for key in ["cloudwatch_log_group_hash", "eventbridge_bus_hash"]:
        value = observability.get(key)
        if not isinstance(value, str) or len(value) != 16:
            fail(f"observability.{key} must be a 16-character hash")
    if observability.get("cloudwatch_correlation_observed") is not True:
        fail("CloudWatch proof must retain the correlation id")
    if int(observability.get("cloudwatch_event_count", 0)) < 1:
        fail("CloudWatch proof must retain at least one correlated event count")
    if int(observability.get("eventbridge_rule_count", 0)) < 1:
        fail("EventBridge proof must retain at least one bridge rule")

    event_schema = summary.get("event_schema", {})
    if event_schema.get("schema") != "adl.csm.api_gateway_bridge.event.v1":
        fail("event_schema.schema mismatch")
    for event_kind in [
        "success",
        "denied",
        "throttled",
        "malformed_request",
        "upstream_failure",
        "degraded_csm_state",
    ]:
        if event_kind not in event_schema.get("event_kinds", []):
            fail(f"event_schema missing {event_kind}")

    negative = summary.get("negative_case_policy", {})
    if negative.get("missing_token") != "api_gateway_authorization_denied":
        fail("negative_case_policy.missing_token mismatch")
    live_negative = summary.get("live_negative_cases", {})
    if live_negative.get("missing_token") != "api_gateway_authorization_denied":
        fail("live missing-token negative case must classify authorization denial")
    if live_negative.get("missing_token_http_status") not in {401, 403}:
        fail("live missing-token negative case must retain an HTTP 401 or 403")
    if live_negative.get("malformed_request") != "api_gateway_malformed_request":
        fail("live malformed-token negative case must classify malformed request")
    if live_negative.get("malformed_request_http_status") not in {401, 403}:
        fail("live malformed-token negative case must retain an HTTP 401 or 403")
    if live_negative.get("malformed_request_error_class") != "api_gateway_malformed_request":
        fail("live malformed-token negative case must retain its exact error class")
    if live_negative.get("raw_error_recorded") is not False:
        fail("live negative case must not retain raw provider error")

    probes = summary.get("live_route_probes", {})
    if probes.get("default_route_is_not_substituted") is not True:
        fail("live_route_probes must prove $default was not substituted")
    for key in ["required_routes", "probed", "missing"]:
        if not isinstance(probes.get(key), list):
            fail(f"live_route_probes.{key} must be retained as a list")
    if probes["required_routes"] != required_routes:
        fail("live_route_probes.required_routes must match the runtime API Gateway route authority")
    if probes["missing"]:
        fail(f"live_route_probes.missing must be empty for retained proof: {probes['missing']!r}")
    probed_routes = [entry.get("route") for entry in probes["probed"] if isinstance(entry, dict)]
    for route in required_routes:
        if route not in probed_routes:
            fail(f"live_route_probes.probed missing {route}")

    policy = summary.get("local_csm_api_policy", {})
    if policy.get("embedded_daemon_api") != "loopback_only":
        fail("local CSM API policy must remain loopback_only")
    if policy.get("runtime_api_path") != "/api-gateway-bridge":
        fail("local CSM API policy must retain the runtime-owned /api-gateway-bridge path")
    if policy.get("direct_public_daemon_bind") is not False:
        fail("direct public daemon bind must remain false")
    if policy.get("per_polis_api_gateway") is not True:
        fail("local CSM API policy must require a per-polis API Gateway")
    if policy.get("polis_id_hash") != polis_hash:
        fail("local CSM API policy polis hash must match polis_ingress")

    redaction = summary.get("redaction", {})
    for key in [
        "raw_account_id_recorded",
        "raw_api_id_recorded",
        "raw_invoke_url_recorded",
        "raw_authorization_material_recorded",
        "credentials_recorded",
    ]:
        if redaction.get(key) is not False:
            fail(f"redaction.{key} must be false")

    text = path.read_text(errors="replace")
    forbidden_patterns = [
        r"\b\d{12}\b",
        r"execute-api\.",
        r"Authorization:",
        r"Bearer ",
        r"AKIA",
        r"ASIA",
        r"aws_secret",
        r"arn:aws:",
        r"\b[0-9a-f]{64}\b",
    ]
    for pattern in forbidden_patterns:
        if re.search(pattern, text, flags=re.IGNORECASE):
            fail(f"summary contains forbidden unredacted pattern: {pattern}")
    if re.search(r"\bapi-[A-Za-z0-9]{8,}\b", text):
        fail("summary contains forbidden unredacted API Gateway id")

    payload = path.with_name("redacted_api_gateway_bridge_payload.json")
    if not payload.exists():
        fail("redacted_api_gateway_bridge_payload.json is missing")
    payload_text = payload.read_text(errors="replace")
    for pattern in forbidden_patterns:
        if re.search(pattern, payload_text, flags=re.IGNORECASE):
            fail(f"redacted payload contains forbidden unredacted pattern: {pattern}")

    print("PASS validate_v0917_csm_api_gateway_bridge_proof")


if __name__ == "__main__":
    main()
