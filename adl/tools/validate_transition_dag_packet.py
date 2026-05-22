#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


REQUIRED_FILES = {
    "README.md": ["# Transition DAG Review Packet", "## Primary Proof Surfaces"],
    "TRANSITION_DAG_PROOF_PACKET_v0.91.3.md": [
        "# Transition DAG Proof Packet v0.91.3",
        "## Focused Validation",
    ],
    "ct_demo_001_transition_dag.md": [
        "# CT Demo 001 Transition DAG",
        "## Serial Nodes",
        "## Shard Nodes",
        "## Barrier Nodes",
        "barrier.review_barrier",
        "barrier.merge_readiness_barrier",
        "barrier.closeout_barrier",
        "coordination latency",
        "implementation time",
    ],
    "ct_demo_001_shard_plan.md": [
        "# CT Demo 001 Shard Plan",
        "## Shards",
        "## Interface Freeze Rules",
        "## Handoff Contracts",
        "## Barrier Contracts",
        "## Coordination Metrics Split",
    ],
}


def fail(message: str) -> int:
    print(message, file=sys.stderr)
    return 1


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        return fail("usage: validate_transition_dag_packet.py <packet-root>")

    root = Path(argv[1])
    if not root.is_dir():
        return fail(f"packet root does not exist: {root}")

    for rel_path, required_snippets in REQUIRED_FILES.items():
        path = root / rel_path
        if not path.is_file():
            return fail(f"missing required packet file: {path}")
        text = path.read_text(encoding="utf-8")
        for snippet in required_snippets:
            if snippet not in text:
                return fail(f"missing required snippet `{snippet}` in {path}")

    print(f"transition_dag_packet: PASS root={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
