#!/usr/bin/env python3
"""Validate PVF manifest contracts for v0.91.4."""

from __future__ import annotations

import json
import sys
from pathlib import Path

EXPECTED_VERSION = "v0.91.4"
LANE_CLASSES = {
    "fast_unit",
    "contract_schema_card",
    "docs",
    "cli_workflow",
    "integration_worktree",
    "provider_live",
    "release_gate",
}
RESOURCE_PROFILES = {"low", "medium", "high"}
DETERMINISM_MODES = {"strict", "fixture_bound", "live"}
CACHE_STRATEGIES = {"none", "local_reuse", "artifact_reuse"}
RELEASE_GATE_CLASSES = {
    "required_on_pr",
    "release_candidate",
    "manual_release_gate",
    "optional",
}
DEFAULT_TRIGGERS = {"always", "changed_paths", "manual", "release_only"}


def fail(message: str) -> None:
    print(f"validate_pvf_manifest: {message}", file=sys.stderr)
    raise SystemExit(1)


def expect_type(value: object, expected: type, label: str) -> None:
    if not isinstance(value, expected):
        fail(f"{label} must be {expected.__name__}")


def expect_non_empty_string(value: object, label: str) -> str:
    expect_type(value, str, label)
    if not value:
        fail(f"{label} must be non-empty")
    return value


def expect_enum(value: object, allowed: set[str], label: str) -> str:
    value = expect_non_empty_string(value, label)
    if value not in allowed:
        allowed_list = ", ".join(sorted(allowed))
        fail(f"{label} must be one of: {allowed_list}")
    return value


def expect_string_array(value: object, label: str) -> list[str]:
    expect_type(value, list, label)
    result: list[str] = []
    for idx, item in enumerate(value):
        result.append(expect_non_empty_string(item, f"{label}[{idx}]"))
    return result


def validate_lane(lane_id: str, lane: object) -> str:
    expect_type(lane, dict, f"lanes.{lane_id}")
    lane = dict(lane)
    allowed_keys = {
        "lane_class",
        "owner_surface",
        "command",
        "resource_profile",
        "determinism",
        "cache_strategy",
        "release_gate_class",
        "default_trigger",
        "changed_path_hints",
        "evidence_outputs",
        "timeout_minutes",
        "requires_credentials",
        "requires_worktree",
        "notes",
    }
    unknown = sorted(set(lane) - allowed_keys)
    if unknown:
        fail(f"lanes.{lane_id} has unknown keys: {', '.join(unknown)}")

    for required in (
        "lane_class",
        "owner_surface",
        "command",
        "resource_profile",
        "determinism",
        "cache_strategy",
        "release_gate_class",
        "default_trigger",
        "changed_path_hints",
        "evidence_outputs",
    ):
        if required not in lane:
            fail(f"lanes.{lane_id} is missing required key: {required}")

    lane_class = expect_enum(lane["lane_class"], LANE_CLASSES, f"lanes.{lane_id}.lane_class")
    expect_non_empty_string(lane["owner_surface"], f"lanes.{lane_id}.owner_surface")
    expect_non_empty_string(lane["command"], f"lanes.{lane_id}.command")
    expect_enum(lane["resource_profile"], RESOURCE_PROFILES, f"lanes.{lane_id}.resource_profile")
    expect_enum(lane["determinism"], DETERMINISM_MODES, f"lanes.{lane_id}.determinism")
    expect_enum(lane["cache_strategy"], CACHE_STRATEGIES, f"lanes.{lane_id}.cache_strategy")
    expect_enum(lane["release_gate_class"], RELEASE_GATE_CLASSES, f"lanes.{lane_id}.release_gate_class")
    expect_enum(lane["default_trigger"], DEFAULT_TRIGGERS, f"lanes.{lane_id}.default_trigger")
    expect_string_array(lane["changed_path_hints"], f"lanes.{lane_id}.changed_path_hints")
    expect_string_array(lane["evidence_outputs"], f"lanes.{lane_id}.evidence_outputs")

    if "timeout_minutes" in lane:
        expect_type(lane["timeout_minutes"], int, f"lanes.{lane_id}.timeout_minutes")
        if lane["timeout_minutes"] < 1:
            fail(f"lanes.{lane_id}.timeout_minutes must be >= 1")
    if "requires_credentials" in lane:
        expect_type(lane["requires_credentials"], bool, f"lanes.{lane_id}.requires_credentials")
    if "requires_worktree" in lane:
        expect_type(lane["requires_worktree"], bool, f"lanes.{lane_id}.requires_worktree")
    if "notes" in lane:
        expect_non_empty_string(lane["notes"], f"lanes.{lane_id}.notes")

    return lane_class


def validate_manifest(path: Path) -> None:
    manifest = json.loads(path.read_text())
    expect_type(manifest, dict, str(path))

    allowed_keys = {"manifest_version", "lane_classes", "lanes"}
    unknown = sorted(set(manifest) - allowed_keys)
    if unknown:
        fail(f"{path}: unknown top-level keys: {', '.join(unknown)}")

    for required in ("manifest_version", "lane_classes", "lanes"):
        if required not in manifest:
            fail(f"{path}: missing required key: {required}")

    version = expect_non_empty_string(manifest["manifest_version"], f"{path}.manifest_version")
    if version != EXPECTED_VERSION:
        fail(f"{path}: manifest_version must be {EXPECTED_VERSION}")

    lane_classes = expect_string_array(manifest["lane_classes"], f"{path}.lane_classes")
    if not lane_classes:
        fail(f"{path}: lane_classes must not be empty")
    if len(set(lane_classes)) != len(lane_classes):
        fail(f"{path}: lane_classes must contain unique values")
    for lane_class in lane_classes:
        if lane_class not in LANE_CLASSES:
            allowed_list = ", ".join(sorted(LANE_CLASSES))
            fail(f"{path}: lane_classes contains unsupported value '{lane_class}' (allowed: {allowed_list})")

    lanes = manifest["lanes"]
    expect_type(lanes, dict, f"{path}.lanes")
    if not lanes:
        fail(f"{path}: lanes must not be empty")

    used_lane_classes: set[str] = set()
    for lane_id, lane in lanes.items():
        if not lane_id or any(ch not in "abcdefghijklmnopqrstuvwxyz0123456789_" for ch in lane_id):
            fail(f"{path}: lane id '{lane_id}' must match ^[a-z0-9_]+$")
        used_lane_classes.add(validate_lane(lane_id, lane))

    declared = set(lane_classes)
    if used_lane_classes != declared:
        missing = sorted(used_lane_classes - declared)
        unused = sorted(declared - used_lane_classes)
        details = []
        if missing:
            details.append(f"missing used classes: {', '.join(missing)}")
        if unused:
            details.append(f"declared but unused classes: {', '.join(unused)}")
        fail(f"{path}: lane_classes must equal the subset actually used by lanes ({'; '.join(details)})")


def main(argv: list[str]) -> int:
    if len(argv) < 2:
        print("Usage: validate_pvf_manifest.py <manifest.json> [manifest.json ...]", file=sys.stderr)
        return 2
    for arg in argv[1:]:
        validate_manifest(Path(arg))
        print(f"PASS: pvf manifest valid for {arg}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
