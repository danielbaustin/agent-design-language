#!/usr/bin/env python3
"""Read-only v0.90 milestone compression drift check.

This is intentionally small: it validates the opened v0.90 issue wave against
the tracked milestone docs and proof-packet directories without mutating repo
state or asking GitHub for live issue status.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class WorkPackage:
    wp: str
    issue: str
    title: str
    queue: str
    outcome: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument(
        "--state",
        default="docs/milestones/v0.90/milestone_compression/CANONICAL_MILESTONE_STATE_v0.90.yaml",
        help="canonical state model path",
    )
    return parser.parse_args()


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError as exc:
        raise SystemExit(f"missing required file: {path}") from exc


def parse_wave(text: str) -> list[WorkPackage]:
    packages: list[WorkPackage] = []
    current: dict[str, str] = {}
    for raw in text.splitlines():
        line = raw.rstrip()
        if line.startswith("  - wp: "):
            if current:
                packages.append(build_wp(current))
            current = {"wp": line.split(": ", 1)[1].strip()}
            continue
        if not current:
            continue
        match = re.match(r"^    (title|queue|issue|outcome): (.+)$", line)
        if match:
            key, value = match.groups()
            current[key] = value.strip().strip('"')
    if current:
        packages.append(build_wp(current))
    return packages


def build_wp(raw: dict[str, str]) -> WorkPackage:
    missing = {"wp", "issue", "title", "queue", "outcome"} - raw.keys()
    if missing:
        raise SystemExit(f"invalid wave entry for {raw.get('wp', '<unknown>')}: missing {sorted(missing)}")
    return WorkPackage(
        wp=raw["wp"],
        issue=raw["issue"],
        title=raw["title"],
        queue=raw["queue"],
        outcome=raw["outcome"],
    )


def status(key: str, state_text: str) -> str:
    pattern = re.compile(rf"^  {re.escape(key)}: (.+)$", re.MULTILINE)
    match = pattern.search(state_text)
    if not match:
        raise SystemExit(f"state model missing {key}")
    return match.group(1).strip()


def main() -> int:
    args = parse_args()
    root = Path(args.root)
    milestone = root / "docs/milestones/v0.90"
    state_text = read(root / args.state)
    wave_path = milestone / "WP_ISSUE_WAVE_v0.90.yaml"
    wave_text = read(wave_path)
    packages = parse_wave(wave_text)

    results: list[tuple[str, str, str]] = []

    if len(packages) == 20:
        results.append(("PASS", "wave_count", "20 work packages recorded"))
    else:
        results.append(("FAIL", "wave_count", f"expected 20 work packages, found {len(packages)}"))

    docs_to_check = [
        milestone / "README.md",
        milestone / "WBS_v0.90.md",
        milestone / "SPRINT_v0.90.md",
        milestone / "WP_ISSUE_WAVE_v0.90.yaml",
    ]
    combined = "\n".join(read(path) for path in docs_to_check)
    for wp in packages:
        needle = f"#{wp.issue}"
        if needle in combined:
            results.append(("PASS", f"{wp.wp}_issue_ref", f"{wp.wp} maps to {needle}"))
        else:
            results.append(("FAIL", f"{wp.wp}_issue_ref", f"{wp.wp} missing {needle} in core docs"))

    repo_visibility_status = status("repo_visibility_packet", state_text)
    repo_visibility_dir = milestone / "repo_visibility"
    if repo_visibility_dir.is_dir() and repo_visibility_status == "landed":
        results.append(("PASS", "repo_visibility_packet", "repo visibility packet directory exists and state says landed"))
    elif repo_visibility_dir.is_dir():
        results.append(("KNOWN_MISMATCH", "repo_visibility_packet", f"directory exists but state says {repo_visibility_status}"))
    else:
        results.append(("FAIL", "repo_visibility_packet", "repo visibility packet directory missing"))

    compression_status = status("milestone_compression_packet", state_text)
    compression_dir = milestone / "milestone_compression"
    if compression_dir.is_dir() and compression_status == "landed":
        results.append(("PASS", "milestone_compression_packet", "compression packet directory exists and state says landed"))
    elif compression_dir.is_dir():
        results.append(("KNOWN_MISMATCH", "milestone_compression_packet", f"directory exists but state says {compression_status}"))
    else:
        results.append(("FAIL", "milestone_compression_packet", "compression packet directory missing"))

    demo_matrix = read(milestone / "DEMO_MATRIX_v0.90.md")
    expected_pending = [
        ("D1", "planned"),
        ("D2", "planned"),
        ("D3", "planned"),
        ("D4", "planned"),
        ("D5", "planned"),
    ]
    for demo_id, expected in expected_pending:
        if re.search(rf"\| {demo_id} \|.*\| {expected} \|", demo_matrix):
            results.append(("PASS", f"{demo_id}_status", f"{demo_id} remains truthfully {expected}"))
        else:
            results.append(("KNOWN_MISMATCH", f"{demo_id}_status", f"{demo_id} is no longer classified as {expected}"))

    for result in results:
        print(" | ".join(result))

    if any(level == "FAIL" for level, _, _ in results):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
