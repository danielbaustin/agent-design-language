#!/usr/bin/env python3
"""Plan whether a bounded issue should split into follow-on issues."""

from __future__ import annotations

import argparse
import json
import re
from collections import defaultdict
from pathlib import Path
from typing import Any


SPLIT_RE = re.compile(r"\b(split|follow[- ]on|separate issue|later issue|spin out)\b", re.I)
KEEP_RE = re.compile(r"\b(must stay together|keep intact|single issue)\b", re.I)
DEFER_RE = re.compile(r"\b(defer|later pass|wait until|after .* proves out)\b", re.I)

BUCKETS: dict[str, tuple[str, ...]] = {
    "runtime": ("runtime", "executor", "classifier", "engine"),
    "tooling": ("tooling", "tool", "cli", "automation"),
    "docs": ("docs", "documentation", "guide", "schema"),
    "tests": ("test", "tests", "contract test", "coverage"),
    "review": ("review", "reviewer", "triage", "findings"),
    "release": ("release", "closeout", "milestone", "ceremony"),
    "security": ("security", "privacy", "secret", "redaction"),
    "process": ("process", "workflow", "policy", "issue graph"),
}


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="utf-8", errors="replace")


def collect_texts(task_bundle: Path | None, source_prompt: Path | None) -> list[dict[str, str]]:
    texts: list[dict[str, str]] = []
    if source_prompt and source_prompt.is_file():
        texts.append({"source": "source_issue_prompt", "text": read_text(source_prompt)})
    if task_bundle and task_bundle.is_dir():
        for name in ("stp.md", "sip.md", "sor.md"):
            path = task_bundle / name
            if path.is_file():
                texts.append({"source": name, "text": read_text(path)})
    return texts


def candidate_lines(text: str) -> list[str]:
    lines: list[str] = []
    for raw in text.splitlines():
        line = " ".join(raw.strip().split())
        if not line or line.startswith("#"):
            continue
        if line.startswith(("-", "*")) or ":" in line or len(line.split()) > 3:
            lines.append(line[:220])
    return lines


def bucket_for_line(line: str) -> str:
    lowered = line.lower().lstrip("-* ").strip()
    for bucket, keywords in BUCKETS.items():
        if lowered.startswith(f"{bucket}:"):
            return bucket
        if any(keyword in lowered for keyword in keywords):
            return bucket
    return "general"


def classify(texts: list[dict[str, str]]) -> dict[str, Any]:
    bucket_items: dict[str, list[dict[str, str]]] = defaultdict(list)
    split_markers: list[dict[str, str]] = []
    keep_markers: list[dict[str, str]] = []
    defer_markers: list[dict[str, str]] = []

    for entry in texts:
        for line in candidate_lines(entry["text"]):
            bucket = bucket_for_line(line)
            bucket_items[bucket].append({"source": entry["source"], "evidence": line})
            if SPLIT_RE.search(line):
                split_markers.append({"source": entry["source"], "evidence": line})
            if KEEP_RE.search(line):
                keep_markers.append({"source": entry["source"], "evidence": line})
            if DEFER_RE.search(line):
                defer_markers.append({"source": entry["source"], "evidence": line})

    non_general = {k: v for k, v in bucket_items.items() if k != "general" and v}
    sorted_buckets = sorted(non_general.items(), key=lambda kv: (-len(kv[1]), kv[0]))

    if split_markers and keep_markers:
        classification = "blocked"
    elif len(sorted_buckets) <= 1:
        classification = "keep_as_is"
    elif defer_markers:
        classification = "defer"
    elif split_markers or len(sorted_buckets) >= 3:
        classification = "split_now"
    else:
        classification = "defer"

    primary_bucket = sorted_buckets[0][0] if sorted_buckets else "general"
    proposed_follow_ons = []
    if classification in {"split_now", "defer"}:
        for bucket, items in sorted_buckets[1:3]:
            proposed_follow_ons.append(
                {
                    "bucket": bucket,
                    "candidate_title": f"Follow-on: {bucket} scope from current issue",
                    "rationale": items[0]["evidence"],
                    "issue_graph_link": f"split_from_current_issue::{bucket}",
                }
            )

    issue_graph_notes = []
    if classification in {"split_now", "defer"}:
        issue_graph_notes.append(f"retain_current_scope::{primary_bucket}")
        issue_graph_notes.extend(item["issue_graph_link"] for item in proposed_follow_ons)

    current_scope = {
        "primary_bucket": primary_bucket,
        "rationale": (
            sorted_buckets[0][1][0]["evidence"] if sorted_buckets else "no strong split markers found"
        ),
    }

    return {
        "classification": classification,
        "concern_buckets": {
            bucket: [item["evidence"] for item in items[:3]]
            for bucket, items in sorted(bucket_items.items())
            if items
        },
        "current_scope_recommendation": current_scope,
        "proposed_follow_ons": proposed_follow_ons,
        "issue_graph_notes": issue_graph_notes,
    }


