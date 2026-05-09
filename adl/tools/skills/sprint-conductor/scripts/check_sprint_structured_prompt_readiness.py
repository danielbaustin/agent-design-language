#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


def parse_csv_ints(raw: str) -> list[int]:
    values: list[int] = []
    for part in raw.split(','):
        part = part.strip()
        if not part:
            continue
        values.append(int(part))
    return values


def find_bundle_dir(repo_root: Path, issue_number: int) -> tuple[Path | None, list[str]]:
    matches = sorted(
        repo_root.glob(f'.adl/*/tasks/issue-{issue_number}__*')
    )
    if not matches:
        return None, [f'No local task bundle found for issue #{issue_number}.']
    if len(matches) > 1:
        rendered = ', '.join(str(path) for path in matches)
        return None, [f'Ambiguous local task bundles for issue #{issue_number}: {rendered}']
    return matches[0], []


def inspect_issue(
    repo_root: Path,
    issue_number: int,
    require_spp: bool,
    require_srp: bool,
) -> dict:
    bundle_dir, notes = find_bundle_dir(repo_root, issue_number)
    if bundle_dir is None:
        return {
            'issue_number': issue_number,
            'bundle_path': None,
            'status': 'blocked',
            'missing_cards': [],
            'contradictory_cards': [],
            'required_editor_skills': [],
            'notes': notes,
        }

    required_cards = {
        'stp.md': 'stp-editor',
        'sip.md': 'sip-editor',
        'sor.md': 'sor-editor',
    }
    if require_spp:
        required_cards['spp.md'] = 'spp-editor'
    if require_srp:
        required_cards['srp.md'] = 'spp-editor'

    missing_cards: list[str] = []
    contradictory_cards: list[str] = []
    required_editor_skills: list[str] = []

    for card_name, editor_skill in required_cards.items():
        card_path = bundle_dir / card_name
        if not card_path.exists():
            missing_cards.append(card_name)
            if editor_skill not in required_editor_skills:
                required_editor_skills.append(editor_skill)

    sor_path = bundle_dir / 'sor.md'
    if sor_path.exists():
        sor_text = sor_path.read_text()
        if 'Status: IN_PROGRESS' in sor_text and 'No implementation has started yet' in sor_text:
            contradictory_cards.append('sor.md')
            if 'sor-editor' not in required_editor_skills:
                required_editor_skills.append('sor-editor')

    status = 'ready'
    if contradictory_cards or missing_cards:
        status = 'needs_editor_repair'

    notes.extend(
        [f'Missing {name}' for name in missing_cards] +
        [f'Contradictory bootstrap residue in {name}' for name in contradictory_cards]
    )

    return {
        'issue_number': issue_number,
        'bundle_path': str(bundle_dir),
        'status': status,
        'missing_cards': missing_cards,
        'contradictory_cards': contradictory_cards,
        'required_editor_skills': required_editor_skills,
        'notes': notes,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--ordered-issues', required=True)
    parser.add_argument('--state')
    parser.add_argument('--require-spp', action='store_true')
    parser.add_argument('--require-srp', action='store_true')
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    repo_root = Path(args.repo_root)
    ordered = parse_csv_ints(args.ordered_issues)
    issue_results = [
        inspect_issue(repo_root, issue, args.require_spp, args.require_srp)
        for issue in ordered
    ]

    overall_status = 'ready'
    if any(result['status'] == 'blocked' for result in issue_results):
        overall_status = 'blocked'
    elif any(result['status'] == 'needs_editor_repair' for result in issue_results):
        overall_status = 'needs_editor_repair'

    result = {
        'status': overall_status,
        'required_card_types': ['stp.md', 'sip.md', 'sor.md'] +
        (['spp.md'] if args.require_spp else []) +
        (['srp.md'] if args.require_srp else []),
        'issue_results': issue_results,
        'notes': [
            'Review and repair all child issue structured cards before starting issue execution.'
            if overall_status != 'ready'
            else 'All ordered child issue structured cards are ready for sprint start.'
        ],
    }

    if args.state:
        state_path = Path(args.state)
        state = json.loads(state_path.read_text()) if state_path.exists() else {}
        state['structured_prompt_preflight'] = result
        state_path.parent.mkdir(parents=True, exist_ok=True)
        state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    elif args.state:
        print(args.state)
    else:
        print(json.dumps(result, indent=2, sort_keys=True))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
