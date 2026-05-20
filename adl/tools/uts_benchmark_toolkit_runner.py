#!/usr/bin/env python3
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from contextlib import nullcontext
from pathlib import Path

CURRENT_DIR = Path(__file__).resolve().parent
BENCHMARK_DIR = CURRENT_DIR / 'benchmark'
REPO_ROOT = CURRENT_DIR.parents[1]
if str(BENCHMARK_DIR) not in sys.path:
    sys.path.insert(0, str(BENCHMARK_DIR))

from portable_benchmark_common import summary_path_for, verbose_path_for, ensure_local_model_loaded, unload_local_model, local_model_execution_lock
from regular_tool_call_baseline import run_candidate as run_regular_candidate
from uts_only_toolkit_runner import run_candidate as run_uts_only_candidate
from deterministic_self_check import run_deterministic_self_check, self_check_path_for
from uts_benchmark_panel import default_panel_path, load_panel, select_models, display_path, host_policy_note
from uts_benchmark_tasks import default_task_panel_path, display_task_panel_path, panel_tasks
from uts_acc_governed_toolkit_runner import _populate_hosted_env, hosted_adapter

GOVERNED_MODEL_TIMEOUT_SECONDS = int(os.getenv('ADL_UTS_GOVERNED_MODEL_TIMEOUT_SECONDS', '300'))
GOVERNED_PROVIDER_TIMEOUT_SECONDS = int(os.getenv('ADL_UTS_GOVERNED_PROVIDER_TIMEOUT_SECONDS', '20'))


def run_subprocess(command, env=None, timeout=None):
    subprocess.run(command, check=True, env=env, timeout=timeout)


def load_json(path: Path):
    return json.loads(path.read_text(encoding='utf-8'))


def safe_name(value: str) -> str:
    return ''.join(ch if ch.isalnum() or ch in ('-', '_', '.') else '_' for ch in value)


