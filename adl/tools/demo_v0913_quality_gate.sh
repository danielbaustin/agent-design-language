#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0913/quality_gate}"
MANIFEST="$OUT_DIR/quality_gate_record.json"
README="$OUT_DIR/README.md"
INSPECT_ONLY="${ADL_V0913_QUALITY_GATE_INSPECT_ONLY:-0}"
ONLY_CHECKS=",${ADL_V0913_QUALITY_GATE_ONLY_CHECKS:-},"
FORCE_FAIL_CHECKS=",${ADL_V0913_QUALITY_GATE_FORCE_FAIL_CHECKS:-},"
RUN_HEAVY="${ADL_V0913_QUALITY_GATE_RUN_HEAVY:-0}"

mkdir -p "$OUT_DIR"
mkdir -p "$OUT_DIR/logs"

run_check() {
  local key="$1"
  shift
  local log="$OUT_DIR/logs/$key.log"
  local log_rel="logs/$key.log"
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
checks_json+="$(run_check transition_manifest_schema cargo test --manifest-path "$ROOT_DIR/adl/Cargo.toml" cognitive_transition_schema -- --nocapture),"
checks_json+="$(run_check transition_dag_packet python3 "$ROOT_DIR/adl/tools/validate_transition_dag_packet.py" "$ROOT_DIR/docs/milestones/v0.91.3/review/transition_dag"),"
checks_json+="$(run_check evidence_bundle_packet python3 "$ROOT_DIR/adl/tools/validate_evidence_bundle_packet.py" "$ROOT_DIR/docs/milestones/v0.91.3/review/evidence_bundle"),"
checks_json+="$(run_check merge_readiness_packet python3 "$ROOT_DIR/adl/tools/validate_merge_readiness_packet.py" "$ROOT_DIR/docs/milestones/v0.91.3/review/merge_readiness"),"
checks_json+="$(run_check obsmem_handoff_packet python3 "$ROOT_DIR/adl/tools/validate_obsmem_handoff_packet.py" "$ROOT_DIR/docs/milestones/v0.91.3/review/obsmem_handoff"),"
checks_json+="$(run_check first_proof_readiness_packet python3 "$ROOT_DIR/adl/tools/validate_first_proof_readiness_packet.py" "$ROOT_DIR/docs/milestones/v0.91.3/review/first_proof_readiness"),"
checks_json+="$(run_check first_proof_demo_packet python3 "$ROOT_DIR/adl/tools/validate_first_proof_demo_packet.py" "$ROOT_DIR/docs/milestones/v0.91.3/review/first_proof_demo"),"
checks_json+="$(run_check quality_gate_doc_surface python3 "$ROOT_DIR/adl/tools/validate_v0913_quality_gate_review_surfaces.py" "$ROOT_DIR" quality_gate_doc),"
checks_json+="$(run_check quality_gate_packet_surface python3 "$ROOT_DIR/adl/tools/validate_v0913_quality_gate_review_surfaces.py" "$ROOT_DIR" quality_gate_packet),"
checks_json+="$(run_check demo_coverage_surface python3 "$ROOT_DIR/adl/tools/validate_v0913_quality_gate_review_surfaces.py" "$ROOT_DIR" demo_coverage),"
checks_json+="$(run_check demo_matrix_wp11_row grep -Fq '| Sprint 4 quality gate | WP-11 / #3227 |' "$ROOT_DIR/docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md"),"
if [[ "$RUN_HEAVY" == "1" ]]; then
  checks_json+="$(run_check fmt cargo fmt --manifest-path "$ROOT_DIR/adl/Cargo.toml" --all --check),"
  checks_json+="$(run_check clippy bash -lc "cd \"$ROOT_DIR\" && cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings")"
else
  checks_json+='"fmt":{"status":"SKIP","log":"artifacts/v0913/quality_gate/fmt.log","reason":"heavy_checks_not_requested"},'
  checks_json+='"clippy":{"status":"SKIP","log":"artifacts/v0913/quality_gate/clippy.log","reason":"heavy_checks_not_requested"}'
fi
checks_json+="}"

printf '{"demo_id":"D19","manifest_version":"adl.v0913.quality_gate.v1","mode":"%s","heavy_mode":"%s","checks":%s}\n' "$(gate_mode)" "$RUN_HEAVY" "$checks_json" >"$MANIFEST"

cat >"$README" <<'EOF'
# v0.91.3 Quality Gate Walkthrough

Canonical command:

```bash
bash adl/tools/demo_v0913_quality_gate.sh
```

Primary proof surface:
- `artifacts/v0913/quality_gate/quality_gate_record.json`

Success signal:
- the quality-gate record captures the strongest current `v0.91.3` first-slice validators, demo-proof validators, and tracked quality-gate/demo-coverage docs in one reviewable location with per-check logs

Important boundaries:
- this walkthrough is the reviewer-facing aggregation surface for `WP-11`
- it does not by itself declare `v0.91.3` docs-reviewed, internally reviewed, externally reviewed, or release-ready
- it intentionally aggregates focused proof lanes rather than running a broad repo-wide test cycle by default
- set `ADL_V0913_QUALITY_GATE_RUN_HEAVY=1` only when you explicitly want `fmt` and `clippy` folded into the same run
- later Sprint 4 docs review, internal review, remediation, next-milestone planning, and release work still gate final milestone closeout
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
