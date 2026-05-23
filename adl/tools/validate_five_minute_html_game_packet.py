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
        "Observed Evidence Status",
        "Keyboard Control",
        "Win/loss",
    ],
    "ct_demo_002_starharvest_proof_report.md": [
        "# Starharvest Proof Report",
        "`partial`",
        "What the Demo Proves",
        "What the Demo Does Not Prove",
    ],
}


def fail(message: str) -> int:
    print(f"five-minute-html-game-packet: FAIL {message}", file=sys.stderr)
    return 1


def code_value(line: str, prefix: str) -> str | None:
    if not line.startswith(prefix):
        return None
    remainder = line[len(prefix) :].strip()
    if remainder.startswith("`"):
        end = remainder.find("`", 1)
        if end != -1:
            return remainder[1:end]
    return remainder


def extract_report_result(text: str) -> str | None:
    lines = text.splitlines()
    for idx, line in enumerate(lines):
        if line.strip() == "## Result":
            for candidate in lines[idx + 1 :]:
                stripped = candidate.strip()
                if not stripped:
                    continue
                if stripped.startswith("`") and stripped.endswith("`"):
                    return stripped[1:-1]
                return stripped
    return None


def require_line(text: str, prefix: str, expected: str) -> bool:
    for line in text.splitlines():
        value = code_value(line.strip(), prefix)
        if value is None:
            continue
        return value == expected or value.startswith(expected + ",")
    return False


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_five_minute_html_game_packet.py <packet-dir>", file=sys.stderr)
        return 2
    packet_dir = Path(sys.argv[1]).resolve()
    missing = [name for name in REQUIRED_FILES if not (packet_dir / name).is_file()]
    if missing:
        return fail(f"missing required files: {missing}")

    packet_text = (packet_dir / "FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md").read_text(
        encoding="utf-8"
    )
    proof_text = (packet_dir / "ct_demo_002_starharvest_proof_report.md").read_text(
        encoding="utf-8"
    )
    qa_text = (packet_dir / "ct_demo_002_starharvest_qa_checklist.md").read_text(
        encoding="utf-8"
    )
    for name, snippets in REQUIRED_SNIPPETS.items():
        text = (packet_dir / name).read_text(encoding="utf-8")
        for snippet in snippets:
            if snippet not in text:
                return fail(f"{name}: missing snippet {snippet!r}")

    if extract_report_result(proof_text) != "partial":
        return fail("proof report result must stay `partial` until browser/gameplay proof is fully captured")
    if not require_line(packet_text, "- browser/gameplay proof in captured environment:", "partial"):
        return fail("packet must record browser/gameplay proof as `partial`")
    if not require_line(
        proof_text,
        "- full browser/gameplay proof in captured environment:",
        "partial",
    ):
        return fail("proof report must record browser/gameplay proof as `partial`")
    if not require_line(
        qa_text,
        "- browser load and interactive gameplay behavior in captured environment:",
        "not run",
    ):
        return fail("QA checklist must explicitly record browser/gameplay behavior as `not run`")
    if not require_line(qa_text, "- helper command prints a valid local URL:", "passed"):
        return fail("QA checklist must explicitly record helper URL proof as `passed`")
    for snippet in [
        "a bounded C-SDLC mini-sprint can produce a real playable browser artifact",
        "the mini-sprint produced a real playable browser artifact",
    ]:
        if snippet in packet_text or snippet in proof_text:
            return fail(f"overclaim retained after downgrade: {snippet!r}")

    print("five-minute-html-game-packet: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
