#!/usr/bin/env python3
"""Summarize nextest timing logs into reviewable hotspot reports."""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from dataclasses import asdict, dataclass
from pathlib import Path


EVENT_RE = re.compile(
    r"^\s*(?P<status>PASS|FAIL|SLOW)\s+"
    r"\[\s*(?P<op>[<>]?)\s*(?P<seconds>\d+(?:\.\d+)?)s\]\s+"
    r"\((?P<progress>[^)]*)\)\s+"
    r"(?P<rest>.+?)\s*$"
)
START_RE = re.compile(r"Starting\s+(?P<count>\d+)\s+tests\s+across\s+(?P<binaries>\d+)\s+binaries")


@dataclass(frozen=True)
class TestEvent:
    status: str
    seconds: float
    binary: str
    test_name: str
    family: str
    cluster: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Summarize cargo-nextest PASS/SLOW timing logs into hotspot families."
    )
    parser.add_argument("log", type=Path, help="Path to a captured nextest log.")
    parser.add_argument("--top", type=int, default=20, help="Number of top completed tests to print.")
    parser.add_argument(
        "--min-seconds",
        type=float,
        default=1.0,
        help="Minimum completed-test duration included in family/cluster summaries.",
    )
    parser.add_argument(
        "--format",
        choices=("markdown", "json"),
        default="markdown",
        help="Output format.",
    )
    return parser.parse_args()


def split_binary_and_test(rest: str) -> tuple[str, str]:
    parts = rest.split(None, 1)
    if len(parts) == 1:
        return "unknown", parts[0]
    return parts[0], parts[1]


def family_for(test_name: str) -> str:
    if "::tests::" in test_name:
        prefix, tail = test_name.split("::tests::", 1)
        first = tail.split("::", 1)[0]
        if first:
            return f"{prefix}::{first}"
        return prefix
    parts = test_name.split("::")
    if len(parts) >= 3:
        return "::".join(parts[:3])
    if len(parts) >= 2:
        return "::".join(parts[:2])
    return test_name


def cluster_for(test_name: str) -> str:
    lower = test_name.lower()
    if "contract_registry" in lower or "accessors" in lower or "shared_registry" in lower:
        return "runtime-v2-contract-registry/accessors"
    if "materializes_fixture" in lower or "materializes_fixtures" in lower:
        return "proof-materialization"
    if "golden" in lower:
        return "golden-fixture"
    if "proof_route" in lower:
        return "proof-route"
    if "validation_rejects" in lower or "validate_against_rejects" in lower:
        return "validation-negative"
    if "runtime_v2" in lower:
        return "runtime-v2-other"
    if "coverage" in lower:
        return "coverage"
    return "other"


def parse_log(path: Path) -> tuple[list[TestEvent], list[TestEvent], dict[str, int]]:
    completed: list[TestEvent] = []
    slow_markers: list[TestEvent] = []
    metadata = {"declared_tests": 0, "declared_binaries": 0, "parsed_lines": 0}

    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        start_match = START_RE.search(line)
        if start_match:
            metadata["declared_tests"] = int(start_match.group("count"))
            metadata["declared_binaries"] = int(start_match.group("binaries"))

        match = EVENT_RE.match(line)
        if not match:
            continue

        binary, test_name = split_binary_and_test(match.group("rest"))
        event = TestEvent(
            status=match.group("status"),
            seconds=float(match.group("seconds")),
            binary=binary,
            test_name=test_name,
            family=family_for(test_name),
            cluster=cluster_for(test_name),
        )
        metadata["parsed_lines"] += 1
        if event.status == "SLOW":
            slow_markers.append(event)
        else:
            completed.append(event)

    return completed, slow_markers, metadata


def summarize(events: list[TestEvent], min_seconds: float) -> dict[str, list[dict[str, float | int | str]]]:
    family_totals: dict[str, dict[str, float | int | str]] = defaultdict(
        lambda: {"family": "", "count": 0, "total_seconds": 0.0, "max_seconds": 0.0}
    )
    cluster_totals: dict[str, dict[str, float | int | str]] = defaultdict(
        lambda: {"cluster": "", "count": 0, "total_seconds": 0.0, "max_seconds": 0.0}
    )

    for event in events:
        if event.seconds < min_seconds:
            continue
        family = family_totals[event.family]
        family["family"] = event.family
        family["count"] = int(family["count"]) + 1
        family["total_seconds"] = float(family["total_seconds"]) + event.seconds
        family["max_seconds"] = max(float(family["max_seconds"]), event.seconds)

        cluster = cluster_totals[event.cluster]
        cluster["cluster"] = event.cluster
        cluster["count"] = int(cluster["count"]) + 1
        cluster["total_seconds"] = float(cluster["total_seconds"]) + event.seconds
        cluster["max_seconds"] = max(float(cluster["max_seconds"]), event.seconds)

    def ordered(rows: dict[str, dict[str, float | int | str]]) -> list[dict[str, float | int | str]]:
        return sorted(
            rows.values(),
            key=lambda row: (float(row["total_seconds"]), float(row["max_seconds"]), str(row.get("family") or row.get("cluster"))),
            reverse=True,
        )

    return {"families": ordered(family_totals), "clusters": ordered(cluster_totals)}


