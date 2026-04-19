#!/usr/bin/env python3
import argparse
import json
from collections import defaultdict
from pathlib import Path


CATEGORY_ORDER = [
    "actionable_now",
    "already_fixed",
    "stale_or_not_reproducible",
    "follow_on_issue_needed",
    "blocked_or_operator_decision",
]


def classify(comment):
    text = (comment.get("expected_category") or "").strip()
    if text:
        if text not in CATEGORY_ORDER:
            text = "blocked_or_operator_decision"
        return text

    state = (comment.get("state") or "").lower()
    if comment.get("already_fixed", False) or state in {"resolved", "fixed", "closed"}:
        return "already_fixed"
    if comment.get("stale", False) or comment.get("no_longer_reproducible", False):
        return "stale_or_not_reproducible"
    if comment.get("follow_on", False) or comment.get("follow_on_issue", False):
        return "follow_on_issue_needed"
    if comment.get("requires_operator_decision", False) or comment.get("blocked", False):
        return "blocked_or_operator_decision"
    if state in {"open", "requested_changes", "request_changes", "changes_requested", "new"}:
        return "actionable_now"
    return "blocked_or_operator_decision"


def main():
    parser = argparse.ArgumentParser(description="Triage review-comment fixtures.")
    parser.add_argument("payload_path", help="Path to JSON payload containing comments.")
    parser.add_argument("--out", dest="out", help="Optional output path for triage JSON.")
    args = parser.parse_args()

    payload_path = Path(args.payload_path)
    with payload_path.open(encoding="utf-8") as fh:
        payload = json.load(fh)

    grouped = {category: [] for category in CATEGORY_ORDER}
    mismatches = []

    for comment in payload.get("comments", []):
        comment_id = comment.get("id")
        source_category = comment.get("expected_category")
        assigned = classify(comment)

        entry = {
            "id": comment_id,
            "file": comment.get("file"),
            "line": comment.get("line"),
            "author": comment.get("author"),
            "link": comment.get("review_url"),
            "summary": comment.get("summary") or comment.get("body"),
        }
        grouped[assigned].append(entry)

        if source_category and source_category != assigned:
            mismatches.append(
                {
                    "id": comment_id,
                    "expected": source_category,
                    "actual": assigned,
                }
            )

    output = {
        "status": "pass",
        "source": payload.get("source", payload_path.as_posix()),
        "counts": {category: len(items) for category, items in grouped.items()},
        "triage": grouped,
        "execution_order": CATEGORY_ORDER,
        "mismatches": mismatches,
    }

    if mismatches:
        output["status"] = "partial"

    rendered = json.dumps(output, indent=2, sort_keys=True) + "\n"
    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(rendered, encoding="utf-8")
    else:
        print(rendered, end="")


if __name__ == "__main__":
    main()
