#!/usr/bin/env python3
"""Fast stdlib smoke test for versioned prompt-card structure schemas."""

from __future__ import annotations

import json
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
REGISTRY_PATH = REPO_ROOT / "docs/templates/prompts/current.json"
EXPECTED_KINDS = ["sip", "stp", "spp", "srp", "sor"]
SCHEMA_ID = "adl.csdlc.prompt_card_structure.v1"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        fail(f"missing {path.relative_to(REPO_ROOT)}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path.relative_to(REPO_ROOT)}: {exc}")


def require_list(schema: dict, key: str, kind: str) -> list:
    value = schema.get(key)
    if not isinstance(value, list):
        fail(f"{kind} schema field {key!r} must be a list")
    return value


def template_set_supports_vpp(template_set: str) -> bool:
    try:
        major, minor, patch = (int(part) for part in template_set.split(".", 2))
    except ValueError:
        fail(f"invalid template set semver: {template_set}")
    return (major, minor, patch) >= (1, 0, 3)


def main() -> None:
    template_set_arg = None
    args = sys.argv[1:]
    if args:
        if len(args) != 2 or args[0] != "--template-set":
            fail("usage: test_prompt_template_structure_schemas.py [--template-set <semver>]")
        template_set_arg = args[1]
    registry = load_json(REGISTRY_PATH)
    active_template_set = registry.get("csdlc_prompt_template_set")
    template_set = template_set_arg or active_template_set
    templates = registry.get("templates")
    expected_kinds = (
        ["sip", "stp", "spp", "vpp", "srp", "sor"]
        if template_set_supports_vpp(template_set)
        else EXPECTED_KINDS
    )
    if not isinstance(template_set, str) or not template_set:
        fail("registry must declare csdlc_prompt_template_set")
    if template_set_arg is None and not isinstance(templates, dict):
        fail("registry templates must be an object")

    if template_set_arg is None:
        missing = [kind for kind in expected_kinds if kind not in templates]
        if missing:
            fail(f"registry missing template entries: {', '.join(missing)}")

    for kind in expected_kinds:
        if template_set_arg is None:
            entry = templates[kind]
            schema_rel = entry.get("structure_schema_path")
            template_rel = entry.get("path")
            if not isinstance(schema_rel, str) or not schema_rel.endswith(".json"):
                fail(f"{kind} structure_schema_path must point at a JSON artifact")
            if not isinstance(template_rel, str) or not template_rel:
                fail(f"{kind} template path must be present")
        else:
            schema_rel = f"docs/templates/prompts/{template_set}/schemas/{kind}.structure.json"
            template_rel = f"docs/templates/prompts/{template_set}/{kind}.md"

        schema = load_json(REPO_ROOT / schema_rel)
        if schema.get("schema") != SCHEMA_ID:
            fail(f"{kind} schema id mismatch")
        if schema.get("template_set") != template_set:
            fail(f"{kind} template_set mismatch")
        if schema.get("card_kind") != kind:
            fail(f"{kind} card_kind mismatch")
        if schema.get("template_path") != template_rel:
            fail(f"{kind} template_path mismatch")
        if "markdown-rs" not in str(schema.get("parser", "")):
            fail(f"{kind} parser must record markdown-rs extraction")

        for key in [
            "editable_sections",
            "frontmatter_keys",
            "headings",
            "fenced_blocks",
            "locked_lines",
        ]:
            require_list(schema, key, kind)

        if not schema["headings"]:
            fail(f"{kind} schema must record Markdown headings")

    print(f"PASS: prompt-card structure schema artifacts are Python-readable for {template_set}")


if __name__ == "__main__":
    main()
