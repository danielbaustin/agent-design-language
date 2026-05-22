#!/usr/bin/env python3
"""
Validate the tracked merge-readiness packet contract.

This validator is intentionally shape-oriented for Sprint 2: it checks required
files and required packet sections/snippets. It does not yet query live GitHub
truth, CI status, or linked artifact existence beyond packet-local files.
"""
import sys
from pathlib import Path


REQUIRED_FILES = [
    "README.md",
    "MERGE_READINESS_PROOF_PACKET_v0.91.3.md",
    "ct_demo_001_merge_gate.md",
]

REQUIRED_GATE_SNIPPETS = [
    "## Gate Identity",
    "## Issue / Branch / Worktree Truth",
    "## PR / CI Truth",
    "## Review Truth",
    "## Evidence Bundle Link",
    "## Blocked Conditions",
    "## Decision",
    "human merge review remains required",
    "`merge_ready`",
]


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

    gate_text = (root / "ct_demo_001_merge_gate.md").read_text(encoding="utf-8")
    missing_snippets = [snippet for snippet in REQUIRED_GATE_SNIPPETS if snippet not in gate_text]
    if missing_snippets:
        return fail(
            "gate record missing required snippets: " + ", ".join(missing_snippets)
        )

    print(f"merge_readiness_packet: PASS root={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
