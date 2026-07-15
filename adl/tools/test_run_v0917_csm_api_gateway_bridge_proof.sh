#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

BIN="$TMP/bin"
mkdir -p "$BIN"

cat >"$BIN/aws" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "aws $*" >>"${FAKE_AWS_LOG:?}"
case "$1 $2" in
  "sts get-caller-identity")
    printf '%s\n' "123456789012"
    ;;
  "apigatewayv2 get-apis")
    printf '%s\n' '{"Items":[{"ApiId":"api-1234567890","Name":"adl-csm-fixture","ProtocolType":"HTTP"}]}'
    ;;
  "apigatewayv2 get-stages")
    printf '%s\n' '{"Items":[{"StageName":"prod","AutoDeploy":true}]}'
    ;;
  "apigatewayv2 get-routes")
    printf '%s\n' '{"Items":[{"RouteKey":"GET /status","Target":"integrations/int-1234567890"},{"RouteKey":"GET /health","Target":"integrations/int-1234567890"},{"RouteKey":"GET /ready","Target":"integrations/int-1234567890"},{"RouteKey":"GET /metrics","Target":"integrations/int-1234567890"},{"RouteKey":"GET /events","Target":"integrations/int-1234567890"},{"RouteKey":"GET /chronosense","Target":"integrations/int-1234567890"},{"RouteKey":"GET /weather","Target":"integrations/int-1234567890"},{"RouteKey":"GET /shepherd","Target":"integrations/int-1234567890"},{"RouteKey":"GET /curiosity","Target":"integrations/int-1234567890"},{"RouteKey":"GET /acip","Target":"integrations/int-1234567890"},{"RouteKey":"GET /freedom-gate","Target":"integrations/int-1234567890"},{"RouteKey":"GET /reasoning","Target":"integrations/int-1234567890"},{"RouteKey":"GET /api-gateway-bridge","Target":"integrations/int-1234567890"},{"RouteKey":"GET /persistence","Target":"integrations/int-1234567890"}]}'
    ;;
  "apigatewayv2 get-integrations")
    printf '%s\n' '{"Items":[{"IntegrationId":"int-1234567890","IntegrationType":"HTTP_PROXY","IntegrationUri":"https://loopback-proxy.invalid/csm"}]}'
    ;;
  "logs filter-log-events")
    printf '%s\n' '{"events":[{"eventId":"evt-1","message":"bridge csm-5039-a91b3eafa2b703d4 success"}]}'
    ;;
  "events list-rules")
    printf '%s\n' '{"Rules":[{"Name":"adl-csm-api-gateway-bridge","Arn":"arn:aws:events:us-west-2:123456789012:rule/adl-csm"}]}'
    ;;
  *)
    echo "unexpected aws args: $*" >&2
    exit 2
    ;;
esac
SH
chmod +x "$BIN/aws"

cat >"$BIN/curl" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "curl argc=$# has_config_stdin=yes" >>"${FAKE_CURL_LOG:?}"
auth="missing"
config="$(cat)"
case "$config" in
  *"Authorization: Bearer"*) auth="present" ;;
esac
if [[ "$config" == *"Bearer malformed-token"* ]]; then
  printf '%s\n%s' '{"schema":"adl.csm.api_gateway_bridge.denied.v1","status":"denied","error":"malformed token"}' "401"
elif [ "$auth" = "present" ]; then
  printf '%s\n%s' '{"schema":"adl.csm.runtime_api.api_gateway_bridge.v1","runtime_owner":"csm","agent_instance_id":"api-agent","status":"available","runtime_api_path":"/api-gateway-bridge","polis_ingress":{"polis_id":"api-agent","ingress_model":"one_api_gateway_api_per_polis","route_target":"authorized_api_gateway_to_csm_loopback_runtime_api","per_polis_api":true},"redaction":{"secret_material":"not_returned"}}' "200"
else
  printf '%s\n%s' '{"schema":"adl.csm.api_gateway_bridge.denied.v1","status":"denied"}' "403"
fi
SH
chmod +x "$BIN/curl"

cat >"$BIN/csm" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
exec "${REAL_CSM:?}" "$@" --aws-bin "${AWS_BIN:?}" --http-bin "${CURL_BIN:?}"
SH
chmod +x "$BIN/csm"

