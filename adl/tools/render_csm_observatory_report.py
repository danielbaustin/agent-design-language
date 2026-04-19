#!/usr/bin/env python3
"""Render a reviewer-facing CSM Observatory operator report from a visibility packet."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


SEVERITY_ORDER = {
    "critical": 0,
    "high": 1,
    "medium": 2,
    "low": 3,
    "info": 4,
}


def as_list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def as_mapping(value: Any) -> dict[str, Any]:
    return value if isinstance(value, dict) else {}


def text(value: Any, fallback: str = "not recorded") -> str:
    if value is None:
        return fallback
    value_text = str(value).strip()
    return value_text if value_text else fallback


def bullet(value: str) -> str:
    return f"- {value}"


def table_row(values: list[Any]) -> str:
    return "| " + " | ".join(text(value).replace("\n", " ") for value in values) + " |"


def evidence_refs(*values: Any) -> list[str]:
    refs: list[str] = []
    for value in values:
        if isinstance(value, str) and value not in refs:
            refs.append(value)
        elif isinstance(value, list):
            for item in value:
                if isinstance(item, str) and item not in refs:
                    refs.append(item)
    return refs


def render_attention_items(packet: dict[str, Any]) -> list[str]:
    items: list[tuple[int, str]] = []

    def add(priority: int, item: str) -> None:
        clean = text(item).strip()
        if clean:
            items.append((priority, clean[:1].upper() + clean[1:]))

    manifold = as_mapping(packet.get("manifold"))
    health = as_mapping(manifold.get("health"))
    for item in as_list(health.get("attention_items")):
        normalized_item = text(item).lower()
        if "snapshot evidence" in normalized_item or "proposed, not active" in normalized_item:
            continue
        add(2, text(item))

    snapshot = as_mapping(manifold.get("snapshot_status"))
    if snapshot.get("state") in {"deferred", "missing", "blocked"}:
        add(1, f"Snapshot evidence is {text(snapshot.get('state'))}: {text(snapshot.get('note'))}")

    for citizen in as_list(packet.get("citizens")):
        citizen_map = as_mapping(citizen)
        state = citizen_map.get("lifecycle_state")
        if state != "active":
            add(
                1,
                (
                    f"{text(citizen_map.get('display_name'))} is {text(state)}, not active; "
                    f"continuity is {text(citizen_map.get('continuity_status'))}."
                ),
            )
        for alert in as_list(citizen_map.get("alerts")):
            alert_map = as_mapping(alert)
            severity = text(alert_map.get("severity"), "info")
            if "proposed, not active" in text(alert_map.get("message")).lower():
                continue
            add(SEVERITY_ORDER.get(severity, 4), f"{severity}: {text(alert_map.get('message'))}")

    for invariant in as_list(packet.get("invariants")):
        invariant_map = as_mapping(invariant)
        state = invariant_map.get("state")
        if state != "healthy":
            severity = text(invariant_map.get("severity"), "info")
            priority = SEVERITY_ORDER.get(severity, 4)
            add(
                priority,
                f"{text(invariant_map.get('name'))} is {text(state)} "
                f"({severity}); evidence: {text(invariant_map.get('evidence_ref'))}.",
            )

    trace = as_mapping(packet.get("trace"))
    for gap in as_list(trace.get("causal_gaps")):
        gap_map = as_mapping(gap)
        severity = text(gap_map.get("severity"), "info")
        add(SEVERITY_ORDER.get(severity, 4), f"{severity}: {text(gap_map.get('summary'))}")

    freedom_gate = as_mapping(packet.get("freedom_gate"))
    for question in as_list(freedom_gate.get("open_questions")):
        add(2, f"Open Freedom Gate question: {text(question)}")

    operator_actions = as_mapping(packet.get("operator_actions"))
    for action in as_list(operator_actions.get("disabled_actions")):
        action_map = as_mapping(action)
        future_issue = action_map.get("future_issue")
        suffix = f" Future issue: #{future_issue}." if future_issue else ""
        reason = text(action_map.get("reason")).rstrip(".")
        add(
            2,
            f"Operator action {text(action_map.get('action'))} remains disabled: {reason}.{suffix}",
        )

    rendered: list[str] = []
    seen: set[str] = set()
    for _, item in sorted(items, key=lambda candidate: (candidate[0], candidate[1].lower())):
        normalized = item.lower().rstrip(".")
        if normalized not in seen:
            rendered.append(item)
            seen.add(normalized)
    return rendered


def render_report(packet: dict[str, Any]) -> str:
    source = as_mapping(packet.get("source"))
    manifold = as_mapping(packet.get("manifold"))
    kernel = as_mapping(packet.get("kernel"))
    resources = as_mapping(packet.get("resources"))
    compute = as_mapping(resources.get("compute_units"))
    freedom_gate = as_mapping(packet.get("freedom_gate"))
    trace = as_mapping(packet.get("trace"))
    operator_actions = as_mapping(packet.get("operator_actions"))
    review = as_mapping(packet.get("review"))

    title = f"CSM Observatory Operator Report: {text(manifold.get('display_name'))}"
    lines: list[str] = [
        f"# {title}",
        "",
        "## Report Identity",
        table_row(["Field", "Value"]),
        table_row(["---", "---"]),
        table_row(["Packet", packet.get("packet_id")]),
        table_row(["Schema", packet.get("schema")]),
        table_row(["Generated", packet.get("generated_at")]),
        table_row(["Source mode", source.get("mode")]),
        table_row(["Evidence level", source.get("evidence_level")]),
        table_row(["Demo classification", review.get("demo_classification")]),
        "",
        "## Operator Summary",
        f"The manifold is {text(manifold.get('state'))} at tick {text(manifold.get('current_tick'))}. "
        f"The kernel pulse is {text(as_mapping(kernel.get('pulse')).get('status'))} through event sequence "
        f"{text(as_mapping(kernel.get('pulse')).get('completed_through_event_sequence'))}. "
        f"Current evidence is {text(source.get('evidence_level'))}; claim boundary: {text(source.get('claim_boundary'))}",
        "",
        "## Attention Items",
    ]

    attention_items = render_attention_items(packet)
    if attention_items:
        lines.extend(bullet(item) for item in attention_items)
    else:
        lines.append("- No operator attention items were reported by this packet.")

    lines.extend(
        [
            "",
            "## Manifold And Kernel",
            table_row(["Field", "Value"]),
            table_row(["---", "---"]),
            table_row(["Manifold", manifold.get("manifold_id")]),
            table_row(["Lifecycle", manifold.get("lifecycle")]),
            table_row(["Policy profile", manifold.get("policy_profile")]),
            table_row(["Health", f"{text(as_mapping(manifold.get('health')).get('level'))}: {text(as_mapping(manifold.get('health')).get('summary'))}"]),
            table_row(["Scheduler", kernel.get("scheduler_state")]),
            table_row(["Trace", kernel.get("trace_state")]),
            table_row(["Invariants", kernel.get("invariant_state")]),
            table_row(["Resources", kernel.get("resource_state")]),
            "",
            "## Citizens",
            table_row(["Citizen", "State", "Continuity", "Episode", "Compute", "Capability"]),
            table_row(["---", "---", "---", "---", "---", "---"]),
        ]
    )

    for citizen in as_list(packet.get("citizens")):
        citizen_map = as_mapping(citizen)
        balance = as_mapping(citizen_map.get("resource_balance"))
        envelope = as_mapping(citizen_map.get("capability_envelope"))
        capability = "episode execution allowed" if envelope.get("can_execute_episodes") else "episode execution disabled"
        lines.append(
            table_row(
                [
                    citizen_map.get("display_name"),
                    citizen_map.get("lifecycle_state"),
                    citizen_map.get("continuity_status"),
                    citizen_map.get("current_episode"),
                    balance.get("compute_units"),
                    capability,
                ]
            )
        )

    lines.extend(
        [
            "",
            "## Freedom Gate Docket",
            f"Counts: allow {text(freedom_gate.get('allow_count'))}, defer {text(freedom_gate.get('defer_count'))}, refuse {text(freedom_gate.get('refuse_count'))}.",
            "",
            table_row(["Decision", "Actor", "Action", "Rationale", "Evidence"]),
            table_row(["---", "---", "---", "---", "---"]),
        ]
    )
    for entry in as_list(freedom_gate.get("recent_docket")):
        entry_map = as_mapping(entry)
        lines.append(
            table_row(
                [
                    entry_map.get("decision"),
                    entry_map.get("actor"),
                    entry_map.get("action"),
                    entry_map.get("rationale"),
                    entry_map.get("evidence_ref"),
                ]
            )
        )

    lines.extend(
        [
            "",
            "## Invariant Review",
            table_row(["Invariant", "State", "Severity", "Evidence"]),
            table_row(["---", "---", "---", "---"]),
        ]
    )
    for invariant in sorted(
        as_list(packet.get("invariants")),
        key=lambda item: (SEVERITY_ORDER.get(text(as_mapping(item).get("severity")), 99), text(as_mapping(item).get("name")).lower()),
    ):
        invariant_map = as_mapping(invariant)
        lines.append(
            table_row(
                [
                    invariant_map.get("name"),
                    invariant_map.get("state"),
                    invariant_map.get("severity"),
                    invariant_map.get("evidence_ref"),
                ]
            )
        )

    lines.extend(
        [
            "",
            "## Resources",
            table_row(["Total compute", compute.get("total")]),
            table_row(["Allocated compute", compute.get("allocated")]),
            table_row(["Available compute", compute.get("available")]),
            table_row(["Memory pressure", resources.get("memory_pressure")]),
            table_row(["Queue depth", resources.get("queue_depth")]),
            "",
            "Fairness notes:",
        ]
    )
    lines.extend(bullet(text(note)) for note in as_list(resources.get("fairness_notes")))

    lines.extend(
        [
            "",
            "## Trace Tail",
            table_row(["Seq", "Actor", "Event", "Summary", "Evidence"]),
            table_row(["---", "---", "---", "---", "---"]),
        ]
    )
    for event in sorted(as_list(trace.get("trace_tail")), key=lambda item: as_mapping(item).get("event_sequence", 0)):
        event_map = as_mapping(event)
        lines.append(
            table_row(
                [
                    event_map.get("event_sequence"),
                    event_map.get("actor"),
                    event_map.get("event_type"),
                    event_map.get("summary"),
                    event_map.get("evidence_ref"),
                ]
            )
        )

    lines.extend(
        [
            "",
            "## Operator Action Boundary",
            "Available read-only actions:",
        ]
    )
    lines.extend(
        bullet(f"{text(as_mapping(action).get('action'))}: {text(as_mapping(action).get('status'))}")
        for action in as_list(operator_actions.get("available_actions"))
    )
    lines.append("")
    lines.append("Disabled mutation actions:")
    lines.extend(
        bullet(f"{text(as_mapping(action).get('action'))}: {text(as_mapping(action).get('reason'))}")
        for action in as_list(operator_actions.get("disabled_actions"))
    )
    lines.append("")
    lines.append("Required confirmations:")
    lines.extend(bullet(text(item)) for item in as_list(operator_actions.get("required_confirmations")))

    primary_refs = evidence_refs(
        source.get("source_refs"),
        manifold.get("evidence_refs"),
        as_mapping(kernel.get("pulse")).get("evidence_refs"),
        review.get("primary_artifacts"),
    )
    missing = as_list(review.get("missing_artifacts"))

    lines.extend(
        [
            "",
            "## Evidence And Caveats",
            "Primary evidence references:",
        ]
    )
    lines.extend(bullet(ref) for ref in primary_refs)
    lines.append("")
    lines.append("Missing or deferred artifacts:")
    for artifact in missing:
        artifact_map = as_mapping(artifact)
        lines.append(
            bullet(
                f"{text(artifact_map.get('artifact'))}: {text(artifact_map.get('status'))}; "
                f"owner {text(artifact_map.get('owner'))}"
            )
        )
    lines.append("")
    lines.append("Caveats:")
    lines.extend(bullet(text(caveat)) for caveat in as_list(review.get("caveats")))

    lines.extend(
        [
            "",
            "## Next Consumers",
            table_row(["Issue", "Consumer"]),
            table_row(["---", "---"]),
        ]
    )
    for consumer in sorted(as_list(review.get("next_consumers")), key=lambda item: as_mapping(item).get("issue", 0)):
        consumer_map = as_mapping(consumer)
        lines.append(table_row([f"#{consumer_map.get('issue')}", consumer_map.get("consumer")]))

    lines.extend(
        [
            "",
            "## Reviewer Use",
            "This report is a proof surface for the packet-to-operator-report path. It is useful for reviewing visibility semantics, attention routing, claim boundaries, and evidence coverage without opening the HTML console.",
            "",
        ]
    )
    return "\n".join(lines)


def load_packet(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        packet = json.load(handle)
    if not isinstance(packet, dict):
        raise ValueError("packet root must be an object")
    return packet


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet", type=Path, help="Visibility packet JSON path")
    parser.add_argument("--output", type=Path, help="Markdown report output path")
    args = parser.parse_args()

    packet = load_packet(args.packet)
    rendered = render_report(packet)

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered, encoding="utf-8")
    else:
        sys.stdout.write(rendered)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
