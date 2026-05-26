#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


def run_json(cmd: list[str]) -> Any:
    out = subprocess.check_output(cmd, text=True)
    return json.loads(out)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--state', required=True)
    parser.add_argument('--print-json', action='store_true')
    parser.add_argument('--require-match', action='store_true')
    args = parser.parse_args()

    state_path = Path(args.state)
    state = json.loads(state_path.read_text())
    issue_records = state.get('issue_records', [])
    issue_numbers = [record.get('issue_number') for record in issue_records if record.get('issue_number') is not None]
    pr_urls = [record.get('pr_url') for record in issue_records if record.get('pr_url')]

    notes: list[str] = []
    drift = False

    for issue_number in issue_numbers:
        issue = run_json([
            'gh', 'issue', 'view', str(issue_number), '--json', 'number,state,title,url'
        ])
        record = next((r for r in issue_records if r.get('issue_number') == issue_number), None)
        if record is None:
            continue
        record['github_issue_state'] = issue.get('state')
        if issue.get('state') == 'CLOSED' and record.get('status') not in {'closed_out'}:
            local_status = record.get('status')
            drift = True
            notes.append(
                f'issue #{issue_number} is CLOSED on GitHub but local status is {local_status}; '
                'record_child_issue_closeout.py must run before sprint state can advance'
            )
        if issue.get('state') == 'OPEN' and record.get('status') == 'closed_out':
            drift = True
            notes.append(f'issue #{issue_number} is OPEN on GitHub but local status is closed_out')

    for pr_url in pr_urls:
        pr = run_json(['gh', 'pr', 'view', pr_url, '--json', 'state,isDraft,url'])
        matching = next((r for r in issue_records if r.get('pr_url') == pr_url), None)
        if matching is None:
            continue
        matching['github_pr_state'] = pr.get('state')
        matching['github_pr_is_draft'] = pr.get('isDraft')

    truth_check = {
        'status': 'drift_detected' if drift else 'matched',
        'source': 'github_live',
        'gate_passed': not drift,
        'checked_issue_numbers': issue_numbers,
        'checked_pr_urls': pr_urls,
        'notes': notes,
    }
    state['truth_check'] = truth_check
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    if args.print_json:
        print(json.dumps(truth_check, indent=2, sort_keys=True))
    else:
        print(state_path)
    if args.require_match and drift:
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
