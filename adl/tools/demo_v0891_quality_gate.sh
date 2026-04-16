#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0891/quality_gate}"
MANIFEST="$OUT_DIR/quality_gate_record.json"
README="$OUT_DIR/README.md"
RUN_TMPDIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-v0891-quality-gate.XXXXXX")"
BUILD_TARGET_DIR="${CARGO_TARGET_DIR:-$RUN_TMPDIR/cargo-target}"
LLVM_COV_TARGET_DIR="${CARGO_LLVM_COV_TARGET_DIR:-$RUN_TMPDIR/llvm-cov-target}"

mkdir -p "$OUT_DIR"
trap 'rm -rf "$RUN_TMPDIR"' EXIT

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
checks_json+="$(run_check tooling_shell_sanity bash -lc "cd \"$ROOT_DIR\" && bash -n adl/tools/*.sh"),"
checks_json+="$(run_check codex_pr_help bash -lc "cd \"$ROOT_DIR\" && sh adl/tools/codex_pr.sh --help"),"
checks_json+="$(run_check codexw_help bash -lc "cd \"$ROOT_DIR\" && sh adl/tools/codexw.sh --help"),"
checks_json+="$(run_check legacy_guardrail bash "$ROOT_DIR/adl/tools/check_no_new_legacy_swarm_refs.sh"),"
checks_json+="$(run_check release_notes_commands bash "$ROOT_DIR/adl/tools/check_release_notes_commands.sh"),"
checks_json+="$(run_check repo_code_review_contract bash "$ROOT_DIR/adl/tools/test_repo_code_review_skill_contracts.sh"),"
checks_json+="$(run_check test_generator_contract bash "$ROOT_DIR/adl/tools/test_test_generator_skill_contracts.sh"),"
checks_json+="$(run_check demo_operator_contract bash "$ROOT_DIR/adl/tools/test_demo_operator_skill_contracts.sh"),"
checks_json+="$(run_check arxiv_paper_writer_contract bash "$ROOT_DIR/adl/tools/test_arxiv_paper_writer_skill_contracts.sh"),"
checks_json+="$(run_check fmt cargo fmt --manifest-path "$ROOT_DIR/adl/Cargo.toml" --all --check),"
checks_json+="$(run_check clippy bash -lc "cd \"$ROOT_DIR\" && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"),"
checks_json+="$(run_check test bash -lc "cd \"$ROOT_DIR\" && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" cargo test --manifest-path adl/Cargo.toml"),"
checks_json+="$(run_check coverage_gate bash -lc "set -euo pipefail; cd \"$ROOT_DIR/adl\" && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" CARGO_LLVM_COV_TARGET_DIR=\"$LLVM_COV_TARGET_DIR\" cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" CARGO_LLVM_COV_TARGET_DIR=\"$LLVM_COV_TARGET_DIR\" cargo llvm-cov report --json --summary-only --output-path coverage-summary.json && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" CARGO_LLVM_COV_TARGET_DIR=\"$LLVM_COV_TARGET_DIR\" cargo llvm-cov --workspace --summary-only | tee coverage-summary.txt && bash tools/enforce_coverage_gates.sh coverage-summary.json"),"
checks_json+="$(run_check demo_smoke bash "$ROOT_DIR/adl/tools/demo_smoke_v07_story.sh"),"
checks_json+="$(run_check wp13_demo_integration bash "$ROOT_DIR/adl/tools/test_demo_v0891_wp13_demo_integration.sh"),"
checks_json+="$(run_check five_agent_hey_jude bash "$ROOT_DIR/adl/tools/test_demo_v0891_five_agent_hey_jude.sh"),"
checks_json+="$(run_check arxiv_manuscript_workflow bash "$ROOT_DIR/adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh"),"
checks_json+="$(run_check rust_module_watch bash "$ROOT_DIR/adl/tools/report_large_rust_modules.sh" --format tsv)"
checks_json+="}"

printf '{"demo_id":"D10","manifest_version":"adl.v0891.quality_gate.v1","checks":%s}\n' "$checks_json" >"$MANIFEST"

cat >"$README" <<'EOF'
# v0.89.1 Demo D10 - Quality Gate Walkthrough

Canonical command:

```bash
bash adl/tools/demo_v0891_quality_gate.sh
```

Primary proof surface:
- `artifacts/v0891/quality_gate/quality_gate_record.json`

Success signal:
- the quality-gate record captures the bounded local command suite, coverage policy check, v0.89.1 proof-package checks, and maintainability-watch output in one reviewable location with per-check logs

Important boundary:
- this walkthrough is the reviewer-facing aggregation surface for the quality gate
- it does not replace CI or the PR closing-linkage guardrail, which remains CI-only
- the live coverage gate has no active per-file exclusion regex
EOF
