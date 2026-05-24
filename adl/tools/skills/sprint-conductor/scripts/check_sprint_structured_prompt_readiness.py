#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
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


def canonical_slug_from_bundle_dir(bundle_dir: Path) -> str:
    name = bundle_dir.name
    marker = '__'
    if marker in name:
        return name.split(marker, 1)[1]
    return name


GENERIC_SIP_MARKERS = [
    'Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.',
    'Keep the linked issue prompt, SIP, and SOR aligned for review.',
    'The linked source issue prompt is reviewable and structurally valid.',
    'files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt',
    'derive the exact command set from the linked issue prompt',
]

GENERIC_SPP_MARKERS = [
    'Bootstrap-generated SPP',
    'Design-time generated SPP; review before execution',
    'Review this SPP before execution; during runtime, update it before continuing if the actual execution sequence changes.',
    'generated from source issue prompt, STP/SIP surfaces',
    'Design-time execution plan for',
    'Use dependency truth from the linked source issue prompt',
    'Use repo inputs from the linked source issue prompt',
    'Use deliverables from the linked source issue prompt',
    'Satisfy the linked source issue prompt acceptance criteria',
    'Run focused proof gates for acceptance: Satisfy the linked source issue prompt acceptance criteria',
    'Record SRP review results and SOR outcome truth',
]

GENERIC_STP_MARKERS = [
    'Issue-local task surface for',
    'Execute the linked issue prompt with bounded, reviewable changes',
    'Ship the outcome required by the linked source issue prompt',
    'Use deliverables from the linked source issue prompt',
    'Satisfy the linked source issue prompt acceptance criteria',
    'Use repo inputs from the linked source issue prompt',
    'Use dependency truth from the linked source issue prompt',
    'Review source issue prompt and scoped repo inputs',
    'Follow demo/proof requirements from the linked source issue prompt',
    'Generated from 1.0.0 C-SDLC prompt template; refine with editor skills before execution if needed',
]


def contains_any(text: str, markers: list[str]) -> bool:
    return any(marker in text for marker in markers)


def has_truncation_sentinel_line(text: str) -> bool:
    sentinels = {'...', '- ...', '* ...', '<...>'}
    return any(line.strip() in sentinels for line in text.splitlines())


V1_PLACEHOLDER_RE = re.compile(r'<[a-z][a-z0-9_]*>')


def has_unfilled_v1_placeholder(text: str) -> bool:
    return V1_PLACEHOLDER_RE.search(text) is not None


def line_value_after_prefix(text: str, prefix: str) -> str:
    for raw in text.splitlines():
        line = raw.strip()
        if line.startswith(prefix):
            return line.split(':', 1)[1].strip().strip('"').strip("'")
    return ''


def card_status_value(text: str) -> str:
    return (
        line_value_after_prefix(text, 'card_status:') or
        line_value_after_prefix(text, 'Card Status:')
    )


def design_time_card_status_defect(card_name: str, text: str) -> str | None:
    status = card_status_value(text)
    if not status:
        return None
    if card_name in {'sip.md', 'stp.md', 'spp.md'} and status not in {'ready', 'approved'}:
        return f'{card_name} card_status must be ready or approved before execution binding'
    return None


def completed_srp_without_review_results(text: str) -> bool:
    if card_status_value(text) != 'completed':
        return False
    findings_status = line_value_after_prefix(text, 'findings_status:')
    recommended_outcome = line_value_after_prefix(text, 'recommended_outcome:')
    has_review_results = (
        findings_status in {'no_findings', 'findings_present'} and
        recommended_outcome in {'pass', 'block', 'needs_followup'}
    )
    exception = line_value_after_prefix(text, 'review_results_exception:')
    has_final_exception = (
        bool(exception) and
        'pre-execution review results are absent' not in exception
    )
    return not (has_review_results or has_final_exception)


