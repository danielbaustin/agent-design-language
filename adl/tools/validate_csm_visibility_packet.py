#!/usr/bin/env python3
"""Validate the fixture-level CSM Observatory visibility packet contract."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import PurePosixPath
from typing import Any


REQUIRED_TOP_LEVEL = [
    "schema",
    "packet_id",
    "generated_at",
    "source",
    "manifold",
    "kernel",
    "citizens",
    "episodes",
    "freedom_gate",
    "invariants",
    "resources",
    "trace",
    "operator_actions",
    "review",
]

REQUIRED_SECTIONS: dict[str, list[str]] = {
    "source": ["mode", "evidence_level", "fixture", "runtime_artifact_root", "claim_boundary"],
    "manifold": [
        "manifold_id",
        "display_name",
        "state",
        "lifecycle",
        "current_tick",
        "uptime",
        "policy_profile",
        "snapshot_status",
        "health",
        "evidence_refs",
    ],
    "kernel": [
        "scheduler_state",
        "trace_state",
        "invariant_state",
        "resource_state",
        "service_states",
        "active_guardrails",
        "pulse",
    ],
    "freedom_gate": [
        "recent_docket",
        "allow_count",
        "defer_count",
        "refuse_count",
        "open_questions",
        "rejected_actions",
    ],
    "resources": [
        "compute_units",
        "memory_pressure",
        "queue_depth",
        "fairness_notes",
        "scarcity_events",
    ],
    "trace": [
        "trace_tail",
        "causal_gaps",
        "latest_operator_event",
        "latest_citizen_event",
        "latest_kernel_event",
    ],
    "operator_actions": [
        "available_actions",
        "disabled_actions",
        "required_confirmations",
        "safety_notes",
    ],
    "review": [
        "primary_artifacts",
        "missing_artifacts",
        "demo_classification",
        "caveats",
        "next_consumers",
    ],
}

MANIFOLD_STATES = {
    "initialized",
    "running",
    "quiescing",
    "sleeping",
    "sealed",
    "rehydrating",
    "degraded",
    "blocked",
}

CITIZEN_STATES = {
    "proposed",
    "active",
    "awake",
    "sleeping",
    "paused",
    "degraded",
    "blocked",
    "suspended",
    "migrating",
}

EPISODE_STATES = {"planned", "active", "completed", "blocked", "deferred", "failed"}
FREEDOM_GATE_DECISIONS = {"allow", "defer", "refuse"}
INVARIANT_STATES = {"healthy", "warning", "violated", "blocked", "missing", "deferred"}
INVARIANT_SEVERITIES = {"info", "low", "medium", "high", "critical"}

LEAK_PATTERNS = [
    re.compile(r"/Users/[^\\s\"']+"),
    re.compile(r"/private/var/[^\\s\"']+"),
    re.compile(r"localhost:\\d+"),
    re.compile(r"192\\.168\\.\\d+\\.\\d+"),
    re.compile(r"(?i)bearer\\s+[A-Za-z0-9._\\-]+"),
    re.compile(r"(?i)(api[_-]?key|secret|token)\\s*[:=]\\s*[A-Za-z0-9._\\-]{8,}"),
]


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def require_mapping(errors: list[str], value: Any, path: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        fail(errors, f"{path} must be an object")
        return {}
    return value


def require_list(errors: list[str], value: Any, path: str, min_items: int = 0) -> list[Any]:
    if not isinstance(value, list):
        fail(errors, f"{path} must be a list")
        return []
    if len(value) < min_items:
        fail(errors, f"{path} must contain at least {min_items} item(s)")
    return value


def require_fields(errors: list[str], value: dict[str, Any], fields: list[str], path: str) -> None:
    for field in fields:
        if field not in value:
            fail(errors, f"{path}.{field} is required")


def check_relative_ref(errors: list[str], value: Any, path: str) -> None:
    if value is None:
        return
    if not isinstance(value, str):
        return
    if value.startswith(("http://", "https://")):
        fail(errors, f"{path} must not be a URL")
        return
    posix = PurePosixPath(value)
    if posix.is_absolute() or ".." in posix.parts:
        fail(errors, f"{path} must be repository-relative")


def walk_strings(value: Any) -> list[str]:
    if isinstance(value, str):
        return [value]
    if isinstance(value, list):
        out: list[str] = []
        for item in value:
            out.extend(walk_strings(item))
        return out
    if isinstance(value, dict):
        out = []
        for item in value.values():
            out.extend(walk_strings(item))
        return out
    return []


def check_leakage(errors: list[str], packet: dict[str, Any]) -> None:
    for text in walk_strings(packet):
        for pattern in LEAK_PATTERNS:
            if pattern.search(text):
                fail(errors, f"private path, endpoint, or secret-like value leaked: {text}")


def validate_packet(packet: dict[str, Any]) -> list[str]:
    errors: list[str] = []

    require_fields(errors, packet, REQUIRED_TOP_LEVEL, "packet")
    if packet.get("schema") != "adl.csm_visibility_packet.v1":
        fail(errors, "packet.schema must be adl.csm_visibility_packet.v1")

    for section, fields in REQUIRED_SECTIONS.items():
        mapping = require_mapping(errors, packet.get(section), f"packet.{section}")
        require_fields(errors, mapping, fields, f"packet.{section}")

    source = require_mapping(errors, packet.get("source"), "packet.source")
    if source.get("mode") not in {"fixture", "captured_artifacts", "live_runtime"}:
        fail(errors, "packet.source.mode is invalid")
    if source.get("mode") == "fixture" and source.get("fixture") is not True:
        fail(errors, "fixture mode must set packet.source.fixture to true")
    if source.get("mode") == "fixture" and "not a live" not in str(source.get("claim_boundary", "")).lower():
        fail(errors, "fixture mode must state that it is not a live runtime capture")

    manifold = require_mapping(errors, packet.get("manifold"), "packet.manifold")
    if manifold.get("state") not in MANIFOLD_STATES:
        fail(errors, f"packet.manifold.state is invalid: {manifold.get('state')}")

    citizens = require_list(errors, packet.get("citizens"), "packet.citizens", min_items=2)
    seen_citizens: set[str] = set()
    for index, citizen_value in enumerate(citizens):
        citizen = require_mapping(errors, citizen_value, f"packet.citizens[{index}]")
        require_fields(
            errors,
            citizen,
            [
                "citizen_id",
                "display_name",
                "role",
                "lifecycle_state",
                "continuity_status",
                "current_episode",
                "resource_balance",
                "recent_decisions",
                "capability_envelope",
                "alerts",
                "evidence_refs",
            ],
            f"packet.citizens[{index}]",
        )
        citizen_id = citizen.get("citizen_id")
        if isinstance(citizen_id, str):
            if citizen_id in seen_citizens:
                fail(errors, f"duplicate citizen_id: {citizen_id}")
            seen_citizens.add(citizen_id)
        if citizen.get("lifecycle_state") not in CITIZEN_STATES:
            fail(errors, f"citizen lifecycle_state is invalid: {citizen.get('lifecycle_state')}")

    for index, episode_value in enumerate(require_list(errors, packet.get("episodes"), "packet.episodes")):
        episode = require_mapping(errors, episode_value, f"packet.episodes[{index}]")
        require_fields(
            errors,
            episode,
            [
                "episode_id",
                "title",
                "state",
                "citizen_ids",
                "started_at",
                "last_event",
                "proof_surface",
                "blocked_reason",
            ],
            f"packet.episodes[{index}]",
        )
        if episode.get("state") not in EPISODE_STATES:
            fail(errors, f"episode state is invalid: {episode.get('state')}")
        for citizen_id in require_list(errors, episode.get("citizen_ids"), f"packet.episodes[{index}].citizen_ids"):
            if citizen_id not in seen_citizens:
                fail(errors, f"episode references unknown citizen_id: {citizen_id}")

    freedom_gate = require_mapping(errors, packet.get("freedom_gate"), "packet.freedom_gate")
    allow = defer = refuse = 0
    for index, entry_value in enumerate(require_list(errors, freedom_gate.get("recent_docket"), "packet.freedom_gate.recent_docket")):
        entry = require_mapping(errors, entry_value, f"packet.freedom_gate.recent_docket[{index}]")
        require_fields(errors, entry, ["decision_id", "actor", "action", "decision", "rationale", "evidence_ref"], f"packet.freedom_gate.recent_docket[{index}]")
        decision = entry.get("decision")
        if decision not in FREEDOM_GATE_DECISIONS:
            fail(errors, f"Freedom Gate decision is invalid: {decision}")
        allow += 1 if decision == "allow" else 0
        defer += 1 if decision == "defer" else 0
        refuse += 1 if decision == "refuse" else 0
        check_relative_ref(errors, entry.get("evidence_ref"), f"packet.freedom_gate.recent_docket[{index}].evidence_ref")
    if freedom_gate.get("allow_count") != allow:
        fail(errors, "packet.freedom_gate.allow_count does not match recent_docket")
    if freedom_gate.get("defer_count") != defer:
        fail(errors, "packet.freedom_gate.defer_count does not match recent_docket")
    if freedom_gate.get("refuse_count") != refuse:
        fail(errors, "packet.freedom_gate.refuse_count does not match recent_docket")

    for index, invariant_value in enumerate(require_list(errors, packet.get("invariants"), "packet.invariants")):
        invariant = require_mapping(errors, invariant_value, f"packet.invariants[{index}]")
        require_fields(errors, invariant, ["invariant_id", "name", "state", "severity", "last_checked", "evidence_ref"], f"packet.invariants[{index}]")
        if invariant.get("state") not in INVARIANT_STATES:
            fail(errors, f"invariant state is invalid: {invariant.get('state')}")
        if invariant.get("severity") not in INVARIANT_SEVERITIES:
            fail(errors, f"invariant severity is invalid: {invariant.get('severity')}")
        check_relative_ref(errors, invariant.get("evidence_ref"), f"packet.invariants[{index}].evidence_ref")

    operator_actions = require_mapping(errors, packet.get("operator_actions"), "packet.operator_actions")
    if source.get("mode") == "fixture":
        for index, action_value in enumerate(require_list(errors, operator_actions.get("available_actions"), "packet.operator_actions.available_actions")):
            action = require_mapping(errors, action_value, f"packet.operator_actions.available_actions[{index}]")
            if action.get("mode") != "read_only":
                fail(errors, "fixture packet available actions must be read_only")

    review = require_mapping(errors, packet.get("review"), "packet.review")
    if source.get("mode") == "fixture" and review.get("demo_classification") != "fixture_backed":
        fail(errors, "fixture packet review.demo_classification must be fixture_backed")
    consumers = require_list(errors, review.get("next_consumers"), "packet.review.next_consumers", min_items=4)
    expected_issues = {2189, 2190, 2191, 2192}
    observed_issues = {item.get("issue") for item in consumers if isinstance(item, dict)}
    if not expected_issues.issubset(observed_issues):
        fail(errors, "packet.review.next_consumers must include issues 2189, 2190, 2191, and 2192")

    for ref_path in walk_refs(packet):
        check_relative_ref(errors, ref_path[1], ref_path[0])
    check_leakage(errors, packet)

    return errors


def walk_refs(value: Any, path: str = "packet") -> list[tuple[str, Any]]:
    refs: list[tuple[str, Any]] = []
    if isinstance(value, dict):
        for key, child in value.items():
            child_path = f"{path}.{key}"
            if key.endswith("_ref") or key.endswith("_refs") or key in {"primary_artifacts", "missing_artifacts"}:
                refs.append((child_path, child))
            refs.extend(walk_refs(child, child_path))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            refs.extend(walk_refs(child, f"{path}[{index}]"))
    return refs


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet")
    args = parser.parse_args()

    with open(args.packet, "r", encoding="utf-8") as handle:
        packet = json.load(handle)

    errors = validate_packet(packet)
    if errors:
        for error in errors:
            print(f"FAIL: {error}", file=sys.stderr)
        return 1

    print(f"PASS: {args.packet} is a valid CSM visibility packet")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
