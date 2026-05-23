#!/usr/bin/env python3
"""Validate the tracked merge-readiness packet contract."""
import sys
import json
from pathlib import Path


REQUIRED_FILES = [
    "README.md",
    "MERGE_READINESS_PROOF_PACKET_v0.91.3.md",
    "ct_demo_001_merge_gate.md",
    "ct_demo_001_merge_gate_snapshot.json",
]

REQUIRED_GATE_SNIPPETS = [
    "## Gate Identity",
    "## Issue / Branch / Worktree Truth",
    "## PR / CI Truth",
    "## Review Truth",
    "## Evidence Bundle Link",
    "## Structured Snapshot",
    "## Blocked Conditions",
    "## Decision",
    "human merge review remains required",
    "`merge_ready`",
]

REQUIRED_SNAPSHOT_KEYS = {
    "transition_id": "cts.v0_91_3.issue_3200.ct_demo_001",
    "gate_kind": "governed_merge_readiness_gate.v1",
    "decision_mode": "reviewable_record",
    "outcome": "merge_ready",
    "issue_number": 3203,
    "issue_state": "CLOSED",
    "pr_number": 3243,
    "pr_state": "MERGED",
    "base_branch": "main",
    "adl_ci": "SUCCESS",
    "adl_coverage": "SUCCESS",
    "bounded_review_outcome": "PASS",
    "open_bounded_review_findings": 0,
    "human_merge_review_required": True,
    "decision": "merge_ready",
}


def require_repo_relative_path(rel_path: str, label: str) -> str | None:
    if rel_path.startswith("/") or rel_path.startswith("../") or "/../" in rel_path:
        return f"{label} is not a safe repo-relative path: {rel_path}"
    return None


def require_repo_relative_file(repo_root: Path, rel_path: str, label: str) -> str | None:
    error = require_repo_relative_path(rel_path, label)
    if error:
        return error
    if not (repo_root / rel_path).is_file():
        return f"{label} missing required tracked file: {rel_path}"
    return None


def fail(message: str) -> int:
    print(f"merge_readiness_packet: FAIL {message}", file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_merge_readiness_packet.py <packet_root>")

    root = Path(sys.argv[1])
    if not root.is_dir():
        return fail(f"root is not a directory: {root}")

    missing = [name for name in REQUIRED_FILES if not (root / name).is_file()]
    if missing:
        return fail(f"missing required files: {', '.join(missing)}")

    gate_path = root / "ct_demo_001_merge_gate.md"
    gate_text = gate_path.read_text(encoding="utf-8")
    missing_snippets = [snippet for snippet in REQUIRED_GATE_SNIPPETS if snippet not in gate_text]
    if missing_snippets:
        return fail(
            "gate record missing required snippets: " + ", ".join(missing_snippets)
        )

    snapshot_path = root / "ct_demo_001_merge_gate_snapshot.json"
    try:
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        return fail(f"snapshot is not valid JSON: {exc}")

    for key, expected in REQUIRED_SNAPSHOT_KEYS.items():
        if snapshot.get(key) != expected:
            return fail(
                f"snapshot field {key!r} expected {expected!r}, found {snapshot.get(key)!r}"
            )

    repo_root = Path(__file__).resolve().parents[2]
    evidence_path = snapshot.get("evidence_bundle_rel_path")
    review_path = snapshot.get("review_synthesis_rel_path")
    sor_path = snapshot.get("sor_rel_path")
    for label, rel_path in [
        ("evidence bundle path", evidence_path),
        ("review synthesis path", review_path),
    ]:
        if not isinstance(rel_path, str):
            return fail(f"{label} missing required string value")
        error = require_repo_relative_file(repo_root, rel_path, label)
        if error:
            return fail(error)
    if not isinstance(sor_path, str):
        return fail("SOR path missing required string value")
    error = require_repo_relative_path(sor_path, "SOR path")
    if error:
        return fail(error)

    markdown_expectations = [
        snapshot["transition_id"],
        f"[#{snapshot['issue_number']}]",
        f"[#{snapshot['pr_number']}]",
        f"PR state: `{snapshot['pr_state']}`",
        f"base branch: `{snapshot['base_branch']}`",
        f"`adl-ci`: `{snapshot['adl_ci']}`",
        f"`adl-coverage`: `{snapshot['adl_coverage']}`",
        f"decision: `{snapshot['decision']}`",
        f"outcome: `{snapshot['outcome']}`",
        evidence_path,
        review_path,
        sor_path,
    ]
    missing_markdown_expectations = [
        snippet for snippet in markdown_expectations if snippet not in gate_text
    ]
    if missing_markdown_expectations:
        return fail(
            "gate record drifted from structured snapshot: "
            + ", ".join(missing_markdown_expectations)
        )

    print(f"merge_readiness_packet: PASS root={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
