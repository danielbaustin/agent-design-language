#!/usr/bin/env python3
"""Validate filled ADL planning-template documents.

This validator intentionally proves only structural readiness:
- registry parses
- selected template exists and is active
- required sections are present
- unresolved placeholders are absent

It does not prove review, approval, release, PR, or closeout truth.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

ANGLE_PLACEHOLDER = re.compile(r"<[A-Za-z][A-Za-z0-9_]*>")
CURLY_PLACEHOLDER = re.compile(r"\{\{[A-Za-z][A-Za-z0-9_]*\}\}")


def load_registry(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise SystemExit(f"registry not found: {path}")
    except json.JSONDecodeError as exc:
        raise SystemExit(f"registry JSON is invalid: {path}: {exc}")
    if data.get("schema") != "adl.planning_template_registry.v1":
        raise SystemExit(f"unsupported registry schema: {data.get('schema')!r}")
    return data


def validate_required_sections(text: str, required_sections: list[str]) -> list[str]:
    missing: list[str] = []
    for section in required_sections:
        pattern = re.compile(rf"^#+\s+{re.escape(section)}\s*$", re.MULTILINE)
        if not pattern.search(text):
            missing.append(section)
    return missing


def validate_document(registry: dict[str, Any], template_key: str, input_path: Path) -> int:
    templates = registry.get("templates", {})
    template = templates.get(template_key)
    if not isinstance(template, dict):
        print(f"unknown template key: {template_key}", file=sys.stderr)
        return 2
    if template.get("status") != "active":
        print(f"template is not active: {template_key}", file=sys.stderr)
        return 2

    template_path_value = template.get("path")
    if not isinstance(template_path_value, str):
        print(f"template path is missing or invalid: {template_key}", file=sys.stderr)
        return 2
    template_path = Path(template_path_value)
    template_root = str(registry.get("template_root", ""))
    if template_root and not template_path_value.startswith(template_root):
        print(
            f"template path is outside active template root: {template_path_value}",
            file=sys.stderr,
        )
        return 2
    if not template_path.exists():
        print(f"registered template file does not exist: {template_path}", file=sys.stderr)
        return 2
    if template.get("version") != registry.get("planning_template_set"):
        print(
            f"template version does not match registry set: {template_key}",
            file=sys.stderr,
        )
        return 2

    try:
        text = input_path.read_text(encoding="utf-8")
    except FileNotFoundError:
        print(f"input not found: {input_path}", file=sys.stderr)
        return 2

    failures: list[str] = []

    unresolved = sorted(set(ANGLE_PLACEHOLDER.findall(text) + CURLY_PLACEHOLDER.findall(text)))
    if unresolved:
        failures.append("unresolved placeholders: " + ", ".join(unresolved))

    required_sections = template.get("required_sections", [])
    if not isinstance(required_sections, list):
        failures.append(f"registry required_sections is not a list for {template_key}")
    else:
        missing = validate_required_sections(text, [str(item) for item in required_sections])
        if missing:
            failures.append("missing required sections: " + ", ".join(missing))

    if failures:
        for failure in failures:
            print(f"FAIL: {failure}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "status": "PASS",
                "template": template_key,
                "input": str(input_path),
                "registry_template_set": registry.get("planning_template_set"),
                "claim_boundary": "structural validation only; not review or approval",
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--registry", default="docs/templates/planning/current.json")
    parser.add_argument("--template", required=True)
    parser.add_argument("--input", required=True)
    args = parser.parse_args()

    registry = load_registry(Path(args.registry))
    return validate_document(registry, args.template, Path(args.input))


if __name__ == "__main__":
    raise SystemExit(main())
