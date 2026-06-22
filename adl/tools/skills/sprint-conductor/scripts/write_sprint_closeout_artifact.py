#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

from issue_goal_metrics import compute_goal_metrics_rollup, default_goal_metrics_summary


def closure_cleanliness(state: dict) -> str:
    records = {record.get('issue_number'): record for record in state.get('issue_records', [])}
    for issue in state.get('ordered_issue_numbers', []):
        record = records.get(issue, {})
        if record.get('status') != 'closed_out':
            return 'residual_debt'
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
    goal_metrics_rollup = compute_goal_metrics_rollup(state.get('issue_records', []))

    lines = [
        '# Sprint Closeout Artifact',
        '',
        f"- sprint issue: `#{state.get('sprint_issue_number')}`",
        f"- ordered issues: `{', '.join(str(i) for i in state.get('ordered_issue_numbers', []))}`",
        f"- execution mode: `{state.get('execution_mode', 'not_recorded')}`",
        f"- closure cleanliness: `{cleanliness}`",
        '',
        '## Ordered Child Issues',
        '',
    ]
    for issue in state.get('ordered_issue_numbers', []):
        record = records.get(issue, {})
        lines.append(f"- `#{issue}` status=`{record.get('status', 'unknown')}` pr=`{record.get('pr_url') or 'not_applicable'}`")
        artifact_paths = record.get('artifact_paths', [])
        if artifact_paths:
            for artifact_path in artifact_paths:
                lines.append(f"  - artifact: `{artifact_path}`")
        closeout_gate = record.get('closeout_gate') or {}
        if closeout_gate:
            gate_bits = [
                f"issue_closed={closeout_gate.get('issue_closed', 'unknown')}",
                f"pr_state={closeout_gate.get('pr_state', 'unknown')}",
                f"root_sor_status={closeout_gate.get('root_sor_status', 'unknown')}",
                f"worktree_status={closeout_gate.get('worktree_status', 'unknown')}",
            ]
            lines.append(f"  - closeout gate: `{', '.join(gate_bits)}`")
            worktree_note = closeout_gate.get('worktree_note')
            if worktree_note:
                lines.append(f"  - worktree note: {worktree_note}")
        goal_metrics = record.get('goal_metrics') or default_goal_metrics_summary()
        lines.append(
            "  - goal metrics: "
            f"`status={goal_metrics.get('status', 'not_recorded')}, "
            f"stage={goal_metrics.get('selected_stage') or 'not_recorded'}, "
            f"goal_id={goal_metrics.get('goal_id') or goal_metrics.get('goal_id_availability', 'unknown')}, "
            f"elapsed={goal_metrics.get('elapsed_seconds') if goal_metrics.get('elapsed_seconds') is not None else goal_metrics.get('elapsed_availability', 'unknown')}, "
            f"total_tokens={goal_metrics.get('token_usage', {}).get('total_tokens') if goal_metrics.get('token_usage', {}).get('total_tokens') is not None else goal_metrics.get('token_usage', {}).get('total_availability', goal_metrics.get('token_usage', {}).get('availability', 'unknown'))}, "
            f"source={goal_metrics.get('data_source') or 'unknown'}`"
        )
        lines.append(
            "  - goal refs: "
            f"`issue={goal_metrics.get('issue_goal_ref') or 'unknown'}, "
            f"sprint={goal_metrics.get('sprint_goal_ref') or 'unknown'}, "
            f"rollup={goal_metrics.get('goal_metrics_rollup_ref') or 'unknown'}`"
        )
        lines.append(
            "  - goal timing buckets: "
            f"`active_work={goal_metrics.get('active_work_seconds') if goal_metrics.get('active_work_seconds') is not None else goal_metrics.get('active_work_availability', 'unknown')}, "
            f"validation={goal_metrics.get('validation_seconds') if goal_metrics.get('validation_seconds') is not None else goal_metrics.get('validation_availability', 'unknown')}, "
            f"pr_wait={goal_metrics.get('pr_wait_seconds') if goal_metrics.get('pr_wait_seconds') is not None else goal_metrics.get('pr_wait_availability', 'unknown')}, "
            f"ci_wait={goal_metrics.get('ci_wait_seconds') if goal_metrics.get('ci_wait_seconds') is not None else goal_metrics.get('ci_wait_availability', 'unknown')}`"
        )
        lines.append(
            "  - goal completion truth: "
            f"`completion_state={goal_metrics.get('completion_state') or 'unknown'}, "
            f"metrics_confidence={goal_metrics.get('metrics_confidence') or 'unknown'}`"
        )
        if goal_metrics.get('raw_log_path'):
            lines.append(f"  - goal metrics log: `{goal_metrics['raw_log_path']}`")

    lines.extend(['', '## Follow-up Issues', ''])
    follow_ups = state.get('follow_up_issues', [])
    if follow_ups:
        for item in follow_ups:
            lines.append(f"- `#{item.get('issue_number')}` disposition=`{item.get('disposition')}` summary={item.get('summary')}")
    else:
        lines.append('- none')

    lines.extend(['', '## Review / Closeout Surfaces', ''])
    closeout = state.get('closeout') or {}
    validation = state.get('validation') or closeout.get('validation') or {}
    coverage = closeout.get('coverage') or state.get('coverage') or {}
    rust_tracker = closeout.get('rust_tracker') or state.get('rust_tracker') or {}
    lines.append(f"- review packet: `{state.get('review', {}).get('packet_path') or 'not_recorded'}`")
    lines.append(f"- review findings summary: `{state.get('review', {}).get('findings_summary') or 'not_recorded'}`")
    lines.append(f"- validation state: `{validation.get('status') or 'not_recorded'}`")
    lines.append(f"- coverage: `source={coverage.get('source') or 'missing'}, summary={coverage.get('summary') or 'not_recorded'}`")
    lines.append(
        "- rust tracker: "
        f"`source={rust_tracker.get('source') or 'missing'}, "
        f"watch_count={rust_tracker.get('watch_count', 'not_recorded')}, "
        f"review_count={rust_tracker.get('review_count', 'not_recorded')}, "
        f"rationale_count={rust_tracker.get('rationale_count', 'not_recorded')}`"
    )
    lines.append(f"- sprint close summary: `{closeout.get('sprint_issue_close_summary') or state.get('sprint_issue_close_summary') or 'not_recorded'}`")

    parallelism = closeout.get('planned_vs_actual_parallelism') or state.get('planned_vs_actual_parallelism') or {}
    lines.extend(['', '## Planned Vs Actual Parallelism', ''])
    lines.append(f"- planned summary: `{parallelism.get('planned_summary') or 'not_recorded'}`")
    lines.append(f"- actual summary: `{parallelism.get('actual_summary') or 'not_recorded'}`")
    prediction_misses = parallelism.get('prediction_misses') or []
    if prediction_misses:
        lines.append('- prediction misses:')
        for item in prediction_misses:
            lane_id = item.get('lane_id') or 'unknown'
            issues = ', '.join(str(issue) for issue in item.get('issue_numbers', [])) or 'none'
            why_wrong = item.get('why_wrong') or 'not_recorded'
            corrective_action = item.get('corrective_action') or 'not_recorded'
            lines.append(
                f"  - lane=`{lane_id}` issues=`{issues}` why_wrong={why_wrong} corrective_action={corrective_action}"
            )
    else:
        lines.append('- prediction misses: none recorded')

    lines.extend(['', '## Goal Metrics Rollup', ''])
    lines.append(f"- issues with recorded metrics: `{goal_metrics_rollup['issues_with_recorded_metrics']}/{goal_metrics_rollup['issue_count']}`")
    lines.append(f"- data sources: `{goal_metrics_rollup['data_source_counts']}`")
    lines.append(f"- goal-id availability: `{goal_metrics_rollup['goal_id_availability_counts']}`")
    lines.append(f"- completion states: `{goal_metrics_rollup['completion_state_counts']}`")
    lines.append(
        f"- elapsed seconds: `known_sum={goal_metrics_rollup['total_elapsed_seconds_known_sum']}, "
        f"known_issue_count={goal_metrics_rollup['issues_with_known_elapsed']}, "
        f"unknown_issue_count={goal_metrics_rollup['issues_with_unknown_elapsed']}, "
        f"availability_counts={goal_metrics_rollup['elapsed_availability_counts']}`"
    )
    lines.append(
        f"- active work seconds: `known_sum={goal_metrics_rollup['total_active_work_seconds_known_sum']}, "
        f"known_issue_count={goal_metrics_rollup['issues_with_known_active_work']}, "
        f"unknown_issue_count={goal_metrics_rollup['issues_with_unknown_active_work']}, "
        f"availability_counts={goal_metrics_rollup['active_work_availability_counts']}`"
    )
    lines.append(
        f"- validation seconds: `known_sum={goal_metrics_rollup['total_validation_seconds_known_sum']}, "
        f"known_issue_count={goal_metrics_rollup['issues_with_known_validation_seconds']}, "
        f"unknown_issue_count={goal_metrics_rollup['issues_with_unknown_validation_seconds']}, "
        f"availability_counts={goal_metrics_rollup['validation_availability_counts']}`"
    )
    lines.append(
        f"- pr wait seconds: `known_sum={goal_metrics_rollup['total_pr_wait_seconds_known_sum']}, "
        f"known_issue_count={goal_metrics_rollup['issues_with_known_pr_wait']}, "
        f"unknown_issue_count={goal_metrics_rollup['issues_with_unknown_pr_wait']}, "
        f"availability_counts={goal_metrics_rollup['pr_wait_availability_counts']}`"
    )
    lines.append(
        f"- ci wait seconds: `known_sum={goal_metrics_rollup['total_ci_wait_seconds_known_sum']}, "
        f"known_issue_count={goal_metrics_rollup['issues_with_known_ci_wait']}, "
        f"unknown_issue_count={goal_metrics_rollup['issues_with_unknown_ci_wait']}, "
        f"availability_counts={goal_metrics_rollup['ci_wait_availability_counts']}`"
    )
    lines.append(
        f"- total tokens: `known_sum={goal_metrics_rollup['total_tokens_known_sum']}, "
        f"known_issue_count={goal_metrics_rollup['issues_with_known_total_tokens']}, "
        f"unknown_issue_count={goal_metrics_rollup['issues_with_unknown_total_tokens']}, "
        f"availability_counts={goal_metrics_rollup['total_token_availability_counts']}`"
    )

    out_path.write_text('\n'.join(lines).rstrip() + '\n', encoding='utf-8')

    state.setdefault('closeout', {})
    state['closeout']['closeout_artifact_path'] = str(out_path)
    state['closeout']['closure_cleanliness'] = cleanliness
    state['closeout']['goal_metrics_rollup'] = goal_metrics_rollup
    state_path.write_text(json.dumps(state, indent=2, sort_keys=True) + '\n')

    result = {
        'closeout_artifact_path': str(out_path),
        'closure_cleanliness': cleanliness,
        'goal_metrics_rollup': goal_metrics_rollup,
    }
    if args.print_json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        print(out_path)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
