#!/usr/bin/env python3
"""Fill one ADL planning template from explicit JSON values.

This helper intentionally creates generated drafts only. It does not review,
approve, publish, merge, or close any planning document.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

PLACEHOLDER = re.compile(r"<([A-Za-z][A-Za-z0-9_]*)>")


def load_json(path: Path) -> dict[str, Any]:
    path = path.resolve()
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise SystemExit(f"JSON file not found: {path}")
    except json.JSONDecodeError as exc:
        raise SystemExit(f"JSON file is invalid: {path}: {exc}")
    if not isinstance(data, dict):
        raise SystemExit(f"JSON file must contain an object: {path}")
    return data


def registry_repo_root(registry_path: Path) -> Path:
    resolved = registry_path.resolve()
    parts = resolved.parts
    suffix = ("docs", "templates", "planning", "current.json")
    if len(parts) >= len(suffix) and tuple(parts[-len(suffix) :]) == suffix:
        return Path(*parts[: -len(suffix)])
    return resolved.parent


def resolve_registered_path(registry_path: Path, path_value: str) -> Path:
    path = Path(path_value)
    if path.is_absolute():
        raise SystemExit(f"registered template path must be relative: {path_value}")
    return registry_repo_root(registry_path) / path


def is_relative_to_path(path: Path, root: Path) -> bool:
    try:
        path.resolve().relative_to(root.resolve())
        return True
    except ValueError:
        return False


def resolve_template(
    registry: dict[str, Any],
    registry_path: Path,
    template_key: str,
) -> tuple[Path, str]:
    templates = registry.get("templates", {})
    template = templates.get(template_key)
    if not isinstance(template, dict):
        raise SystemExit(f"unknown template key: {template_key}")
    if template.get("status") != "active":
        raise SystemExit(f"template is not active: {template_key}")
    template_path_value = template.get("path")
    if not isinstance(template_path_value, str):
        raise SystemExit(f"template path is missing or invalid: {template_key}")
    template_root = str(registry.get("template_root", ""))
    template_path = resolve_registered_path(registry_path, template_path_value)
    if template_root and not is_relative_to_path(
        template_path,
        resolve_registered_path(registry_path, template_root),
    ):
        raise SystemExit(f"template path is outside active template root: {template_path_value}")
    if not template_path.exists():
        raise SystemExit(f"registered template file does not exist: {template_path}")
    return template_path, template_path_value


def stringify(value: Any) -> str:
    if isinstance(value, list):
        return "\n".join(str(item) for item in value)
    if isinstance(value, dict):
        return json.dumps(value, sort_keys=True)
    return str(value)


def fill_template(template_text: str, values: dict[str, Any]) -> tuple[str, list[str]]:
    missing: set[str] = set()

    def replace(match: re.Match[str]) -> str:
        key = match.group(1)
        if key not in values:
            missing.add(key)
            return match.group(0)
        return stringify(values[key])

    filled = PLACEHOLDER.sub(replace, template_text)
    return filled, sorted(missing)


def generated_header(registry: dict[str, Any], template_key: str, template_path: str) -> str:
    return "\n".join(
        [
            "<!--",
            "Generated Planning Draft",
            f"planning_template_set: {registry.get('planning_template_set')}",
            f"template: {template_key}",
            f"template_path: {template_path}",
            "generation_status: generated_draft",
            "claim_boundary: generated draft only; not reviewed or approved",
            "-->",
            "",
            "> Generated planning draft. This file proves only template filling;",
            "> it is not reviewed, approved, released, merged, or lifecycle-true.",
            "",
        ]
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--registry", default="docs/templates/planning/current.json")
    parser.add_argument("--template", required=True)
    parser.add_argument("--values", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument(
        "--allow-missing",
        action="store_true",
        help="Write a draft even if some placeholders remain unresolved.",
    )
    args = parser.parse_args()

    registry_path = Path(args.registry)
    registry = load_json(registry_path)
    if registry.get("schema") != "adl.planning_template_registry.v1":
        raise SystemExit(f"unsupported registry schema: {registry.get('schema')!r}")
    values = load_json(Path(args.values))
    template_path, template_display_path = resolve_template(registry, registry_path, args.template)
    template_text = template_path.read_text(encoding="utf-8")
    filled, missing = fill_template(template_text, values)

    if missing and not args.allow_missing:
        print("missing values: " + ", ".join(missing), file=sys.stderr)
        return 1

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        generated_header(registry, args.template, template_display_path) + filled,
        encoding="utf-8",
    )
    print(
        json.dumps(
            {
                "status": "PASS" if not missing else "PARTIAL",
                "template": args.template,
                "template_path": template_display_path,
                "values": args.values,
                "output": str(output_path),
                "missing_values": missing,
                "claim_boundary": "generated draft only; not review or approval",
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
