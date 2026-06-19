#!/usr/bin/env python3
import json
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(f"FAIL validate_v0916_deepseek_suitability: {message}", file=sys.stderr)
    raise SystemExit(1)


EXPECTED_TASKS = {
    "watcher_state_v1",
    "card_validator_v1",
    "review_findings_v1",
    "bounded_planner_v1",
    "closeout_checker_v1",
    "worker_contract_v1",
}
EXPECTED_SCORES = {
    "pass",
    "pass_with_limits",
    "fail_format",
    "fail_truth",
    "fail_authority",
    "timeout_or_empty",
    "skipped_blocked",
}


def repo_root_from_packet_dir(packet_dir: Path) -> Path:
    for candidate in [packet_dir, *packet_dir.parents]:
        if (candidate / "adl").is_dir() and (candidate / "docs").is_dir():
            return candidate
    fail(f"could not determine repo root from {packet_dir}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0916_deepseek_suitability.py <packet_dir>")
    packet_dir = Path(sys.argv[1]).resolve()
    root = repo_root_from_packet_dir(packet_dir)
    state_paths = sorted(packet_dir.glob("*.json"))
    packet_paths = sorted(
        path for path in packet_dir.glob("*.md") if path.name.lower() != "readme.md"
    )
    if len(state_paths) != 1:
        fail("expected exactly one state file")
    state = json.loads(state_paths[0].read_text())
    if state.get("schema") not in {
        "adl.deepseek_csdlc_suitability.v1",
        "adl.agent_suitability_panel.v1",
    }:
        fail("unexpected state schema")
    artifacts = state.get("artifacts") or {}
    packet_ref = artifacts.get("packet")
    if not packet_ref:
        fail("state missing artifacts.packet")
    packet_path = root / packet_ref
    if not packet_path.exists():
        fail(f"missing packet artifact {packet_ref}")
    if packet_paths and packet_path not in packet_paths:
        fail("top-level packet markdown does not match state artifact ref")
    candidates = state.get("candidates") or []
    rows = state.get("rows") or []
    task_ids = state.get("task_ids") or sorted({row.get("task_id") for row in rows if row.get("task_id")})
    if not candidates:
        fail("state contains no candidates")
    if not rows:
        fail("state contains no task rows")
    candidate_ids = {candidate["candidate_id"] for candidate in candidates}
    seen = {(row["candidate_id"], row["task_id"]) for row in rows}
    for row in rows:
        if row.get("candidate_id") not in candidate_ids:
            fail(f"row references unknown candidate {row.get('candidate_id')}")
        if row.get("task_id") not in EXPECTED_TASKS:
            fail(f"unknown task id {row.get('task_id')}")
        if row.get("score") not in EXPECTED_SCORES:
            fail(f"unknown score {row.get('score')}")
        for key in ("raw_output_ref", "result_path", "log_path"):
            ref = row.get(key)
            if not ref:
                fail(f"missing {key} for {row.get('candidate_id')} / {row.get('task_id')}")
            if not (root / ref).exists():
                fail(f"missing referenced artifact {ref}")
    for candidate_id in candidate_ids:
        for task_id in task_ids:
            if (candidate_id, task_id) not in seen:
                fail(f"missing row for {candidate_id} / {task_id}")
    packet_text = packet_path.read_text()
    required_snippets = [
        "## Candidate matrix",
        "## Per-task evidence",
        "## Non-claims",
    ]
    for snippet in required_snippets:
        if snippet not in packet_text:
            fail(f"packet missing snippet {snippet}")
    print("PASS validate_v0916_deepseek_suitability")


if __name__ == "__main__":
    main()
