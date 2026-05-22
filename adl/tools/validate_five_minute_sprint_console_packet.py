#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

REQUIRED_FILES = [
    "README.md",
    "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md",
    "ct_demo_003_sprint_console_storyboard.md",
    "ct_demo_003_sprint_console_proof_report.md",
]

REQUIRED_SNIPPETS = {
    "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md": [
        "# Five-Minute Sprint Console Packet v0.91.3",
        "## Demo Identity",
        "## Claims",
        "## Non-Claims",
        "## Run Path",
        "## Timebox Truth",
        "## Result Classification",
    ],
    "ct_demo_003_sprint_console_storyboard.md": [
        "# Sprint Console Storyboard",
        "Compressed Replay Clock",
        "Role Rail",
        "Review And Friction",
    ],
    "ct_demo_003_sprint_console_proof_report.md": [
        "# Sprint Console Proof Report",
        "What the Demo Proves",
        "What the Demo Suggests",
        "What the Demo Does Not Prove",
    ],
}


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_five_minute_sprint_console_packet.py <packet-dir>", file=sys.stderr)
        return 2
    packet_dir = Path(sys.argv[1]).resolve()
    missing = [name for name in REQUIRED_FILES if not (packet_dir / name).is_file()]
    if missing:
        print(f"missing required files: {missing}", file=sys.stderr)
        return 1
    for name, snippets in REQUIRED_SNIPPETS.items():
        text = (packet_dir / name).read_text(encoding="utf-8")
        for snippet in snippets:
            if snippet not in text:
                print(f"{name}: missing snippet {snippet!r}", file=sys.stderr)
                return 1
    print("five-minute-sprint-console-packet: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