def completed_sor_without_terminal_closeout(text: str) -> bool:
    if card_status_value(text) != 'completed':
        return False
    integration_state = line_value_after_prefix(text, '- Integration state:')
    status = line_value_after_prefix(text, 'Status:')
    result = line_value_after_prefix(text, '- Result:')
    worktree_only = line_value_after_prefix(text, '- Worktree-only paths remaining:')
    validation_body = text.split('## Validation', 1)[1].split('\n## ', 1)[0].strip() if '## Validation' in text else ''
    return not (
        integration_state in {'merged', 'closed_no_pr'} and
        (status, result) in {('DONE', 'PASS'), ('FAILED', 'FAIL')} and
        worktree_only == 'none' and
        bool(validation_body)
    )


def design_time_defect(card_name: str, text: str) -> str | None:
    if has_unfilled_v1_placeholder(text):
        return 'unfilled prompt-template placeholder'
    status_defect = design_time_card_status_defect(card_name, text)
    if status_defect:
        return status_defect
    if card_name == 'sip.md' and contains_any(text, GENERIC_SIP_MARKERS):
        return 'generic design-time SIP scaffold'
    if card_name == 'stp.md':
        if '## Required Outcome' not in text or '## Acceptance Criteria' not in text:
            return 'incomplete design-time STP acceptance surface'
        if contains_any(text, GENERIC_STP_MARKERS):
            return 'generic design-time STP scaffold'
    if card_name == 'spp.md':
        status = line_value_after_prefix(text, 'status:')
        if contains_any(text, GENERIC_SPP_MARKERS) or has_truncation_sentinel_line(text):
            return 'generic or truncated design-time SPP scaffold'
        if status not in {'reviewed', 'approved'}:
            return 'SPP is not reviewed or approved for design-time execution'
    if card_name == 'srp.md':
        if '# Structured Review Policy' in text or 'artifact_type: "structured_review_policy"' in text:
            return 'legacy SRP policy scaffold'
        if '# Structured Review Prompt' not in text or 'artifact_type: "structured_review_prompt"' not in text:
            return 'missing Structured Review Prompt semantics'
        if completed_srp_without_review_results(text):
            return 'completed SRP lacks review results or a final policy exception'
    if card_name == 'sor.md':
        if completed_sor_without_terminal_closeout(text):
            return 'completed SOR lacks terminal closeout truth'
    return None


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
            'canonical_slug': None,
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
        required_cards['srp.md'] = 'srp-editor'

    missing_cards: list[str] = []
    contradictory_cards: list[str] = []
    design_time_defects: list[str] = []
    required_editor_skills: list[str] = []

    for card_name, editor_skill in required_cards.items():
        card_path = bundle_dir / card_name
        if not card_path.exists():
            missing_cards.append(card_name)
            if editor_skill not in required_editor_skills:
                required_editor_skills.append(editor_skill)
            continue
        defect = design_time_defect(card_name, card_path.read_text())
        if defect:
            design_time_defects.append(f'{card_name}: {defect}')
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
    if contradictory_cards or missing_cards or design_time_defects:
        status = 'needs_editor_repair'

    notes.extend(
        [f'Missing {name}' for name in missing_cards] +
        [f'Contradictory bootstrap residue in {name}' for name in contradictory_cards] +
        [f'Design-time card defect in {defect}' for defect in design_time_defects]
    )

    return {
        'issue_number': issue_number,
        'bundle_path': str(bundle_dir),
        'canonical_slug': canonical_slug_from_bundle_dir(bundle_dir),
        'status': status,
        'missing_cards': missing_cards,
        'contradictory_cards': contradictory_cards,
        'design_time_defects': design_time_defects,
        'required_editor_skills': required_editor_skills,
        'notes': notes,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--ordered-issues', required=True)
    parser.add_argument('--state')
    parser.add_argument('--require-spp', dest='require_spp', action='store_true', default=True)
    parser.add_argument('--skip-spp', dest='require_spp', action='store_false')
    parser.add_argument('--require-srp', dest='require_srp', action='store_true', default=True)
    parser.add_argument('--skip-srp', dest='require_srp', action='store_false')
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
