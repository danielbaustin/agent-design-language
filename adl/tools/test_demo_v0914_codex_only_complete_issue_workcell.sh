#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0914_codex_only_complete_issue_workcell.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/run_manifest.json" \
  "$OUT_DIR/evidence/fixture_index.json" \
  "$OUT_DIR/lanes/worker_a_hosted_codex.md" \
  "$OUT_DIR/lanes/worker_b_hosted_codex.md" \
  "$OUT_DIR/review/reviewer_evidence.md" \
  "$OUT_DIR/gates/01_scope_bound.json" \
  "$OUT_DIR/gates/02_worker_lane_sync.json" \
  "$OUT_DIR/gates/03_reviewer_packet_ready.json" \
  "$OUT_DIR/gates/04_release_recommendation.json" \
  "$OUT_DIR/gates/gate_log.json" \
  "$OUT_DIR/README.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"schema_version": "adl.v0914.codex_only_complete_issue_workcell.v1"' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: manifest schema mismatch" >&2
  exit 1
}

grep -Fq '"hosted_worker_count": 2' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: hosted worker count missing" >&2
  exit 1
}

grep -Fq '"publication_allowed": false' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: publication boundary flag missing" >&2
  exit 1
}

grep -Fq '"merge_approval_claimed": false' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: merge approval boundary flag missing" >&2
  exit 1
}

grep -Fq '"cli_model_invocation": false' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: CLI model invocation guard missing" >&2
  exit 1
}

grep -Fq 'Hosted Codex Worker Lane A' "$OUT_DIR/lanes/worker_a_hosted_codex.md" || {
  echo "assertion failed: worker A lane header missing" >&2
  exit 1
}

grep -Fq 'Hosted Codex Worker Lane B' "$OUT_DIR/lanes/worker_b_hosted_codex.md" || {
  echo "assertion failed: worker B lane header missing" >&2
  exit 1
}

grep -Fq '[P2] Serialized gates are mandatory' "$OUT_DIR/review/reviewer_evidence.md" || {
  echo "assertion failed: reviewer finding missing" >&2
  exit 1
}

grep -Fq '"serialization": [' "$OUT_DIR/gates/gate_log.json" || {
  echo "assertion failed: gate serialization log missing" >&2
  exit 1
}

grep -Fq '"depends_on": [' "$OUT_DIR/gates/04_release_recommendation.json" || {
  echo "assertion failed: gate dependency record missing" >&2
  exit 1
}

python3 - "$OUT_DIR" <<'PY'
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
log = json.loads((out / "gates" / "gate_log.json").read_text(encoding="utf-8"))
expected = [
    "01_scope_bound",
    "02_worker_lane_sync",
    "03_reviewer_packet_ready",
    "04_release_recommendation",
]
if log.get("serialization") != expected:
    raise SystemExit("assertion failed: serialized gate order mismatch")
for gate_id in expected[1:]:
    gate = json.loads((out / "gates" / f"{gate_id}.json").read_text(encoding="utf-8"))
    if not gate.get("depends_on"):
        raise SystemExit(f"assertion failed: {gate_id} should depend on an earlier gate")
PY

CUSTOM_DIR="$TMPDIR_ROOT/existing-custom-output"
mkdir -p "$CUSTOM_DIR"
printf 'keep\n' > "$CUSTOM_DIR/keep.txt"
if bash "$ROOT_DIR/adl/tools/demo_v0914_codex_only_complete_issue_workcell.sh" "$CUSTOM_DIR" >/dev/null 2>&1; then
  echo "assertion failed: demo accepted unsafe existing custom output directory" >&2
  exit 1
fi
[[ -f "$CUSTOM_DIR/keep.txt" ]] || {
  echo "assertion failed: unsafe custom output directory contents were removed" >&2
  exit 1
}

echo "demo_v0914_codex_only_complete_issue_workcell: ok"