def routing_hint(cluster: str, count: int, max_seconds: float) -> str:
    if cluster == "runtime-v2-contract-registry/accessors":
        return "Route to Runtime v2 registry/accessor refactor or authoritative slow-proof consolidation."
    if cluster == "proof-materialization":
        return "Route to proof-materialization slow-lane review; keep one authoritative root proof where needed."
    if count >= 3 and max_seconds >= 45.0:
        return "Repeated family-level setup pattern; consider table/collapse refactor."
    if max_seconds >= 60.0:
        return "Individual slow outlier; inspect whether the expensive proof path is still intentional."
    return "Monitor; no immediate refactor signal from timing alone."


def markdown_report(
    completed: list[TestEvent],
    slow_markers: list[TestEvent],
    metadata: dict[str, int],
    top: int,
    min_seconds: float,
) -> str:
    summaries = summarize(completed, min_seconds)
    top_events = sorted(completed, key=lambda event: event.seconds, reverse=True)[:top]
    total_seconds = sum(event.seconds for event in completed)
    slow_thresholds = defaultdict(int)
    for marker in slow_markers:
        slow_thresholds[f">{marker.seconds:.0f}s"] += 1

    lines = [
        "# Nextest Timing Hotspot Report",
        "",
        "## Summary",
        "",
        f"- Declared nextest run: {metadata['declared_tests']} tests across {metadata['declared_binaries']} binaries",
        f"- Completed timing rows parsed: {len(completed)}",
        f"- Slow threshold markers parsed: {len(slow_markers)}",
        f"- Total parsed completed runtime: {total_seconds:.3f}s",
        f"- Family/cluster summaries include completed tests >= {min_seconds:.3f}s",
        "",
    ]

    if slow_thresholds:
        lines.extend(["## Slow Threshold Markers", ""])
        for threshold, count in sorted(slow_thresholds.items()):
            lines.append(f"- {threshold}: {count}")
        lines.append("")

    lines.extend(["## Top Completed Tests", ""])
    lines.append("| Rank | Seconds | Family | Cluster | Test |")
    lines.append("| ---: | ---: | --- | --- | --- |")
    for index, event in enumerate(top_events, start=1):
        lines.append(
            f"| {index} | {event.seconds:.3f} | `{event.family}` | `{event.cluster}` | `{event.test_name}` |"
        )
    lines.append("")

    lines.extend(["## Hotspot Families", ""])
    lines.append("| Rank | Total Seconds | Count | Max Seconds | Family | Routing Hint |")
    lines.append("| ---: | ---: | ---: | ---: | --- | --- |")
    for index, row in enumerate(summaries["families"][:top], start=1):
        family_events = [event for event in completed if event.family == row["family"]]
        dominant_cluster = summarize(family_events, min_seconds)["clusters"][0]["cluster"] if family_events else "other"
        hint = routing_hint(str(dominant_cluster), int(row["count"]), float(row["max_seconds"]))
        lines.append(
            f"| {index} | {float(row['total_seconds']):.3f} | {int(row['count'])} | "
            f"{float(row['max_seconds']):.3f} | `{row['family']}` | {hint} |"
        )
    lines.append("")

    lines.extend(["## Hotspot Clusters", ""])
    lines.append("| Rank | Total Seconds | Count | Max Seconds | Cluster | Routing Hint |")
    lines.append("| ---: | ---: | ---: | ---: | --- | --- |")
    for index, row in enumerate(summaries["clusters"][:top], start=1):
        hint = routing_hint(str(row["cluster"]), int(row["count"]), float(row["max_seconds"]))
        lines.append(
            f"| {index} | {float(row['total_seconds']):.3f} | {int(row['count'])} | "
            f"{float(row['max_seconds']):.3f} | `{row['cluster']}` | {hint} |"
        )
    lines.append("")

    lines.extend(
        [
            "## Routing Guidance",
            "",
            "- Treat this report as diagnosis only; it is not a substitute for passing tests, coverage, or release proof.",
            "- Create test-refactor issues when a family has repeated slow completed tests with the same setup shape.",
            "- Keep a test in an authoritative slow-proof lane when it is the one end-to-end proof path for a feature contract.",
            "- Use focused reruns for changed bounded surfaces, but keep broad or ambiguous Rust changes fail-closed.",
        ]
    )
    return "\n".join(lines) + "\n"


def json_report(
    completed: list[TestEvent],
    slow_markers: list[TestEvent],
    metadata: dict[str, int],
    top: int,
    min_seconds: float,
) -> str:
    summaries = summarize(completed, min_seconds)
    payload = {
        "metadata": metadata,
        "completed_count": len(completed),
        "slow_marker_count": len(slow_markers),
        "total_completed_seconds": round(sum(event.seconds for event in completed), 3),
        "top_completed_tests": [asdict(event) for event in sorted(completed, key=lambda event: event.seconds, reverse=True)[:top]],
        "families": summaries["families"][:top],
        "clusters": summaries["clusters"][:top],
    }
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def main() -> int:
    args = parse_args()
    if args.top < 1:
        print("summarize_nextest_timings: --top must be >= 1", file=sys.stderr)
        return 2
    if not args.log.exists():
        print(f"summarize_nextest_timings: input not found: {args.log}", file=sys.stderr)
        return 2

    completed, slow_markers, metadata = parse_log(args.log)
    if args.format == "json":
        sys.stdout.write(json_report(completed, slow_markers, metadata, args.top, args.min_seconds))
    else:
        sys.stdout.write(markdown_report(completed, slow_markers, metadata, args.top, args.min_seconds))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
