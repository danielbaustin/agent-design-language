#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/quality_gate}"
MANIFEST="$OUT_DIR/quality_gate_record.json"
README="$OUT_DIR/README.md"
mkdir -p "$OUT_DIR"

run_check() {
  local key="$1"
  shift
  local log="$OUT_DIR/$key.log"
  if "$@" >"$log" 2>&1; then
    printf '"%s":{"status":"PASS","log":"%s"}' "$key" "$log"
  else
    printf '"%s":{"status":"FAIL","log":"%s"}' "$key" "$log"
  fi
}

checks_json="{"
checks_json+="$(run_check fmt cargo fmt --manifest-path "$ROOT_DIR/adl/Cargo.toml" --all --check),"
checks_json+="$(run_check runtime_rows bash "$ROOT_DIR/adl/tools/test_demo_v0871_runtime_rows.sh"),"
checks_json+="$(run_check operator_surface bash "$ROOT_DIR/adl/tools/test_demo_v0871_operator_surface.sh"),"
checks_json+="$(run_check runtime_state bash "$ROOT_DIR/adl/tools/test_demo_v0871_runtime_state.sh"),"
checks_json+="$(run_check review_surface bash "$ROOT_DIR/adl/tools/test_demo_v0871_review_surface.sh")"
checks_json+="}"

printf '{"demo_id":"D11","manifest_version":"adl.v0871.quality_gate.v1","checks":%s}\n' "$checks_json" >"$MANIFEST"

cat >"$README" <<'EOF'
# v0.87.1 Demo D11 - Quality Gate Walkthrough

Canonical command:

```bash
bash adl/tools/demo_v0871_quality_gate.sh
```

Primary proof surface:
- `artifacts/v0871/quality_gate/quality_gate_record.json`

Success signal:
- the quality-gate record captures the current bounded demo/test/fmt statuses in one reviewable location with per-check logs
EOF
