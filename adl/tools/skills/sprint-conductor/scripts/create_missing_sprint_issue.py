#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path
from typing import Any


def parse_csv_ints(raw: str) -> list[int]:
    values: list[int] = []
    for part in raw.split(','):
        part = part.strip()
        if not part:
            continue
        values.append(int(part))
    return values


def run_json(cmd: list[str]) -> Any:
    out = subprocess.check_output(cmd, text=True)
    return json.loads(out)


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


def build_body(goal: str, ordered: list[int], child_titles: dict[int, str], notes: str | None) -> str:
    ordered_lines = '\n'.join(
        f"{idx}. #{issue} {child_titles.get(issue, '').strip()}".rstrip()
        for idx, issue in enumerate(ordered, start=1)
    )
    body = f"""## Summary

Create the concrete sprint-management issue required to run one bounded `sprint-conductor` sprint.

## Goal

{goal}

## Ordered Child Issue List

{ordered_lines}

## Trial Rules

- Run child issues sequentially only.
- Do not start the next child issue until the current one is fully closed out.
- Use the existing issue lifecycle and editor skills for child issue execution.
- Allow the bounded review-subagent exception only during sprint review when sprint policy explicitly enables it.
- Stop and ask the operator if the sprint scope needs to widen.

## Acceptance Criteria

- A concrete sprint-management issue exists for this ordered child-issue list.
- The sprint issue can be used directly as `sprint.issue_number` for `sprint-conductor`.
- The ordered child-issue list is explicit and stable.

## Non-goals

- Do not widen this sprint beyond the declared ordered child-issue list.
- Do not treat this as a release-closeout or roadmap issue.
"""
    if notes:
        body += f"\n## Notes\n\n{notes.strip()}\n"
    return body


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--ordered-issues', required=True)
    parser.add_argument('--title', required=True)
    parser.add_argument('--goal', required=True)
    parser.add_argument('--state')
    parser.add_argument('--notes')
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    ordered = parse_csv_ints(args.ordered_issues)
    child_titles: dict[int, str] = {}
    for issue in ordered:
        issue_json = run_json(['gh', 'issue', 'view', str(issue), '--json', 'number,title'])
        child_titles[issue] = issue_json.get('title', '')

    body = build_body(args.goal, ordered, child_titles, args.notes)
    with tempfile.NamedTemporaryFile('w', delete=False, suffix='.md') as handle:
        handle.write(body)
        body_path = Path(handle.name)

    issue_url = subprocess.check_output(
        ['gh', 'issue', 'create', '--title', args.title, '--body-file', str(body_path)],
        text=True,
    ).strip()
    match = re.search(r'/issues/(\d+)$', issue_url)
    if not match:
        raise SystemExit(f'Unable to parse created issue number from URL: {issue_url}')
    sprint_issue_number = int(match.group(1))

    result = {
        'created': True,
        'sprint_issue_number': sprint_issue_number,
        'sprint_issue_url': issue_url,
        'ordered_issue_numbers': ordered,
    }

    if args.state:
        state_path = Path(args.state)
        state = {
            'sprint_issue_number': sprint_issue_number,
            'sprint_issue_url': issue_url,
            'sprint_issue_created_by_skill': True,
            'issue_created_by_skill': True,
            'ordered_issue_numbers': ordered,
            'current_issue_number': ordered[0] if ordered else None,
            'completed_issue_numbers': [],
            'blocked_issue_number': None,
            'continuation': 'continue',
            'issue_records': default_issue_records(ordered),
            'truth_check': {
                'status': 'not_run',
                'source': 'sprint_state_only',
                'gate_passed': False,
                'checked_issue_numbers': [],
                'checked_pr_urls': [],
                'notes': ['Sprint issue created by skill; run live GitHub truth check before the first state transition.'],
            },
        }
        state_path.parent.mkdir(parents=True, exist_ok=True)
        state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')
        result['state_path'] = str(state_path)

    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(issue_url)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
