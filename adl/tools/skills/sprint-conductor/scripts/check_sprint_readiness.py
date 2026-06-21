#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent


def parse_csv_ints(raw: str) -> list[int]:
    values: list[int] = []
    for part in raw.split(','):
        part = part.strip()
        if not part:
            continue
        values.append(int(part))
    return values


def run_json(cmd: list[str]) -> Any:
    completed = subprocess.run(cmd, capture_output=True, text=True)
    stdout = completed.stdout.strip()
    stderr = completed.stderr.strip()
    if completed.returncode not in {0, 2}:
        detail = stderr or stdout or 'no output'
        raise RuntimeError(f'command failed ({completed.returncode}): {" ".join(cmd)} :: {detail}')
    if not stdout:
        detail = stderr or 'no output'
        raise RuntimeError(f'command returned no JSON: {" ".join(cmd)} :: {detail}')
    return json.loads(stdout)


def resolve_optional_path(repo_root: Path, raw: str | None) -> Path | None:
    if not raw:
        return None
    candidate = Path(raw)
    if candidate.is_absolute():
        return candidate
    return repo_root / candidate


def resolve_paths(repo_root: Path, raw_paths: list[str]) -> list[Path]:
    return [resolve_optional_path(repo_root, raw) for raw in raw_paths if raw]


def inspect_execution_packet(repo_root: Path, execution_mode: str, raw_path: str | None) -> dict[str, Any]:
    packet_path = resolve_optional_path(repo_root, raw_path)
    if execution_mode == 'sequential':
        if packet_path is None:
            return {
                'status': 'not_required',
                'path': None,
                'missing_sections': [],
                'notes': ['execution packet is optional for sequential mode'],
            }
    else:
        if packet_path is None:
            return {
                'status': 'blocked',
                'path': None,
                'missing_sections': [],
                'notes': [f'execution packet path is required for {execution_mode} mode'],
            }

    assert packet_path is not None
    if not packet_path.exists():
        return {
            'status': 'blocked',
            'path': str(packet_path),
            'missing_sections': [],
            'notes': [f'execution packet missing: {packet_path}'],
        }

    text = packet_path.read_text()
    required_sections = ['## Child Issue Wave', '## Recommended Execution Order', '## Watcher Policy']
    if execution_mode in {'parallel', 'hybrid'}:
        required_sections.extend(
            [
                '## Safe Parallel Lanes',
                '## Candidate Parallel Lanes',
                '## Serial Gates',
                '## Parallelism Outcome Plan',
            ]
        )
    missing_sections = [heading for heading in required_sections if heading not in text]
    notes = [f'execution packet present: {packet_path}']
    if missing_sections:
        notes.append('execution packet is missing one or more required readiness sections')
        return {
            'status': 'needs_repair',
            'path': str(packet_path),
            'missing_sections': missing_sections,
            'notes': notes,
        }
    return {
        'status': 'present',
        'path': str(packet_path),
        'missing_sections': [],
        'notes': notes,
    }


def inspect_declared_paths(kind: str, paths: list[Path]) -> dict[str, Any]:
    if not paths:
        return {
            'status': 'needs_repair',
            'paths': [],
            'missing_paths': [],
            'notes': [f'no {kind} path declared'],
        }
    missing = [str(path) for path in paths if not path.exists()]
    return {
        'status': 'declared',
        'paths': [str(path) for path in paths],
        'missing_paths': missing,
        'notes': (
            [f'{kind} path(s) declared']
            + [f'{kind} path declared but not created yet: {path}' for path in missing]
        ),
    }


def next_skills_for_issue(result: dict[str, Any]) -> list[str]:
    required_editor_skills = [
        skill for skill in result.get('required_editor_skills', []) if isinstance(skill, str)
    ]
    if required_editor_skills:
        return required_editor_skills
    notes = [note for note in result.get('notes', []) if isinstance(note, str)]
    if any(note.startswith('No local task bundle found') for note in notes):
        return ['pr-init']
    if any(note.startswith('Ambiguous local task bundles') for note in notes):
        return ['workflow-conductor']
    return []


