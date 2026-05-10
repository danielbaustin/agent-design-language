#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


def closure_cleanliness(state: dict) -> str:
    if state.get('blocked_issue_number') is not None:
        return 'residual_debt'
    if state.get('deferred_issue_numbers'):
        return 'residual_debt'
    follow_ups = state.get('follow_up_issues', [])
    if any(item.get('disposition') == 'must_land_before_sprint_close' for item in follow_ups):
        return 'residual_debt'
    if follow_ups:
        return 'clean_with_post_sprint_followups'
    return 'clean'


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--state', required=True)
    parser.add_argument('--out', required=True)
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    state_path = Path(args.state)
    state = json.loads(state_path.read_text())
    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)

    records = {record.get('issue_number'): record for record in state.get('issue_records', [])}
    cleanliness = closure_cleanliness(state)

    lines = [
        '# Sprint Closeout Artifact',
        '',
        f"- sprint issue: `#{state.get('sprint_issue_number')}`",
        f"- ordered issues: `{', '.join(str(i) for i in state.get('ordered_issue_numbers', []))}`",
        f"- closure cleanliness: `{cleanliness}`",
        '',
        '## Ordered Child Issues',
        '',
    ]
    for issue in state.get('ordered_issue_numbers', []):
        record = records.get(issue, {})
        lines.append(f"- `#{issue}` status=`{record.get('status', 'unknown')}` pr=`{record.get('pr_url') or 'not_applicable'}`")

    lines.extend(['', '## Follow-up Issues', ''])
    follow_ups = state.get('follow_up_issues', [])
    if follow_ups:
        for item in follow_ups:
            lines.append(f"- `#{item.get('issue_number')}` disposition=`{item.get('disposition')}` summary={item.get('summary')}")
    else:
        lines.append('- none')

    lines.extend(['', '## Review / Closeout Surfaces', ''])
    lines.append(f"- review packet: `{state.get('review', {}).get('packet_path') or 'not_recorded'}`")
    lines.append(f"- sprint close summary: `{state.get('sprint_issue_close_summary') or 'not_recorded'}`")

    out_path.write_text('\n'.join(lines).rstrip() + '\n', encoding='utf-8')

    state.setdefault('closeout', {})
    state['closeout']['closeout_artifact_path'] = str(out_path)
    state['closeout']['closure_cleanliness'] = cleanliness
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    result = {
        'closeout_artifact_path': str(out_path),
        'closure_cleanliness': cleanliness,
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(out_path)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
