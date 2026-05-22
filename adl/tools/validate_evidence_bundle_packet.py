#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


REQUIRED_FILES = {
    "README.md": ["# Evidence Bundle Review Packet", "## Primary Proof Surfaces"],
    "EVIDENCE_BUNDLE_PROOF_PACKET_v0.91.3.md": [
        "# Evidence Bundle Proof Packet v0.91.3",
        "## Focused Validation",
    ],
    "ct_demo_001_evidence_bundle.md": [
        "# CT Demo 001 Evidence Bundle",
        "## Transition Identity",
        "## Changed Artifact Inventory",
        "## Validation Record",
        "## Validation Not Run",
        "## Review Inputs",
        "## Review Findings",
        "## Finding Dispositions",
        "## Trace / Proof References",
        "## Residual Risks",
    ],
    "ct_demo_001_review_synthesis.md": [
        "# CT Demo 001 Review Synthesis",
        "## Conclusion",
        "## Confirmed Findings",
        "## Deferred Findings",
        "## Recommended Outcome",
    ],
}


def fail(message: str) -> int:
    print(message, file=sys.stderr)
    return 1


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        return fail("usage: validate_evidence_bundle_packet.py <packet-root>")

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

    print(f"evidence_bundle_packet: PASS root={root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
