#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-success}"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

EXAMPLE="swarm/examples/v0-7-enterprise-signed-remote.adl.yaml"
RUN_ID="v0-7-enterprise-signed-remote"
REMOTE_LOG=".tmp/d11-remote.log"
OUT_DIR=".tmp/d11-out"
KEY_DIR=".tmp/d11-keys"

mkdir -p .tmp
rm -rf "$KEY_DIR"

cleanup() {
  if [[ -n "${REMOTE_PID:-}" ]]; then
    kill "$REMOTE_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

run_success() {
  umask 077
  cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- keygen --out-dir "$KEY_DIR"
  chmod 600 "$KEY_DIR/ed25519-private.b64" "$KEY_DIR/ed25519-public.b64" 2>/dev/null || true

  export ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64
  ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64="$(tr -d '\n' < "$KEY_DIR/ed25519-private.b64")"
  export ADL_REMOTE_REQUEST_SIGNING_KEY_ID="demo-key-1"

  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
    cargo run -q --manifest-path swarm/Cargo.toml --bin adl-remote -- 127.0.0.1:8787 >"$REMOTE_LOG" 2>&1 &
  REMOTE_PID=$!
  sleep 1

  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
    cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- "$EXAMPLE" --run --trace --allow-unsigned

  # Prove remote step actually executed by checking run step outcomes.
  steps_file=".adl/runs/$RUN_ID/steps.json"
  python3 - "$steps_file" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    steps = json.load(f)

status = {item.get("step_id"): item.get("status") for item in steps}
if status.get("remote.mid") != "success":
    raise SystemExit(f"D11 ERROR: expected remote.mid success in {path}; got {status.get('remote.mid')!r}")
if status.get("local.last") != "success":
    raise SystemExit(f"D11 ERROR: expected local.last success in {path}; got {status.get('local.last')!r}")
PY

  echo "D11 success complete"
  echo "  run_id=$RUN_ID"
  echo "  remote_log=$REMOTE_LOG"
  echo "  remote_step=verified (remote.mid=success, local.last=success)"
}

run_negative() {
  unset ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64
  unset ADL_REMOTE_REQUEST_SIGNING_KEY_ID

  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
    cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- "$EXAMPLE" --run --trace --allow-unsigned
}

run_tamper() {
  # Deterministic tamper-path proof using the canonical signed-request unit
  # regression. This validates that payload mutation after signing is rejected
  # with REMOTE_REQUEST_SIGNATURE_MISMATCH.
  cargo test -q --manifest-path swarm/Cargo.toml remote_exec::tests::security_envelope_rejects_tampered_signed_request -- --nocapture
}

case "$MODE" in
  success)
    run_success
    ;;
  negative)
    run_negative
    ;;
  tamper)
    run_tamper
    ;;
  *)
    echo "Usage: $0 [success|negative|tamper]" >&2
    exit 2
    ;;
esac
