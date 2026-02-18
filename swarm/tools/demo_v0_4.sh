#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MOCK_BIN="$ROOT/swarm/tools/mock_ollama_v0_4.sh"
OUT_ROOT="$ROOT/.adl/reports/demo-v0.4"

export SWARM_OLLAMA_BIN="$MOCK_BIN"

run_demo() {
  local name="$1"
  local yaml="$2"
  local out_dir="$OUT_ROOT/$name"

  rm -rf "$out_dir"
  mkdir -p "$out_dir"

  echo "==> Running $name"
  cargo run -q --manifest-path "$ROOT/swarm/Cargo.toml" -- "$yaml" --run --trace --out "$out_dir"
}

run_demo "fork-join-swarm" "$ROOT/swarm/examples/v0-4-demo-fork-join-swarm.adl.yaml"

bounded_start="$(date +%s)"
run_demo "bounded-parallelism" "$ROOT/swarm/examples/v0-4-demo-bounded-parallelism.adl.yaml"
bounded_end="$(date +%s)"
echo "bounded-parallelism elapsed_seconds=$((bounded_end - bounded_start))"

run_demo "deterministic-replay-pass-1" "$ROOT/swarm/examples/v0-4-demo-deterministic-replay.adl.yaml"
run_demo "deterministic-replay-pass-2" "$ROOT/swarm/examples/v0-4-demo-deterministic-replay.adl.yaml"

hash_cmd=""
if command -v sha256sum >/dev/null 2>&1; then
  hash_cmd="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  hash_cmd="shasum -a 256"
fi

join1="$OUT_ROOT/deterministic-replay-pass-1/replay/join.txt"
join2="$OUT_ROOT/deterministic-replay-pass-2/replay/join.txt"

if [[ "$hash_cmd" == "sha256sum" ]]; then
  h1="$(sha256sum "$join1" | awk '{print $1}')"
  h2="$(sha256sum "$join2" | awk '{print $1}')"
elif [[ "$hash_cmd" == "shasum -a 256" ]]; then
  h1="$(shasum -a 256 "$join1" | awk '{print $1}')"
  h2="$(shasum -a 256 "$join2" | awk '{print $1}')"
else
  h1="$(cksum "$join1" | awk '{print $1}')"
  h2="$(cksum "$join2" | awk '{print $1}')"
fi

if [[ "$h1" != "$h2" ]]; then
  echo "deterministic replay failed: join hashes differ" >&2
  echo "pass1=$h1" >&2
  echo "pass2=$h2" >&2
  exit 1
fi

echo "deterministic replay verified: join_hash=$h1"
echo "artifacts root: $OUT_ROOT"
