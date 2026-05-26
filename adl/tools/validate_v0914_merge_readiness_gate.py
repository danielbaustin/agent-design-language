#!/usr/bin/env python3
"""Validate the tracked v0.91.4 merge-readiness gate packet."""

from __future__ import annotations

import json
import sys
from pathlib import Path


REQUIRED_FILES = [
    "README.md",
    "MERGE_READINESS_GATE_PACKET_v0.91.4.md",
    "ct_demo_001_merge_gate_profile_report.md",
    "ct_demo_001_merge_gate_snapshot.json",
]

REQUIRED_REPORT_SNIPPETS = [
    "## Gate Identity",
    "## Validation Profile Truth",
    "## Lifecycle Blockers",
    "## Review / PR Truth Boundary",
    "## Evidence / Dependency Link",
    "## Structured Snapshot",
    "## Decision",
    "human review and merge authority remain required",
]

REQUIRED_SNAPSHOT = {
    "transition_id": "csdlc.v0_91_4.wp_07.ct_demo_001",
    "gate_kind": "governed_merge_readiness_gate.v2",
    "issue_number": 3355,
    "depends_on_issue_number": 3354,
    "depends_on_pr_number": 3388,
    "docs_only_finish_mode": "DocsOnly",
    "focused_local_ci_mode": "FocusedLocalCiGated",
    "focused_rust_test_filter": "cli::pr_cmd",
    "stale_lifecycle_blocker_test": "card_lifecycle_blocks_completed_sor_before_terminal_closeout",
    "docs_only_review_exception_test": "card_lifecycle_allows_explicit_srp_policy_exception",
    "remote_ci_state_preserved": True,
    "merge_truth_inferred_from_local_validation": False,
    "decision": "gate_hardened",
}


def fail(message: str) -> int:
    print(f"v0914_merge_readiness_gate: FAIL {message}", file=sys.stderr)
    return 1


def require_repo_relative_path(path: str, label: str) -> str | None:
    if path.startswith("/") or path.startswith("../") or "/../" in path:
        return f"{label} is not a safe repo-relative path: {path}"
    return None


def require_repo_relative_file(repo_root: Path, rel_path: str, label: str) -> str | None:
    error = require_repo_relative_path(rel_path, label)
    if error:
        return error
    if not (repo_root / rel_path).is_file():
        return f"{label} missing required tracked file: {rel_path}"
    return None


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_v0914_merge_readiness_gate.py <packet_root>")

    root = Path(sys.argv[1])
    if not root.is_dir():
        return fail(f"root is not a directory: {root}")

    missing = [name for name in REQUIRED_FILES if not (root / name).is_file()]
    if missing:
        return fail(f"missing required files: {', '.join(missing)}")

    report_path = root / "ct_demo_001_merge_gate_profile_report.md"
    report_text = report_path.read_text(encoding="utf-8")
    missing_snippets = [
        snippet for snippet in REQUIRED_REPORT_SNIPPETS if snippet not in report_text
    ]
    if missing_snippets:
        return fail("report missing required snippets: " + ", ".join(missing_snippets))

    snapshot_path = root / "ct_demo_001_merge_gate_snapshot.json"
    try:
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        return fail(f"snapshot is not valid JSON: {exc}")

    for key, expected in REQUIRED_SNAPSHOT.items():
        if snapshot.get(key) != expected:
            return fail(
                f"snapshot field {key!r} expected {expected!r}, found {snapshot.get(key)!r}"
            )

    repo_root = Path(__file__).resolve().parents[2]
    for label, rel_path in [
        ("feature doc path", snapshot.get("feature_doc_rel_path")),
        ("tooling policy path", snapshot.get("tooling_policy_rel_path")),
        ("evidence bundle path", snapshot.get("evidence_bundle_rel_path")),
        ("review synthesis path", snapshot.get("review_synthesis_rel_path")),
        ("signed trace path", snapshot.get("signed_trace_rel_path")),
    ]:
        if not isinstance(rel_path, str):
            return fail(f"{label} missing required string value")
        error = require_repo_relative_file(repo_root, rel_path, label)
        if error:
            return fail(error)
    markdown_expectations = [
        snapshot["transition_id"],
        f"[#${snapshot['issue_number']}]".replace("$", ""),
        f"[#${snapshot['depends_on_issue_number']}]".replace("$", ""),
        f"[#${snapshot['depends_on_pr_number']}]".replace("$", ""),
        f"`{snapshot['docs_only_finish_mode']}`",
        f"`{snapshot['focused_local_ci_mode']}`",
        f"`{snapshot['focused_rust_test_filter']}`",
        snapshot["stale_lifecycle_blocker_test"],
        snapshot["docs_only_review_exception_test"],
        "merge truth is not inferred from local validation",
        snapshot["feature_doc_rel_path"],
        snapshot["tooling_policy_rel_path"],
        snapshot["evidence_bundle_rel_path"],
        snapshot["review_synthesis_rel_path"],
        snapshot["signed_trace_rel_path"],
    ]
    missing_markdown = [item for item in markdown_expectations if item not in report_text]
    if missing_markdown:
        return fail(
            "report drifted from structured snapshot: " + ", ".join(missing_markdown)
        )

    print(f"v0914_merge_readiness_gate: PASS root={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
