#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> int:
    print(f"v0913_quality_gate_review_surfaces: FAIL {message}", file=sys.stderr)
    return 1


def require_text(path: Path, snippets: list[str]) -> str | None:
    text = path.read_text(encoding="utf-8")
    missing = [snippet for snippet in snippets if snippet not in text]
    if missing:
        return f"{path} missing required snippets: {', '.join(missing)}"
    return None


def main() -> int:
    if len(sys.argv) != 3:
        return fail(
            "usage: validate_v0913_quality_gate_review_surfaces.py <repo_root> <surface>"
        )

    repo_root = Path(sys.argv[1]).resolve()
    surface = sys.argv[2]
    docs_root = repo_root / "docs/milestones/v0.91.3"

    if surface == "quality_gate_doc":
        path = docs_root / "QUALITY_GATE_v0.91.3.md"
        error = require_text(
            path,
            [
                "## Primary Run Path",
                "bash adl/tools/demo_v0913_quality_gate.sh",
                "## Current Gate Dimensions",
                "## Review Gate",
                "## Blockers",
                "## Non-Claims",
                "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md",
            ],
        )
        if error:
            return fail(error)
        linked = docs_root / "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md"
        if not linked.is_file():
            return fail(f"missing linked packet surface: {linked}")
    elif surface == "quality_gate_packet":
        path = docs_root / "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md"
        error = require_text(
            path,
            [
                "## Scope",
                "## Packet Contents",
                "## Demo Command",
                "## Focused Validation",
                "## Current Gate Dimensions",
                "## Boundaries",
                "README.md",
                "bash adl/tools/demo_v0913_quality_gate.sh",
                "bash adl/tools/test_demo_v0913_quality_gate.sh",
            ],
        )
        if error:
            return fail(error)
        linked = docs_root / "review/quality_gate/README.md"
        if not linked.is_file():
            return fail(f"missing linked packet README: {linked}")
    elif surface == "demo_coverage":
        path = docs_root / "review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md"
        error = require_text(
            path,
            [
                "## Claim Boundary",
                "## Primary Artifact",
                "## Review Use",
                "## Validation",
                "every current `v0.91.3` feature has a bounded reviewer-facing demo or proof path",
                "ct_demo_006_feature_demo_map.md",
            ],
        )
        if error:
            return fail(error)
        linked = docs_root / "review/demo_coverage/ct_demo_006_feature_demo_map.md"
        if not linked.is_file():
            return fail(f"missing linked demo-coverage map: {linked}")
    else:
        return fail(f"unknown surface: {surface}")

    print(f"v0913_quality_gate_review_surfaces: PASS surface={surface}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
