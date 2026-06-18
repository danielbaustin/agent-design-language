#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

from close_sprint_issue import require_child_closeout_truth, require_clean_close_boundary
from write_sprint_closeout_artifact import closure_cleanliness


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")


def gather_blockers(state: dict[str, Any]) -> list[str]:
    blockers: list[str] = []
    if state.get('sprint_issue_number') is None:
        blockers.append('Cannot evaluate sprint closeout because sprint_issue_number is missing from state.')
    for check in (require_child_closeout_truth, require_clean_close_boundary):
        try:
            check(state)
        except SystemExit as exc:
            blockers.append(str(exc))
    return blockers


def coverage_state(state: dict[str, Any]) -> dict[str, Any]:
    closeout = state.get('closeout') or {}
    return closeout.get('coverage') or state.get('coverage') or {}


def rust_tracker_state(state: dict[str, Any]) -> dict[str, Any]:
    closeout = state.get('closeout') or {}
    return closeout.get('rust_tracker') or state.get('rust_tracker') or {}


def validation_state(state: dict[str, Any]) -> dict[str, Any]:
    closeout = state.get('closeout') or {}
    return state.get('validation') or closeout.get('validation') or {}


def gather_remediation(state: dict[str, Any], artifact_path: str | None) -> list[str]:
    remediation: list[str] = []
    review = state.get('review') or {}
    if review.get('status') != 'done':
        remediation.append('Sprint review status is not done.')
    if not review.get('packet_path'):
        remediation.append('Sprint review packet path is not recorded.')
    if not review.get('code_review_path'):
        remediation.append('Sprint code review artifact path is not recorded.')
    if not review.get('test_review_path'):
        remediation.append('Sprint test review artifact path is not recorded.')
    if not review.get('synthesis_path'):
        remediation.append('Sprint review synthesis path is not recorded.')

    coverage = coverage_state(state)
    coverage_source = coverage.get('source') or 'missing'
    if coverage_source == 'missing':
        remediation.append('Sprint coverage closeout source is not recorded.')
    elif coverage_source != 'not_applicable' and not coverage.get('summary'):
        remediation.append('Sprint coverage closeout summary is not recorded.')

    rust_tracker = rust_tracker_state(state)
    rust_source = rust_tracker.get('source') or 'missing'
    if rust_source == 'missing':
        remediation.append('Sprint Rust tracker closeout source is not recorded.')
    elif rust_source != 'not_applicable':
        for field in ('watch_count', 'review_count', 'rationale_count'):
            if rust_tracker.get(field) is None:
                remediation.append(f'Sprint Rust tracker `{field}` is not recorded.')

    if not artifact_path:
        remediation.append('No sprint closeout artifact path is available; pass --out or record one in sprint state.')

    return remediation


def maybe_write_artifact(state_path: Path, state: dict[str, Any], out_arg: str | None) -> str | None:
    closeout = state.get('closeout') or {}
    out_path = out_arg or closeout.get('closeout_artifact_path')
    if not out_path:
        return None

    script_path = Path(__file__).with_name('write_sprint_closeout_artifact.py')
    subprocess.run(
        [
            sys.executable,
            str(script_path),
            '--state',
            str(state_path),
            '--out',
            str(out_path),
        ],
        check=True,
        capture_output=True,
        text=True,
    )
    refreshed = load_json(state_path)
    state.clear()
    state.update(refreshed)
    return str((state.get('closeout') or {}).get('closeout_artifact_path') or out_path)


def render_summary(
    state: dict[str, Any],
    classification: str,
    cleanliness: str,
    artifact_path: str | None,
    blockers: list[str],
    remediation: list[str],
) -> str:
    lines = [
        f"Sprint closeout evaluation for #{state.get('sprint_issue_number')}: `{classification}`.",
        f"Ordered issues: {', '.join(f'#{issue}' for issue in state.get('ordered_issue_numbers', [])) or 'none recorded'}.",
        f"Closure cleanliness: `{cleanliness}`.",
    ]
    if artifact_path:
        lines.append(f"Closeout artifact: `{artifact_path}`.")

    if blockers:
        lines.append('Blocking conditions:')
        for item in blockers:
            lines.append(f"- {item}")
    if remediation:
        lines.append('Remediation required before sprint close:')
        for item in remediation:
            lines.append(f"- {item}")

    follow_ups = state.get('follow_up_issues', [])
    if follow_ups:
        lines.append('Follow-up routing:')
        for item in follow_ups:
            lines.append(
                f"- #{item.get('issue_number')} disposition=`{item.get('disposition')}` summary={item.get('summary')}"
            )
    else:
        lines.append('Follow-up routing: none.')

    return '\n'.join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--state', required=True)
    parser.add_argument('--out')
    parser.add_argument('--summary-out')
    parser.add_argument('--print-json', action='store_true')
    args = parser.parse_args()

    state_path = Path(args.state)
    state = load_json(state_path)

    artifact_path = maybe_write_artifact(state_path, state, args.out)
    blockers = gather_blockers(state)
    cleanliness = closure_cleanliness(state)
    remediation = [] if blockers else gather_remediation(state, artifact_path)

    if blockers:
        classification = 'blocked'
        closeout_status = 'blocked'
    elif remediation:
        classification = 'needs_remediation'
        closeout_status = 'in_progress'
    else:
        classification = 'ready_to_close'
        closeout_status = 'done'

    summary = render_summary(state, classification, cleanliness, artifact_path, blockers, remediation)

    closeout = state.setdefault('closeout', {})
    closeout['status'] = closeout_status
    closeout['readiness'] = classification
    closeout['closure_cleanliness'] = cleanliness
    closeout['notes'] = blockers + remediation
    if artifact_path:
        closeout['closeout_artifact_path'] = artifact_path
    if args.summary_out:
        summary_path = Path(args.summary_out)
        summary_path.parent.mkdir(parents=True, exist_ok=True)
        summary_path.write_text(summary + '\n', encoding='utf-8')
        closeout['closeout_note_path'] = str(summary_path)
    closeout['sprint_issue_close_summary'] = summary
    state['sprint_issue_close_summary'] = summary
    write_json(state_path, state)

    result = {
        'classification': classification,
        'closeout_status': closeout_status,
        'closure_cleanliness': cleanliness,
        'closeout_artifact_path': artifact_path,
        'summary': summary,
        'summary_out': closeout.get('closeout_note_path'),
        'blockers': blockers,
        'remediation': remediation,
        'state_path': str(state_path),
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(classification)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
