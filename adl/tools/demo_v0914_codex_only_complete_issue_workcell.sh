#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0914/codex_only_complete_issue_workcell}"

artifact_label="custom-artifact-root"
case "$OUT_DIR" in
  "$ROOT_DIR"/*)
    artifact_label="${OUT_DIR#"$ROOT_DIR"/}"
    ;;
esac

if [[ -e "$OUT_DIR" ]]; then
  if [[ ! -d "$OUT_DIR" ]]; then
    echo "refusing to overwrite non-directory output path: $OUT_DIR" >&2
    exit 1
  fi
  case "$OUT_DIR" in
    "$ROOT_DIR"/artifacts/v0914/codex_only_complete_issue_workcell|\
    "$ROOT_DIR"/artifacts/v0914/codex_only_complete_issue_workcell/*|\
    /tmp/*|/private/tmp/*)
      ;;
    *)
      echo "refusing to delete existing custom output directory outside approved demo roots: $OUT_DIR" >&2
      exit 1
      ;;
  esac
fi

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"/{lanes,review,gates,evidence}

python3 - "$OUT_DIR" "$artifact_label" <<'PY'
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
artifact_label = sys.argv[2]


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.strip() + "\n", encoding="utf-8")


def write_json(path: Path, payload: dict) -> None:
    write(path, json.dumps(payload, indent=2, sort_keys=True))


manifest = {
    "schema_version": "adl.v0914.codex_only_complete_issue_workcell.v1",
    "demo_id": "v0914-codex-only-complete-issue-workcell",
    "classification": "proving_fixture",
    "classification_reason": "Deterministic replayable proof surface for a Codex-only complete-issue workcell using hosted worker lanes, reviewer evidence, and serialized gates without live provider execution.",
    "artifact_root": artifact_label,
    "live_provider_execution": False,
    "cli_model_invocation": False,
    "hosted_worker_count": 2,
    "reviewer_present": True,
    "serialized_gate_count": 4,
    "publication_allowed": False,
    "merge_approval_claimed": False,
    "issue_number": 3484,
    "ownership": {
        "worker_a": "demo lane synthesis and gate serialization fixture",
        "worker_b": "parallel implementation lane fixture",
        "reviewer": "bounded evidence review and release recommendation fixture",
    },
}
write_json(out / "run_manifest.json", manifest)

write_json(
    out / "evidence" / "fixture_index.json",
    {
        "schema_version": "adl.v0914.codex_only_complete_issue_workcell.fixture_index.v1",
        "entries": [
            {
                "id": "lane_a",
                "artifact": "lanes/worker_a_hosted_codex.md",
                "kind": "hosted_worker_lane",
                "claim": "Worker A prepares a deterministic bounded implementation lane.",
            },
            {
                "id": "lane_b",
                "artifact": "lanes/worker_b_hosted_codex.md",
                "kind": "hosted_worker_lane",
                "claim": "Worker B prepares a second hosted Codex lane with non-overlapping evidence.",
            },
            {
                "id": "reviewer",
                "artifact": "review/reviewer_evidence.md",
                "kind": "review_surface",
                "claim": "Reviewer records findings-first evidence and gate disposition.",
            },
            {
                "id": "gate_log",
                "artifact": "gates/gate_log.json",
                "kind": "serialized_gate_record",
                "claim": "Execution proceeds through explicit serialized gates with stable outcomes.",
            },
        ],
    },
)

write(
    out / "lanes" / "worker_a_hosted_codex.md",
    """
# Hosted Codex Worker Lane A

## Identity
- Lane: `worker-a`
- Runtime: hosted Codex worker
- Issue: `#3484`
- Scope: bounded replayable demo/proof runner surface

## Inputs
- Repository contract: `AGENTS.md`
- Owned files: `adl/tools/demo_v0914_codex_only_complete_issue_workcell.sh`, `adl/tools/test_demo_v0914_codex_only_complete_issue_workcell.sh`
- Constraints: no secrets, no live provider calls, no CLI model invocation

## Deterministic Work Packet
1. Serialize the workcell claim into fixture artifacts.
2. Emit explicit gate receipts before reviewer release.
3. Keep all evidence repository-relative.

## Evidence Produced
- `run_manifest.json`
- `gates/01_scope_bound.json`
- `gates/02_worker_lane_sync.json`

## Lane Result
- Status: passed
- Reviewer handoff: ready
- Notes: lane A owns orchestration framing and first half of the gate chain.
""",
)

write(
    out / "lanes" / "worker_b_hosted_codex.md",
    """
# Hosted Codex Worker Lane B

## Identity
- Lane: `worker-b`
- Runtime: hosted Codex worker
- Issue: `#3484`
- Scope: companion implementation and reviewer-facing proof evidence

## Deterministic Work Packet
1. Mirror the hosted-worker shape with an independent evidence lane.
2. Produce reviewer-readable implementation notes.
3. Stop before merge or publication authority.

## Evidence Produced
- `gates/03_reviewer_packet_ready.json`
- `gates/04_release_recommendation.json`
- `review/reviewer_evidence.md`

## Lane Result
- Status: passed
- Reviewer handoff: ready
- Notes: lane B proves the workcell requires at least two hosted Codex workers before serialized review release.
""",
)

write(
    out / "review" / "reviewer_evidence.md",
    """
# Reviewer Evidence

## Review Mode
- Reviewer: hosted Codex reviewer lane
- Method: documentary fixture replay
- Findings-first: yes

## Findings
### [P2] Serialized gates are mandatory for the complete-issue workcell claim
- Evidence: `gates/gate_log.json` records four ordered gates with no parallel release shortcut.
- Why it matters: the proof would overclaim autonomy if reviewer release could happen before explicit gate receipts.
- Recommended action: keep gate receipts as first-class artifacts in any follow-on proof packet.

## Non-Findings
- No secrets are referenced.
- No live provider execution is claimed.
- No CLI model invocation is used.

## Reviewer Disposition
- Result: acceptable as replayable proof surface
- Publication: blocked by design
- Merge approval: not claimed
""",
)

gates = [
    {
        "gate_id": "01_scope_bound",
        "title": "Scope Bound",
        "depends_on": [],
        "result": "passed",
        "receipt": "Owned-file-only execution surface confirmed.",
    },
    {
        "gate_id": "02_worker_lane_sync",
        "title": "Worker Lane Sync",
        "depends_on": ["01_scope_bound"],
        "result": "passed",
        "receipt": "Two hosted Codex worker lanes recorded with complementary evidence.",
    },
    {
        "gate_id": "03_reviewer_packet_ready",
        "title": "Reviewer Packet Ready",
        "depends_on": ["02_worker_lane_sync"],
        "result": "passed",
        "receipt": "Reviewer evidence assembled after both worker lanes completed.",
    },
    {
        "gate_id": "04_release_recommendation",
        "title": "Release Recommendation",
        "depends_on": ["03_reviewer_packet_ready"],
        "result": "passed",
        "receipt": "Reviewer accepts the proof shape while preserving publication and merge limits.",
    },
]

for gate in gates:
    write_json(out / "gates" / f"{gate['gate_id']}.json", gate)

write_json(
    out / "gates" / "gate_log.json",
    {
        "schema_version": "adl.v0914.codex_only_complete_issue_workcell.gate_log.v1",
        "serialization": [gate["gate_id"] for gate in gates],
        "gates": gates,
    },
)

write(
    out / "README.md",
    """
# v0914 Codex-Only Complete-Issue Workcell

This artifact is a deterministic replayable proof surface for issue `#3484`.
It demonstrates a Codex-only complete-issue workcell with:

- at least two hosted Codex worker lanes
- reviewer evidence
- explicit serialized gates
- repository-relative artifacts only
- no secrets, live provider calls, or CLI model invocations

The fixture proves workcell shape and review discipline rather than live runtime autonomy.
""",
)
PY

echo "wrote fixture to $OUT_DIR"
