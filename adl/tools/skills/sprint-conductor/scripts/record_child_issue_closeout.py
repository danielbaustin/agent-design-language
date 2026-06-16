#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


DETERMINISTIC_CLOSEOUT_ELIGIBLE_STATUSES = {'pending', 'active', 'waiting_for_review'}


def parse_bool(raw: str) -> bool:
    lowered = raw.strip().lower()
    if lowered in {'1', 'true', 'yes', 'on'}:
        return True
    if lowered in {'0', 'false', 'no', 'off'}:
        return False
    raise ValueError(f'invalid boolean: {raw}')


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


def require_truth_gate(state: dict[str, Any], issue_number: int) -> None:
    truth_check = state.get('truth_check') or {}
    if truth_check.get('status') == 'matched' and truth_check.get('gate_passed') is True:
        return
    if truth_check.get('status') == 'drift_detected':
        eligible_drift_issues: set[int] = set()
        blocking_drift: list[str] = []
        for record in state.get('issue_records', []):
            record_issue = record.get('issue_number')
            github_issue_state = record.get('github_issue_state')
            local_status = record.get('status', 'pending')
            if github_issue_state == 'CLOSED' and local_status != 'closed_out':
                if (
                    isinstance(record_issue, int)
                    and local_status in DETERMINISTIC_CLOSEOUT_ELIGIBLE_STATUSES
                ):
                    eligible_drift_issues.add(record_issue)
                else:
                    blocking_drift.append(
                        f'issue #{record_issue} is CLOSED on GitHub but local status is {local_status}'
                    )
            elif github_issue_state == 'OPEN' and local_status == 'closed_out':
                blocking_drift.append(
                    f'issue #{record_issue} is OPEN on GitHub but local status is closed_out'
                )
        closeout_notes = [
            note
            for note in truth_check.get('notes', [])
            if isinstance(note, str)
            and 'record_child_issue_closeout.py must run before sprint state can advance' in note
        ]
        if blocking_drift:
            raise SystemExit(
                'Refusing to record child closeout while non-closeout GitHub truth drift is present: '
                + '; '.join(blocking_drift)
            )
        if issue_number in eligible_drift_issues and len(closeout_notes) == len(eligible_drift_issues):
            return
    raise SystemExit('Refusing to record child closeout without a fresh matched GitHub truth check.')


def require_structured_prompt_preflight(state: dict[str, Any]) -> None:
    preflight = state.get('structured_prompt_preflight') or {}
    if preflight.get('status') == 'ready':
        return
    raise SystemExit('Refusing to record child closeout before sprint-wide structured prompt preflight is ready.')


def consume_truth_gate(state: dict[str, Any]) -> None:
    truth_check = state.setdefault('truth_check', {})
    truth_check['gate_passed'] = False
    notes = [note for note in truth_check.get('notes', []) if isinstance(note, str)]
    reminder = 'Truth gate consumed; rerun live GitHub truth check before the next sprint-state transition.'
    if reminder not in notes:
        notes.append(reminder)
    truth_check['notes'] = notes


def select_next_issue(state: dict[str, Any]) -> None:
    completed = set(state.get('completed_issue_numbers', []))
    if state.get('blocked_issue_number') is not None:
        state['current_issue_number'] = state['blocked_issue_number']
        state['continuation'] = 'ask_operator'
        return
    issue_statuses = {
        record.get('issue_number'): record.get('status', 'pending')
        for record in state.get('issue_records', [])
    }
    for issue in state.get('ordered_issue_numbers', []):
        if issue not in completed:
            state['current_issue_number'] = issue
            if issue_statuses.get(issue) in {'blocked', 'deferred'}:
                state['continuation'] = 'ask_operator'
            else:
                state['continuation'] = 'continue'
            return
    follow_ups = state.get('follow_up_issues', [])
    if any(item.get('disposition') == 'must_land_before_sprint_close' for item in follow_ups):
        state['current_issue_number'] = None
        state['continuation'] = 'ask_operator'
        return
    state['current_issue_number'] = None
    state['continuation'] = 'stop'


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--state', required=True)
    parser.add_argument('--issue-number', type=int, required=True)
    parser.add_argument('--issue-closed', required=True)
    parser.add_argument('--pr-state', required=True, choices=['merged', 'closed_no_merge', 'not_applicable'])
    parser.add_argument('--root-sor-status', required=True, choices=['done', 'failed'])
    parser.add_argument('--worktree-status', required=True, choices=['pruned', 'retained_with_reason', 'not_applicable'])
    parser.add_argument('--worktree-note')
    parser.add_argument('--pr-url')
    parser.add_argument('--artifact-path', action='append', default=[])
    parser.add_argument('--follow-up-issue', action='append', default=[])
    parser.add_argument('--follow-up-summary', action='append', default=[])
    parser.add_argument('--follow-up-disposition', choices=['post_sprint_follow_on', 'must_land_before_sprint_close'])
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    if args.worktree_status == 'retained_with_reason' and not args.worktree_note:
        raise SystemExit('worktree_note is required when worktree_status=retained_with_reason')

    state_path = Path(args.state)
    state = json.loads(state_path.read_text())
    require_structured_prompt_preflight(state)
    require_truth_gate(state, args.issue_number)

    if not parse_bool(args.issue_closed):
        raise SystemExit('child closeout gate failed: issue_closed must be true before advancement')

    record = ensure_issue_record(state, args.issue_number)
    record['status'] = 'closed_out'
    if args.pr_url:
        record['pr_url'] = args.pr_url
    if args.artifact_path:
        existing = set(record.get('artifact_paths', []))
        existing.update(args.artifact_path)
        record['artifact_paths'] = sorted(existing)
    record['closeout_gate'] = {
        'issue_closed': True,
        'pr_state': args.pr_state,
        'root_sor_status': args.root_sor_status,
        'worktree_status': args.worktree_status,
        'worktree_note': args.worktree_note or None,
    }

    completed = set(state.setdefault('completed_issue_numbers', []))
    completed.add(args.issue_number)
    state['completed_issue_numbers'] = sorted(completed)

    policy = state.get('follow_up_issue_policy') or state.get('policy', {}).get('follow_up_issue_policy') or 'post_sprint_follow_on'
    state['follow_up_issue_policy'] = policy
    follow_up_issues = state.setdefault('follow_up_issues', [])
    disposition = args.follow_up_disposition or policy
    for idx, raw_issue in enumerate(args.follow_up_issue):
        summary = args.follow_up_summary[idx] if idx < len(args.follow_up_summary) else f'Follow-up discovered while closing issue #{args.issue_number}.'
        follow_up_issues.append({
            'issue_number': int(raw_issue),
            'disposition': disposition,
            'summary': summary,
        })

    select_next_issue(state)
    consume_truth_gate(state)
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    if args.print_json:
        print(json.dumps(state, indent=2, sort_keys=True))
    else:
        print(state_path)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
