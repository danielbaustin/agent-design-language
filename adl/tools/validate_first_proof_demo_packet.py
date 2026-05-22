#!/usr/bin/env python3
"""
Validate the tracked WP-09 first-proof demo packet.

This validator is stronger than section-presence:
- required files must exist
- required upstream proof refs must exist
- the tracked metrics/report outputs must be reproducible from the tracked timeline snapshot
- packet/readme/report must all carry the bounded proof classification surface
"""

from __future__ import annotations

import filecmp
import subprocess
import sys
from pathlib import Path
from tempfile import TemporaryDirectory


REQUIRED_FILES = [
    "README.md",
    "FIRST_PROOF_DEMO_PACKET_v0.91.3.md",
    "ct_demo_001_timeline_snapshot.json",
    "ct_demo_001_first_proof_metrics.json",
    "ct_demo_001_first_proof_report.md",
]

REQUIRED_PACKET_SNIPPETS = [
    "## Scope",
    "## Packet Contents",
    "## Demo Command",
    "## Focused Validation",
    "## Proof Boundary",
    "literal five-minute target",
]

REQUIRED_REPORT_SNIPPETS = [
    "## Demo Identity",
    "## Executive Verdict",
    "## Key Metrics",
    "## Transition Timeline",
    "## Per-WP Timing",
    "## Proof Classification",
    "`proving`",
    "`non_proving`",
]

REQUIRED_SUPPORTING_PATHS = [
    "docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json",
    "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/README.md",
    "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md",
    "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md",
    "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_evidence_bundle.md",
    "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_review_synthesis.md",
    "docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md",
    "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md",
    "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json",
    "docs/milestones/v0.91.3/review/first_proof_readiness/FIRST_PROOF_READINESS_PACKET_v0.91.3.md",
]


def fail(message: str) -> int:
    print(f"first_proof_demo_packet: FAIL {message}", file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_first_proof_demo_packet.py <packet_root>")

    packet_root = Path(sys.argv[1]).resolve()
    if not packet_root.is_dir():
        return fail(f"packet root is not a directory: {packet_root}")

    repo = Path(__file__).resolve().parents[2]

    missing = [name for name in REQUIRED_FILES if not (packet_root / name).is_file()]
    if missing:
        return fail(f"missing required files: {', '.join(missing)}")

    packet_text = (packet_root / "FIRST_PROOF_DEMO_PACKET_v0.91.3.md").read_text(
        encoding="utf-8"
    )
    report_text = (packet_root / "ct_demo_001_first_proof_report.md").read_text(
        encoding="utf-8"
    )
    missing_packet = [snippet for snippet in REQUIRED_PACKET_SNIPPETS if snippet not in packet_text]
    if missing_packet:
        return fail("packet missing required snippets: " + ", ".join(missing_packet))
    missing_report = [snippet for snippet in REQUIRED_REPORT_SNIPPETS if snippet not in report_text]
    if missing_report:
        return fail("report missing required snippets: " + ", ".join(missing_report))

    if ".adl/docs/TBD" in packet_text or ".adl/docs/TBD" in report_text:
        return fail("tracked demo packet must not depend on local-only TBD notes")

    missing_paths = [
        rel for rel in REQUIRED_SUPPORTING_PATHS if not (repo / rel).is_file()
    ]
    if missing_paths:
        return fail("required supporting proof paths missing: " + ", ".join(missing_paths))

    with TemporaryDirectory() as tmpdir:
        tmp_root = Path(tmpdir)
        subprocess.run(
            [
                sys.executable,
                str(repo / "adl/tools/demo_v0913_first_proof_demo.py"),
                "--timeline",
                str(packet_root / "ct_demo_001_timeline_snapshot.json"),
                "--out",
                str(tmp_root),
            ],
            check=True,
            stdout=subprocess.DEVNULL,
        )
        for filename in ["ct_demo_001_first_proof_metrics.json", "ct_demo_001_first_proof_report.md"]:
            if not filecmp.cmp(packet_root / filename, tmp_root / filename, shallow=False):
                return fail(f"tracked {filename} drifted from deterministic generator output")

    print(f"first_proof_demo_packet: PASS root={packet_root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
