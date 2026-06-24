#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent / "skills" / "sprint-conductor" / "scripts"
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from issue_goal_metrics import evaluate_goal_terminal_state


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Evaluate whether an issue-bound goal may truthfully be marked complete."
    )
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
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    result = evaluate_goal_terminal_state(
        goal_kind=args.goal_kind,
        declared_boundary=args.goal_boundary,
        issue_state=args.issue_state,
        pr_state=args.pr_state,
        checks_state=args.checks_state,
        review_truth=args.review_truth,
        closeout_truth=args.closeout_truth,
        merge_conflicts=True if args.merge_conflicts else False if args.no_merge_conflicts else None,
        watch_target_status=args.watch_target_status,
        sprint_rollup_status=args.sprint_rollup_status,
    )
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if result.get("completion_allowed") is True else 1


if __name__ == "__main__":
    raise SystemExit(main())
