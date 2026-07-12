#!/usr/bin/env python3
"""Fail closed when retained Spot artifacts contain AWS identifiers."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


PATTERNS = (
    re.compile(r"\b\d{12}\b"),
    re.compile(r"arn:aws(?:-[a-z]+)?:[^\s,\"']+"),
    re.compile(r"\bi-[0-9a-f]{8,17}\b"),
    re.compile(r"\bvol-[0-9a-f]{8,17}\b"),
    re.compile(r"\b(?:vpc|subnet|sg|sir)-[0-9a-f]{8,17}\b"),
    re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"),
    re.compile(r"\b(?:\d{1,3}\.){3}\d{1,3}\b"),
)

REPLACEMENTS = (
    (PATTERNS[0], "<aws-account-id-redacted>"),
    (PATTERNS[1], "<aws-arn-redacted>"),
    (PATTERNS[2], "<ec2-instance-id-redacted>"),
    (PATTERNS[3], "<ebs-volume-id-redacted>"),
    (PATTERNS[4], "<aws-resource-id-redacted>"),
    (PATTERNS[5], "<aws-access-key-redacted>"),
    (PATTERNS[6], "<ip-address-redacted>"),
)


def string_has_identifier(value: str) -> bool:
    return any(pattern.search(value) for pattern in PATTERNS)


def json_has_identifier(value: Any) -> bool:
    if isinstance(value, str):
        return string_has_identifier(value)
    if isinstance(value, dict):
        return any(json_has_identifier(key) or json_has_identifier(item) for key, item in value.items())
    if isinstance(value, list):
        return any(json_has_identifier(item) for item in value)
    return False


def redact_string(value: str) -> str:
    for pattern, replacement in REPLACEMENTS:
        value = pattern.sub(replacement, value)
    return value


def redact_json(value: Any) -> Any:
    if isinstance(value, str):
        return redact_string(value)
    if isinstance(value, dict):
        return {redact_string(str(key)): redact_json(item) for key, item in value.items()}
    if isinstance(value, list):
        return [redact_json(item) for item in value]
    return value


def transform_embedded_json(text: str) -> tuple[str, bool]:
    """Redact JSON values embedded in logs without rewriting numeric metrics."""
    decoder = json.JSONDecoder()
    output: list[str] = []
    cursor = 0
    parsed_any = False
    while cursor < len(text):
        candidates = [position for token in ("{", "[") if (position := text.find(token, cursor)) >= 0]
        if not candidates:
            output.append(redact_string(text[cursor:]))
            break
        candidate = min(candidates)
        try:
            value, consumed = decoder.raw_decode(text[candidate:])
        except json.JSONDecodeError:
            output.append(redact_string(text[cursor : candidate + 1]))
            cursor = candidate + 1
            continue
        output.append(redact_string(text[cursor:candidate]))
        output.append(json.dumps(redact_json(value), separators=(",", ":"), sort_keys=True))
        cursor = candidate + consumed
        parsed_any = True
    return "".join(output), parsed_any


def embedded_json_has_identifier(text: str) -> bool:
    decoder = json.JSONDecoder()
    cursor = 0
    parsed_any = False
    while cursor < len(text):
        candidates = [position for token in ("{", "[") if (position := text.find(token, cursor)) >= 0]
        if not candidates:
            return string_has_identifier(text[cursor:])
        candidate = min(candidates)
        if string_has_identifier(text[cursor:candidate]):
            return True
        try:
            value, consumed = decoder.raw_decode(text[candidate:])
        except json.JSONDecodeError:
            if string_has_identifier(text[candidate : candidate + 1]):
                return True
            cursor = candidate + 1
            continue
        if json_has_identifier(value):
            return True
        cursor = candidate + consumed
        parsed_any = True
    return False if parsed_any else string_has_identifier(text)


def sanitize_artifact(path: Path) -> None:
    text = path.read_text(encoding="utf-8", errors="replace")
    try:
        parsed = json.loads(text)
    except json.JSONDecodeError:
        parsed = None
    if parsed is not None:
        path.write_text(json.dumps(redact_json(parsed), indent=2, sort_keys=True) + "\n", encoding="utf-8")
        return

    lines = text.splitlines()
    if lines:
        parsed_lines: list[Any] = []
        for line in lines:
            try:
                parsed_lines.append(json.loads(line))
            except json.JSONDecodeError:
                break
        else:
            path.write_text(
                "".join(json.dumps(redact_json(value), sort_keys=True) + "\n" for value in parsed_lines),
                encoding="utf-8",
            )
            return
    transformed, _ = transform_embedded_json(text)
    path.write_text(transformed, encoding="utf-8")


def artifact_has_identifier(path: Path) -> bool:
    text = path.read_text(encoding="utf-8", errors="replace")
    try:
        return json_has_identifier(json.loads(text))
    except json.JSONDecodeError:
        pass

    lines = text.splitlines()
    if lines:
        parsed_lines: list[Any] = []
        for line in lines:
            try:
                parsed_lines.append(json.loads(line))
            except json.JSONDecodeError:
                break
        else:
            return any(json_has_identifier(value) for value in parsed_lines)
    return embedded_json_has_identifier(text)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument(
        "--sanitize",
        action="store_true",
        help="redact identifiers in partial retained artifacts before fail-closed verification",
    )
    args = parser.parse_args()

    if args.sanitize:
        for path in args.root.rglob("*"):
            if path.is_file() and ".private" not in path.parts:
                sanitize_artifact(path)

    failures = []
    for path in args.root.rglob("*"):
        if path.is_file() and ".private" not in path.parts and artifact_has_identifier(path):
            failures.append(str(path))
    if failures:
        raise SystemExit("unredacted Spot artifacts: " + ", ".join(sorted(failures)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
