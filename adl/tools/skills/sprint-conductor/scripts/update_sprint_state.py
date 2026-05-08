#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def parse_csv_ints(raw: str) -> list[int]:
    values = []
    for part in raw.split(','):
        part = part.strip()
        if not part:
            continue
        values.append(int(part))
    return values


def default_issue_records(ordered: list[int]) -> list[dict[str, Any]]:
    return [
        {
            'issue_number': issue,
            'status': 'pending',
            'pr_url': None,
            'artifact_paths': [],
        }
        for issue in ordered
    ]


def load_state(path: Path, sprint_issue: int, ordered: list[int]) -> dict[str, Any]:
    if path.exists():
        return json.loads(path.read_text())
    return {
        'sprint_issue_number': sprint_issue,
        'ordered_issue_numbers': ordered,
        'current_issue_number': ordered[0] if ordered else None,
        'completed_issue_numbers': [],
        'blocked_issue_number': None,
        'continuation': 'continue',
        'issue_records': default_issue_records(ordered),
    }


def ensure_issue_record(state: dict[str, Any], issue_number: int) -> dict[str, Any]:
    for record in state.setdefault('issue_records', []):
        if record.get('issue_number') == issue_number:
            record.setdefault('artifact_paths', [])
            record.setdefault('pr_url', None)
            return record
    record = {
        'issue_number': issue_number,
        'status': 'pending',
        'pr_url': None,
        'artifact_paths': [],
    }
    state['issue_records'].append(record)
    return record


def select_next_issue(state: dict[str, Any]) -> None:
    completed = set(state.get('completed_issue_numbers', []))
    blocked = state.get('blocked_issue_number')
    if blocked is not None:
        state['current_issue_number'] = blocked
        state['continuation'] = 'ask_operator'
        return
    record_by_issue = {
        record.get('issue_number'): record for record in state.get('issue_records', [])
    }
    for issue in state.get('ordered_issue_numbers', []):
        if issue not in completed:
            record = record_by_issue.get(issue, {})
            if record.get('status') == 'waiting_for_review':
                state['current_issue_number'] = issue
                state['continuation'] = 'waiting_for_review'
                return
            if record.get('status') in {'blocked', 'deferred'}:
                state['current_issue_number'] = issue
                state['continuation'] = 'ask_operator'
                return
            state['current_issue_number'] = issue
            state['continuation'] = 'continue'
            return
    state['current_issue_number'] = None
    state['continuation'] = 'stop'


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--state', required=True)
    parser.add_argument('--sprint-issue', type=int, required=True)
    parser.add_argument('--ordered-issues', required=True)
    parser.add_argument('--current-issue', type=int)
    parser.add_argument('--mark-status', choices=['pending', 'active', 'waiting_for_review', 'closed_out', 'blocked', 'deferred'])
    parser.add_argument('--pr-url')
    parser.add_argument('--artifact-path', action='append', default=[])
    parser.add_argument('--blocked-issue', type=int)
    parser.add_argument('--clear-blocked', action='store_true')
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    state_path = Path(args.state)
    ordered = parse_csv_ints(args.ordered_issues)
    state = load_state(state_path, args.sprint_issue, ordered)
    state['sprint_issue_number'] = args.sprint_issue
    state['ordered_issue_numbers'] = ordered

    issue_number = args.current_issue or state.get('current_issue_number') or (ordered[0] if ordered else None)
    if issue_number is not None:
        record = ensure_issue_record(state, issue_number)
        if args.mark_status:
            record['status'] = args.mark_status
            if args.mark_status == 'closed_out':
                completed = set(state.setdefault('completed_issue_numbers', []))
                completed.add(issue_number)
                state['completed_issue_numbers'] = sorted(completed)
            elif args.mark_status == 'blocked':
                state['blocked_issue_number'] = issue_number
            elif args.mark_status == 'waiting_for_review':
                state['continuation'] = 'waiting_for_review'
        if args.pr_url:
            record['pr_url'] = args.pr_url
        if args.artifact_path:
            existing = set(record.get('artifact_paths', []))
            for path in args.artifact_path:
                existing.add(path)
            record['artifact_paths'] = sorted(existing)

    if args.blocked_issue is not None:
        state['blocked_issue_number'] = args.blocked_issue
        state['continuation'] = 'ask_operator'
    if args.clear_blocked:
        state['blocked_issue_number'] = None

    select_next_issue(state)
    state_path.parent.mkdir(parents=True, exist_ok=True)
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')
    if args.print_json:
        print(json.dumps(state, indent=2, sort_keys=True))
    else:
        print(state_path)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