export FAKE_AWS_LOG="$TMP/aws.log"
export FAKE_CURL_LOG="$TMP/curl.log"
export AWS_BIN="$BIN/aws"
export CURL_BIN="$BIN/curl"
export REAL_CSM="${CARGO_TARGET_DIR:-$ROOT/adl/target}/debug/csm"
printf '%s\n' "fixture-token" >"$TMP/operator-token"

cargo build --manifest-path "$ROOT/adl/Cargo.toml" --bin csm >/dev/null

"$BIN/csm" cloud-control api-gateway-bridge \
  --out "$TMP/proof" \
  --profile agent-logic-admin \
  --region us-west-2 \
  --run-id fixture-run \
  --polis-id api-agent \
  --expected-account-sha256 2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972 \
  --api-id api-1234567890 \
  --stage prod \
  --invoke-url https://fixture.execute-api.us-west-2.amazonaws.com/prod \
  --operator-token-file "$TMP/operator-token" \
  --cloudwatch-log-group /aws/apigateway/adl-csm \
  --eventbridge-bus adl-csm-bus \
  --json >/dev/null

python3 "$ROOT/adl/tools/validate_v0917_csm_api_gateway_bridge_proof.py" \
  "$TMP/proof/api_gateway_bridge_summary.json" >/dev/null

python3 - "$TMP/proof/api_gateway_bridge_summary.json" "$FAKE_AWS_LOG" "$FAKE_CURL_LOG" <<'PY'
import json
import sys
from pathlib import Path

summary_path = Path(sys.argv[1])
summary = json.loads(summary_path.read_text())
aws_log = Path(sys.argv[2]).read_text()
curl_log = Path(sys.argv[3]).read_text()

assert summary["schema"] == "adl.csm.api_gateway_bridge_proof.v1"
assert summary["status"] == "bounded_smoke"
assert summary["polis_ingress"]["ingress_model"] == "one_api_gateway_api_per_polis"
assert summary["polis_ingress"]["route_target"] == "authorized_api_gateway_to_csm_loopback_runtime_api"
assert summary["polis_ingress"]["per_polis_api"] is True
assert summary["polis_ingress"]["runtime_identity_verified"] is True
assert summary["api_gateway"]["route_target_count"] >= 7
assert "GET /weather" in summary["api_gateway"]["supported_route_keys"]
assert "GET /weather" in summary["api_gateway"]["planned_route_keys"]
assert "GET /reasoning" in summary["api_gateway"]["supported_route_keys"]
assert "GET /reasoning" in summary["api_gateway"]["planned_route_keys"]
assert "GET /acip" in summary["api_gateway"]["supported_route_keys"]
assert "GET /acip" in summary["api_gateway"]["planned_route_keys"]
assert "GET /persistence" in summary["api_gateway"]["supported_route_keys"]
assert "GET /persistence" in summary["api_gateway"]["planned_route_keys"]
assert "GET /acip/ws" not in summary["api_gateway"]["planned_route_keys"]
assert summary["api_gateway"]["integration_count"] >= 1
assert summary["bridge"]["endpoint"] == "/api-gateway-bridge"
assert summary["bridge"]["response_schema"] == "adl.csm.runtime_api.api_gateway_bridge.v1"
assert summary["live_negative_cases"]["missing_token"] == "api_gateway_authorization_denied"
assert summary["live_negative_cases"]["missing_token_http_status"] in {401, 403}
assert summary["live_negative_cases"]["malformed_request"] == "api_gateway_malformed_request"
assert summary["live_negative_cases"]["malformed_request_http_status"] in {401, 403}
for required in [
    "sts get-caller-identity",
    "apigatewayv2 get-apis",
    "apigatewayv2 get-stages",
    "apigatewayv2 get-routes",
    "apigatewayv2 get-integrations",
    "logs filter-log-events",
    "events list-rules",
]:
    assert required in aws_log, required
assert "Authorization: Bearer fixture-token" not in curl_log
assert "fixture-token" not in curl_log
text = summary_path.read_text()
assert "123456789012" not in text
assert "api-1234567890" not in text
assert "fixture.execute-api" not in text
assert "fixture-token" not in text
PY

echo "PASS test_run_v0917_csm_api_gateway_bridge_proof"
