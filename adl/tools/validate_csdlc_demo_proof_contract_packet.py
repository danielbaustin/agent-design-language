#!/usr/bin/env python3
"""Validate the bounded v0.91.3 C-SDLC demo proof contract packet."""

from __future__ import annotations

import sys
from pathlib import Path


REQUIRED_FILES = {
    "README.md": [
        "## Focused Validation",
        "This packet defines the shared evidence contract",
    ],
    "C_SDLC_DEMO_PROOF_CONTRACT_v0.91.3.md": [
        "## Required Packet Sections",
        "## Claim Ledger Rules",
        "## Result Classification",
        "## Timebox Truth Rules",
        "## Validation Minimums",
        "## Review Minimums",
    ],
    "C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md": [
        "## Demo Identity",
        "## Claims",
        "## Non-Claims",
        "## Run Path",
        "## Timebox Truth",
        "## Validation Evidence",
        "## Review Evidence",
        "## Result Classification",
    ],
}


def main() -> int:
    if len(sys.argv) != 2:
        print(
            "usage: validate_csdlc_demo_proof_contract_packet.py <packet-dir>",
            file=sys.stderr,
        )
        return 2

    packet_dir = Path(sys.argv[1])
    if not packet_dir.is_dir():
        print(f"packet dir missing: {packet_dir}", file=sys.stderr)
        return 1

    failures: list[str] = []
    for rel_path, snippets in REQUIRED_FILES.items():
        target = packet_dir / rel_path
        if not target.is_file():
            failures.append(f"missing file: {rel_path}")
            continue
        content = target.read_text(encoding="utf-8")
        for snippet in snippets:
            if snippet not in content:
                failures.append(f"{rel_path} missing snippet: {snippet}")

    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1

    print(f"packet ok: {packet_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
