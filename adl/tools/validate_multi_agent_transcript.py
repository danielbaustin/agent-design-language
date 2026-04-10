#!/usr/bin/env python3
"""Validate the bounded v0.87.1 multi-agent transcript artifact."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


REQUIRED_TITLE = "# Claude + ChatGPT Multi-Agent Tea Discussion Transcript"
PROVENANCE_PHRASE = (
    "assembled from the runtime-written step outputs under `out/discussion/`"
)
REQUIRED_HEADINGS = [
    "# Turn 1 - ChatGPT",
    "# Turn 2 - Claude",
    "# Turn 3 - ChatGPT",
    "# Turn 4 - Claude",
    "# Turn 5 - ChatGPT",
]
REQUIRED_CONTRACT_SCHEMA = "multi_agent_discussion_transcript.v1"


def validate_transcript(path: Path) -> list[str]:
    errors: list[str] = []
    if not path.is_file():
        return [f"transcript missing: {path}"]

    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError as exc:
        return [f"transcript is not valid UTF-8: {exc}"]

    if REQUIRED_TITLE not in text:
        errors.append("missing canonical transcript title")
    if PROVENANCE_PHRASE not in text:
        errors.append("missing runtime-output provenance statement")
    if "{{" in text or "}}" in text:
        errors.append("transcript contains unresolved template marker")

    lines = text.splitlines()
    heading_lines = {heading: [] for heading in REQUIRED_HEADINGS}
    for line_no, line in enumerate(lines):
        if line in heading_lines:
            heading_lines[line].append(line_no)

    positions: list[int] = []
    for heading in REQUIRED_HEADINGS:
        count = len(heading_lines[heading])
        if count != 1:
            errors.append(f"expected exactly one heading '{heading}', found {count}")
            continue
        positions.append(heading_lines[heading][0])

    if positions != sorted(positions):
        errors.append("turn headings are not in required order")

    observed_turn_headings = [line for line in lines if line.startswith("# Turn ")]
    if len(observed_turn_headings) != len(REQUIRED_HEADINGS):
        errors.append(
            "expected "
            f"{len(REQUIRED_HEADINGS)} turn headings, found {len(observed_turn_headings)}"
        )

    separator_count = sum(1 for line in lines if line.strip() == "---")
    if separator_count != len(REQUIRED_HEADINGS):
        errors.append(
            "expected one stable separator before each turn, "
            f"found {separator_count}"
        )

    return errors


def validate_contract(path: Path, transcript_path: Path) -> list[str]:
    errors: list[str] = []
    if not path.is_file():
        return [f"transcript contract missing: {path}"]

    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        return [f"transcript contract is not valid JSON: {exc}"]

    allowed_keys = {
        "schema_version",
        "transcript_path",
        "turn_count",
        "turns",
        "companion_artifacts",
    }
    extra_keys = sorted(set(payload) - allowed_keys)
    if extra_keys:
        errors.append(f"transcript contract has unexpected keys: {extra_keys}")

    if payload.get("schema_version") != REQUIRED_CONTRACT_SCHEMA:
        errors.append("transcript contract has unsupported schema_version")
    if payload.get("transcript_path") != transcript_path.name:
        errors.append("transcript contract transcript_path must name the transcript file")
    if payload.get("turn_count") != len(REQUIRED_HEADINGS):
        errors.append("transcript contract turn_count does not match required headings")

    turns = payload.get("turns")
    if not isinstance(turns, list):
        errors.append("transcript contract turns must be a list")
        turns = []
    if len(turns) != len(REQUIRED_HEADINGS):
        errors.append(f"transcript contract must declare {len(REQUIRED_HEADINGS)} turns")

    expected_speakers = ["ChatGPT", "Claude", "ChatGPT", "Claude", "ChatGPT"]
    for idx, turn in enumerate(turns[: len(REQUIRED_HEADINGS)], start=1):
        if not isinstance(turn, dict):
            errors.append(f"turn {idx} must be an object")
            continue
        turn_allowed = {"turn_id", "ordinal", "speaker", "heading", "source_output"}
        extra_turn_keys = sorted(set(turn) - turn_allowed)
        if extra_turn_keys:
            errors.append(f"turn {idx} has unexpected keys: {extra_turn_keys}")
        expected_turn_id = f"turn_{idx:02d}"
        if turn.get("turn_id") != expected_turn_id:
            errors.append(f"turn {idx} turn_id must be {expected_turn_id}")
        if turn.get("ordinal") != idx:
            errors.append(f"turn {idx} ordinal must be {idx}")
        if turn.get("speaker") != expected_speakers[idx - 1]:
            errors.append(f"turn {idx} speaker mismatch")
        if turn.get("heading") != REQUIRED_HEADINGS[idx - 1]:
            errors.append(f"turn {idx} heading mismatch")
        source_output = turn.get("source_output")
        if not isinstance(source_output, str) or not source_output.startswith(
            "out/discussion/"
        ):
            errors.append(f"turn {idx} source_output must be under out/discussion/")

    companions = payload.get("companion_artifacts")
    if not isinstance(companions, dict):
        errors.append("transcript contract companion_artifacts must be an object")
    else:
        for key in ("demo_manifest", "run_summary", "trace"):
            value = companions.get(key)
            if not isinstance(value, str) or not value:
                errors.append(f"companion_artifacts.{key} must be a non-empty string")

    return errors


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Validate the bounded v0.87.1 multi-agent transcript artifact."
    )
    parser.add_argument("transcript", type=Path)
    parser.add_argument(
        "--contract",
        type=Path,
        help="optional transcript_contract.json path to validate with the transcript",
    )
    args = parser.parse_args(argv)

    errors = validate_transcript(args.transcript)
    if args.contract is not None:
        errors.extend(validate_contract(args.contract, args.transcript))
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    print("multi_agent_transcript: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