def append_runner_log(path: Path, message: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    import time
    timestamp = time.strftime('%Y-%m-%dT%H:%M:%S%z')
    with path.open('a', encoding='utf-8') as handle:
        handle.write(f'[{timestamp}] {message}\n')


def summarize_lane_result(result, key_name):
    return {
        'status': 'evaluated',
        'passed_count': result['passed_count'],
        'total_cases': result['total_cases'],
        'full_support': result.get(key_name, False),
        'cases': result['cases'],
    }


def skipped_lane_result(note: str):
    return {
        'status': 'skipped',
        'passed_count': 0,
        'total_cases': 0,
        'full_support': False,
        'cases': [],
        'note': note,
    }


def lane_stats(lane_result):
    if not lane_result:
        return None, None
    values = [case.get('duration_ms') for case in lane_result.get('cases', []) if case.get('duration_ms') is not None]
    if not values:
        return None, None
    return int(sum(values) / len(values)), int(sum(values))


def lane_status(lane_result):
    if lane_result is None:
        return 'not_run'
    if lane_result.get('status'):
        return lane_result['status']
    if lane_result.get('provider_failure_kind'):
        return 'provider_failure'
    if lane_result.get('note') and lane_result.get('total_cases', 0) == 0:
        return 'skipped'
    if lane_result.get('passed_count') == lane_result.get('total_cases') and lane_result.get('total_cases', 0) > 0:
        return 'full_support'
    return 'partial'


def write_auxiliary_artifacts(report, out_path: Path):
    failures = {
        'schema_version': 'uts_benchmark_failures.v1',
        'source_results': str(out_path),
        'models': [],
    }
    provider_status = {
        'schema_version': 'uts_benchmark_provider_status.v1',
        'source_results': str(out_path),
        'models': [],
    }
    for model in report['models']:
        model_failures = []
        provider_events = []
        lane_statuses = {}
        for lane_name, lane_result in model['lanes'].items():
            if lane_result is None:
                lane_statuses[lane_name] = {
                    'status': 'not_run',
                    'provider_failure_kind': None,
                    'note': None,
                }
                continue
            lane_statuses[lane_name] = {
                'status': lane_status(lane_result),
                'provider_failure_kind': lane_result.get('provider_failure_kind'),
                'note': lane_result.get('note'),
            }
            if lane_result.get('provider_failure_kind') or lane_result.get('note'):
                provider_events.append({
                    'lane': lane_name,
                    'status': lane_status(lane_result),
                    'provider_failure_kind': lane_result.get('provider_failure_kind'),
                    'note': lane_result.get('note'),
                })
            for case in lane_result.get('cases', []):
                if not case.get('passed', False):
                    model_failures.append({
                        'lane': lane_name,
                        'task_id': case.get('task_id'),
                        'classification': case.get('classification'),
                        'note': case.get('note'),
                        'duration_ms': case.get('duration_ms'),
                    })
        failures['models'].append({
            'candidate_id': model['candidate_id'],
            'failures': model_failures,
        })
        provider_status['models'].append({
            'candidate_id': model['candidate_id'],
            'provider': model['provider'],
            'lane_status': lane_statuses,
            'provider_events': provider_events,
        })
    failures_path = out_path.with_name(f'{out_path.stem}_failures.json')
    provider_status_path = out_path.with_name(f'{out_path.stem}_provider_status.json')
    failures_path.write_text(json.dumps(failures, indent=2) + '\n', encoding='utf-8')
    provider_status_path.write_text(json.dumps(provider_status, indent=2) + '\n', encoding='utf-8')
    return failures_path, provider_status_path


def write_reports(report, out_path: Path):
    summary_lines = [
        '# Unified UTS Benchmark Summary',
        '',
        '## Executive Summary',
        '',
        f"- Models evaluated: `{len(report['models'])}`",
        f"- Governed lane included: `{str(report['include_governed']).lower()}`",
        f"- Model panel: `{report['selection']['panel_file']}`",
        f"- Task panel: `{report['selection']['task_panel_file']}`",
        '',
        '## Overview Table',
        '',
        '| Model | Tier | Provider | Regular | UTS-only | UTS+ACC | Regular avg ms | UTS avg ms | Governed avg ms |',
        '|---|---|---|---:|---:|---:|---:|---:|---:|',
    ]
    for model in report['models']:
        reg = f"{model['lanes']['regular']['passed_count']}/{model['lanes']['regular']['total_cases']}"
        uts = f"{model['lanes']['uts_only']['passed_count']}/{model['lanes']['uts_only']['total_cases']}"
        governed = model['lanes'].get('uts_acc')
        gov_text = 'n/a' if governed is None else ('skipped' if governed.get('note') and governed['total_cases'] == 0 else f"{governed['passed_count']}/{governed['total_cases']}")
        reg_avg, _ = lane_stats(model['lanes']['regular'])
        uts_avg, _ = lane_stats(model['lanes']['uts_only'])
        gov_avg, _ = lane_stats(governed)
        summary_lines.append(
            f"| `{model['candidate_id']}` | `{model['tier']}` | `{model['provider']}` | `{reg}` | `{uts}` | `{gov_text}` | `{reg_avg if reg_avg is not None else 'n/a'}` | `{uts_avg if uts_avg is not None else 'n/a'}` | `{gov_avg if gov_avg is not None else 'n/a'}` |"
        )
    summary_lines.extend([
        '',
        'This summary is intentionally compact for comparison and presentation use.',
    ])
    summary_path = summary_path_for(out_path)
    summary_path.write_text('\n'.join(summary_lines) + '\n', encoding='utf-8')

    verbose_lines = [
        '# Unified UTS Benchmark Verbose Report',
        '',
        '## Executive Summary',
        '',
        f"- Models evaluated: `{len(report['models'])}`",
        f"- Governed lane included: `{str(report['include_governed']).lower()}`",
        f"- Model panel: `{report['selection']['panel_file']}`",
        f"- Task panel: `{report['selection']['task_panel_file']}`",
        '',
        '## Overview Table',
        '',
        '| Model | Tier | Provider | Regular | UTS-only | UTS+ACC | Regular avg ms | UTS avg ms | Governed avg ms |',
        '|---|---|---|---:|---:|---:|---:|---:|---:|',
    ]
    for model in report['models']:
        reg = f"{model['lanes']['regular']['passed_count']}/{model['lanes']['regular']['total_cases']}"
        uts = f"{model['lanes']['uts_only']['passed_count']}/{model['lanes']['uts_only']['total_cases']}"
        governed = model['lanes'].get('uts_acc')
        gov_text = 'n/a' if governed is None else ('skipped' if governed.get('note') and governed['total_cases'] == 0 else f"{governed['passed_count']}/{governed['total_cases']}")
        reg_avg, _ = lane_stats(model['lanes']['regular'])
        uts_avg, _ = lane_stats(model['lanes']['uts_only'])
        gov_avg, _ = lane_stats(governed)
        verbose_lines.append(
            f"| `{model['candidate_id']}` | `{model['tier']}` | `{model['provider']}` | `{reg}` | `{uts}` | `{gov_text}` | `{reg_avg if reg_avg is not None else 'n/a'}` | `{uts_avg if uts_avg is not None else 'n/a'}` | `{gov_avg if gov_avg is not None else 'n/a'}` |"
        )
    verbose_lines.append('')
    for model in report['models']:
        verbose_lines.extend([
            f"## {model['candidate_id']}",
            '',
            f"- Tier: `{model['tier']}`",
            f"- Provider: `{model['provider']}`",
            f"- Runtime model id: `{model['model_id']}`",
            '',
        ])
        for lane_name, lane_result in model['lanes'].items():
            if lane_result is None:
                continue
            avg_ms, total_ms = lane_stats(lane_result)
            verbose_lines.extend([
                f"### {lane_name}",
                '',
                f"- Passed: `{lane_result['passed_count']}` / `{lane_result['total_cases']}`",
                f"- Full support: `{str(lane_result.get('full_support', False)).lower()}`",
                f"- Average duration per test: `{avg_ms if avg_ms is not None else 'n/a'}` ms",
                f"- Total known duration: `{total_ms if total_ms is not None else 'n/a'}` ms",
            ])
            lane_note = lane_result.get('note')
            if lane_note:
                verbose_lines.append(f"- Note: {lane_note}")
            verbose_lines.extend([
                '',
                '| Task | Classification | Passed | Duration ms | Note |',
                '|---|---|---:|---:|---|',
            ])
            for case in lane_result['cases']:
                duration = case.get('duration_ms')
                duration_text = str(duration) if duration is not None else 'n/a'
                note = str(case.get('note', '')).replace('|', '/').replace('\n', ' ')
                verbose_lines.append(
                    f"| `{case['task_id']}` | `{case['classification']}` | `{str(case['passed']).lower()}` | `{duration_text}` | {note} |"
                )
            verbose_lines.append('')
    verbose_path = verbose_path_for(out_path)
    verbose_path.write_text('\n'.join(verbose_lines) + '\n', encoding='utf-8')


def load_existing_report(path: Path):
    if not path.is_file():
        return None
    try:
        report = json.loads(path.read_text(encoding='utf-8'))
    except json.JSONDecodeError:
        return None
    if report.get('schema_version') != 'uts_benchmark_toolkit_runner.v1':
        return None
    return report


def load_profile(path: str | None):
    if not path:
        return {}
    return load_json(Path(path))


def load_models_file(path: str | None):
    if not path:
        return []
    models = []
    for raw_line in Path(path).read_text(encoding='utf-8').splitlines():
        line = raw_line.strip()
        if not line or line.startswith('#'):
            continue
        models.append(line)
    return models


def resolve_config_path(path_value: str | None, profile_file: str | None):
    if not path_value:
        return path_value
    candidate = Path(path_value)
    if candidate.is_absolute():
        return str(candidate)
    if profile_file:
        profile_dir_candidate = Path(profile_file).resolve().parent / candidate
        if profile_dir_candidate.exists():
            return str(profile_dir_candidate)
    repo_candidate = REPO_ROOT / candidate
    return str(repo_candidate)


def index_existing_models(report):
    return {model['candidate_id']: model for model in report.get('models', [])}


def invalid_lane_result(note: str, failure_kind: str):
    return {
        'status': 'provider_failed',
        'passed_count': 0,
        'total_cases': 0,
        'full_support': False,
        'cases': [],
        'note': note,
        'provider_failure_kind': failure_kind,
    }


def governed_backend_unavailable_note():
    if shutil.which('cargo') is None:
        return 'governed lane skipped: optional Rust cargo backend is not installed'
    if not (CURRENT_DIR.parent / 'Cargo.toml').is_file():
        return 'governed lane skipped: optional Rust cargo backend manifest is missing'
    return None


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--profile-file')
    parser.add_argument('--models-file')
    parser.add_argument('--panel-file', default=str(default_panel_path()))
    parser.add_argument('--task-panel-file', default=str(default_task_panel_path()))
    parser.add_argument('--model')
    parser.add_argument('--tier')
    parser.add_argument('--provider-kind')
    parser.add_argument('--include-governed', action='store_true')
    parser.add_argument('--skip-self-check', action='store_true')
    parser.add_argument('--no-resume', action='store_true')
    parser.add_argument('--list-models', action='store_true')
    parser.add_argument('--out')
    args = parser.parse_args()

    profile = load_profile(args.profile_file)
    panel_file = resolve_config_path(profile.get('panel_file', args.panel_file), args.profile_file)
    task_panel_file = resolve_config_path(profile.get('task_panel_file', args.task_panel_file), args.profile_file)
    selected_model = args.model if args.model is not None else profile.get('model')
    selected_tier = args.tier if args.tier is not None else profile.get('tier')
    selected_provider_kind = args.provider_kind if args.provider_kind is not None else profile.get('provider_kind')
    include_governed = args.include_governed or bool(profile.get('include_governed', False))
    profile_models = profile.get('models') or []
    models_file = resolve_config_path(args.models_file, args.profile_file)
    listed_models = load_models_file(models_file) if models_file else []
    selected_models = listed_models or profile_models

    panel = load_panel(panel_file)
    tasks = panel_tasks(task_panel_file)
    selected = select_models(panel, model=selected_model, tier=selected_tier, provider_kind=selected_provider_kind)
    if selected_models:
        allowed_ids = set(selected_models)
        selected = [entry for entry in selected if entry.get('id') in allowed_ids]
        selected_ids = {entry.get('id') for entry in selected}
        missing_ids = [model_id for model_id in selected_models if model_id not in selected_ids]
        if missing_ids:
            raise SystemExit(
                'profile references models that are not available in the current panel/selection: '
                + ', '.join(missing_ids)
            )
        selected.sort(key=lambda entry: selected_models.index(entry.get('id')))
    if args.list_models:
        for entry in selected or panel.get('models', []):
            print(f"{entry['id']}\t{entry['tier']}\t{entry['provider_kind']}\t{entry['provider']}")
        return
    if not args.out:
        raise SystemExit('--out is required unless --list-models is used')
    if not selected:
        raise SystemExit('no models matched the requested selection')

    out_path = Path(args.out)
    existing_report = None if args.no_resume else load_existing_report(out_path)
    existing_models = index_existing_models(existing_report) if existing_report else {}
    self_check = run_deterministic_self_check(args.panel_file, args.task_panel_file)
    self_check_out = self_check_path_for(out_path)
    self_check_out.parent.mkdir(parents=True, exist_ok=True)
    self_check_out.write_text(json.dumps(self_check, indent=2) + '\n', encoding='utf-8')
    if not args.skip_self_check and not self_check['passed']:
        raise SystemExit(f"deterministic self-check failed; see {self_check_out}")
    log_dir = out_path.with_name(f'{out_path.stem}_logs')
    log_dir.mkdir(parents=True, exist_ok=True)
    runner_log = log_dir / 'runner.log'

    report = existing_report or {
        'schema_version': 'uts_benchmark_toolkit_runner.v1',
        'selection': {
            'profile_file': display_path(args.profile_file),
            'models_file': display_path(models_file),
            'profile_name': profile.get('name'),
            'model': selected_model,
            'tier': selected_tier,
            'provider_kind': selected_provider_kind,
            'panel_file': display_path(panel_file),
            'task_panel_file': display_task_panel_path(task_panel_file),
            'profile_models': selected_models,
        },
        'include_governed': include_governed,
        'deterministic_self_check': {
            'artifact': str(self_check_out),
            'passed': self_check['passed'],
            'failures': self_check['failures'],
        },
        'artifacts': {
            'runner_log': str(runner_log),
            'log_directory': str(log_dir),
            'per_model_logs': {},
            'governed_progress_logs': {},
        },
        'models': [],
    }
    report['selection'] = {
        'profile_file': display_path(args.profile_file),
        'models_file': display_path(models_file),
        'profile_name': profile.get('name'),
        'model': selected_model,
        'tier': selected_tier,
        'provider_kind': selected_provider_kind,
        'panel_file': display_path(panel_file),
        'task_panel_file': display_task_panel_path(task_panel_file),
        'profile_models': selected_models,
    }
    report['include_governed'] = include_governed
    report['deterministic_self_check'] = {
        'artifact': str(self_check_out),
        'passed': self_check['passed'],
        'failures': self_check['failures'],
    }
    report.setdefault('artifacts', {})
    report['artifacts']['runner_log'] = str(runner_log)
    report['artifacts']['log_directory'] = str(log_dir)
    report['artifacts'].setdefault('per_model_logs', {})
    report['artifacts'].setdefault('governed_progress_logs', {})
    append_runner_log(
        runner_log,
        f"run_start profile={display_path(args.profile_file)} models_file={display_path(models_file)} panel={display_path(panel_file)} task_panel={display_task_panel_path(task_panel_file)} include_governed={str(include_governed).lower()} self_check_passed={str(self_check['passed']).lower()} selected_models={','.join(entry['id'] for entry in selected)} resume={str(not args.no_resume).lower()}",
    )
    governed_unavailable_note = governed_backend_unavailable_note() if include_governed else None
    if governed_unavailable_note:
        append_runner_log(runner_log, governed_unavailable_note)

    with tempfile.TemporaryDirectory(prefix='uts-toolkit-run-') as tmp_dir:
        tmp = Path(tmp_dir)
        for entry in selected:
            existing_model = existing_models.get(entry['id'])
            if existing_model is not None:
                append_runner_log(runner_log, f"model_resume_skip id={entry['id']}")
                continue
            warmed_local = False
            lock_context = local_model_execution_lock() if entry.get('provider_kind') == 'local' else nullcontext()
            with lock_context:
                try:
                    model_log = log_dir / f"{safe_name(entry['id'])}.log"
                    governed_progress_log = log_dir / f"{safe_name(entry['id'])}_uts_acc_progress.log"
                    report['artifacts']['per_model_logs'][entry['id']] = str(model_log)
                    report['artifacts']['governed_progress_logs'][entry['id']] = str(governed_progress_log)
                    os.environ['BENCHMARK_LOG_PATH'] = str(model_log)
                    os.environ['ADL_UTS_ACC_PROGRESS_PATH'] = str(governed_progress_log)
                    append_runner_log(runner_log, f"model_start id={entry['id']} provider_kind={entry.get('provider_kind')} include_governed={str(include_governed).lower()}")
                    blocked_note = host_policy_note(entry) if entry.get('provider_kind') == 'local' else None
                    if blocked_note:
                        append_runner_log(runner_log, f"model_blocked id={entry['id']} note={blocked_note}")
                        report['models'].append({
                            'candidate_id': entry['id'],
                            'tier': entry['tier'],
                            'provider': entry['provider'],
                            'model_id': entry['model_id'],
                            'lanes': {
                                'regular': skipped_lane_result(blocked_note),
                                'uts_only': skipped_lane_result(blocked_note),
                                'uts_acc': skipped_lane_result(blocked_note) if include_governed else None,
                            },
                        })
                        continue
                    if entry.get('provider_kind') == 'local':
                        ensure_local_model_loaded(entry['model_id'])
                        warmed_local = True
                        append_runner_log(runner_log, f"model_loaded id={entry['id']} runtime_model_id={entry['model_id']}")
                    hosted_env = os.environ.copy()
                    if entry.get('provider_kind') == 'hosted':
                        _populate_hosted_env(hosted_env)
                    adapter_context = hosted_adapter(
                        [entry],
                        timeout=int(hosted_env.get('ADL_LIVE_PROVIDER_TIMEOUT_SECS', '240')),
                        env=hosted_env,
                    ) if entry.get('provider_kind') == 'hosted' else nullcontext(None)
                    with adapter_context as adapter:
                        previous_hosted_base_url = os.environ.get('ADL_HOSTED_BASE_URL')
                        if adapter:
                            os.environ['ADL_HOSTED_BASE_URL'] = adapter['host']
                            append_runner_log(runner_log, f"hosted_adapter_start id={entry['id']} host={adapter['host']}")
                        try:
                            regular_result = run_regular_candidate(entry, tasks)
                            append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=regular passed={regular_result['passed_count']}/{regular_result['total_cases']}")
                            uts_result = run_uts_only_candidate(entry, tasks)
                            append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=uts_only passed={uts_result['passed_count']}/{uts_result['total_cases']}")
                        finally:
                            if previous_hosted_base_url is None:
                                os.environ.pop('ADL_HOSTED_BASE_URL', None)
                            else:
                                os.environ['ADL_HOSTED_BASE_URL'] = previous_hosted_base_url
                    governed = None
                    if include_governed:
                        if governed_unavailable_note:
                            governed = {
                                'status': 'skipped',
                                'passed_count': 0,
                                'total_cases': 0,
                                'full_support': False,
                                'cases': [],
                                'note': governed_unavailable_note,
                            }
                            append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=uts_acc skipped=true reason=governed_backend_unavailable")
                        elif entry.get('provider_kind') in {'local', 'hosted'}:
                            governed_out = tmp / 'uts_acc.json'
                            command = [
                                'python3',
                                str(CURRENT_DIR / 'uts_acc_governed_toolkit_runner.py'),
                                '--panel-file', str(panel_file),
                                '--task-panel-file', str(task_panel_file),
                                '--model', entry['id'],
                                '--provider-kind', entry.get('provider_kind', 'local'),
                                '--out', str(governed_out),
                            ]
                            governed_env = os.environ.copy()
                            governed_env['ADL_TIMEOUT_SECS'] = str(GOVERNED_PROVIDER_TIMEOUT_SECONDS)
                            append_runner_log(
                                runner_log,
                                f"lane_start id={entry['id']} lane=uts_acc provider_timeout_s={GOVERNED_PROVIDER_TIMEOUT_SECONDS} model_timeout_s={GOVERNED_MODEL_TIMEOUT_SECONDS}",
                            )
                            try:
                                run_subprocess(command, env=governed_env, timeout=GOVERNED_MODEL_TIMEOUT_SECONDS)
                                governed_doc = load_json(governed_out)
                                first = governed_doc['results'][0] if governed_doc.get('results') else {'passed_count': 0, 'total_cases': 0, 'supports_governed_tool_use': False, 'cases': []}
                                governed = {
                                    'status': 'evaluated',
                                    'passed_count': first['passed_count'],
                                    'total_cases': first['total_cases'],
                                    'full_support': first.get('supports_governed_tool_use', False),
                                    'cases': first['cases'],
                                }
                                if first.get('note'):
                                    governed['note'] = first['note']
                                if first.get('provider_failure_kind'):
                                    governed['provider_failure_kind'] = first['provider_failure_kind']
                                append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=uts_acc passed={governed['passed_count']}/{governed['total_cases']}")
                            except subprocess.TimeoutExpired:
                                governed = invalid_lane_result(
                                    f'provider_timeout: governed lane exceeded {GOVERNED_MODEL_TIMEOUT_SECONDS}s wall-clock budget',
                                    'provider_timeout',
                                )
                                append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=uts_acc provider_failure=timeout")
                            except subprocess.CalledProcessError as exc:
                                governed = invalid_lane_result(
                                    f'provider_runner_failed: governed subprocess exited with status {exc.returncode}',
                                    'provider_runner_failed',
                                )
                                append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=uts_acc provider_failure=subprocess_exit_{exc.returncode}")
                        else:
                            governed = {
                                'status': 'skipped',
                                'passed_count': 0,
                                'total_cases': 0,
                                'full_support': False,
                                'cases': [],
                                'note': 'governed lane requires the ADL-local UTS+ACC path and is skipped for hosted models in this toolkit build',
                            }
                            append_runner_log(runner_log, f"lane_complete id={entry['id']} lane=uts_acc skipped=true")
                    report['models'].append({
                        'candidate_id': entry['id'],
                        'tier': entry['tier'],
                        'provider': entry['provider'],
                        'model_id': entry['model_id'],
                        'lanes': {
                            'regular': summarize_lane_result(regular_result, 'supports_regular_tool_calls'),
                            'uts_only': summarize_lane_result(uts_result, 'supports_uts_only'),
                            'uts_acc': governed,
                        },
                    })
                    existing_models[entry['id']] = report['models'][-1]
                    out_path.parent.mkdir(parents=True, exist_ok=True)
                    out_path.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
                    append_runner_log(runner_log, f"model_complete id={entry['id']}")
                finally:
                    if warmed_local:
                        unload_local_model(entry['model_id'])
                        append_runner_log(runner_log, f"model_unloaded id={entry['id']} runtime_model_id={entry['model_id']}")
                    os.environ.pop('BENCHMARK_LOG_PATH', None)
                    os.environ.pop('ADL_UTS_ACC_PROGRESS_PATH', None)

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    write_reports(report, out_path)
    failures_path, provider_status_path = write_auxiliary_artifacts(report, out_path)
    report['artifacts']['failures_json'] = str(failures_path)
    report['artifacts']['provider_status_json'] = str(provider_status_path)
    out_path.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    append_runner_log(runner_log, f"run_complete out={out_path}")
    print(f"Wrote {out_path}")


if __name__ == '__main__':
    main()
