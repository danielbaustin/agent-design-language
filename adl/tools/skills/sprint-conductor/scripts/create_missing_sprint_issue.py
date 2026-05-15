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


def sanitize_slug(raw: str) -> str:
    slug = raw.strip().lower()
    slug = re.sub(r'^\[[^\]]+\]', '', slug).strip()
    slug = re.sub(r'[^a-z0-9]+', '-', slug)
    slug = re.sub(r'-{2,}', '-', slug).strip('-')
    return slug or 'sprint-management-issue'


def infer_version_from_title(title: str) -> str:
    match = re.search(r'\[(v[0-9][^\]]*)\]', title)
    if not match:
        raise SystemExit(
            f'Unable to infer milestone version from sprint title: {title!r}'
        )
    return match.group(1)


def issue_prompt_path(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return repo_root / '.adl' / version / 'bodies' / f'issue-{issue_number:04d}-{slug}.md'


def task_bundle_dir(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return repo_root / '.adl' / version / 'tasks' / f'issue-{issue_number:04d}__{slug}'


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


def fallback_bootstrap_local_bundle(
    repo_root: Path,
    sprint_issue_number: int,
    title: str,
    issue_url: str,
    body: str,
) -> dict[str, str]:
    version = infer_version_from_title(title)
    slug = sanitize_slug(title)
    source_path = issue_prompt_path(repo_root, version, sprint_issue_number, slug)
    bundle_dir = task_bundle_dir(repo_root, version, sprint_issue_number, slug)
    stp_path = bundle_dir / 'stp.md'
    sip_path = bundle_dir / 'sip.md'
    sor_path = bundle_dir / 'sor.md'
    spp_path = bundle_dir / 'spp.md'
    srp_path = bundle_dir / 'srp.md'

    write_file(source_path, body + '\n')
    write_file(stp_path, body + '\n')
    write_file(
        sip_path,
        (
            "# ADL Input Card\n\n"
            f"Task ID: issue-{sprint_issue_number}\n"
            f"Run ID: issue-{sprint_issue_number}\n"
            f"Version: {version}\n"
            f"Title: {title}\n"
            "Branch: not bound yet\n\n"
            "Context:\n"
            f"- Issue: {issue_url}\n"
            f"- Source Issue Prompt: {source_path.relative_to(repo_root)}\n\n"
            "## Agent Execution Rules\n"
            "- This issue is not started yet; do not assume a branch or worktree already exists.\n"
        ),
    )
    write_file(
        sor_path,
        (
            f"# issue-{sprint_issue_number}\n\n"
            f"Task ID: issue-{sprint_issue_number}\n"
            f"Run ID: issue-{sprint_issue_number}\n"
            f"Version: {version}\n"
            f"Title: {title}\n"
            "Branch: not bound yet\n"
            "Status: NOT_STARTED\n"
        ),
    )
    write_file(
        spp_path,
        (
            "issue: {issue}\n"
            "task_id: \"issue-{issue}\"\n"
            "run_id: \"issue-{issue}\"\n"
            "codex_plan:\n"
            "  status: pending\n"
            "  step: \"Sprint management issue created; detailed plan pending execution.\"\n"
        ).format(issue=sprint_issue_number),
    )
    write_file(
        srp_path,
        (
            "issue: {issue}\n"
            "task_id: \"issue-{issue}\"\n"
            "review_status: pending\n"
            "notes:\n"
            "  - \"Sprint management issue created; review policy pending execution.\"\n"
        ).format(issue=sprint_issue_number),
    )
    return {
        'version': version,
        'slug': slug,
        'source_path': str(source_path),
        'bundle_dir': str(bundle_dir),
        'stp_path': str(stp_path),
        'sip_path': str(sip_path),
        'sor_path': str(sor_path),
        'spp_path': str(spp_path),
        'srp_path': str(srp_path),
    }


def bootstrap_local_bundle(
    repo_root: Path,
    sprint_issue_number: int,
    title: str,
    issue_url: str,
    body: str,
) -> dict[str, str]:
    init_script = repo_root / 'adl' / 'tools' / 'pr.sh'
    if init_script.is_file():
        subprocess.check_call(
            [
                'bash',
                str(init_script),
                'init',
                str(sprint_issue_number),
            ],
            cwd=repo_root,
        )
        version = infer_version_from_title(title)
        slug = sanitize_slug(title)
        source_path = issue_prompt_path(repo_root, version, sprint_issue_number, slug)
        bundle_dir = task_bundle_dir(repo_root, version, sprint_issue_number, slug)
        stp_path = bundle_dir / 'stp.md'
        write_file(source_path, body + '\n')
        write_file(stp_path, body + '\n')
        return {
            'version': version,
            'slug': slug,
            'source_path': str(source_path),
            'bundle_dir': str(bundle_dir),
            'stp_path': str(stp_path),
            'sip_path': str(bundle_dir / 'sip.md'),
            'sor_path': str(bundle_dir / 'sor.md'),
            'spp_path': str(bundle_dir / 'spp.md'),
            'srp_path': str(bundle_dir / 'srp.md'),
        }
    return fallback_bootstrap_local_bundle(repo_root, sprint_issue_number, title, issue_url, body)


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

    repo_root = Path(args.repo_root)
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
    local_bundle = bootstrap_local_bundle(repo_root, sprint_issue_number, args.title, issue_url, body)

    result = {
        'created': True,
        'sprint_issue_number': sprint_issue_number,
        'sprint_issue_url': issue_url,
        'ordered_issue_numbers': ordered,
        'local_bundle': local_bundle,
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
            'local_bundle': local_bundle,
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
