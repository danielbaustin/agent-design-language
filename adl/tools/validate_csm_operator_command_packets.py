#!/usr/bin/env python3
"""Validate CSM Observatory operator command packet examples."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import PurePosixPath
from typing import Any


COMMAND_KINDS = {
    "inspect_manifold",
    "inspect_citizen",
    "inspect_episode",
    "open_freedom_gate_decision",
    "annotate_trace",
    "ask_shepherd",
    "pause_citizen",
    "resume_citizen",
    "request_snapshot",
    "request_quiesce",
    "request_wake",
    "request_recovery_review",
}

TARGET_KINDS = {
    "manifold",
    "citizen",
    "episode",
    "trace",
    "invariant",
    "freedom_gate",
    "shepherd",
    "kernel_service",
    "snapshot",
}

AVAILABILITY_STATES = {
    "available",
    "disabled",
    "deferred",
    "requires_confirmation",
    "submitted",
    "refused",
}

SAFETY_CLASSIFICATIONS = {
    "read_only",
    "advisory",
    "reversible_state_change",
    "guarded_mutation",
    "destructive_or_irreversible",
}

EVENT_DECISIONS = {"recorded", "accepted", "deferred", "refused", "blocked"}

REQUIRED_COMMAND_FIELDS = [
    "schema",
    "command_id",
    "command_kind",
    "requested_by",
    "requested_at",
    "target",
    "intent",
    "availability",
    "safety",
    "confirmation",
    "kernel_handoff",
    "event_logging",
    "evidence_refs",
    "claim_boundary",
]

REQUIRED_EVENT_FIELDS = [
    "event_schema",
    "event_id",
    "command_id",
    "command_kind",
    "requested_by",
    "requested_at",
    "target",
    "availability_state",
    "safety_classification",
    "decision",
    "decision_reason",
    "kernel_service",
    "evidence_refs",
]

LEAK_PATTERNS = [
    re.compile(r"/Users/[^\s\"']+"),
    re.compile(r"/private/var/[^\s\"']+"),
    re.compile(r"localhost:\d+"),
    re.compile(r"192\.168\.\d+\.\d+"),
    re.compile(r"(?i)bearer\s+[A-Za-z0-9._\-]+"),
    re.compile(r"(?i)(api[_-]?key|secret|token)\s*[:=]\s*[A-Za-z0-9._\-]{8,}"),
]


def add(errors: list[str], message: str) -> None:
    errors.append(message)


def mapping(errors: list[str], value: Any, path: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        add(errors, f"{path} must be an object")
        return {}
    return value


def values(errors: list[str], value: Any, path: str, min_items: int = 0) -> list[Any]:
    if not isinstance(value, list):
        add(errors, f"{path} must be a list")
        return []
    if len(value) < min_items:
        add(errors, f"{path} must contain at least {min_items} item(s)")
    return value


def require_fields(errors: list[str], value: dict[str, Any], fields: list[str], path: str) -> None:
    for field in fields:
        if field not in value:
            add(errors, f"{path}.{field} is required")


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


def check_leakage(errors: list[str], value: Any) -> None:
    for text in walk_strings(value):
        for pattern in LEAK_PATTERNS:
            if pattern.search(text):
                add(errors, f"private path, endpoint, or secret-like value leaked: {text}")


def check_ref(errors: list[str], value: Any, path: str) -> None:
    if not isinstance(value, str):
        add(errors, f"{path} must be a string")
        return
    if value.startswith(("http://", "https://")):
        add(errors, f"{path} must not be a URL")
        return
    posix = PurePosixPath(value)
    if posix.is_absolute() or ".." in posix.parts:
        add(errors, f"{path} must be repository-relative")


def validate_command(errors: list[str], command: dict[str, Any], path: str) -> None:
    require_fields(errors, command, REQUIRED_COMMAND_FIELDS, path)
    if command.get("schema") != "adl.csm_operator_command_packet.v1":
        add(errors, f"{path}.schema must be adl.csm_operator_command_packet.v1")
    if command.get("command_kind") not in COMMAND_KINDS:
        add(errors, f"{path}.command_kind is invalid: {command.get('command_kind')}")

    target = mapping(errors, command.get("target"), f"{path}.target")
    require_fields(errors, target, ["target_kind", "target_id", "manifold_id", "evidence_ref"], f"{path}.target")
    if target.get("target_kind") not in TARGET_KINDS:
        add(errors, f"{path}.target.target_kind is invalid: {target.get('target_kind')}")
    check_ref(errors, target.get("evidence_ref"), f"{path}.target.evidence_ref")

    availability = mapping(errors, command.get("availability"), f"{path}.availability")
    require_fields(errors, availability, ["state", "reason", "enabled_in"], f"{path}.availability")
    if availability.get("state") not in AVAILABILITY_STATES:
        add(errors, f"{path}.availability.state is invalid: {availability.get('state')}")

    safety = mapping(errors, command.get("safety"), f"{path}.safety")
    require_fields(
        errors,
        safety,
        ["classification", "policy_checks", "side_effect_scope", "operator_risk", "citizen_risk"],
        f"{path}.safety",
    )
    safety_classification = safety.get("classification")
    if safety_classification not in SAFETY_CLASSIFICATIONS:
        add(errors, f"{path}.safety.classification is invalid: {safety_classification}")
    values(errors, safety.get("policy_checks"), f"{path}.safety.policy_checks", min_items=1)

    confirmation = mapping(errors, command.get("confirmation"), f"{path}.confirmation")
    require_fields(
        errors,
        confirmation,
        ["required", "prompt", "required_phrase", "expires_after_seconds"],
        f"{path}.confirmation",
    )
    if safety_classification in {"guarded_mutation", "destructive_or_irreversible"}:
        if availability.get("state") not in {"disabled", "requires_confirmation", "refused"}:
            add(errors, f"{path} guarded mutation must be disabled, refused, or require confirmation")
        if availability.get("state") != "disabled" and confirmation.get("required") is not True:
            add(errors, f"{path} enabled guarded mutation must require confirmation")

    handoff = mapping(errors, command.get("kernel_handoff"), f"{path}.kernel_handoff")
    require_fields(
        errors,
        handoff,
        [
            "service",
            "operation",
            "policy_checks",
            "trace_append_required",
            "invariant_check_required",
            "ui_direct_mutation_allowed",
        ],
        f"{path}.kernel_handoff",
    )
    if handoff.get("ui_direct_mutation_allowed") is not False:
        add(errors, f"{path}.kernel_handoff.ui_direct_mutation_allowed must be false")
    if handoff.get("trace_append_required") is not True:
        add(errors, f"{path}.kernel_handoff.trace_append_required must be true")
    values(errors, handoff.get("policy_checks"), f"{path}.kernel_handoff.policy_checks", min_items=1)

    event_logging = mapping(errors, command.get("event_logging"), f"{path}.event_logging")
    require_fields(
        errors,
        event_logging,
        [
            "event_schema",
            "event_ref",
            "required_fields",
            "redact_prompt_or_tool_args",
            "append_before_effect",
        ],
        f"{path}.event_logging",
    )
    if event_logging.get("event_schema") != "adl.csm_operator_event.v1":
        add(errors, f"{path}.event_logging.event_schema is invalid")
    if event_logging.get("redact_prompt_or_tool_args") is not True:
        add(errors, f"{path}.event_logging.redact_prompt_or_tool_args must be true")
    check_ref(errors, event_logging.get("event_ref"), f"{path}.event_logging.event_ref")
    required_fields = set(values(errors, event_logging.get("required_fields"), f"{path}.event_logging.required_fields", min_items=10))
    missing = set(REQUIRED_EVENT_FIELDS) - required_fields
    if missing:
        add(errors, f"{path}.event_logging.required_fields missing: {', '.join(sorted(missing))}")

    for index, ref in enumerate(values(errors, command.get("evidence_refs"), f"{path}.evidence_refs", min_items=1)):
        check_ref(errors, ref, f"{path}.evidence_refs[{index}]")


def validate_event(errors: list[str], event: dict[str, Any]) -> None:
    require_fields(errors, event, REQUIRED_EVENT_FIELDS, "event")
    if event.get("event_schema") != "adl.csm_operator_event.v1":
        add(errors, "event.event_schema must be adl.csm_operator_event.v1")
    if event.get("command_kind") not in COMMAND_KINDS:
        add(errors, f"event.command_kind is invalid: {event.get('command_kind')}")
    if event.get("availability_state") not in AVAILABILITY_STATES:
        add(errors, f"event.availability_state is invalid: {event.get('availability_state')}")
    if event.get("safety_classification") not in SAFETY_CLASSIFICATIONS:
        add(errors, f"event.safety_classification is invalid: {event.get('safety_classification')}")
    if event.get("decision") not in EVENT_DECISIONS:
        add(errors, f"event.decision is invalid: {event.get('decision')}")
    target = mapping(errors, event.get("target"), "event.target")
    require_fields(errors, target, ["target_kind", "target_id", "manifold_id", "evidence_ref"], "event.target")
    check_ref(errors, event.get("trace_ref"), "event.trace_ref")
    for index, ref in enumerate(values(errors, event.get("evidence_refs"), "event.evidence_refs", min_items=1)):
        check_ref(errors, ref, f"event.evidence_refs[{index}]")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_set")
    parser.add_argument("event")
    args = parser.parse_args()

    with open(args.packet_set, "r", encoding="utf-8") as handle:
        packet_set = json.load(handle)
    with open(args.event, "r", encoding="utf-8") as handle:
        event = json.load(handle)

    errors: list[str] = []
    if packet_set.get("schema") != "adl.csm_operator_command_packet_examples.v1":
        add(errors, "packet_set.schema must be adl.csm_operator_command_packet_examples.v1")
    commands = values(errors, packet_set.get("commands"), "packet_set.commands", min_items=3)
    observed = set()
    for index, command_value in enumerate(commands):
        command = mapping(errors, command_value, f"packet_set.commands[{index}]")
        validate_command(errors, command, f"packet_set.commands[{index}]")
        kind = command.get("command_kind")
        if kind in observed:
            add(errors, f"duplicate command_kind example: {kind}")
        observed.add(kind)
    for required in ["inspect_citizen", "request_snapshot", "ask_shepherd"]:
        if required not in observed:
            add(errors, f"missing required command example: {required}")

    validate_event(errors, event)
    if event.get("command_id") != "csm-cmd-proto-csm-01-request-snapshot-0001":
        add(errors, "event must correspond to the disabled request_snapshot example")
    if event.get("decision") != "blocked":
        add(errors, "fixture request_snapshot event must remain blocked")

    check_leakage(errors, packet_set)
    check_leakage(errors, event)

    if errors:
        for error in errors:
            print(f"FAIL: {error}", file=sys.stderr)
        return 1

    print(f"PASS: {args.packet_set} and {args.event} are valid CSM operator command packet fixtures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
