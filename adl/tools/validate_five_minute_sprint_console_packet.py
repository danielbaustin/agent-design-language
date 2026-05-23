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


def fail(message: str) -> int:
    print(f"five-minute-sprint-console-packet: FAIL {message}", file=sys.stderr)
    return 1


def code_value(line: str, prefix: str) -> str | None:
    if not line.startswith(prefix):
        return None
    remainder = line[len(prefix) :].strip()
    if remainder.startswith("`") and remainder.endswith("`"):
        return remainder[1:-1]
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
        return value == expected
    return False


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_five_minute_sprint_console_packet.py <packet-dir>", file=sys.stderr)
        return 2
    packet_dir = Path(sys.argv[1]).resolve()
    missing = [name for name in REQUIRED_FILES if not (packet_dir / name).is_file()]
    if missing:
        return fail(f"missing required files: {missing}")
    packet_text = (packet_dir / "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md").read_text(
        encoding="utf-8"
    )
    report_text = (packet_dir / "ct_demo_003_sprint_console_proof_report.md").read_text(
        encoding="utf-8"
    )
    for name, snippets in REQUIRED_SNIPPETS.items():
        text = (packet_dir / name).read_text(encoding="utf-8")
        for snippet in snippets:
            if snippet not in text:
                return fail(f"{name}: missing snippet {snippet!r}")
    if extract_report_result(report_text) != "passed":
        return fail("sprint console proof report must keep top-level result `passed`")
    if not require_line(packet_text, "- run status:", "passed"):
        return fail("packet run status must stay `passed`")
    if not require_line(packet_text, "- evidence type:", "estimated"):
        return fail("timebox truth must stay `estimated`")
    if "`partial`" not in packet_text or "`partial`" not in report_text:
        return fail("literal five-minute claim must remain explicitly partial")
    print("five-minute-sprint-console-packet: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
