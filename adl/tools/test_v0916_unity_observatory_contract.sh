#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
README_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/README.md"
PROOF_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
BOOTSTRAP_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
SHELL_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
RESOURCE_DIR="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Resources"
RESOURCE_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Resources.meta"
CONTRACT_PATH="${RESOURCE_DIR}/observatory_contract.json"
CONTRACT_META_PATH="${RESOURCE_DIR}/observatory_contract.json.meta"

for path in \
  "${README_PATH}" \
  "${PROOF_PATH}" \
  "${BOOTSTRAP_PATH}" \
  "${SHELL_PATH}" \
  "${RESOURCE_META_PATH}" \
  "${CONTRACT_PATH}" \
  "${CONTRACT_META_PATH}"
do
  if [[ ! -f "${path}" ]]; then
    echo "missing Unity Observatory contract artifact: ${path}" >&2
    exit 1
  fi
done

require_contains() {
  local path="$1"
  local needle="$2"
  grep -Fq "${needle}" "${path}" || {
    echo "missing required content '${needle}' in ${path}" >&2
    exit 1
  }
}

for path in "${README_PATH}" "${PROOF_PATH}"; do
  require_contains "${path}" "Assets/Resources/observatory_contract.json"
  require_contains "${path}" "unity_observatory_contract.json"
  require_contains "${path}" "#4032"
done

require_contains "${BOOTSTRAP_PATH}" "ContractResourcePath = \"observatory_contract\""
require_contains "${BOOTSTRAP_PATH}" "Resources.Load<TextAsset>(ContractResourcePath)"
require_contains "${BOOTSTRAP_PATH}" "ConfigureFromContract"
require_contains "${BOOTSTRAP_PATH}" "ConfigureFallback"

require_contains "${SHELL_PATH}" "UnityObservatoryContractDocument"
require_contains "${SHELL_PATH}" "ConfigureFromContract"
require_contains "${SHELL_PATH}" "Observed summary"
require_contains "${SHELL_PATH}" "Inhabitant readiness"
require_contains "${SHELL_PATH}" "Citizen explorer"
require_contains "${SHELL_PATH}" "Freedom Gate counts:"
require_contains "${SHELL_PATH}" "deterministic Unity-facing contract"

require_contains "${RESOURCE_META_PATH}" "guid: 40320000000000000000000000003000"
require_contains "${CONTRACT_META_PATH}" "guid: 40320000000000000000000000003001"

require_contains "${CONTRACT_PATH}" "\"schema\": \"adl.unity_observatory_contract.v1\""
require_contains "${CONTRACT_PATH}" "\"source_packet_ref\": \"demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json\""
require_contains "${CONTRACT_PATH}" "\"runtime_artifact_root\": \"adl/tests/fixtures/runtime_v2/observatory\""
require_contains "${CONTRACT_PATH}" "\"default_room_label\": \"World / Reality\""
require_contains "${CONTRACT_PATH}" "\"default_lens_label\": \"Operator lens\""
require_contains "${CONTRACT_PATH}" "\"operator_report_ref\": \"runtime_v2/observatory/operator_report.md\""
require_contains "${CONTRACT_PATH}" "\"identity_visibility\": \"withheld_pending_wp08\""
require_contains "${CONTRACT_PATH}" "\"security_floor_ref\": \"docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md\""

if grep -Eq '"/|/Users/|[A-Za-z]:\\\\' "${CONTRACT_PATH}"; then
  echo "absolute-path leakage detected in ${CONTRACT_PATH}" >&2
  exit 1
fi

if grep -Eq '"display_label"|"continuity_status"|"current_episode"' "${CONTRACT_PATH}"; then
  echo "identity-sensitive inhabitant fields should be withheld from ${CONTRACT_PATH}" >&2
  exit 1
fi

echo "v0.91.6 Unity Observatory contract guardrails passed"
