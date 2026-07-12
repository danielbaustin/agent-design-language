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
    re.compile(r"arn:aws:"),
    re.compile(r"\bi-[0-9a-f]{8,17}\b"),
    re.compile(r"\bvol-[0-9a-f]{8,17}\b"),
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
    return string_has_identifier(text)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()

    failures = []
    for path in args.root.rglob("*"):
        if path.is_file() and ".private" not in path.parts and artifact_has_identifier(path):
            failures.append(str(path))
    if failures:
        raise SystemExit("unredacted Spot artifacts: " + ", ".join(sorted(failures)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
