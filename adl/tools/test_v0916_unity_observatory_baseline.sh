#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
README_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/README.md"
PROOF_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
SCENE_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity"
SCENE_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity.meta"
BOOTSTRAP_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
BOOTSTRAP_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs.meta"
SHELL_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
SHELL_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs.meta"
UXML_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uxml"
UXML_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uxml.meta"
USS_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uss"
USS_META_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uss.meta"
MANIFEST_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Packages/manifest.json"
BUILD_SETTINGS_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/ProjectSettings/EditorBuildSettings.asset"
PROJECT_VERSION_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/ProjectSettings/ProjectVersion.txt"
REVIEW_PATH="${ROOT_DIR}/docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_IMPLEMENTATION_BASELINE_4030.md"

for path in \
  "${README_PATH}" \
  "${PROOF_PATH}" \
  "${SCENE_PATH}" \
  "${SCENE_META_PATH}" \
  "${BOOTSTRAP_PATH}" \
  "${BOOTSTRAP_META_PATH}" \
  "${SHELL_PATH}" \
  "${SHELL_META_PATH}" \
  "${UXML_PATH}" \
  "${UXML_META_PATH}" \
  "${USS_PATH}" \
  "${USS_META_PATH}" \
  "${MANIFEST_PATH}" \
  "${BUILD_SETTINGS_PATH}" \
  "${PROJECT_VERSION_PATH}" \
  "${REVIEW_PATH}"
do
  if [[ ! -f "${path}" ]]; then
    echo "missing Unity Observatory baseline artifact: ${path}" >&2
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

require_contains_any() {
  local path="$1"
  shift
  local needle
  for needle in "$@"; do
    if grep -Fq "${needle}" "${path}"; then
      return 0
    fi
  done
  echo "missing required content set in ${path}: $*" >&2
  exit 1
}

for path in "${README_PATH}" "${PROOF_PATH}" "${REVIEW_PATH}"; do
  require_contains "${path}" "demos/v0.91.6/unity-observatory"
  require_contains "${path}" "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json"
  require_contains "${path}" "adl.csm_visibility_packet.v1"
done

for needle in \
  "Assets/Scenes/UnityObservatory.unity" \
  "Assets/Scripts/UnityObservatoryBootstrap.cs" \
  "#4031" \
  "#4032" \
  "#4033" \
  "#4034" \
  "#4035"
do
  require_contains "${README_PATH}" "${needle}"
done
require_contains "${README_PATH}" "Reference UI asset:"
require_contains "${README_PATH}" "Assets/UI/ObservatoryShell.uxml"
require_contains "${README_PATH}" "Reference style asset:"
require_contains "${README_PATH}" "Assets/UI/ObservatoryShell.uss"
require_contains "${README_PATH}" "This issue does not claim that the runtime path already loads"
require_contains "${README_PATH}" "those assets directly."

require_contains "${SCENE_PATH}" "m_Name: Unity Observatory Bootstrap"
require_contains "${SCENE_PATH}" "guid: 40310000000000000000000000001001"
require_contains "${SCENE_PATH}" "packetSchema: adl.csm_visibility_packet.v1"
require_contains "${SCENE_PATH}" "packetRef: demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json"
require_contains "${SCENE_PATH}" "defaultRoomLabel: World / Reality"
require_contains "${SCENE_PATH}" "defaultLensLabel: Operator lens"
require_contains "${SCENE_META_PATH}" "guid: 40310000000000000000000000000001"

require_contains "${BOOTSTRAP_PATH}" "namespace ADL.Demos.UnityObservatory"
require_contains "${BOOTSTRAP_PATH}" "CreateObservatoryShell"
require_contains "${BOOTSTRAP_PATH}" "UnityObservatoryShellController"
require_contains "${BOOTSTRAP_PATH}" "adl.csm_visibility_packet.v1"
require_contains "${BOOTSTRAP_META_PATH}" "guid: 40310000000000000000000000001001"

require_contains "${SHELL_PATH}" "namespace ADL.Demos.UnityObservatory"
require_contains "${SHELL_PATH}" "BuildSummaryCard"
require_contains "${SHELL_PATH}" "BuildBoundaryCard"
require_contains "${SHELL_PATH}" "BuildPacketCard"
require_contains_any \
  "${SHELL_PATH}" \
  "This launch baseline shows the bounded ingress seam that #4032 will deepen into a real Unity-facing loader." \
  "This shell is reading a deterministic Unity-facing contract derived from"
require_contains "${SHELL_META_PATH}" "guid: 40310000000000000000000000001002"

require_contains_any "${UXML_PATH}" "Launch-baseline governed shell" "fixture backed governed shell"
require_contains "${UXML_PATH}" "packet-ref"
require_contains_any \
  "${UXML_PATH}" \
  "This launch baseline shows the bounded ingress seam that #4032 will deepen into a real Unity-facing loader." \
  "This shell is reading a deterministic Unity-facing contract derived from fixture_backed Observatory evidence."
require_contains "${UXML_META_PATH}" "guid: 40310000000000000000000000002001"

require_contains "${USS_PATH}" ".observatory-screen"
require_contains "${USS_PATH}" ".observatory-shell"
require_contains "${USS_PATH}" ".card"
require_contains "${USS_META_PATH}" "guid: 40310000000000000000000000002002"

require_contains "${MANIFEST_PATH}" "\"com.unity.ui\": \"1.0.0\""
require_contains "${MANIFEST_PATH}" "\"com.unity.ugui\": \"1.0.0\""
require_contains "${BUILD_SETTINGS_PATH}" "path: Assets/Scenes/UnityObservatory.unity"
require_contains "${BUILD_SETTINGS_PATH}" "guid: 40310000000000000000000000000001"
require_contains "${PROJECT_VERSION_PATH}" "m_EditorVersion: 2022.3.62f1"
require_contains "${PROOF_PATH}" "not claimed as"
require_contains "${PROOF_PATH}" "live-loaded runtime assets in this issue"

echo "v0.91.6 Unity Observatory baseline guardrails passed"
