#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0891_arxiv_manuscript_workflow.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/writer_skill_packet/writer_skill_status.json" \
  "$OUT_DIR/writer_skill_packet/workflow_contract.md" \
  "$OUT_DIR/source_packets/source_packet_manifest.json" \
  "$OUT_DIR/source_packets/what_is_adl_source_packet.md" \
  "$OUT_DIR/source_packets/godel_agents_and_adl_source_packet.md" \
  "$OUT_DIR/source_packets/cognitive_spacetime_manifold_source_packet.md" \
  "$OUT_DIR/manuscript_status/three_paper_status.json" \
  "$OUT_DIR/manuscript_status/what_is_adl_status.md" \
  "$OUT_DIR/manuscript_status/godel_agents_and_adl_status.md" \
  "$OUT_DIR/manuscript_status/cognitive_spacetime_manifold_status.md" \
  "$OUT_DIR/review/review_gates.json" \
  "$OUT_DIR/review/claim_boundaries.md" \
  "$OUT_DIR/review/reviewer_brief.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$ROOT_DIR" "$OUT_DIR/demo_manifest.json" "$OUT_DIR/writer_skill_packet/writer_skill_status.json" "$OUT_DIR/source_packets/source_packet_manifest.json" "$OUT_DIR/manuscript_status/three_paper_status.json" "$OUT_DIR/review/review_gates.json" <<'PY'
import json
import sys
from pathlib import Path

repo_root = Path(sys.argv[1])
manifest = json.load(open(sys.argv[2], encoding="utf-8"))
writer = json.load(open(sys.argv[3], encoding="utf-8"))
sources = json.load(open(sys.argv[4], encoding="utf-8"))
status = json.load(open(sys.argv[5], encoding="utf-8"))
gates = json.load(open(sys.argv[6], encoding="utf-8"))

assert manifest["schema_version"] == "adl.v0891.arxiv_manuscript_workflow_demo.v1"
assert manifest["demo_id"] == "D9"
assert manifest["disposition"] == "proving_packet_only"
assert manifest["dependency_truth"]["writer_skill_status"] == "wp08_contract_defined_packet_only"
assert manifest["security_privacy"]["submits_to_arxiv"] is False
assert writer["skill_status"] == "wp08_contract_defined_packet_only"
assert writer["runnable_in_this_demo"] is False
assert [role["order"] for role in writer["role_order"]] == [1, 2, 3, 4, 5]
assert sources["packet_count"] == 3
for packet in sources["packets"]:
    for source_ref in packet["source_refs"]:
        assert (repo_root / source_ref).exists(), source_ref
assert len(status["papers"]) == 3
assert {paper["title"] for paper in status["papers"]} == {
    "What Is ADL?",
    "Gödel Agents and ADL",
    "Cognitive Spacetime Manifold",
}
assert any(
    gate["gate_id"] == "wp08_writer_contract_defined" and gate["status"] == "pass"
    for gate in gates["gates"]
)
assert any(
    gate["gate_id"] == "wp13_manuscript_follow_through" and gate["status"] == "not_in_scope"
    for gate in gates["gates"]
)
PY

grep -Fq "Review this packet in the following order" "$OUT_DIR/review/reviewer_brief.md" || {
  echo "assertion failed: reviewer brief missing review order" >&2
  exit 1
}

grep -Fq "Review-ready manuscript packets are not final arXiv submissions" "$OUT_DIR/review/claim_boundaries.md" || {
  echo "assertion failed: claim boundary missing submission distinction" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/|Bearer |OPENAI_API_KEY|ANTHROPIC_API_KEY|\.adl/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: private path, control-plane path, or secret-like token leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v0891_arxiv_manuscript_workflow: ok"
