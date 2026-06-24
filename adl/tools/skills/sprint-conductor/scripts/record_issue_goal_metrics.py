#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from issue_goal_metrics import (
    CAPTURE_STAGES,
    DATA_SOURCE_VALUES,
    build_issue_goal_metrics_record,
    compute_goal_metrics_rollup,
    default_goal_metrics_summary,
    summarize_issue_goal_metrics,
    validate_terminal_completion_allowed,
)


def load_state(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def write_state(path: Path, payload: dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")


def ensure_issue_record(state: dict[str, Any], issue_number: int) -> dict[str, Any]:
    for record in state.setdefault("issue_records", []):
        if record.get("issue_number") == issue_number:
            record.setdefault("artifact_paths", [])
            record.setdefault("pr_url", None)
            record.setdefault("goal_metrics", default_goal_metrics_summary())
            return record
    raise ValueError(f"issue #{issue_number} is not present in sprint issue_records")


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        rows.append(json.loads(stripped))
    return rows


def append_jsonl(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, sort_keys=True) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--state", required=True)
    parser.add_argument("--issue-number", type=int, required=True)
    parser.add_argument("--sink", required=True)
    parser.add_argument("--capture-stage", required=True, choices=sorted(CAPTURE_STAGES))
    parser.add_argument("--data-source", required=True, choices=sorted(DATA_SOURCE_VALUES))
    parser.add_argument("--recorded-at")
    parser.add_argument("--issue-goal-ref")
    parser.add_argument("--sprint-goal-ref")
    parser.add_argument("--goal-metrics-rollup-ref")
    parser.add_argument("--goal-id")
    parser.add_argument("--goal-id-state", choices=["known", "unknown", "not_collected", "not_applicable", "not_available"])
    parser.add_argument("--started-at")
    parser.add_argument("--completed-at")
    parser.add_argument("--elapsed-seconds")
    parser.add_argument("--active-work-seconds")
    parser.add_argument("--validation-seconds")
    parser.add_argument("--pr-wait-seconds")
    parser.add_argument("--ci-wait-seconds")
    parser.add_argument("--total-tokens")
    parser.add_argument("--prompt-tokens")
    parser.add_argument("--completion-tokens")
    parser.add_argument("--metrics-confidence", default="unknown", choices=["low", "medium", "high", "unknown"])
    parser.add_argument(
        "--completion-state",
        default="unknown",
        choices=[
            "completed",
            "completed_with_follow_on",
            "blocked",
            "failed",
            "deferred",
            "cancelled",
            "unknown",
        ],
    )
    parser.add_argument("--model-ref")
    parser.add_argument("--session-ref")
    parser.add_argument("--thread-id")
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

    state_path = Path(args.state)
    state = load_state(state_path)
    sink_path = Path(args.sink)
    ordered_issue_numbers = set(state.get("ordered_issue_numbers", []))
    if args.issue_number not in ordered_issue_numbers:
        raise ValueError(f"issue #{args.issue_number} is not present in ordered_issue_numbers")

    record = build_issue_goal_metrics_record(
        sprint_issue_number=state.get("sprint_issue_number"),
        issue_number=args.issue_number,
        capture_stage=args.capture_stage,
        data_source=args.data_source,
        raw_log_path=str(sink_path),
        recorded_at=args.recorded_at,
        issue_goal_ref=args.issue_goal_ref,
        sprint_goal_ref=args.sprint_goal_ref,
        goal_metrics_rollup_ref=args.goal_metrics_rollup_ref,
        goal_id=args.goal_id,
        goal_id_state=args.goal_id_state,
        started_at=args.started_at,
        completed_at=args.completed_at,
        elapsed_seconds_raw=args.elapsed_seconds,
        active_work_seconds_raw=args.active_work_seconds,
        validation_seconds_raw=args.validation_seconds,
        pr_wait_seconds_raw=args.pr_wait_seconds,
        ci_wait_seconds_raw=args.ci_wait_seconds,
        total_tokens_raw=args.total_tokens,
        prompt_tokens_raw=args.prompt_tokens,
        completion_tokens_raw=args.completion_tokens,
        metrics_confidence=args.metrics_confidence,
        completion_state=args.completion_state,
        model_ref=args.model_ref,
        session_ref=args.session_ref,
        thread_id=args.thread_id,
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

    all_rows = read_jsonl(sink_path)
    issue_rows = [row for row in all_rows if row.get("issue_number") == args.issue_number]
    issue_record = ensure_issue_record(state, args.issue_number)
    issue_record["goal_metrics"] = summarize_issue_goal_metrics(issue_rows, str(sink_path))

    closeout = state.setdefault("closeout", {})
    closeout["goal_metrics_rollup"] = compute_goal_metrics_rollup(state.get("issue_records", []))

    write_state(state_path, state)

    result = {
        "recorded": record,
        "issue_goal_metrics_summary": issue_record["goal_metrics"],
        "goal_metrics_rollup": closeout["goal_metrics_rollup"],
        "state_path": str(state_path),
        "sink_path": str(sink_path),
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(str(sink_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
