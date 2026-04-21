#!/usr/bin/env python3
"""Inspect review surfaces for structural readiness cleanup needs."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any


TEXT_SUFFIXES = {".md", ".txt", ".yaml", ".yml", ".json"}
PLACEHOLDER_RE = re.compile(r"\b(TBD|TODO|FIXME|placeholder|not started)\b", re.I)
BLOCKER_RE = re.compile(r"\b(BLOCKED|P0|P1|release-blocking|review-blocking|must fix)\b", re.I)
SKIPPED_RE = re.compile(r"\b(skipped|not applicable|out of scope|intentionally absent)\b", re.I)
FOLLOW_ON_RE = re.compile(r"\b(follow[- ]on|deferred|future work|later milestone|post-review)\b", re.I)


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="utf-8", errors="replace")


def collect_docs(root: Path) -> list[dict[str, str]]:
    docs: list[dict[str, str]] = []
    for path in sorted(root.rglob("*")):
        if not path.is_file() or path.suffix.lower() not in TEXT_SUFFIXES:
            continue
        docs.append({"path": path.relative_to(root).as_posix(), "text": read_text(path)})
    return docs


def first_matching_line(text: str, pattern: re.Pattern[str]) -> str:
    for line in text.splitlines():
        compact = " ".join(line.strip().split())
        if compact and pattern.search(compact):
            return compact[:180]
    return "marker present"


def add_item(
    items: list[dict[str, str]],
    category: str,
    source: str,
    reason: str,
    evidence: str,
) -> None:
    items.append(
        {
            "category": category,
            "source": source,
            "reason": reason,
            "evidence": evidence,
        }
    )


def classify_docs(docs: list[dict[str, str]]) -> list[dict[str, str]]:
    items: list[dict[str, str]] = []
    if not docs:
        add_item(
            items,
            "skipped",
            "review_root",
            "No readable review documents were found.",
            "review root empty or unsupported",
        )
        return items

    combined_paths = " ".join(doc["path"].lower() for doc in docs)
    if not any(term in combined_paths for term in ("finding", "review", "gap")):
        add_item(
            items,
            "blocker",
            "review_root",
            "No finding, review, or gap register path was found.",
            "expected review truth surface missing",
        )
    if not any(term in combined_paths for term in ("demo", "proof", "coverage")):
        add_item(
            items,
            "follow_on_needed",
            "review_root",
            "No demo or proof register path was found.",
            "proof linkage may need a follow-on surface",
        )

    for doc in docs:
        text = doc["text"]
        if BLOCKER_RE.search(text):
            add_item(
                items,
                "blocker",
                doc["path"],
                "Explicit blocker or high-priority review marker found.",
                first_matching_line(text, BLOCKER_RE),
            )
        if PLACEHOLDER_RE.search(text):
            add_item(
                items,
                "safe_mechanical_cleanup",
                doc["path"],
                "Placeholder or stale readiness marker found.",
                first_matching_line(text, PLACEHOLDER_RE),
            )
        if SKIPPED_RE.search(text):
            add_item(
                items,
                "skipped",
                doc["path"],
                "Skipped or out-of-scope surface is explicitly recorded.",
                first_matching_line(text, SKIPPED_RE),
            )
        if FOLLOW_ON_RE.search(text):
            add_item(
                items,
                "follow_on_needed",
                doc["path"],
                "Follow-on or deferred cleanup marker found.",
                first_matching_line(text, FOLLOW_ON_RE),
            )

    if not items:
        add_item(
            items,
            "safe_mechanical_cleanup",
            "review_root",
            "No blockers found; only final human skim remains before review start.",
            "structural scan completed",
        )
    return items


def status_from_counts(counts: Counter[str]) -> str:
    if counts.get("blocker", 0):
        return "blocked"
    if counts.get("skipped", 0) and sum(counts.values()) == counts["skipped"]:
        return "skipped"
    if counts.get("safe_mechanical_cleanup", 0) or counts.get("follow_on_needed", 0):
        return "cleanup_needed"
    return "ready"


def build_report(args: argparse.Namespace) -> dict[str, Any]:
    root = Path(args.review_root)
    if not root.exists() or not root.is_dir():
        items = [
            {
                "category": "skipped",
                "source": "review_root",
                "reason": "Review root is missing or unreadable.",
                "evidence": "missing review root",
            }
        ]
    else:
        items = classify_docs(collect_docs(root))

    counts = Counter(item["category"] for item in items)
    status = status_from_counts(counts)
    return {
        "schema": "adl.review_readiness_cleanup_report.v1",
        "run_id": args.run_id,
        "status": status,
        "summary": f"Review readiness cleanup classified {sum(counts.values())} item(s).",
        "counts": {
            "safe_mechanical_cleanup": counts.get("safe_mechanical_cleanup", 0),
            "blocker": counts.get("blocker", 0),
            "skipped": counts.get("skipped", 0),
            "follow_on_needed": counts.get("follow_on_needed", 0),
        },
        "items": items,
        "recommended_handoffs": {
            "safe_mechanical_cleanup": "documentation-specialist",
            "blocker": "gap-analysis",
            "skipped": "operator-review",
            "follow_on_needed": "finding-to-issue-planner",
        },
        "non_claims": [
            "This report does not approve review readiness.",
            "This report does not remediate findings.",
            "This report does not rewrite finding severity or disagreement.",
            "This report does not publish customer-facing reports.",
        ],
        "safety_flags": {
            "review_approved": False,
            "findings_rewritten": False,
            "published_report": False,
            "created_issues": False,
            "created_prs": False,
            "mutated_repository": False,
        },
    }


def markdown_report(report: dict[str, Any]) -> str:
    lines = [
        "# Review Readiness Cleanup Summary",
        "",
        f"- Run id: `{report['run_id']}`",
        f"- Status: `{report['status']}`",
        f"- Summary: {report['summary']}",
        "",
        "## Classification Counts",
        "",
    ]
    for category, count in report["counts"].items():
        lines.append(f"- {category}: {count}")

    lines.extend(["", "## Items", ""])
    for item in report["items"]:
        lines.append(f"- `{item['category']}` in `{item['source']}`: {item['reason']} Evidence: {item['evidence']}")

    grouped = {
        "Safe Mechanical Cleanup": "safe_mechanical_cleanup",
        "Blockers": "blocker",
        "Skipped Surfaces": "skipped",
        "Follow-On Needed": "follow_on_needed",
    }
    for heading, category in grouped.items():
        lines.extend(["", f"## {heading}", ""])
        matching = [item for item in report["items"] if item["category"] == category]
        if not matching:
            lines.append("- none")
        for item in matching:
            lines.append(f"- `{item['source']}`: {item['reason']}")

    lines.extend(["", "## Non-Claims", ""])
    for item in report["non_claims"]:
        lines.append(f"- {item}")

    lines.extend(["", "## Safety Flags", ""])
    for key, value in report["safety_flags"].items():
        lines.append(f"- {key}: {str(value).lower()}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--review-root", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--run-id", default="review-readiness-cleanup")
    args = parser.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    report = build_report(args)
    (out / "review_readiness_cleanup_report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    (out / "review_readiness_cleanup_report.md").write_text(
        markdown_report(report),
        encoding="utf-8",
    )
    print(f"WROTE {out / 'review_readiness_cleanup_report.json'}")
    print(f"WROTE {out / 'review_readiness_cleanup_report.md'}")
    print(f"STATUS {report['status']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

