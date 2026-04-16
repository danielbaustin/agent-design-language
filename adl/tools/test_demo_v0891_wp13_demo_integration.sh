#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0891_wp13_demo_integration.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/integration_manifest.json" \
  "$OUT_DIR/demo_rows.json" \
  "$OUT_DIR/reviewer_brief.md" \
  "$OUT_DIR/dependency_and_scope.md" \
  "$OUT_DIR/validation_plan.json" \
  "$OUT_DIR/README.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$ROOT_DIR" "$OUT_DIR/integration_manifest.json" "$OUT_DIR/demo_rows.json" "$OUT_DIR/validation_plan.json" <<'PY'
import json
import sys
from pathlib import Path

repo_root = Path(sys.argv[1])
manifest = json.load(open(sys.argv[2], encoding="utf-8"))
rows = json.load(open(sys.argv[3], encoding="utf-8"))
validation = json.load(open(sys.argv[4], encoding="utf-8"))

assert manifest["schema_version"] == "adl.v0891.wp13_demo_integration.v1"
assert manifest["work_package"] == "WP-13"
assert manifest["issue"] == "#1934"
assert manifest["dependency_truth"]["wp12_issue"] == "#1933"
assert manifest["dependency_truth"]["wp12_state"] == "merged_before_wp13_publication"

by_id = {row["demo_id"]: row for row in manifest["demo_rows"]}
assert {demo_id for demo_id in by_id} == {"D7", "D8", "D9"}
assert by_id["D7"]["status"] == "LANDED"
assert by_id["D8"]["status"] == "LANDED"
assert by_id["D9"]["status"] == "LANDED"
assert any(command == "bash adl/tools/demo_v0891_five_agent_hey_jude.sh" for command in by_id["D8"]["entry_commands"])
assert any(command == "bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh" for command in by_id["D9"]["entry_commands"])
assert any(surface.endswith("performance_manifest.json") for surface in by_id["D8"]["primary_proof_surfaces"])
assert any(surface.endswith("three_paper_status.json") for surface in by_id["D9"]["primary_proof_surfaces"])

for row in (by_id["D8"], by_id["D9"]):
    for tracked_path in row["tracked_repo_paths"]:
        assert (repo_root / tracked_path).exists(), tracked_path

validation_commands = [item["command"] for item in validation["required_commands"]]
assert "bash adl/tools/test_demo_v0891_wp13_demo_integration.sh" in validation_commands
assert "bash adl/tools/test_demo_v0891_five_agent_hey_jude.sh" in validation_commands
assert "bash adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh" in validation_commands

assert rows["schema_version"] == "adl.v0891.wp13_demo_rows.v1"
assert [row["demo_id"] for row in rows["rows"]] == ["D7", "D8", "D9"]
PY

grep -Fq "This packet makes D7, D8, and D9 reviewer-legible together" "$OUT_DIR/reviewer_brief.md" || {
  echo "assertion failed: reviewer brief missing integration framing" >&2
  exit 1
}

grep -Fq "WP-13 depends on WP-12" "$OUT_DIR/dependency_and_scope.md" || {
  echo "assertion failed: dependency note missing WP-12 truth" >&2
  exit 1
}

grep -Fq "consumes the merged WP-12 convergence surface" "$OUT_DIR/dependency_and_scope.md" || {
  echo "assertion failed: dependency note missing merged WP-12 consumption truth" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/|Bearer |OPENAI_API_KEY|ANTHROPIC_API_KEY|ANTHROPIC_AUTH_TOKEN|GITHUB_TOKEN' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: private path or secret-like token leaked into generated artifacts" >&2
  exit 1
fi

if grep -R -F "submitted to arXiv" "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: generated artifacts imply arXiv submission" >&2
  exit 1
fi

echo "demo_v0891_wp13_demo_integration: ok"
