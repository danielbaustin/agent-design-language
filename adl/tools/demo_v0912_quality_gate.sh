#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0912/quality_gate}"
MANIFEST="$OUT_DIR/quality_gate_record.json"
README="$OUT_DIR/README.md"
INSPECT_ONLY="${ADL_V0912_QUALITY_GATE_INSPECT_ONLY:-0}"
ONLY_CHECKS=",${ADL_V0912_QUALITY_GATE_ONLY_CHECKS:-},"
FORCE_FAIL_CHECKS=",${ADL_V0912_QUALITY_GATE_FORCE_FAIL_CHECKS:-},"
RUN_HEAVY="${ADL_V0912_QUALITY_GATE_RUN_HEAVY:-0}"

mkdir -p "$OUT_DIR"

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
checks_json+="$(run_check ci_path_policy_contract bash "$ROOT_DIR/adl/tools/test_ci_path_policy.sh"),"
checks_json+="$(run_check ci_runtime_contracts bash "$ROOT_DIR/adl/tools/test_ci_runtime_contracts.sh"),"
checks_json+="$(run_check authoritative_coverage_lane_contract bash "$ROOT_DIR/adl/tools/test_run_authoritative_coverage_lane.sh"),"
checks_json+="$(run_check coverage_impact_contract bash "$ROOT_DIR/adl/tools/test_check_coverage_impact.sh"),"
checks_json+="$(run_check quality_gate_doc_surface grep -Fq '## Current Gate Dimensions' "$ROOT_DIR/docs/milestones/v0.91.2/QUALITY_GATE_v0.91.2.md"),"
checks_json+="$(run_check quality_gate_packet_surface grep -Fq 'Current Gate Dimensions' "$ROOT_DIR/docs/milestones/v0.91.2/review/quality_gate/QUALITY_GATE_PACKET_v0.91.2.md"),"
checks_json+="$(run_check feature_proof_row grep -Fq '| Demo/proof convergence | WP-17 |' "$ROOT_DIR/docs/milestones/v0.91.2/FEATURE_PROOF_COVERAGE_v0.91.2.md"),"
checks_json+="$(run_check authoritative_coverage_plan bash -lc "cd \"$ROOT_DIR\" && bash adl/tools/run_authoritative_coverage_lane.sh --print-plan"),"
if [[ "$RUN_HEAVY" == "1" ]]; then
  checks_json+="$(run_check fmt cargo fmt --manifest-path "$ROOT_DIR/adl/Cargo.toml" --all --check),"
  checks_json+="$(run_check clippy bash -lc "cd \"$ROOT_DIR\" && cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"),"
  checks_json+="$(run_check authoritative_coverage_full_run bash -lc "cd \"$ROOT_DIR\" && bash adl/tools/run_authoritative_coverage_lane.sh"),"
else
  checks_json+='"fmt":{"status":"SKIP","log":"artifacts/v0912/quality_gate/fmt.log","reason":"heavy_checks_not_requested"},'
  checks_json+='"clippy":{"status":"SKIP","log":"artifacts/v0912/quality_gate/clippy.log","reason":"heavy_checks_not_requested"},'
  checks_json+='"authoritative_coverage_full_run":{"status":"SKIP","log":"artifacts/v0912/quality_gate/authoritative_coverage_full_run.log","reason":"heavy_checks_not_requested"}'
fi
checks_json+="}"

printf '{"demo_id":"D18","manifest_version":"adl.v0912.quality_gate.v1","mode":"%s","heavy_mode":"%s","checks":%s}\n' "$(gate_mode)" "$RUN_HEAVY" "$checks_json" >"$MANIFEST"

cat >"$README" <<'EOF'
# v0.91.2 Quality Gate Walkthrough

Canonical command:

```bash
bash adl/tools/demo_v0912_quality_gate.sh
```

Primary proof surface:
- `artifacts/v0912/quality_gate/quality_gate_record.json`

Success signal:
- the quality-gate record captures the current `v0.91.2` CI/coverage policy contracts, closed-issue closeout-truth gate, and tracked quality-gate packet/doc surfaces in one reviewable location with per-check logs

Important boundaries:
- this walkthrough is a reviewer-facing aggregation surface for `WP-18`
- it does not by itself declare `v0.91.2` release-ready
- docs-only or contract-only checks are not the same thing as full release coverage evidence
- set `ADL_V0912_QUALITY_GATE_RUN_HEAVY=1` only when you explicitly want `fmt`, `clippy`, and the authoritative full coverage lane in the same run
- later Sprint 4 review, remediation, and ceremony work still gate final milestone closeout even if this walkthrough is green
EOF

python3 - "$MANIFEST" "$INSPECT_ONLY" <<'PY'
import json
import sys

manifest_path, inspect_only = sys.argv[1], sys.argv[2]
manifest = json.load(open(manifest_path, encoding="utf-8"))
checks = manifest.get("checks", {})
failed = sorted(key for key, value in checks.items() if value.get("status") == "FAIL")
if failed:
    print("quality gate failed checks: " + ", ".join(failed), file=sys.stderr)
    print(f"quality gate manifest preserved at {manifest_path}", file=sys.stderr)
    if inspect_only != "1":
        raise SystemExit(1)
PY
