#!/usr/bin/env python3
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
import re
import shlex
import subprocess
import tempfile
from pathlib import Path
from typing import Any

from issue_goal_metrics import default_goal_metrics_summary

SCRIPT_REPO_ROOT = Path(__file__).resolve().parents[5]


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


def run_capture(cmd: list[str]) -> str:
    return subprocess.check_output(cmd, text=True).strip()


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


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace('+00:00', 'Z')


def milestone_doc_prefix(version: str) -> str:
    digits = re.sub(r'[^0-9]', '', version)
    return f'V{digits}' if digits else 'VUNKNOWN'


def issue_prompt_path(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return repo_root / '.adl' / version / 'bodies' / f'issue-{issue_number:04d}-{slug}.md'


def task_bundle_dir(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return repo_root / '.adl' / version / 'tasks' / f'issue-{issue_number:04d}__{slug}'


def sprint_bundle_dir(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return repo_root / '.adl' / version / 'sprints' / f'issue-{issue_number:04d}__{slug}'


def sprint_execution_packet_path(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return sprint_bundle_dir(repo_root, version, issue_number, slug) / 'SPRINT_EXECUTION_PACKET.md'


def sprint_closeout_note_path(repo_root: Path, version: str, issue_number: int, slug: str) -> Path:
    return sprint_bundle_dir(repo_root, version, issue_number, slug) / 'SPRINT_CLOSEOUT_SUMMARY.md'


def sprint_review_dir(repo_root: Path, version: str) -> Path:
    return repo_root / 'docs' / 'milestones' / version / 'review' / 'sprint_execution_packets'


def sprint_activity_log_path(repo_root: Path, version: str, issue_number: int) -> Path:
    return sprint_review_dir(repo_root, version) / f'{milestone_doc_prefix(version)}_SPRINT_ACTIVITY_LOG_{issue_number}.md'


def sprint_review_path(repo_root: Path, version: str, issue_number: int) -> Path:
    return sprint_review_dir(repo_root, version) / f'{milestone_doc_prefix(version)}_SPRINT_REVIEW_{issue_number}.md'


def sprint_closeout_artifact_path(repo_root: Path, version: str, issue_number: int) -> Path:
    return sprint_review_dir(repo_root, version) / f'{milestone_doc_prefix(version)}_SPRINT_CLOSEOUT_{issue_number}.md'


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


def default_issue_command(subcommand: str) -> list[str]:
    issue_binary = SCRIPT_REPO_ROOT / 'adl' / 'target' / 'debug' / 'adl-issue'
    if issue_binary.is_file():
        return [str(issue_binary), subcommand]
    return ['bash', str(SCRIPT_REPO_ROOT / 'adl' / 'tools' / 'pr.sh'), 'issue', subcommand]


def command_with_override(env_var: str, default: list[str]) -> list[str]:
    raw = os.environ.get(env_var)
    if not raw:
        return default
    return shlex.split(raw)


def issue_view(issue_number: int) -> dict[str, Any]:
    cmd = command_with_override('ADL_SPRINT_ISSUE_VIEW_CMD', default_issue_command('view'))
    return run_json(cmd + [str(issue_number), '--json'])


def issue_create(title: str, body_path: Path) -> dict[str, Any]:
    cmd = command_with_override('ADL_SPRINT_ISSUE_CREATE_CMD', default_issue_command('create'))
    raw = run_capture(cmd + ['--title', title, '--body-file', str(body_path), '--json'])
    try:
        payload = json.loads(raw)
    except json.JSONDecodeError:
        payload = {'url': raw}
    if 'number' not in payload:
        issue_url = payload.get('url', raw)
        match = re.search(r'/issues/(\d+)$', issue_url)
        if not match:
            raise SystemExit(f'Unable to parse created issue number from issue-create output: {raw}')
        payload['number'] = int(match.group(1))
        payload['url'] = issue_url
    return payload


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


def render_execution_packet(
    repo_root: Path,
    version: str,
    sprint_issue_number: int,
    title: str,
    goal: str,
    ordered: list[int],
    child_titles: dict[int, str],
    execution_mode: str,
    notes: str | None,
) -> str:
    activity_log = sprint_activity_log_path(repo_root, version, sprint_issue_number)
    review_path = sprint_review_path(repo_root, version, sprint_issue_number)
    ordered_issue_lines = '\n'.join(
        f"| #{issue} | child_issue | pending | issue-specific implementation surface | {child_titles.get(issue, '').strip() or 'title not yet resolved'} |"
        for issue in ordered
    )
    execution_order = '\n'.join(
        f"{idx}. Start `#{issue}` only when its sprint gate is clear; title: `{child_titles.get(issue, '').strip() or 'title not yet resolved'}`."
        for idx, issue in enumerate(ordered, start=1)
    )
    if not execution_order:
        execution_order = '1. No child issues were declared.'
    safe_parallel = (
        '| serial-default | none yet | Default helper posture keeps execution serial until an operator or later issue proves safe parallel lanes. | Add explicit write-set and PVF coordination before widening. |'
    )
    candidate_parallel = '\n'.join(
        f"| issue-{issue} | {'serial_gate' if idx == 0 else 'blocked_until_dependency'} | #{issue} | issue-specific write set not yet classified | not_recorded_yet | not_recorded_yet | closeout of prior ordered gate | possible control-plane collisions until classified | required | optional | helper bootstrap keeps this lane serial until issue-local proof exists | reclassify during sprint planning or follow-on automation |"
        for idx, issue in enumerate(ordered)
    )
    if not candidate_parallel:
        candidate_parallel = '| none | serial_gate | none | no child issues declared | not_recorded_yet | not_recorded_yet | no child issues declared | not_applicable | required | optional | no lane to classify | none |'
    serial_gates = '\n'.join(
        f"| ordered-issue-{idx} | later ordered child issues | `#{issue}` closes out truthfully before the next issue starts unless the SEP is updated with a safe parallel lane. | sprint-conductor |"
        for idx, issue in enumerate(ordered, start=1)
    )
    if not serial_gates:
        serial_gates = '| no-child-issues | none | no child issues declared | sprint-conductor |'
    shared_docs = [
        str((repo_root / 'docs' / 'templates' / 'sprints' / 'current.json').relative_to(repo_root)),
        str((repo_root / 'docs' / 'templates' / 'sprints' / '1.0.0' / 'sprint_execution_packet.md').relative_to(repo_root)),
    ]
    lines = [
        f'# Sprint Execution Packet: #{sprint_issue_number} {title}',
        '',
        '## Metadata',
        '',
        f'- Sprint issue: `#{sprint_issue_number}`',
        f'- Sprint title: `{title}`',
        f'- Milestone: `{version}`',
        f'- Execution mode: `{execution_mode}`',
        '- Owner: `sprint-conductor`',
        f'- Last updated: `{utc_now()}`',
        '',
        '## Sprint Goal',
        '',
        goal,
        '',
        '## Sprint Boundary',
        '',
        'In scope:',
        '',
        '- Create the sprint-management issue and local bundle through repo-native issue operations.',
        '- Materialize a retained Sprint Execution Packet with watcher, review, and closeout expectations.',
        '',
        'Out of scope:',
        '',
        '- Executing the child issues directly from this helper.',
        '- Claiming safe parallelism before write-set and PVF coordination is proven.',
        '',
        '## Child Issue Wave',
        '',
        '| Issue | Role | Status | Primary surface | Notes |',
        '|---|---|---|---|---|',
        ordered_issue_lines or '| none | none | pending | not_applicable | no child issues declared |',
        '',
        '## Dependency Graph',
        '',
        '```mermaid',
        'flowchart LR',
    ]
    if ordered:
        for idx, issue in enumerate(ordered):
            node = f'N{idx + 1}'
            lines.append(f'  {node}["#{issue}"]')
            if idx > 0:
                prev = f'N{idx}'
                lines.append(f'  {prev} --> {node}')
    else:
        lines.append('  A["No child issues declared"]')
    lines.extend(
        [
            '```',
            '',
            '## Recommended Execution Order',
            '',
            execution_order,
            '',
            '## Issue Lifecycle Policy',
            '',
            '- Each child issue must end in one explicit terminal state: `closed_after_merge`, `closed_no_merge`, `deferred_with_route`, or `failed_with_route`.',
            '- Completed child issues must close as soon as review, merge outcome, closeout, and worktree pruning are complete.',
            '- Every child issue closeout should be owned by an explicit closeout handoff or closeout agent.',
            '',
            '## Watcher Policy',
            '',
            '- Every active child issue must have a watcher or equivalent lifecycle monitor for readiness, implementation, PR checks, review, merge, closeout, and worktree pruning.',
            '- Watchers must classify `complete`, `failed`, `blocked`, or `waiting_with_next_check`.',
            '- Wait states without a watcher are not valid sprint state.',
            '',
            '## Safe Parallel Lanes',
            '',
            '| Lane | Issues | Why parallel-safe | Required coordination |',
            '|---|---|---|---|',
            safe_parallel,
            '',
            '## Candidate Parallel Lanes',
            '',
            '| Lane | Classification | Issues | Expected write sets | Expected PVF lanes | Validation lanes | Dependency gates | Collision risks | Watcher | Subagent | Why safe or why not | Required coordination |',
            '|---|---|---|---|---|---|---|---|---|---|---|---|',
            candidate_parallel,
            '',
            '## Serial Gates',
            '',
            '| Gate | Blocks | Exit condition | Owner |',
            '|---|---|---|---|',
            serial_gates,
            '',
            '## PVF / Validation-Tail Notes',
            '',
            '- Immediate issue-local proof: `Use the smallest proving lane per child issue; this helper does not widen proof automatically.`',
            '- Parallel validation lanes: `not_recorded_yet`',
            '- Serial validation gates: `child closeout and review truth remain mandatory before advancing the ordered wave.`',
            '- Reusable proof criteria: `only when the touched surface and declared lane truly match.`',
            '- Fail-closed rule: `missing lane truth, missing watcher truth, or missing closeout truth blocks sprint advancement.`',
            '',
            '## Parallelism Outcome Plan',
            '',
            f'- Planned summary: `Sprint bootstrap starts in {execution_mode} mode; actual achieved parallelism must be recorded during execution.`',
            '- Actual summary placeholder: `Fill this during closeout once actual concurrency is known.`',
            '- Prediction-miss capture rule: `Record any lane that turned out not to be safe and why.`',
            '- Closeout requirement: record every lane that proved safe, every lane that stayed serial, and every lane prediction that was wrong.',
            '',
            '## Sprint Activity Log',
            '',
            f'- Log artifact path: `{activity_log.relative_to(repo_root)}`',
            '- Required events: issue start, card repair, worktree bind, PR publication, watcher state, validation result, review result, merge/closeout, and worktree prune.',
            '- Log policy: `append-only retained sprint activity log with issue/PR/check state transitions.`',
            '',
            '## Sprint-Level Review',
            '',
            f'- Sprint review artifact: `{review_path.relative_to(repo_root)}`',
            '- Review scope: child issues, PRs, changed files, logs, validation proof, closeout truth, residual routing, and failed/deferred lanes.',
            '- All actionable sprint-level findings must be fixed, routed, or explicitly accepted before the sprint umbrella closes.',
            '',
            '## Subagent / Local Model Policy',
            '',
            '- Subagent strategy: `bounded reviewer or watcher subagents only when policy explicitly enables them.`',
            '- Local model candidates: `not_recorded_yet`',
            '- Simple delegated roles: watcher, card validator, docs lint reviewer, closeout checker, issue-state summarizer.',
            '- Local agents must produce bounded, reviewable output and must not mutate repo state unless an issue explicitly grants that authority.',
            '',
            '## Template/AST Policy',
            '',
            '- SEP templates should be maintained through the same AST-backed template path as other C-SDLC templates once the markdown.rs editor lane is available.',
            '- Direct Markdown edits are acceptable only as a temporary bootstrap surface until the AST-backed SEP renderer is implemented and proven.',
            '',
            '## Shared Inputs And Artifacts',
            '',
            f"- Shared source docs: `{', '.join(shared_docs)}`",
            '- Shared code surfaces: `adl/tools/skills/sprint-conductor/*`',
            '- Shared review packets: `not_recorded_yet`',
            '- Shared logs or observability surfaces: `issue/PR watcher outputs and sprint activity log.`',
            '',
            '## Cross-Sprint Dependencies',
            '',
            '- Upstream dependencies: `child issue cards must exist and be ready before execution begins.`',
            '- Downstream consumers: `sprint review, closeout artifact, and child issue execution sessions.`',
            '- Collision risks: `shared workflow-control files until lanes are explicitly classified.`',
            '- Routing rule: `do not widen scope silently; route follow-ons instead.`',
            '',
            '## Review Bar',
            '',
            '- Review scope: `code, docs, validation proof, watcher state, and closeout truth for the sprint wave.`',
            '- Required review skills: `repo-packet-builder`, `repo-review-code`, `repo-review-tests`, and synthesis; add docs/security review when the touched surface warrants it.',
            '- Code-facing review required: `true`',
            '- Docs-facing review required: `true`',
            '- Security review required: `false unless the sprint changes trust boundaries or credential flows.`',
            '',
            '## Closeout Bar',
            '',
            '- Every child issue is closed or explicitly deferred with rationale.',
            '- Every child PR is merged, closed without merge, or routed with truthful state.',
            '- Sprint review findings are either fixed, routed, or recorded as residual risk.',
            '- Sprint closeout artifact records child issue status, PR URLs, proof surfaces, validation state, and follow-up routing.',
            '- Worktrees are pruned or retained with an explicit reason.',
            '',
            '## Residual Routing Policy',
            '',
            '- Must-fix-before-sprint-close: `issue-local closeout truth, watcher coverage, and sprint review completion.`',
            '- Post-sprint follow-ons: `default disposition is post_sprint_follow_on unless the sprint policy says otherwise.`',
            '- Deferred work: `must be recorded explicitly rather than left ambient.`',
            '- Explicit non-blockers: `descriptive sprint objective and planning notes are not substitutes for child issue execution proof.`',
            '',
            '## Non-Claims',
            '',
            '- This bootstrap helper does not execute child issues or prove safe parallelism by itself.',
            '- Creating the sprint issue and SEP does not replace child issue review, PR publication, or closeout truth.',
        ]
    )
    if notes:
        lines.extend(['', '## Notes', '', notes.strip()])
    return '\n'.join(lines).rstrip() + '\n'


def build_body(goal: str, ordered: list[int], child_titles: dict[int, str], execution_mode: str, notes: str | None) -> str:
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
- Selected execution mode for this sprint-management issue: `{execution_mode}`.
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
    parser.add_argument('--execution-mode', choices=['sequential', 'parallel', 'hybrid'], default='sequential')
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
        issue_json = issue_view(issue)
        child_titles[issue] = issue_json.get('title', '')

    body = build_body(args.goal, ordered, child_titles, args.execution_mode, args.notes)
    with tempfile.NamedTemporaryFile('w', delete=False, suffix='.md') as handle:
        handle.write(body)
        body_path = Path(handle.name)

    created_issue = issue_create(args.title, body_path)
    sprint_issue_number = int(created_issue['number'])
    issue_url = str(created_issue['url'])
    local_bundle = bootstrap_local_bundle(repo_root, sprint_issue_number, args.title, issue_url, body)
    packet_path = sprint_execution_packet_path(
        repo_root,
        local_bundle['version'],
        sprint_issue_number,
        local_bundle['slug'],
    )
    write_file(
        packet_path,
        render_execution_packet(
            repo_root,
            local_bundle['version'],
            sprint_issue_number,
            args.title,
            args.goal,
            ordered,
            child_titles,
            args.execution_mode,
            args.notes,
        ),
    )
    review_path = sprint_review_path(repo_root, local_bundle['version'], sprint_issue_number)
    activity_log_path = sprint_activity_log_path(repo_root, local_bundle['version'], sprint_issue_number)
    closeout_artifact_path = sprint_closeout_artifact_path(repo_root, local_bundle['version'], sprint_issue_number)
    closeout_note_path = sprint_closeout_note_path(
        repo_root,
        local_bundle['version'],
        sprint_issue_number,
        local_bundle['slug'],
    )

    result = {
        'created': True,
        'sprint_issue_number': sprint_issue_number,
        'sprint_issue_url': issue_url,
        'ordered_issue_numbers': ordered,
        'execution_mode': args.execution_mode,
        'execution_packet_path': str(packet_path),
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
            'execution_mode': args.execution_mode,
            'execution_packet_path': str(packet_path),
            'follow_up_issue_policy': 'post_sprint_follow_on',
            'current_issue_number': ordered[0] if ordered else None,
            'completed_issue_numbers': [],
            'blocked_issue_number': None,
            'deferred_issue_numbers': [],
            'continuation': 'continue',
            'goal_policy': default_goal_policy(),
            'local_bundle': local_bundle,
            'issue_records': default_issue_records(ordered),
            'follow_up_issues': [],
            'structured_prompt_preflight': {
                'status': 'not_run',
                'required_card_types': ['stp.md', 'sip.md', 'sor.md', 'spp.md', 'srp.md'],
                'issue_results': [],
                'notes': [
                    'Run sprint-wide structured prompt preflight before starting issue execution, including SPP and SRP design-time readiness.',
                ],
            },
            'readiness_sweep': {
                'status': 'not_run',
                'ordered_issue_numbers': ordered,
                'execution_mode': args.execution_mode,
                'goal_policy': default_goal_policy(),
                'execution_packet': {
                    'status': 'present',
                    'path': str(packet_path),
                    'missing_sections': [],
                    'notes': [f'execution packet bootstrapped by helper: {packet_path}'],
                },
                'review_paths': {
                    'status': 'declared',
                    'paths': [str(review_path)],
                    'missing_paths': [str(review_path)],
                    'notes': ['review artifact path declared; file creation is expected later during sprint review.'],
                },
                'activity_log_paths': {
                    'status': 'declared',
                    'paths': [str(activity_log_path)],
                    'missing_paths': [str(activity_log_path)],
                    'notes': ['activity log path declared; file creation is expected later during sprint execution.'],
                },
                'issue_repairs': [],
                'notes': ['Run the full readiness sweep before starting child execution; helper declarations are bootstrap truth only.'],
            },
            'truth_check': {
                'status': 'not_run',
                'source': 'sprint_state_only',
                'gate_passed': False,
                'checked_issue_numbers': [],
                'checked_pr_urls': [],
                'notes': ['Sprint issue created by skill; run live GitHub truth check before the first state transition.'],
            },
            'current_state': {
                'selected_skill': 'workflow-conductor',
                'current_phase': 'intake',
                'blocker_reason': 'none',
            },
            'review': {
                'status': 'not_started',
                'selected_skills': [],
                'review_subagent_ids': [],
                'packet_path': str(review_path),
                'code_review_path': None,
                'test_review_path': None,
                'docs_review_path': None,
                'security_review_path': None,
                'synthesis_path': None,
                'findings_summary': {
                    'confirmed_findings': None,
                    'unresolved_questions': None,
                },
            },
            'closeout': {
                'status': 'not_started',
                'readiness': 'unknown',
                'closeout_note_path': str(closeout_note_path),
                'closeout_artifact_path': str(closeout_artifact_path),
                'sprint_issue_close_summary': None,
                'planned_vs_actual_parallelism': {
                    'planned_summary': f'Bootstrap declared execution mode `{args.execution_mode}`; actual achieved parallelism is not recorded yet.',
                    'actual_summary': None,
                    'prediction_misses': [],
                },
            },
            'actions_taken': [
                'Created sprint-management issue through the repo-native issue command surface.',
                'Bootstrapped the local sprint task bundle through the canonical issue init path.',
                'Materialized a retained Sprint Execution Packet with watcher, review, and closeout declaration surfaces.',
            ],
            'next_handoff': {
                'status': 'continue',
                'target_issue_number': ordered[0] if ordered else None,
                'target_pr_url': None,
                'next_skill': 'workflow-conductor',
                'child_session_goal': {
                    'required': bool(ordered),
                    'create_after_bind': bool(ordered),
                    'sprint_issue_number': sprint_issue_number,
                    'child_issue_number': ordered[0] if ordered else None,
                    'bounded_objective': 'Start the first ordered child issue only after readiness and bind succeed.' if ordered else None,
                },
                'rationale': 'Sprint-management issue and SEP bootstrap are complete; continue through workflow-conductor and the readiness sweep before child execution.',
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
