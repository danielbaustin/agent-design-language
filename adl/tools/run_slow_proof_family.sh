#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ADL_DIR="$ROOT_DIR/adl"
FAMILY_CONFIG="$ADL_DIR/config/slow_proof_families.v0.91.6.json"

family=""
mode="run"

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/run_slow_proof_family.sh --family <id> [--list|--run|--print-plan|--json]

Modes:
  --list        Run `cargo nextest list` for the selected family feature.
  --run         Run `cargo nextest run` for the selected family feature. Default.
  --print-plan  Print key=value plan lines and exit.
  --json        Print the selected family plan as JSON and exit.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --family)
      family="${2:-}"
      shift 2
      ;;
    --list)
      mode="list"
      shift
      ;;
    --run)
      mode="run"
      shift
      ;;
    --print-plan)
      mode="print-plan"
      shift
      ;;
    --json)
      mode="json"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ -z "$family" ]; then
  echo "run_slow_proof_family: --family is required" >&2
  usage >&2
  exit 2
fi

family_payload="$(
  python3 - "$FAMILY_CONFIG" "$family" <<'PY'
import json
import sys
from pathlib import Path

config = json.loads(Path(sys.argv[1]).read_text())
if config.get("schema_version") != "adl.slow_proof_families.v1":
    raise SystemExit("unsupported slow-proof family config schema")
family_id = sys.argv[2]
for family in config.get("families", []):
    if family.get("id") == family_id:
        print(json.dumps({
            "id": family["id"],
            "feature": family["feature"],
            "proof_role": family.get("proof_role", "slow_proof"),
            "description": family.get("description", ""),
            "sample_tests": family.get("sample_tests", []),
            "umbrella_feature": config.get("umbrella_feature", "slow-proof-tests"),
        }))
        break
else:
    raise SystemExit(f"unknown slow-proof family: {family_id}")
PY
)" || {
  echo "run_slow_proof_family: failed to resolve family '$family'" >&2
  exit 2
}

feature="$(python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["feature"])' <<<"$family_payload")"
description="$(python3 -c 'import json,sys; print(json.loads(sys.stdin.read())["description"])' <<<"$family_payload")"

command_list=(cargo nextest list --lib --features "$feature" runtime_v2_)
command_run=(cargo nextest run --lib --features "$feature" runtime_v2_ --status-level all --final-status-level slow)

case "$mode" in
  print-plan)
    printf 'family=%s\n' "$family"
    printf 'feature=%s\n' "$feature"
    printf 'description=%s\n' "$description"
    printf 'list_command=%q ' "${command_list[@]}"
    printf '\n'
    printf 'run_command=%q ' "${command_run[@]}"
    printf '\n'
    ;;
  json)
    python3 - <<'PY' "$family_payload"
import json
import sys

payload = json.loads(sys.argv[1])
payload["list_command"] = [
    "cargo", "nextest", "list", "--lib", "--features", payload["feature"], "runtime_v2_"
]
payload["run_command"] = [
    "cargo", "nextest", "run", "--lib", "--features", payload["feature"],
    "runtime_v2_",
    "--status-level", "all", "--final-status-level", "slow",
]
print(json.dumps(payload, indent=2, sort_keys=True))
PY
    ;;
  list)
    (
      cd "$ADL_DIR"
      "${command_list[@]}"
    )
    ;;
  run)
    (
      cd "$ADL_DIR"
      "${command_run[@]}"
    )
    ;;
esac
