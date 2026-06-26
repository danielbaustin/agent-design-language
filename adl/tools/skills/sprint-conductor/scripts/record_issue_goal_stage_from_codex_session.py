#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from tempfile import NamedTemporaryFile

from issue_goal_metrics import (
    build_issue_goal_metrics_record,
    build_issue_goal_metrics_record_from_codex_goal_snapshot,
    build_unknown_issue_goal_metrics_record,
    find_codex_session_transcript,
    iso_now_utc,
    load_codex_goal_payload_from_session_transcript,
    summarize_issue_goal_metrics,
    validate_terminal_completion_allowed,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Capture canonical issue goal-metrics artifacts from the host Codex session transcript."
    )
    parser.add_argument("--issue-number", type=int, required=True)
    parser.add_argument("--artifacts-dir", required=True)
    parser.add_argument("--capture-stage", required=True)
    parser.add_argument("--issue-goal-ref")
    parser.add_argument("--sprint-goal-ref")
    parser.add_argument("--goal-metrics-rollup-ref")
    parser.add_argument("--metrics-confidence", default="unknown")
    parser.add_argument("--completion-state-override")
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
    parser.add_argument("--thread-id", default=os.environ.get("CODEX_THREAD_ID"))
    parser.add_argument("--session-root", default=os.path.expanduser("~/.codex/sessions"))
    parser.add_argument("--transcript-path")
    parser.add_argument("--fallback-data-source", choices=["unknown", "derived_sprint_state"])
    return parser.parse_args()


def snapshot_filename(issue_number: int, capture_stage: str) -> str:
    if capture_stage == "issue_start":
        return f"issue-{issue_number}-goal-state.json"
    return f"issue-{issue_number}-goal-state-{capture_stage.replace('_', '-')}.json"


def load_existing_records(path: Path) -> list[dict]:
    if not path.exists():
        return []
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def main() -> int:
    args = parse_args()
    artifacts_dir = Path(args.artifacts_dir)
    artifacts_dir.mkdir(parents=True, exist_ok=True)

    use_derived_issue_start = (
        args.capture_stage == "issue_start"
        and args.fallback_data_source == "derived_sprint_state"
    )

    transcript_path = None
    goal_payload = None
    if not use_derived_issue_start:
        transcript_path = args.transcript_path
        if transcript_path is None:
            transcript_path = find_codex_session_transcript(
                args.thread_id,
                args.session_root,
                issue_number=args.issue_number,
            )

        if transcript_path is not None:
            goal_payload = load_codex_goal_payload_from_session_transcript(
                transcript_path,
                thread_id=args.thread_id,
                issue_number=args.issue_number,
            )

    raw_log_path = transcript_path or args.session_root
    if goal_payload is not None:
        snapshot_path = artifacts_dir / snapshot_filename(args.issue_number, args.capture_stage)
        snapshot_path.write_text(json.dumps(goal_payload, indent=2) + "\n")
        with NamedTemporaryFile("w", encoding="utf-8", delete=False) as handle:
            json.dump(goal_payload, handle)
            handle.write("\n")
            temp_snapshot_path = handle.name
        try:
            record = build_issue_goal_metrics_record_from_codex_goal_snapshot(
                issue_number=args.issue_number,
                capture_stage=args.capture_stage,
                goal_state_path=temp_snapshot_path,
                raw_log_path=raw_log_path,
                issue_goal_ref=args.issue_goal_ref,
                sprint_goal_ref=args.sprint_goal_ref,
                goal_metrics_rollup_ref=args.goal_metrics_rollup_ref,
                metrics_confidence=args.metrics_confidence,
                completion_state_override=args.completion_state_override,
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
        finally:
            os.unlink(temp_snapshot_path)
    else:
        if args.fallback_data_source == "derived_sprint_state":
            recorded_at = iso_now_utc()
            session_ref = args.session_ref
            if session_ref is None and args.thread_id:
                session_ref = f"codex-thread:{args.thread_id}"
            record = build_issue_goal_metrics_record(
                sprint_issue_number=None,
                issue_number=args.issue_number,
                capture_stage=args.capture_stage,
                data_source="derived_sprint_state",
                raw_log_path=raw_log_path,
                recorded_at=recorded_at,
                issue_goal_ref=args.issue_goal_ref,
                sprint_goal_ref=args.sprint_goal_ref,
                goal_metrics_rollup_ref=args.goal_metrics_rollup_ref,
                goal_id=None,
                goal_id_state="not_available",
                started_at=recorded_at if args.capture_stage == "issue_start" else None,
                completed_at=None,
                elapsed_seconds_raw="unknown",
                active_work_seconds_raw="unknown",
                validation_seconds_raw="unknown",
                pr_wait_seconds_raw="unknown",
                ci_wait_seconds_raw="unknown",
                total_tokens_raw="unknown",
                prompt_tokens_raw="unknown",
                completion_tokens_raw="unknown",
                metrics_confidence=args.metrics_confidence,
                completion_state=args.completion_state_override or "unknown",
                model_ref=args.model_ref,
                session_ref=session_ref,
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
        else:
            record = build_unknown_issue_goal_metrics_record(
                issue_number=args.issue_number,
                capture_stage=args.capture_stage,
                raw_log_path=raw_log_path,
                issue_goal_ref=args.issue_goal_ref,
                sprint_goal_ref=args.sprint_goal_ref,
                goal_metrics_rollup_ref=args.goal_metrics_rollup_ref,
                metrics_confidence=args.metrics_confidence,
                completion_state=args.completion_state_override or "unknown",
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

    jsonl_path = artifacts_dir / f"issue-{args.issue_number}-goal-metrics.jsonl"
    records = [
        existing
        for existing in load_existing_records(jsonl_path)
        if existing.get("capture_stage") != args.capture_stage
    ]
    records.append(record)
    records.sort(key=lambda existing: (existing.get("capture_stage") or "", existing.get("recorded_at") or ""))
    jsonl_path.write_text("".join(json.dumps(existing, sort_keys=True) + "\n" for existing in records))

    summary = summarize_issue_goal_metrics(records, raw_log_path)
    summary_path = artifacts_dir / f"issue-{args.issue_number}-goal-metrics-summary.json"
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")

    print(
        json.dumps(
            {
                "status": "recorded",
                "issue_number": args.issue_number,
                "capture_stage": args.capture_stage,
                "transcript_path": transcript_path,
                "summary_path": str(summary_path),
                "data_source": record["data_source"],
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
