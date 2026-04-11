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
  local log_rel="${log#$ROOT_DIR/}"
  if "$@" >"$log" 2>&1; then
    printf '"%s":{"status":"PASS","log":"%s"}' "$key" "$log_rel"
  else
    printf '"%s":{"status":"FAIL","log":"%s"}' "$key" "$log_rel"
  fi
}

checks_json="{"
checks_json+="$(run_check fmt cargo fmt --manifest-path "$ROOT_DIR/adl/Cargo.toml" --all --check),"
checks_json+="$(run_check clippy cargo clippy --manifest-path "$ROOT_DIR/adl/Cargo.toml" --all-targets -- -D warnings),"
checks_json+="$(run_check test cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml"),"
checks_json+="$(run_check coverage_gate bash -lc "cd \"$ROOT_DIR/adl\" && cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info && cargo llvm-cov report --json --summary-only --output-path coverage-summary.json && cargo llvm-cov --workspace --summary-only | tee coverage-summary.txt && bash tools/enforce_coverage_gates.sh coverage-summary.json"),"
checks_json+="$(run_check legacy_guardrail bash "$ROOT_DIR/adl/tools/check_no_new_legacy_swarm_refs.sh"),"
checks_json+="$(run_check release_notes_commands bash "$ROOT_DIR/adl/tools/check_release_notes_commands.sh"),"
checks_json+="$(run_check runtime_rows bash "$ROOT_DIR/adl/tools/test_demo_v0871_runtime_rows.sh"),"
checks_json+="$(run_check operator_surface bash "$ROOT_DIR/adl/tools/test_demo_v0871_operator_surface.sh"),"
checks_json+="$(run_check runtime_state bash "$ROOT_DIR/adl/tools/test_demo_v0871_runtime_state.sh"),"
checks_json+="$(run_check review_surface bash "$ROOT_DIR/adl/tools/test_demo_v0871_review_surface.sh"),"
checks_json+="$(run_check rust_module_watch bash "$ROOT_DIR/adl/tools/report_large_rust_modules.sh" --format tsv)"
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
- the quality-gate record captures the current bounded fmt/clippy/test/coverage, guardrail, release-note, demo-surface, and maintainability-watch statuses in one reviewable location with per-check logs
EOF
