#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

REQUIRED_FILES = [
    "README.md",
    "FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md",
    "ct_demo_002_starharvest_design_note.md",
    "ct_demo_002_starharvest_implementation_summary.md",
    "ct_demo_002_starharvest_qa_checklist.md",
    "ct_demo_002_starharvest_proof_report.md",
]

REQUIRED_SNIPPETS = {
    "FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md": [
        "# Five-Minute HTML Game Packet v0.91.3",
        "Result Vocabulary",
        "What This Demo Proves",
        "What This Demo Suggests",
        "What This Demo Does Not Prove",
        "Primary Demo Command",
    ],
    "ct_demo_002_starharvest_design_note.md": [
        "# Starharvest Design Note",
        "Visual Thesis",
        "Gameplay Loop",
        "Scope Cuts",
    ],
    "ct_demo_002_starharvest_qa_checklist.md": [
        "# Starharvest QA Checklist",
        "Keyboard Control",
        "Win/loss",
    ],
    "ct_demo_002_starharvest_proof_report.md": [
        "# Starharvest Proof Report",
        "passed",
        "What the Demo Proves",
        "What the Demo Does Not Prove",
    ],
}


def main() -> int:
    if len(sys.argv) != 2:
      print("usage: validate_five_minute_html_game_packet.py <packet-dir>", file=sys.stderr)
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
    print("five-minute-html-game-packet: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
