#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from issue_goal_metrics import default_goal_metrics_summary


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
            'goal_metrics': default_goal_metrics_summary(),
        }
        for issue in ordered
    ]


def default_goal_policy() -> dict[str, Any]:
    return {
        'status': 'descriptive_only',
        'sprint_goal_role': 'descriptive_sprint_objective',
        'active_session_goal_required': 'child_issue_only',
        'notes': [
            'Sprint state may record a descriptive sprint objective, but that objective does not satisfy the active Codex session-goal requirement for child issue execution.',
            'Each child issue session must create its own issue-bound goal after bind/readiness succeeds and before implementation starts.',
            'Do not keep a competing sprint-global active session goal in the same thread while a child issue implementation session is active.',
        ],
    }


def load_state(path: Path, sprint_issue: int, ordered: list[int]) -> dict[str, Any]:
    if path.exists():
        state = json.loads(path.read_text())
        state.setdefault('goal_policy', default_goal_policy())
        return state
    return {
        'sprint_issue_number': sprint_issue,
        'ordered_issue_numbers': ordered,
        'current_issue_number': ordered[0] if ordered else None,
        'completed_issue_numbers': [],
        'blocked_issue_number': None,
        'continuation': 'continue',
        'goal_policy': default_goal_policy(),
        'issue_records': default_issue_records(ordered),
        'structured_prompt_preflight': {
            'status': 'not_run',
            'required_card_types': ['stp.md', 'sip.md', 'sor.md', 'spp.md', 'srp.md'],
            'issue_results': [],
            'notes': [
                'Run sprint-wide structured prompt preflight before starting issue execution, including SPP and SRP design-time readiness.',
            ],
        },
        'truth_check': {
            'status': 'not_run',
            'source': 'sprint_state_only',
            'gate_passed': False,
            'checked_issue_numbers': [],
            'checked_pr_urls': [],
            'notes': ['Run live GitHub truth check before the first sprint-state transition.'],
        },
    }


def ensure_issue_record(state: dict[str, Any], issue_number: int) -> dict[str, Any]:
    for record in state.setdefault('issue_records', []):
        if record.get('issue_number') == issue_number:
            record.setdefault('artifact_paths', [])
            record.setdefault('pr_url', None)
            record.setdefault('goal_metrics', default_goal_metrics_summary())
            return record
    record = {
        'issue_number': issue_number,
        'status': 'pending',
        'pr_url': None,
        'artifact_paths': [],
        'goal_metrics': default_goal_metrics_summary(),
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


def mutation_requested(args: argparse.Namespace) -> bool:
    return any(
        [
            args.current_issue is not None,
            args.mark_status is not None,
            args.pr_url is not None,
            bool(args.artifact_path),
            args.blocked_issue is not None,
            args.clear_blocked,
        ]
    )


def require_truth_gate(state: dict[str, Any]) -> None:
    truth_check = state.get('truth_check') or {}
    if truth_check.get('status') == 'matched' and truth_check.get('gate_passed') is True:
        return
    raise SystemExit(
        'Refusing to advance sprint state without a fresh matched GitHub truth check. '
        'Run check_sprint_truth.py --require-match before calling update_sprint_state.py.'
    )


def require_structured_prompt_preflight(state: dict[str, Any]) -> None:
    preflight = state.get('structured_prompt_preflight') or {}
    if preflight.get('status') == 'ready':
        return
    raise SystemExit(
        'Refusing to advance sprint state before sprint-wide structured prompt review and repair is complete. '
        'Run check_sprint_structured_prompt_readiness.py and fix any flagged child issue cards first.'
    )


def consume_truth_gate(state: dict[str, Any]) -> None:
    truth_check = state.setdefault('truth_check', {})
    truth_check['gate_passed'] = False
    notes = [note for note in truth_check.get('notes', []) if isinstance(note, str)]
    reminder = 'Truth gate consumed; rerun live GitHub truth check before the next sprint-state transition.'
    if reminder not in notes:
        notes.append(reminder)
    truth_check['notes'] = notes


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
    state_preexisted = state_path.exists()
    if not state_preexisted and mutation_requested(args):
        raise SystemExit(
            'Refusing to create and mutate sprint state in one step. '
            'Create the state artifact first, then run structured prompt preflight and a matched live GitHub truth check before the first sprint-state transition.'
        )
    ordered = parse_csv_ints(args.ordered_issues)
    state = load_state(state_path, args.sprint_issue, ordered)
    state['sprint_issue_number'] = args.sprint_issue
    state['ordered_issue_numbers'] = ordered
    if mutation_requested(args):
        require_structured_prompt_preflight(state)
        require_truth_gate(state)

    issue_number = args.current_issue or state.get('current_issue_number') or (ordered[0] if ordered else None)
    if issue_number is not None:
        record = ensure_issue_record(state, issue_number)
        if args.mark_status:
            record['status'] = args.mark_status
            if args.mark_status == 'closed_out':
                completed = set(state.setdefault('completed_issue_numbers', []))
                completed.add(issue_number)
                state['completed_issue_numbers'] = sorted(completed)
                if state.get('blocked_issue_number') == issue_number:
                    state['blocked_issue_number'] = None
            else:
                completed = set(state.setdefault('completed_issue_numbers', []))
                if issue_number in completed:
                    completed.remove(issue_number)
                    state['completed_issue_numbers'] = sorted(completed)
            if args.mark_status == 'blocked':
                state['blocked_issue_number'] = issue_number
            else:
                if state.get('blocked_issue_number') == issue_number:
                    state['blocked_issue_number'] = None
            if args.mark_status == 'waiting_for_review':
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
    if mutation_requested(args):
        consume_truth_gate(state)
    state_path.parent.mkdir(parents=True, exist_ok=True)
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')
    if args.print_json:
        print(json.dumps(state, indent=2, sort_keys=True))
    else:
        print(state_path)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
