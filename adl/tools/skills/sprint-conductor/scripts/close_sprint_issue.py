#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path


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
