#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path


def require_child_closeout_truth(state: dict) -> None:
    ordered_issues = state.get('ordered_issue_numbers', [])
    records = {record.get('issue_number'): record for record in state.get('issue_records', [])}
    stale = [
        issue for issue in ordered_issues
        if records.get(issue, {}).get('status') != 'closed_out'
    ]
    if stale:
        formatted = ', '.join(f'#{issue}' for issue in stale)
        raise SystemExit(
            'Cannot close sprint-management issue because child closeout truth is incomplete for '
            f'{formatted}.'
        )


def require_clean_close_boundary(state: dict) -> None:
    if state.get('blocked_issue_number') is not None:
        raise SystemExit('Cannot close sprint-management issue because a blocked child issue is still recorded in sprint state.')
    if state.get('deferred_issue_numbers'):
        raise SystemExit('Cannot close sprint-management issue because deferred child issues are still recorded in sprint state.')
    follow_ups = state.get('follow_up_issues', [])
    must_land = [
        item.get('issue_number')
        for item in follow_ups
        if item.get('disposition') == 'must_land_before_sprint_close'
    ]
    if must_land:
        formatted = ', '.join(f'#{issue}' for issue in must_land if issue is not None)
        raise SystemExit(
            'Cannot close sprint-management issue because must-land-before-close follow-up issues remain: '
            f'{formatted}.'
        )


def require_closeout_artifact(state: dict) -> None:
    closeout = state.get('closeout') or {}
    closeout_artifact_path = closeout.get('closeout_artifact_path')
    if not closeout_artifact_path:
        raise SystemExit('Cannot close sprint-management issue because no retained sprint closeout artifact is recorded in state.')
    artifact_path = Path(closeout_artifact_path)
    if not artifact_path.exists():
        raise SystemExit(
            'Cannot close sprint-management issue because the retained sprint closeout artifact is missing: '
            f'{closeout_artifact_path}.'
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--state', required=True)
    parser.add_argument('--summary', required=True)
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    state_path = Path(args.state)
    state = json.loads(state_path.read_text())
    sprint_issue_number = state.get('sprint_issue_number')
    if sprint_issue_number is None:
        raise SystemExit('Cannot close sprint-management issue because sprint_issue_number is missing from state.')
    require_child_closeout_truth(state)
    require_clean_close_boundary(state)
    require_closeout_artifact(state)

    subprocess.check_call(
        [
            'gh',
            'issue',
            'close',
            str(sprint_issue_number),
            '--comment',
            args.summary,
        ]
    )

    state['sprint_issue_closed'] = True
    state.setdefault('closeout', {})
    state['closeout']['sprint_issue_close_summary'] = args.summary
    state['sprint_issue_close_summary'] = args.summary
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    result = {
        'closed': True,
        'sprint_issue_number': sprint_issue_number,
        'state_path': str(state_path),
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(sprint_issue_number)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
