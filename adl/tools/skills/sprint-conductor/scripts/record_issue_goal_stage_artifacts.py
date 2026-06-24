#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shutil
from pathlib import Path

from issue_goal_metrics import (
    CAPTURE_STAGES,
    METRICS_CONFIDENCE_VALUES,
    build_issue_goal_metrics_record_from_codex_goal_snapshot,
    summarize_issue_goal_metrics,
    validate_terminal_completion_allowed,
)


STAGE_SNAPSHOT_SUFFIX = {
    "issue_init": "-issue-init",
    "doctor_readiness": "-doctor-readiness",
    "card_repair": "-card-repair",
    "execution_ready": "-execution-ready",
    "issue_start": "",
    "pr_publication": "-pr-publication",
    "review_handoff": "-review-handoff",
    "merge_closeout": "-merge-closeout",
    "sprint_closeout": "-sprint-closeout",
}


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


def write_jsonl(path: Path, rows: list[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")


def snapshot_name(issue_number: int, capture_stage: str) -> str:
    return f"issue-{issue_number}-goal-state{STAGE_SNAPSHOT_SUFFIX[capture_stage]}.json"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--goal-state", required=True, help="Path to a saved get_goal JSON payload.")
    parser.add_argument("--issue-number", type=int, required=True)
    parser.add_argument("--artifacts-dir", required=True)
    parser.add_argument("--capture-stage", required=True, choices=sorted(CAPTURE_STAGES))
    parser.add_argument("--issue-goal-ref")
    parser.add_argument("--sprint-goal-ref")
    parser.add_argument("--goal-metrics-rollup-ref")
    parser.add_argument(
        "--metrics-confidence",
        default="high",
        choices=sorted(METRICS_CONFIDENCE_VALUES),
    )
    parser.add_argument("--model-ref")
    parser.add_argument("--session-ref")
    parser.add_argument("--completion-state")
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

    artifacts_dir = Path(args.artifacts_dir)
    artifacts_dir.mkdir(parents=True, exist_ok=True)

    snapshot_path = artifacts_dir / snapshot_name(args.issue_number, args.capture_stage)
    sink_path = artifacts_dir / f"issue-{args.issue_number}-goal-metrics.jsonl"
    summary_path = artifacts_dir / f"issue-{args.issue_number}-goal-metrics-summary.json"

    shutil.copyfile(args.goal_state, snapshot_path)

    record = build_issue_goal_metrics_record_from_codex_goal_snapshot(
        issue_number=args.issue_number,
        capture_stage=args.capture_stage,
        goal_state_path=str(snapshot_path),
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

    rows = [
        row
        for row in read_jsonl(sink_path)
        if not (
            row.get("issue_number") == args.issue_number
            and row.get("capture_stage") == args.capture_stage
        )
    ]
    rows.append(record)
    rows.sort(key=lambda row: ((row.get("issue_number") or 0), row.get("recorded_at") or "", row.get("capture_stage") or ""))
    write_jsonl(sink_path, rows)

    issue_rows = [row for row in rows if row.get("issue_number") == args.issue_number]
    summary = summarize_issue_goal_metrics(issue_rows, str(sink_path))
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")

    result = {
        "recorded": record,
        "summary": summary,
        "snapshot_path": str(snapshot_path),
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
