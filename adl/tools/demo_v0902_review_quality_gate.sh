#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0902/review_quality_gate}"
MANIFEST="$OUT_DIR/review_quality_gate_record.json"
README="$OUT_DIR/README.md"
RUN_TMPDIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-v0902-review-quality-gate.XXXXXX")"
BUILD_TARGET_DIR="${CARGO_TARGET_DIR:-$RUN_TMPDIR/cargo-target}"
INSPECT_ONLY="${ADL_V0902_REVIEW_GATE_INSPECT_ONLY:-0}"
ONLY_CHECKS=",${ADL_V0902_REVIEW_GATE_ONLY_CHECKS:-},"
FORCE_FAIL_CHECKS=",${ADL_V0902_REVIEW_GATE_FORCE_FAIL_CHECKS:-},"

mkdir -p "$OUT_DIR"
trap 'rm -rf "$RUN_TMPDIR"' EXIT

run_check() {
  local key="$1"
  shift
  local log="$OUT_DIR/$key.log"
  local log_rel="${log#$ROOT_DIR/}"
  if [[ "$ONLY_CHECKS" != ",," && "$ONLY_CHECKS" != *",$key,"* ]]; then
    printf '"%s":{"status":"SKIP","log":"%s","reason":"filtered"}' "$key" "$log_rel"
  elif "$@" >"$log" 2>&1; then
    if [[ "$FORCE_FAIL_CHECKS" == *",$key,"* ]]; then
      printf 'forced failure for %s\n' "$key" >>"$log"
      printf '"%s":{"status":"FAIL","log":"%s","reason":"forced_failure"}' "$key" "$log_rel"
    else
      printf '"%s":{"status":"PASS","log":"%s"}' "$key" "$log_rel"
    fi
  else
    printf '"%s":{"status":"FAIL","log":"%s"}' "$key" "$log_rel"
  fi
}

gate_mode() {
  if [[ "$INSPECT_ONLY" == "1" ]]; then
    printf 'inspect_only'
  elif [[ "$ONLY_CHECKS" != ",," ]]; then
    printf 'filtered'
  else
    printf 'required'
  fi
}

checks_json="{"
checks_json+="$(run_check skill_documentation_completeness bash "$ROOT_DIR/adl/tools/test_skill_documentation_completeness.sh"),"
checks_json+="$(run_check repo_review_specialist_contracts bash "$ROOT_DIR/adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh"),"
checks_json+="$(run_check multi_agent_repo_review_proof bash "$ROOT_DIR/adl/tools/test_demo_v0902_multi_agent_repo_review_proof.sh"),"
checks_json+="$(run_check arxiv_writer_field_test bash "$ROOT_DIR/adl/tools/test_demo_v0902_arxiv_writer_field_test.sh"),"
checks_json+="$(run_check paper_sonata_expansion bash "$ROOT_DIR/adl/tools/test_demo_v0902_paper_sonata_expansion.sh"),"
checks_json+="$(run_check milestone_dashboard bash "$ROOT_DIR/adl/tools/test_milestone_dashboard.sh"),"
checks_json+="$(run_check runtime_v2_feature_proof_coverage bash -lc "cd \"$ROOT_DIR\" && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture"),"
checks_json+="$(run_check runtime_v2_csm_integrated_run bash -lc "cd \"$ROOT_DIR\" && CARGO_TARGET_DIR=\"$BUILD_TARGET_DIR\" cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture")"
checks_json+="}"

printf '{"gate_id":"v0902-review-quality-gate","manifest_version":"adl.v0902.review_quality_gate.v1","mode":"%s","checks":%s}\n' "$(gate_mode)" "$checks_json" >"$MANIFEST"

cat >"$README" <<'EOF'
# v0.90.2 Review Quality Gate

Canonical command:

```bash
bash adl/tools/demo_v0902_review_quality_gate.sh
```

Primary proof surface:
- `artifacts/v0902/review_quality_gate/review_quality_gate_record.json`

Success signal:
- the review-quality record captures the bounded v0.90.2 internal/external review proof set without running the inherited full v0.90.1 quality-gate cargo-test and coverage sweep

Included checks:
- skill documentation completeness
- multi-agent repo-review specialist contract proof
- v0.90.2 multi-agent repo-review proof fixture
- v0.90.2 arXiv writer field-test packet
- v0.90.2 Paper Sonata expansion proof fixture
- milestone dashboard smoke proof
- focused Runtime v2 feature-proof coverage tests
- focused Runtime v2 integrated CSM run tests

Important boundary:
- this is the bounded v0.90.2 review proof wrapper for internal/external review confidence
- it does not replace full release ceremony validation
- it does not run full workspace `cargo test`, full clippy, or coverage gates
- use the inherited v0.90.1 quality gate only as broader release-tail support evidence
- required mode exits nonzero when any required check records FAIL
- set `ADL_V0902_REVIEW_GATE_INSPECT_ONLY=1` only for diagnostic artifact collection when preserving failed logs is more important than a green process signal
EOF

python3 - "$MANIFEST" "$INSPECT_ONLY" <<'PY'
import json
import sys

manifest_path, inspect_only = sys.argv[1], sys.argv[2]
manifest = json.load(open(manifest_path, encoding="utf-8"))
checks = manifest.get("checks", {})
failed = sorted(key for key, value in checks.items() if value.get("status") == "FAIL")
if failed:
    print("v0.90.2 review quality gate failed checks: " + ", ".join(failed), file=sys.stderr)
    print(f"review quality gate manifest preserved at {manifest_path}", file=sys.stderr)
    if inspect_only != "1":
        raise SystemExit(1)
PY
