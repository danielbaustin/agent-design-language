#!/usr/bin/env python3
"""Validate multi-agent workcell state packets for v0.91.4."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from pathlib import PurePosixPath

EXPECTED_SCHEMA = "adl.multi_agent_workcell_state.v1"
ROLE_VALUES = {"worker", "reviewer", "janitor", "closeout"}
BACKEND_VALUES = {"local_ollama", "hosted_codex"}
ADMISSION_VALUES = {
    "parallel_admitted",
    "serial_only",
    "review_ready",
    "janitor_ready",
    "closeout_ready",
    "blocked",
}
VALIDATION_GATE_VALUES = {
    "parallel_pvf_lane",
    "serialized_gate",
    "published_artifact_gate",
    "pr_blocker_gate",
    "closeout_truth_gate",
    "blocked",
}
REVIEW_LANE_VALUES = {
    "bounded_subagent_review",
    "pr_janitor",
    "pr_closeout",
    "not_applicable",
}
CLOSEOUT_VALUES = {
    "not_started",
    "waiting_for_review",
    "janitor_pending",
    "ready_to_close",
    "closed_out",
}
CARD_VALUES = {"missing", "draft", "ready", "approved", "reviewed", "done"}
PR_VALUES = {"none", "draft", "open", "merged"}
GITHUB_ISSUE_VALUES = {"OPEN", "CLOSED"}
REQUIRED_CARD_KEYS = {"sip", "stp", "spp", "srp", "sor"}
READY_CARD_KEYS = ("sip", "stp", "spp")
HOOK_SKILLS = {
    "worker_admission": "pr-run",
    "review_publication": "pr-finish",
    "janitor_remediation": "pr-janitor",
    "closeout_reconciliation": "pr-closeout",
}
REQUIRED_HOOK_TRUTH = {
    "worker_admission": {"github_issue_state", "sip", "stp", "spp", "sprint_state"},
    "review_publication": {"srp", "sor", "pr_state", "published_artifacts"},
    "janitor_remediation": {"pr_state", "check_status", "review_findings"},
    "closeout_reconciliation": {"github_issue_state", "sor", "sprint_state", "closeout_artifacts"},
}


def fail(message: str) -> None:
    print(f"validate_multi_agent_workcell_state: {message}", file=sys.stderr)
    raise SystemExit(1)


def expect_type(value: object, expected: type, label: str) -> None:
    if not isinstance(value, expected):
        fail(f"{label} must be {expected.__name__}")


def expect_non_empty_string(value: object, label: str) -> str:
    expect_type(value, str, label)
    if not value:
        fail(f"{label} must be non-empty")
    return value


def expect_relative_path(value: object, label: str) -> str:
    path = expect_non_empty_string(value, label)
    pure = PurePosixPath(path)
    if pure.is_absolute():
        fail(f"{label} must be repo-relative, not absolute")
    if any(part in {"..", ""} for part in pure.parts):
        fail(f"{label} must stay within the repo-relative tree")
    return path


def expect_enum(value: object, allowed: set[str], label: str) -> str:
    value = expect_non_empty_string(value, label)
    if value not in allowed:
        fail(f"{label} must be one of: {', '.join(sorted(allowed))}")
    return value


def expect_bool(value: object, label: str) -> bool:
    expect_type(value, bool, label)
    return bool(value)


def expect_int(value: object, label: str) -> int:
    expect_type(value, int, label)
    return int(value)


def expect_string_array(value: object, label: str) -> list[str]:
    expect_type(value, list, label)
    result: list[str] = []
    for idx, item in enumerate(value):
        result.append(expect_non_empty_string(item, f"{label}[{idx}]"))
    return result


def validate_hooks(packet: dict[str, object], label: str) -> None:
    hooks = packet.get("conductor_hooks")
    expect_type(hooks, dict, f"{label}.conductor_hooks")
    unknown = sorted(set(hooks) - set(HOOK_SKILLS))
    if unknown:
        fail(f"{label}.conductor_hooks has unknown keys: {', '.join(unknown)}")
    missing = sorted(set(HOOK_SKILLS) - set(hooks))
    if missing:
        fail(f"{label}.conductor_hooks missing required hooks: {', '.join(missing)}")

    for hook_name, expected_skill in HOOK_SKILLS.items():
        hook = hooks[hook_name]
        expect_type(hook, dict, f"{label}.conductor_hooks.{hook_name}")
        selected_skill = expect_non_empty_string(
            hook.get("selected_skill"), f"{label}.conductor_hooks.{hook_name}.selected_skill"
        )
        if selected_skill != expected_skill:
            fail(
                f"{label}.conductor_hooks.{hook_name}.selected_skill must be {expected_skill}"
            )
        canonical_truth = expect_string_array(
            hook.get("canonical_truth", []),
            f"{label}.conductor_hooks.{hook_name}.canonical_truth",
        )
        if not canonical_truth:
            fail(f"{label}.conductor_hooks.{hook_name}.canonical_truth must not be empty")
        missing_truth = sorted(REQUIRED_HOOK_TRUTH[hook_name] - set(canonical_truth))
        if missing_truth:
            fail(
                f"{label}.conductor_hooks.{hook_name}.canonical_truth missing required surfaces: "
                f"{', '.join(missing_truth)}"
            )


def validate_cards(cards: object, label: str) -> dict[str, str]:
    expect_type(cards, dict, label)
    cards = dict(cards)
    unknown = sorted(set(cards) - REQUIRED_CARD_KEYS)
    if unknown:
        fail(f"{label} has unknown keys: {', '.join(unknown)}")
    missing = sorted(REQUIRED_CARD_KEYS - set(cards))
    if missing:
        fail(f"{label} missing required keys: {', '.join(missing)}")
    normalized: dict[str, str] = {}
    for key in sorted(REQUIRED_CARD_KEYS):
        normalized[key] = expect_enum(cards[key], CARD_VALUES, f"{label}.{key}")
    return normalized


def ensure_ready_worker_cards(cards: dict[str, str], label: str) -> None:
    for key in READY_CARD_KEYS:
        if cards[key] not in {"ready", "approved"}:
            fail(f"{label}.{key} must be ready or approved for admitted worker shards")


def validate_assignment(index: int, assignment: object) -> tuple[str, list[str]]:
    label = f"shard_assignments[{index}]"
    expect_type(assignment, dict, label)
    assignment = dict(assignment)

    required = {
        "shard_id",
        "issue_number",
        "role",
        "branch",
        "worktree_path",
        "write_paths",
        "dependencies",
        "admission_status",
        "validation_gate",
        "review_lane",
        "closeout_status",
        "cards",
        "github_issue_state",
        "pr_state",
    }
    unknown = sorted(set(assignment) - (required | {"execution_backend", "model_hint", "notes"}))
    if unknown:
        fail(f"{label} has unknown keys: {', '.join(unknown)}")
    missing = sorted(required - set(assignment))
    if missing:
        fail(f"{label} missing required keys: {', '.join(missing)}")

    shard_id = expect_non_empty_string(assignment["shard_id"], f"{label}.shard_id")
    expect_int(assignment["issue_number"], f"{label}.issue_number")
    role = expect_enum(assignment["role"], ROLE_VALUES, f"{label}.role")
    expect_non_empty_string(assignment["branch"], f"{label}.branch")
    expect_relative_path(assignment["worktree_path"], f"{label}.worktree_path")
    write_paths = expect_string_array(assignment["write_paths"], f"{label}.write_paths")
    for idx_path, path in enumerate(write_paths):
        expect_relative_path(path, f"{label}.write_paths[{idx_path}]")
    dependencies = expect_string_array(assignment["dependencies"], f"{label}.dependencies")
    if len(set(dependencies)) != len(dependencies):
        fail(f"{label}.dependencies must contain unique shard ids")
    admission_status = expect_enum(assignment["admission_status"], ADMISSION_VALUES, f"{label}.admission_status")
    validation_gate = expect_enum(assignment["validation_gate"], VALIDATION_GATE_VALUES, f"{label}.validation_gate")
    review_lane = expect_enum(assignment["review_lane"], REVIEW_LANE_VALUES, f"{label}.review_lane")
    closeout_status = expect_enum(assignment["closeout_status"], CLOSEOUT_VALUES, f"{label}.closeout_status")
    github_issue_state = expect_enum(assignment["github_issue_state"], GITHUB_ISSUE_VALUES, f"{label}.github_issue_state")
    pr_state = expect_enum(assignment["pr_state"], PR_VALUES, f"{label}.pr_state")
    cards = validate_cards(assignment["cards"], f"{label}.cards")

    backend = assignment.get("execution_backend")
    model_hint = assignment.get("model_hint")
    if role == "worker":
        expect_type(write_paths, list, f"{label}.write_paths")
        if not write_paths:
            fail(f"{label}.write_paths must not be empty for worker shards")
        expect_enum(backend, BACKEND_VALUES, f"{label}.execution_backend")
        expect_non_empty_string(model_hint, f"{label}.model_hint")
        if admission_status == "parallel_admitted":
            ensure_ready_worker_cards(cards, f"{label}.cards")
            if github_issue_state != "OPEN":
                fail(f"{label}.github_issue_state must be OPEN for parallel-admitted worker shards")
            if validation_gate != "parallel_pvf_lane":
                fail(f"{label}.validation_gate must be parallel_pvf_lane for parallel-admitted worker shards")
            if review_lane != "bounded_subagent_review":
                fail(f"{label}.review_lane must be bounded_subagent_review for parallel-admitted worker shards")
            if closeout_status not in {"not_started", "waiting_for_review"}:
                fail(f"{label}.closeout_status must be not_started or waiting_for_review for parallel-admitted worker shards")
        if admission_status == "serial_only":
            ensure_ready_worker_cards(cards, f"{label}.cards")
            if validation_gate != "serialized_gate":
                fail(f"{label}.validation_gate must be serialized_gate for serial-only worker shards")
        if admission_status == "blocked" and validation_gate != "blocked":
            fail(f"{label}.validation_gate must be blocked when admission_status is blocked")
    else:
        if backend is not None or model_hint is not None:
            fail(f"{label} must not declare execution_backend/model_hint for non-worker roles")

    if role == "reviewer":
        if admission_status != "review_ready":
            fail(f"{label}.admission_status must be review_ready for reviewer lanes")
        if validation_gate != "published_artifact_gate":
            fail(f"{label}.validation_gate must be published_artifact_gate for reviewer lanes")
        if review_lane != "bounded_subagent_review":
            fail(f"{label}.review_lane must be bounded_subagent_review for reviewer lanes")
        if pr_state not in {"draft", "open"}:
            fail(f"{label}.pr_state must be draft or open for reviewer lanes")

    if role == "janitor":
        if admission_status != "janitor_ready":
            fail(f"{label}.admission_status must be janitor_ready for janitor lanes")
        if validation_gate != "pr_blocker_gate":
            fail(f"{label}.validation_gate must be pr_blocker_gate for janitor lanes")
        if review_lane != "pr_janitor":
            fail(f"{label}.review_lane must be pr_janitor for janitor lanes")
        if closeout_status != "janitor_pending":
            fail(f"{label}.closeout_status must be janitor_pending for janitor lanes")
        if pr_state != "open":
            fail(f"{label}.pr_state must be open for janitor lanes")

    if role == "closeout":
        if admission_status != "closeout_ready":
            fail(f"{label}.admission_status must be closeout_ready for closeout lanes")
        if validation_gate != "closeout_truth_gate":
            fail(f"{label}.validation_gate must be closeout_truth_gate for closeout lanes")
        if review_lane != "pr_closeout":
            fail(f"{label}.review_lane must be pr_closeout for closeout lanes")
        if closeout_status not in {"ready_to_close", "closed_out"}:
            fail(f"{label}.closeout_status must be ready_to_close or closed_out for closeout lanes")
        if pr_state != "merged":
            fail(f"{label}.pr_state must be merged for closeout lanes")
        if closeout_status == "closed_out" and github_issue_state != "CLOSED":
            fail(f"{label}.github_issue_state must be CLOSED when closeout_status is closed_out")

    if cards["sor"] == "done" and github_issue_state != "CLOSED":
        fail(f"{label}.github_issue_state must be CLOSED when SOR is done")

    return shard_id, dependencies


def validate_packet(path: Path) -> None:
    packet = json.loads(path.read_text())
    expect_type(packet, dict, str(path))
    required = {
        "schema_version",
        "workcell_id",
        "sprint_issue_number",
        "planner_manifest_path",
        "conductor_hooks",
        "shard_assignments",
    }
    unknown = sorted(set(packet) - (required | {"notes"}))
    if unknown:
        fail(f"{path} has unknown top-level keys: {', '.join(unknown)}")
    missing = sorted(required - set(packet))
    if missing:
        fail(f"{path} missing required top-level keys: {', '.join(missing)}")

    schema_version = expect_non_empty_string(packet["schema_version"], f"{path}.schema_version")
    if schema_version != EXPECTED_SCHEMA:
        fail(f"{path}.schema_version must be {EXPECTED_SCHEMA}")
    expect_non_empty_string(packet["workcell_id"], f"{path}.workcell_id")
    expect_int(packet["sprint_issue_number"], f"{path}.sprint_issue_number")
    expect_relative_path(packet["planner_manifest_path"], f"{path}.planner_manifest_path")
    validate_hooks(packet, str(path))

    assignments = packet["shard_assignments"]
    expect_type(assignments, list, f"{path}.shard_assignments")
    if not assignments:
        fail(f"{path}.shard_assignments must not be empty")

    seen_ids: set[str] = set()
    dependency_map: dict[str, list[str]] = {}
    for idx, assignment in enumerate(assignments):
        shard_id, dependencies = validate_assignment(idx, assignment)
        if shard_id in seen_ids:
            fail(f"{path}.shard_assignments contains duplicate shard_id {shard_id}")
        seen_ids.add(shard_id)
        dependency_map[shard_id] = dependencies

    for shard_id, dependencies in dependency_map.items():
        if shard_id in dependencies:
            fail(f"{path}.shard_assignments dependency list for {shard_id} must not contain itself")
        missing_dependencies = [dependency for dependency in dependencies if dependency not in seen_ids]
        if missing_dependencies:
            fail(
                f"{path}.shard_assignments dependency list for {shard_id} references unknown shard ids: "
                f"{', '.join(missing_dependencies)}"
            )


def main(argv: list[str]) -> int:
    if len(argv) < 2:
        print("Usage: validate_multi_agent_workcell_state.py <packet.json> [packet.json ...]", file=sys.stderr)
        return 2
    for arg in argv[1:]:
        validate_packet(Path(arg))
        print(f"PASS: multi-agent workcell state valid for {arg}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
