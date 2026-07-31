#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
README_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/README.md"
PROOF_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
BOOTSTRAP_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
SHELL_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
FLAGSHIP_BUILDER_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryFlagshipStageBuilder.cs"
VALIDATOR_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryBatchValidator.cs"
MANIFEST_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Packages/manifest.json"
RESOURCE_DIR="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Resources"
RESOURCE_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Resources.meta"
CONTRACT_PATH="${RESOURCE_DIR}/observatory_contract.json"
CONTRACT_META_PATH="${RESOURCE_DIR}/observatory_contract.json.meta"

for path in \
  "${README_PATH}" \
  "${PROOF_PATH}" \
  "${BOOTSTRAP_PATH}" \
  "${SHELL_PATH}" \
  "${FLAGSHIP_BUILDER_PATH}" \
  "${VALIDATOR_PATH}" \
  "${MANIFEST_PATH}" \
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
  grep -Fq -- "${needle}" "${path}" || {
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
require_contains "${SHELL_PATH}" "Observability and security"
require_contains "${SHELL_PATH}" "hasObservabilityContract"
require_contains "${SHELL_PATH}" "contract.observability"
require_contains "${SHELL_PATH}" "Observability/security consumption proof is not bound in this contract."
require_contains "${SHELL_PATH}" "Freedom Gate counts:"
require_contains "${SHELL_PATH}" "deterministic Unity-facing contract"
require_contains "${SHELL_PATH}" "RuntimeTruthMode"
require_contains "${SHELL_PATH}" "ADL_CSM_API_BASE"
require_contains "${SHELL_PATH}" "--csm-api-base="
require_contains "${SHELL_PATH}" "ADL_RUNTIME_OBSERVATORY_URL"
require_contains "${SHELL_PATH}" "ADL_RUNTIME_OBSERVATORY_TOKEN"
require_contains "${SHELL_PATH}" "adl.runtime_v3.observatory_feed.v2"
require_contains "${SHELL_PATH}" "\"/v1/observatory\""
require_contains "${SHELL_PATH}" "\"/v1/control\""
require_contains "${SHELL_PATH}" "Authorization"
require_contains "${SHELL_PATH}" "ConfigureRuntimeV3Request"
require_contains "${SHELL_PATH}" "IsRuntimeTransportFailure"
require_contains "${SHELL_PATH}" "ProbeRuntimeLoop"
require_contains "${SHELL_PATH}" "ProbeRuntimeV3Once"
require_contains "${SHELL_PATH}" "UnityWebRequest"
require_contains "${SHELL_PATH}" "ClassifyRuntimeProbe"
require_contains "${SHELL_PATH}" "ClassifyRuntimeSnapshot"
require_contains "${SHELL_PATH}" "ClassifyRuntimeV3Feed"
require_contains "${SHELL_PATH}" "ConfigureRuntimeEndpoint"
require_contains "${SHELL_PATH}" "TryNormalizeLoopbackEndpoint"
require_contains "${SHELL_PATH}" "TryNormalizeRuntimeEndpointForProof"
require_contains "${SHELL_PATH}" "\"/status\", \"/health\", \"/ready\", \"/metrics\", \"/events\""
require_contains "${SHELL_PATH}" "successfulEndpointCount != 5"
require_contains "${SHELL_PATH}" "adl.csm.runtime_api."
require_contains "${SHELL_PATH}" "only a root loopback HTTP origin is allowed"
require_contains "${SHELL_PATH}" "Runtime v3 requires HTTPS"
require_contains "${SHELL_PATH}" "RUNTIME TAIL"
require_contains "${SHELL_PATH}" "runtimeEventEntries"
require_contains "${SHELL_PATH}" "runtimeContinuityState"
require_contains "${SHELL_PATH}" "runtimeCloudWatchRoute"
require_contains "${SHELL_PATH}" "Observed from the CSM /status and /metrics contracts."
require_contains "${SHELL_PATH}" "SIGNED CONTROL REQUIRED"
require_contains "${SHELL_PATH}" "runtime control exists, but Unity has no governed signed operator-proposal mapping."
require_contains "${SHELL_PATH}" "DISCONNECTED"
require_contains "${SHELL_PATH}" "SelectProjection"
require_contains "${SHELL_PATH}" "NOT SENT: fixture mode exposes no operator transport."
require_contains "${SHELL_PATH}" "BuildCommandBody"
require_contains "${SHELL_PATH}" "root.style.flexGrow = 1f"
require_contains "${SHELL_PATH}" "CreateNavigationButton"
require_contains "${SHELL_PATH}" "event-stream-scroll"
require_contains "${SHELL_PATH}" "inspector-scroll"
require_contains "${FLAGSHIP_BUILDER_PATH}" "DisableIncompatibleHeroParticles"
require_contains "${FLAGSHIP_BUILDER_PATH}" "ParticleSystemStopBehavior.StopEmittingAndClear"
require_contains "${FLAGSHIP_BUILDER_PATH}" "CaptureCameraProof(camera, HeroProofPath, 1920, 1080)"
require_contains "${FLAGSHIP_BUILDER_PATH}" "CaptureCameraProof(camera, HeroProofQhdPath, 2560, 1440)"
require_contains "${FLAGSHIP_BUILDER_PATH}" "SetGameViewResolution(1920, 1080, \"ADL Full HD\")"
require_contains "${FLAGSHIP_BUILDER_PATH}" "SetGameViewResolution(2560, 1440, \"ADL QHD\")"
require_contains "${FLAGSHIP_BUILDER_PATH}" "SetGameViewFullHd();"
require_contains "${FLAGSHIP_BUILDER_PATH}" "SetGameViewQhd();"
require_contains "${FLAGSHIP_BUILDER_PATH}" "proofImage.LoadImage"
require_contains "${FLAGSHIP_BUILDER_PATH}" "proofImage.width != expectedWidth"
require_contains "${FLAGSHIP_BUILDER_PATH}" "proofImage.height != expectedHeight"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Observation Perimeter Front"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Observation Perimeter Rear"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Observation Perimeter Left"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Observation Perimeter Right"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Investor Observatory Room Floor"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Investor Observatory Front Apron"
require_contains "${FLAGSHIP_BUILDER_PATH}" "Investor Observatory Rear Wall"
require_contains "${BOOTSTRAP_PATH}" "controller.Build(root)"
require_contains "${BOOTSTRAP_PATH}" "built synchronously before the first bootstrap yield"
require_contains "${BOOTSTRAP_PATH}" "Application.runInBackground = true"
require_contains "${MANIFEST_PATH}" "\"com.unity.modules.unitywebrequest\": \"1.0.0\""
require_contains "${MANIFEST_PATH}" "\"com.unity.modules.particlesystem\": \"1.0.0\""
require_contains "${VALIDATOR_PATH}" "ValidateCommandRoomShell"
require_contains "${VALIDATOR_PATH}" "ValidateRuntimeAdapterBehavior"
require_contains "${VALIDATOR_PATH}" "external plaintext origins"
require_contains "${VALIDATOR_PATH}" "unhealthy CSM contract must degrade"
require_contains "${VALIDATOR_PATH}" "non-exact CSM schema versions"
require_contains "${VALIDATOR_PATH}" "Runtime v3 feed classifier"
require_contains "${VALIDATOR_PATH}" "/v1/hostile-control"
require_contains "${VALIDATOR_PATH}" "Bearer validator-token"
require_contains "${VALIDATOR_PATH}" "UnityWebRequest.Result.ProtocolError"
require_contains "${VALIDATOR_PATH}" "checkpoint generation 12"
require_contains "${VALIDATOR_PATH}" "vector.runtime_v3_cloudwatch_emf"
require_contains "${VALIDATOR_PATH}" "switch not authorized"
require_contains "${PROOF_PATH}" "superseded intermediate history"

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
require_contains "${CONTRACT_PATH}" "\"consumption_status\": \"reviewed_floor_not_live_export\""
require_contains "${CONTRACT_PATH}" "\"otel_boundary_ref\": \"docs/milestones/v0.91.6/review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md\""
require_contains "${CONTRACT_PATH}" "\"event_stream_example_ref\": \"docs/milestones/v0.91.6/review/logging_observability/observatory_event_stream_example_3999.jsonl\""
require_contains "${CONTRACT_PATH}" "\"logging_validation_ref\": \"docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md\""
require_contains "${CONTRACT_PATH}" "\"security_review_ref\": \"docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md\""
require_contains "${CONTRACT_PATH}" "\"proof_packet_ref\": \"docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md\""
require_contains "${CONTRACT_PATH}" "\"claim_boundary\": \"Observatory consumers may reuse the redacted event-stream vocabulary and operator-report surfaces, but this contract does not claim a live OpenTelemetry collector or exporter integration.\""
require_contains "${CONTRACT_PATH}" "\"private_state_posture\": \"No private paths, secrets, raw logs, or identity-sensitive state are required by this Unity surface.\""
require_contains "${CONTRACT_PATH}" "\"findings_disposition\": \"Accepted WP-07 security findings remain explicit, while identity-safe display and final closeout stay routed to their owning issues.\""

if grep -Eq '"/|/Users/|[A-Za-z]:\\\\' "${CONTRACT_PATH}"; then
  echo "absolute-path leakage detected in ${CONTRACT_PATH}" >&2
  exit 1
fi

if grep -Eq '"display_label"|"continuity_status"|"current_episode"' "${CONTRACT_PATH}"; then
  echo "identity-sensitive inhabitant fields should be withheld from ${CONTRACT_PATH}" >&2
  exit 1
fi

echo "v0.91.6 Unity Observatory contract guardrails passed"
