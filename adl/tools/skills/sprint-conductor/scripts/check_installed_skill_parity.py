#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path


def is_generated_artifact(rel: str) -> bool:
    parts = rel.split('/')
    if '__pycache__' in parts:
        return True
    if rel.endswith(('.pyc', '.pyo')):
        return True
    if rel.endswith('.DS_Store'):
        return True
    return False


def collect_files(root: Path) -> set[str]:
    files: set[str] = set()
    for path in root.rglob('*'):
        if path.is_file():
            rel = path.relative_to(root).as_posix()
            if is_generated_artifact(rel):
                continue
            files.add(rel)
    return files


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo-root', required=True)
    parser.add_argument('--state')
    parser.add_argument('--tracked-skill-dir')
    parser.add_argument('--installed-skill-dir')
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    repo_root = Path(args.repo_root)
    tracked = Path(args.tracked_skill_dir) if args.tracked_skill_dir else repo_root / 'adl' / 'tools' / 'skills' / 'sprint-conductor'
    installed = Path(args.installed_skill_dir) if args.installed_skill_dir else Path.home() / '.codex' / 'skills' / 'sprint-conductor'

    notes: list[str] = []
    left_only: list[str] = []
    right_only: list[str] = []
    diff_files: list[str] = []
    ignored_generated_files: list[str] = []
    status = 'matched'

    if not tracked.is_dir():
        status = 'blocked'
        notes.append(f'tracked skill dir missing: {tracked}')
    if not installed.is_dir():
        status = 'blocked'
        notes.append(f'installed skill dir missing: {installed}')

    if status != 'blocked':
        ignored_generated_files = sorted(
            rel
            for root in (tracked, installed)
            for rel in (path.relative_to(root).as_posix() for path in root.rglob('*') if path.is_file())
            if is_generated_artifact(rel)
        )
        tracked_files = collect_files(tracked)
        installed_files = collect_files(installed)
        left_only = sorted(tracked_files - installed_files)
        right_only = sorted(installed_files - tracked_files)
        for rel in sorted(tracked_files & installed_files):
            if (tracked / rel).read_bytes() != (installed / rel).read_bytes():
                diff_files.append(rel)
        if left_only or right_only or diff_files:
            status = 'drift_detected'
            notes.append('installed sprint-conductor differs from tracked bundle after generated-file filtering')
            notes.append('repair by running: bash adl/tools/install_adl_operational_skills.sh')
        else:
            notes.append('installed sprint-conductor matches tracked bundle after generated-file filtering')
        if ignored_generated_files:
            notes.append('ignored generated parity artifacts such as __pycache__, .pyc, .pyo, and .DS_Store')

    result = {
        'status': status,
        'tracked_skill_dir': str(tracked),
        'installed_skill_dir': str(installed),
        'left_only': left_only,
        'right_only': right_only,
        'diff_files': diff_files,
        'ignored_generated_files': ignored_generated_files,
        'parity_policy': 'installed bundle must match tracked bundle except generated cache artifacts',
        'repair_command': 'bash adl/tools/install_adl_operational_skills.sh',
        'notes': notes,
    }

    if args.state:
        state_path = Path(args.state)
        state = json.loads(state_path.read_text()) if state_path.exists() else {}
        state['installed_skill_parity'] = result
        state_path.parent.mkdir(parents=True, exist_ok=True)
        state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    payload = json.dumps(result, indent=2, sort_keys=True)
    if args.print_json or not args.state:
        print(payload)
    else:
        print(args.state)
    return 0 if status == 'matched' else 2 if status == 'drift_detected' else 1


if __name__ == '__main__':
    raise SystemExit(main())