def build_report(args: argparse.Namespace) -> dict[str, Any]:
    task_bundle = Path(args.task_bundle) if args.task_bundle else None
    source_prompt = Path(args.source_prompt) if args.source_prompt else None
    texts = collect_texts(task_bundle, source_prompt)

    if not texts:
        classification = "blocked"
        concern_buckets = {}
        current_scope = {
            "primary_bucket": "unknown",
            "rationale": "expected source issue prompt or task bundle files missing",
        }
        proposed_follow_ons = []
        issue_graph_notes = []
    else:
        result = classify(texts)
        classification = result["classification"]
        concern_buckets = result["concern_buckets"]
        current_scope = result["current_scope_recommendation"]
        proposed_follow_ons = result["proposed_follow_ons"]
        issue_graph_notes = result["issue_graph_notes"]

    summary = {
        "keep_as_is": "Issue packet remains cohesive enough to stay intact.",
        "split_now": "Issue packet should split into bounded follow-on work now.",
        "defer": "Issue packet shows split pressure, but the split should wait.",
        "blocked": "Issue packet has conflicting or insufficient split signals.",
    }[classification]

    return {
        "schema": "adl.issue_splitter_report.v1",
        "run_id": args.run_id,
        "status": classification,
        "classification": classification,
        "summary": summary,
        "concern_buckets": concern_buckets,
        "current_scope_recommendation": current_scope,
        "proposed_follow_ons": proposed_follow_ons,
        "issue_graph_notes": issue_graph_notes,
        "recommended_handoff": (
            "workflow-conductor"
            if classification == "keep_as_is"
            else "operator-review"
            if classification == "blocked"
            else "finding-to-issue-planner"
        ),
        "non_claims": [
            "This report does not create follow-on issues.",
            "This report does not rewrite the current issue cards.",
            "This report does not claim implementation scope already changed.",
        ],
        "safety_flags": {
            "issues_created": False,
            "cards_mutated": False,
            "tracker_mutated": False,
            "scope_silently_rewritten": False,
            "implementation_claimed": False,
        },
    }


def markdown_report(report: dict[str, Any]) -> str:
    lines = [
        "# Issue Splitter Summary",
        "",
        f"- Run id: `{report['run_id']}`",
        f"- Status: `{report['status']}`",
        f"- Summary: {report['summary']}",
        "",
        "## Classification",
        "",
        f"- classification: `{report['classification']}`",
        "",
        "## Concern Buckets",
        "",
    ]
    if report["concern_buckets"]:
        for bucket, items in report["concern_buckets"].items():
            lines.append(f"- `{bucket}`")
            for item in items:
                lines.append(f"  - {item.lstrip('- ').strip()}")
    else:
        lines.append("- none")

    lines.extend(
        [
            "",
            "## Current Scope Recommendation",
            "",
            f"- primary_bucket: `{report['current_scope_recommendation']['primary_bucket']}`",
            f"- rationale: {report['current_scope_recommendation']['rationale']}",
            "",
            "## Proposed Follow-Ons",
            "",
        ]
    )
    if report["proposed_follow_ons"]:
        for item in report["proposed_follow_ons"]:
            lines.append(
                f"- `{item['bucket']}` -> {item['candidate_title']} ({item['issue_graph_link']})"
            )
    else:
        lines.append("- none")

    lines.extend(["", "## Issue Graph Notes", ""])
    if report["issue_graph_notes"]:
        for note in report["issue_graph_notes"]:
            lines.append(f"- {note}")
    else:
        lines.append("- none")

    lines.extend(
        [
            "",
            "## Recommended Handoff",
            "",
            f"- recommended_handoff: `{report['recommended_handoff']}`",
            "",
            "## Non-Claims",
            "",
        ]
    )
    for item in report["non_claims"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Safety Flags", ""])
    for key, value in report["safety_flags"].items():
        lines.append(f"- {key}: {str(value).lower()}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--task-bundle")
    parser.add_argument("--source-prompt")
    parser.add_argument("--out", required=True)
    parser.add_argument("--run-id", default="issue-splitter")
    args = parser.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    report = build_report(args)
    (out / "issue_splitter_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (out / "issue_splitter_report.md").write_text(
        markdown_report(report),
        encoding="utf-8",
    )
    print(f"WROTE {out / 'issue_splitter_report.json'}")
    print(f"WROTE {out / 'issue_splitter_report.md'}")
    print(f"STATUS {report['status']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
