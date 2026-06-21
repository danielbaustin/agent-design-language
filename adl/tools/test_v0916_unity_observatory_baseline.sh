#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
README_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/README.md"
REVIEW_PATH="${ROOT_DIR}/docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_IMPLEMENTATION_BASELINE_4030.md"

for path in "${README_PATH}" "${REVIEW_PATH}"; do
  if [[ ! -f "${path}" ]]; then
    echo "missing Unity Observatory baseline artifact: ${path}" >&2
    exit 1
  fi
done

required_strings=(
  "demos/v0.91.6/unity-observatory"
  "Assets/Scenes/UnityObservatory.unity"
  "Assets/UI/ObservatoryShell.uxml"
  "Assets/UI/ObservatoryShell.uss"
  "Assets/Scripts/UnityObservatoryBootstrap.cs"
  "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json"
  "adl.csm_visibility_packet.v1"
  "#4031"
  "#4032"
  "#4033"
  "#4034"
  "#4035"
)

for path in "${README_PATH}" "${REVIEW_PATH}"; do
  for needle in "${required_strings[@]}"; do
    grep -Fq "${needle}" "${path}" || {
      echo "missing baseline coordinate '${needle}' in ${path}" >&2
      exit 1
    }
  done
done

echo "v0.91.6 Unity Observatory baseline guardrails passed"
