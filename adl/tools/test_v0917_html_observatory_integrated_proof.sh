#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEMO_DIR="${ROOT_DIR}/demos/html-observatory"
HTML="${DEMO_DIR}/index.html"
CSS="${DEMO_DIR}/styles.css"
JS="${DEMO_DIR}/app.js"
README="${DEMO_DIR}/README.md"
PACKET="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json"
REPORT="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/operator_report.md"
CSM_SERVICE="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_service_4903/service/service_manifest.json"
CSM_API="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md"
CLOUDWATCH="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json"
CLOUDWATCH_EVENTS="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/aws/cloudwatch_recent_events.redacted.json"
ACIP_SNS="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json"
SNS_RESOURCE="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/sns_resource_summary.json"
CSM_STATUS="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/status.json"
CSM_HEALTH="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/health.json"
CSM_READY="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/ready.json"
CSM_METRICS="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/metrics.json"
CSM_EVENTS="${ROOT_DIR}/docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/events.json"

for path in "${HTML}" "${CSS}" "${JS}" "${README}" "${PACKET}" "${REPORT}" "${CSM_SERVICE}" "${CSM_API}" "${CLOUDWATCH}" "${CLOUDWATCH_EVENTS}" "${ACIP_SNS}" "${SNS_RESOURCE}" "${CSM_STATUS}" "${CSM_HEALTH}" "${CSM_READY}" "${CSM_METRICS}" "${CSM_EVENTS}"; do
  [[ -f "${path}" ]] || {
    echo "missing HTML Observatory artifact: ${path}" >&2
    exit 1
  }
done

require_readme() {
  local needle="$1"
  grep -Fq "${needle}" "${README}" || {
    echo "README missing required phrase: ${needle}" >&2
    exit 1
  }
}

ADL_REPO_ROOT="${ROOT_DIR}" bash "${ROOT_DIR}/adl/tools/validate_v0917_csm_service_4903_status.sh" >/dev/null
python3 "${ROOT_DIR}/adl/tools/validate_wp08_heartbeat_live_proof.py" "${CLOUDWATCH}" >/dev/null
python3 "${ROOT_DIR}/adl/tools/validate_wp08_acip_sns_live_proof.py" "${ACIP_SNS}" "${SNS_RESOURCE}" >/dev/null
python3 "${ROOT_DIR}/adl/tools/validate_v0917_html_observatory.py" \
  --html "${HTML}" \
  --css "${CSS}" \
  --js "${JS}" \
  --packet "${PACKET}" \
  --report "${REPORT}" \
  --csm-service "${CSM_SERVICE}" \
  --csm-api "${CSM_API}" \
  --cloudwatch "${CLOUDWATCH}" \
  --cloudwatch-events "${CLOUDWATCH_EVENTS}" \
  --acip-sns "${ACIP_SNS}" \
  --sns-resource "${SNS_RESOURCE}" \
  --csm-status "${CSM_STATUS}" \
  --csm-health "${CSM_HEALTH}" \
  --csm-ready "${CSM_READY}" \
  --csm-metrics "${CSM_METRICS}" \
  --csm-events "${CSM_EVENTS}" >/dev/null
python3 -m json.tool "${PACKET}" >/dev/null
cargo test \
  --manifest-path "${ROOT_DIR}/adl-runtime-kernel/Cargo.toml" \
  --test control \
  observatory_https_reads_are_public_and_report_weather_freshness \
  -- --nocapture >/dev/null

require_readme "Magic UI Pro AI Agent Template"
require_readme "bounded runtime capture"
require_readme "CSM API"
require_readme "CloudWatch"
require_readme "ACIP-SNS"
require_readme "Runtime v3"
require_readme "/v1/observatory"
require_readme "20997"
require_readme "browser-owned AWS publish authority"
require_readme "WP-08"
require_readme "communication rail"
grep -Fq "Runtime v3 opt-in + CSM Runtime + AWS CloudWatch + ACIP/SNS" "${HTML}" || {
  echo "HTML Observatory status bar must name the Runtime v3 opt-in and ACIP/SNS data source" >&2
  exit 1
}
grep -Fq "retained_proof_status: acipSnsSummary.status || \"unknown\"" "${JS}" || {
  echo "HTML Observatory operator envelope must report the retained ACIP/SNS proof status without stale hygiene-blocked state" >&2
  exit 1
}

echo "v0.91.7 HTML Observatory integrated proof passed"
