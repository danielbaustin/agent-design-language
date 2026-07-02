#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
STDOUT_LOG="$TMP_DIR/run_v0913_proof_validation_lane.stdout"
STDERR_LOG="$TMP_DIR/run_v0913_proof_validation_lane.stderr"
VALIDATOR_BIN="$TMP_DIR/adl-validate-structured-prompt"
VALIDATOR_LOG="$TMP_DIR/adl-validate-structured-prompt.log"

cat >"$VALIDATOR_BIN" <<EOF_VALIDATOR
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$*" >>"$VALIDATOR_LOG"
EOF_VALIDATOR
chmod +x "$VALIDATOR_BIN"

if ! ADL_STRUCTURED_PROMPT_VALIDATOR_BIN="$VALIDATOR_BIN" \
  ADL_V0913_PROOF_ONLY_CHECKS="transition_manifest_schema,card_lifecycle_bundle,card_lifecycle_contract,merge_readiness_packet,merge_readiness_contract,quality_gate_doc_surface,quality_gate_packet_surface,demo_coverage_surface" \
  bash "$ROOT_DIR/adl/tools/run_v0913_proof_validation_lane.sh" >"$STDOUT_LOG" 2>"$STDERR_LOG"; then
  cat "$STDOUT_LOG"
  cat "$STDERR_LOG" >&2
  exit 1
fi

validator_invocations="$(wc -l <"$VALIDATOR_LOG" | tr -d ' ')"
if [[ "$validator_invocations" != "5" ]]; then
  echo "expected 5 structured prompt validator calls, saw $validator_invocations" >&2
  cat "$VALIDATOR_LOG" >&2
  exit 1
fi

echo "PASS test_run_v0913_proof_validation_lane"
