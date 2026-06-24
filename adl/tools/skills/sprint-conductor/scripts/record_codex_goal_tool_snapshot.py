#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

from issue_goal_metrics import (
    CAPTURE_STAGES,
    COMPLETION_STATE_VALUES,
    METRICS_CONFIDENCE_VALUES,
    build_issue_goal_metrics_record_from_codex_goal_snapshot,
    validate_terminal_completion_allowed,
)

# Keep this script standalone and avoid importing sprint-state machinery. It
# writes a durable per-issue goal-metrics sink plus a summary artifact that
# ordinary issue workflows can reference after live goal state disappears.
from issue_goal_metrics import summarize_issue_goal_metrics


def read_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        return []
    rows = []
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        rows.append(json.loads(stripped))
    return rows


def append_jsonl(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, sort_keys=True) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--goal-state", required=True)
    parser.add_argument("--issue-number", type=int, required=True)
    parser.add_argument("--sink", required=True)
    parser.add_argument("--summary-out", required=True)
    parser.add_argument("--capture-stage", required=True, choices=sorted(CAPTURE_STAGES))
    parser.add_argument("--issue-goal-ref")
    parser.add_argument("--sprint-goal-ref")
    parser.add_argument("--goal-metrics-rollup-ref")
    parser.add_argument(
        "--metrics-confidence",
        default="high",
        choices=sorted(METRICS_CONFIDENCE_VALUES),
    )
    parser.add_argument(
        "--completion-state",
        choices=sorted(COMPLETION_STATE_VALUES),
        help="Optional override when the saved goal snapshot should be treated as terminal issue truth.",
    )
    parser.add_argument("--model-ref")
    parser.add_argument("--session-ref")
    parser.add_argument("--goal-kind")
    parser.add_argument("--goal-boundary")
    parser.add_argument("--issue-state")
    parser.add_argument("--pr-state")
    parser.add_argument("--checks-state")
    parser.add_argument("--review-truth")
    parser.add_argument("--closeout-truth")
    parser.add_argument("--watch-target-status")
    parser.add_argument("--sprint-rollup-status")
    parser.add_argument("--merge-conflicts", action="store_true")
    parser.add_argument("--no-merge-conflicts", action="store_true")
    parser.add_argument("--print-json", action="store_true")
    args = parser.parse_args()

    sink_path = Path(args.sink)
    summary_path = Path(args.summary_out)

    record = build_issue_goal_metrics_record_from_codex_goal_snapshot(
        issue_number=args.issue_number,
        capture_stage=args.capture_stage,
        goal_state_path=args.goal_state,
        raw_log_path=str(sink_path),
        issue_goal_ref=args.issue_goal_ref,
        sprint_goal_ref=args.sprint_goal_ref,
        goal_metrics_rollup_ref=args.goal_metrics_rollup_ref,
        metrics_confidence=args.metrics_confidence,
        completion_state_override=args.completion_state,
        model_ref=args.model_ref,
        session_ref=args.session_ref,
        goal_kind=args.goal_kind,
        goal_boundary=args.goal_boundary,
        issue_state=args.issue_state,
        pr_state=args.pr_state,
        checks_state=args.checks_state,
        review_truth=args.review_truth,
        closeout_truth=args.closeout_truth,
        merge_conflicts=True if args.merge_conflicts else False if args.no_merge_conflicts else None,
        watch_target_status=args.watch_target_status,
        sprint_rollup_status=args.sprint_rollup_status,
    )
    validate_terminal_completion_allowed(record)
    append_jsonl(sink_path, record)

    issue_rows = [row for row in read_jsonl(sink_path) if row.get("issue_number") == args.issue_number]
    summary = summarize_issue_goal_metrics(issue_rows, str(sink_path))
    summary_path.parent.mkdir(parents=True, exist_ok=True)
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")

    result = {
        "recorded": record,
        "summary": summary,
        "sink_path": str(sink_path),
        "summary_path": str(summary_path),
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(str(summary_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
