#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path
from typing import Any

from issue_goal_metrics import default_goal_metrics_summary


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


def render_prompt_template(repo_root: Path, kind: str, replacements: dict[str, str]) -> str | None:
    template_path = repo_root / 'docs' / 'templates' / 'prompts' / '1.0.0' / f'{kind}.md'
    if not template_path.is_file():
        return None
    text = template_path.read_text()
    for token, value in replacements.items():
        text = text.replace(token, value)
    return text


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
    issue_padded = f'{sprint_issue_number:04d}'
    source_rel = str(source_path.relative_to(repo_root))
    stp_rel = str(stp_path.relative_to(repo_root))
    sip_rel = str(sip_path.relative_to(repo_root))
    spp_rel = str(spp_path.relative_to(repo_root))
    srp_rel = str(srp_path.relative_to(repo_root))
    sor_rel = str(sor_path.relative_to(repo_root))
    replacements = {
        '<issue>': str(sprint_issue_number),
        '<issue_padded>': issue_padded,
        '<task_id>': f'issue-{issue_padded}',
        '<run_id>': f'issue-{issue_padded}',
        '<version>': version,
        '<slug>': slug,
        '<title>': title,
        '<branch>': 'not bound yet',
        '<issue_url>': issue_url,
        '<source_issue_prompt>': source_rel,
        '<docs_context>': 'none',
        '<output_card>': sor_rel,
        '<stp_card>': stp_rel,
        '<sip_card>': sip_rel,
        '<spp_card>': spp_rel,
        '<srp_card>': srp_rel,
        '<sor_card>': sor_rel,
        '<wp>': 'sprint',
        '<required_outcome_type>': 'docs',
        '<demo_required>': 'false',
        '<issue_graph_note>': 'Sprint management issue created by sprint-conductor helper.',
        '<summary>': f'Sprint-management surface for {title}.',
        '<goal>': body,
        '<required_outcome>': 'Create a concrete sprint-management issue with complete local C-SDLC cards.',
        '<deliverables>': 'Sprint issue body and five-card local task bundle.',
        '<acceptance_criteria>': 'All five local cards exist and are ready for sprint preflight review.',
        '<repo_inputs>': 'Ordered child issue list and descriptive sprint objective.',
        '<dependencies>': 'Child issue cards must exist and validate before sprint execution.',
        '<target_files_surfaces>': 'Local sprint issue body and task bundle.',
        '<validation_plan>': 'Run sprint structured-prompt readiness before execution starts.',
        '<demo_proof_requirements>': 'No demo required for sprint management issue creation.',
        '<non_goals>': 'Do not execute child issues from this bootstrap helper.',
        '<issue_graph_notes>': 'Sprint umbrella orchestrates children; it does not replace child issue closeout.',
        '<notes_risks>': 'Review cards before sprint execution starts.',
        '<tooling_notes>': 'Generated from docs/templates/prompts/1.0.0/ when available.',
        '<target_files_surfaces_inline>': 'Local sprint issue body and task bundle.',
        '<non_goals_inline>': 'Do not execute child issues from this bootstrap helper.',
        '<plan_summary>': f'Design-time sprint-management plan for {title}.',
        '<dependencies_inline>': 'Child issue cards must exist and validate before execution.',
        '<repo_inputs_inline>': 'Ordered child issue list and descriptive sprint objective.',
        '<deliverables_inline>': 'Sprint issue body and five-card local task bundle.',
        '<acceptance_criteria_inline>': 'All five cards exist and pass preflight readiness.',
        '<risks_inline>': 'Sprint state can drift if child closeout is skipped.',
        '<validation_plan_inline>': 'Run sprint structured-prompt readiness before execution starts.',
        '<notes_risks_inline>': 'Review and update before sprint execution starts.',
        '<status>': 'NOT_STARTED',
        '<timestamp>': '1970-01-01T00:00:00Z',
        '<branch_action>': 'Preserved pre-run branch truth; no execution branch or worktree is bound yet.',
    }

    write_file(source_path, body + '\n')
    for kind, path in {
        'stp': stp_path,
        'sip': sip_path,
        'sor': sor_path,
        'spp': spp_path,
        'srp': srp_path,
    }.items():
        rendered = render_prompt_template(repo_root, kind, replacements)
        if rendered is None and kind == 'stp':
            rendered = body + '\n'
        if rendered is None:
            rendered = f'# {kind.upper()}\n\nissue: {sprint_issue_number}\ntask_id: "issue-{issue_padded}"\n'
        write_file(path, rendered)
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
        # pr.sh init establishes the canonical local paths; render the sprint body
        # back through the 1.0.0 templates so the local bundle is not left as a
        # generic bootstrap stub or a plain issue-body STP.
        return fallback_bootstrap_local_bundle(
            repo_root,
            sprint_issue_number,
            title,
            issue_url,
            body,
        )
    return fallback_bootstrap_local_bundle(repo_root, sprint_issue_number, title, issue_url, body)


def default_issue_records(ordered: list[int]) -> list[dict[str, Any]]:
    return [
        {
            'issue_number': issue,
            'status': 'pending',
            'pr_url': None,
            'artifact_paths': [],
            'goal_metrics': default_goal_metrics_summary(),
        }
        for issue in ordered
    ]


def default_goal_policy() -> dict[str, Any]:
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


def build_body(goal: str, ordered: list[int], child_titles: dict[int, str], notes: str | None) -> str:
    ordered_lines = '\n'.join(
        f"{idx}. #{issue} {child_titles.get(issue, '').strip()}".rstrip()
        for idx, issue in enumerate(ordered, start=1)
    )
    body = f"""## Summary

Create the concrete sprint-management issue required to run one bounded `sprint-conductor` sprint.

## Sprint Objective

{goal}

## Ordered Child Issue List

{ordered_lines}

## Trial Rules

- Declare sprint execution mode: `sequential`, `parallel`, or `hybrid`.
- For `parallel` or `hybrid` sprints, include or link a Sprint Execution Packet with safe lanes, serial gates, PVF notes, and residual routing.
- Current sprint helper state remains single-current-issue; use separate issue workers or sessions for intentional parallel lanes.
- Treat the sprint objective as descriptive coordination context, not as the active session goal during child issue implementation.
- Each child issue execution session must create its own issue-bound goal after bind/readiness succeeds and before implementation starts.
- Do not start the next child issue until the current one is fully closed out.
- Use the existing issue lifecycle and editor skills for child issue execution.
- Allow the bounded review-subagent exception only during sprint review when sprint policy explicitly enables it.
- Stop and ask the operator if the sprint scope needs to widen.

## Acceptance Criteria

- A concrete sprint-management issue exists for this child-issue wave and declared execution mode.
- The sprint issue can be used directly as `sprint.issue_number` for `sprint-conductor`.
- The child-issue list, execution mode, safe lanes, and serial gates are explicit and stable.

## Non-goals

- Do not widen this sprint beyond the declared child-issue wave or SEP boundary.
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
            'goal_policy': default_goal_policy(),
            'local_bundle': local_bundle,
            'issue_records': default_issue_records(ordered),
            'structured_prompt_preflight': {
                'status': 'not_run',
                'required_card_types': ['stp.md', 'sip.md', 'sor.md', 'spp.md', 'srp.md'],
                'issue_results': [],
                'notes': [
                    'Run sprint-wide structured prompt preflight before starting issue execution, including SPP and SRP design-time readiness.',
                ],
            },
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
