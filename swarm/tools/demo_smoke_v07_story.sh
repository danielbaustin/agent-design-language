#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TMP_DIR=".tmp/ci-demo-smoke"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"

run_cmd() {
  local demo_id="$1"
  local cmd="$2"
  echo "[demo-smoke] ${demo_id}"
  echo "[demo-smoke] cmd: ${cmd}"
  if ! bash -lc "$cmd"; then
    echo "DEMO_SMOKE_FAIL id=${demo_id} cmd=${cmd}" >&2
    exit 1
  fi
}

# S-01: Determinism baseline command (single-run smoke)
run_cmd "S-01" \
  "ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --trace --allow-unsigned --out ${TMP_DIR}/s01"
[[ -s "${TMP_DIR}/s01/s1.txt" ]] || { echo "DEMO_SMOKE_FAIL id=S-01 missing artifact s1.txt" >&2; exit 1; }

# S-02: Deterministic failure surface
s02_log="${TMP_DIR}/s02.log"
set +e
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- \
  swarm/examples/failure-missing-file.adl.yaml --run --allow-unsigned >"$s02_log" 2>&1
s02_rc=$?
set -e
if [[ "$s02_rc" -eq 0 ]]; then
  echo "DEMO_SMOKE_FAIL id=S-02 expected failure but command succeeded" >&2
  exit 1
fi
if ! grep -En "failed to stat input file|failed to materialize inputs|No such file or directory|THIS_FILE_DOES_NOT_EXIST\\.txt|missing|not found" "$s02_log" >/dev/null 2>&1; then
  echo "[demo-smoke] S-02 captured output:" >&2
  sed -n '1,120p' "$s02_log" >&2 || true
  echo "DEMO_SMOKE_FAIL id=S-02 expected deterministic missing-file signal" >&2
  exit 1
fi

# S-03: Learning export smoke
run_cmd "S-03" \
  "cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- learn export --format jsonl --runs-dir .adl/runs --out ${TMP_DIR}/s03.jsonl"
[[ -s "${TMP_DIR}/s03.jsonl" ]] || { echo "DEMO_SMOKE_FAIL id=S-03 missing jsonl output" >&2; exit 1; }

# S-04: Enterprise trust-boundary tamper check (daemon-free)
run_cmd "S-04" "./swarm/tools/demo_d11_signed_remote.sh tamper"

# S-05: Canonical naming path
run_cmd "S-05" "cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- --help >/dev/null"

echo "[demo-smoke] PASS S-01..S-05"
