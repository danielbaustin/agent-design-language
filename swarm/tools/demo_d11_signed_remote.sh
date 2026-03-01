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
rm -rf "$OUT_DIR" "$KEY_DIR"

cleanup() {
  if [[ -n "${REMOTE_PID:-}" ]]; then
    kill "$REMOTE_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

run_success() {
  cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- keygen --out-dir "$KEY_DIR"

  export ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64
  ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64="$(tr -d '\n' < "$KEY_DIR/ed25519-private.b64")"
  export ADL_REMOTE_REQUEST_SIGNING_KEY_ID="demo-key-1"

  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
    cargo run -q --manifest-path swarm/Cargo.toml --bin adl-remote -- 127.0.0.1:8787 >"$REMOTE_LOG" 2>&1 &
  REMOTE_PID=$!
  sleep 1

  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
    cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- "$EXAMPLE" --run --trace --allow-unsigned --out "$OUT_DIR"

  echo "D11 success complete"
  echo "  run_id=$RUN_ID"
  echo "  out_dir=$OUT_DIR"
  echo "  remote_log=$REMOTE_LOG"
}

run_negative() {
  unset ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64
  unset ADL_REMOTE_REQUEST_SIGNING_KEY_ID

  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh \
    cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- "$EXAMPLE" --run --trace --allow-unsigned
}

case "$MODE" in
  success)
    run_success
    ;;
  negative)
    run_negative
    ;;
  *)
    echo "Usage: $0 [success|negative]" >&2
    exit 2
    ;;
esac