def build_issue_repairs(preflight: dict[str, Any]) -> list[dict[str, Any]]:
    repairs: list[dict[str, Any]] = []
    for result in preflight.get('issue_results', []):
        status = result.get('status')
        if status == 'ready':
            continue
        next_skills = next_skills_for_issue(result)
        repairs.append(
            {
                'issue_number': result.get('issue_number'),
                'status': status,
                'bundle_path': result.get('bundle_path'),
                'next_skills': next_skills,
                'rationale': '; '.join(result.get('notes', [])) or 'issue readiness repair required',
            }
        )
    return repairs


def overall_status(
    parity: dict[str, Any],
    preflight: dict[str, Any],
    packet: dict[str, Any],
    review_paths: dict[str, Any],
    activity_log_paths: dict[str, Any],
) -> str:
    component_statuses = [
        parity.get('status'),
        preflight.get('status'),
        packet.get('status'),
        review_paths.get('status'),
        activity_log_paths.get('status'),
    ]
    if 'blocked' in component_statuses:
        return 'blocked'
    if any(status in {'drift_detected', 'needs_editor_repair', 'needs_repair'} for status in component_statuses):
        return 'needs_repair'
    return 'ready'


def sprint_goal_policy() -> dict[str, Any]:
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


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--ordered-issues', required=True)
    parser.add_argument('--execution-mode', required=True, choices=['sequential', 'parallel', 'hybrid'])
    parser.add_argument('--execution-packet-path')
    parser.add_argument('--activity-log-path', action='append', default=[])
    parser.add_argument('--review-path', action='append', default=[])
    parser.add_argument('--state')
    parser.add_argument('--tracked-skill-dir')
    parser.add_argument('--installed-skill-dir')
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    repo_root = Path(args.repo_root)
    ordered_issues = parse_csv_ints(args.ordered_issues)
    state: dict[str, Any] = {}
    if args.state:
        state_path = Path(args.state)
        if state_path.exists():
            state = json.loads(state_path.read_text())
    else:
        state_path = None

    parity_cmd = [
        'python3',
        str(SCRIPT_DIR / 'check_installed_skill_parity.py'),
        '--repo-root',
        str(repo_root),
        '--print-json',
    ]
    if args.tracked_skill_dir:
        parity_cmd.extend(['--tracked-skill-dir', args.tracked_skill_dir])
    if args.installed_skill_dir:
        parity_cmd.extend(['--installed-skill-dir', args.installed_skill_dir])
    parity = run_json(parity_cmd)

    preflight = run_json(
        [
            'python3',
            str(SCRIPT_DIR / 'check_sprint_structured_prompt_readiness.py'),
            '--repo-root',
            str(repo_root),
            '--ordered-issues',
            ','.join(str(issue) for issue in ordered_issues),
            '--print-json',
        ]
    )

    packet = inspect_execution_packet(repo_root, args.execution_mode, args.execution_packet_path)
    review_paths = inspect_declared_paths('review', resolve_paths(repo_root, args.review_path))
    activity_log_paths = inspect_declared_paths('activity log', resolve_paths(repo_root, args.activity_log_path))
    issue_repairs = build_issue_repairs(preflight)

    readiness = {
        'status': overall_status(parity, preflight, packet, review_paths, activity_log_paths),
        'ordered_issue_numbers': ordered_issues,
        'execution_mode': args.execution_mode,
        'goal_policy': sprint_goal_policy(),
        'execution_packet': packet,
        'review_paths': review_paths,
        'activity_log_paths': activity_log_paths,
        'issue_repairs': issue_repairs,
        'notes': (
            parity.get('notes', [])
            + preflight.get('notes', [])
            + packet.get('notes', [])
            + review_paths.get('notes', [])
            + activity_log_paths.get('notes', [])
        ),
    }

    state['installed_skill_parity'] = parity
    state['structured_prompt_preflight'] = preflight
    state['readiness_sweep'] = readiness

    if state_path is not None:
        state_path.parent.mkdir(parents=True, exist_ok=True)
        state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    payload = json.dumps(readiness, indent=2, sort_keys=True)
    if args.print_json or state_path is None:
        print(payload)
    else:
        print(state_path)

    return 0 if readiness['status'] == 'ready' else 2 if readiness['status'] == 'needs_repair' else 1


if __name__ == '__main__':
    raise SystemExit(main())
