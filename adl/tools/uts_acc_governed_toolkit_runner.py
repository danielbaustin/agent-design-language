#!/usr/bin/env python3
import argparse
import json
import os
import subprocess
import sys
import time
import socket
import tempfile
from contextlib import contextmanager, nullcontext
from pathlib import Path

CURRENT_DIR = Path(__file__).resolve().parent
BENCHMARK_DIR = CURRENT_DIR / 'benchmark'
if str(BENCHMARK_DIR) not in sys.path:
    sys.path.insert(0, str(BENCHMARK_DIR))

from uts_benchmark_panel import default_panel_path, load_panel, select_models, display_path
from uts_benchmark_tasks import default_task_panel_path, display_task_panel_path
from portable_benchmark_common import write_markdown_reports, print_console_table, append_benchmark_log


HOSTED_PROVIDER_KEYS_FILE = BENCHMARK_DIR / 'hosted_provider_key_files.json'


def append_governed_progress(message):
    progress = os.environ.get('ADL_UTS_ACC_PROGRESS_PATH')
    progress_path = Path(progress) if progress else None
    if progress_path:
        progress_path.parent.mkdir(parents=True, exist_ok=True)
        timestamp = time.strftime('%Y-%m-%dT%H:%M:%S%z')
        with progress_path.open('a', encoding='utf-8') as handle:
            handle.write(f'[{timestamp}] {message}\n')


def _find_free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind(("127.0.0.1", 0))
        sock.listen(1)
        return int(sock.getsockname()[1])


def _load_key_file_map() -> dict[str, str]:
    override = os.environ.get('ADL_HOSTED_PROVIDER_KEYS_FILE')
    path = Path(override) if override else HOSTED_PROVIDER_KEYS_FILE
    if not path.is_file():
        return {}
    doc = json.loads(path.read_text(encoding='utf-8'))
    return {
        str(name): str(value)
        for name, value in (doc.get('keys') or {}).items()
        if isinstance(name, str) and isinstance(value, str)
    }


def _extract_key_value(env_name: str, path: Path) -> str:
    raw = path.read_text(encoding='utf-8').strip()
    value = raw
    for line in raw.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith('#'):
            continue
        if stripped.startswith(env_name + '='):
            value = stripped.split('=', 1)[1].strip().strip("'\"")
            break
        value = stripped.strip("'\"")
        break
    return value


def _populate_hosted_env(command_env: dict[str, str]) -> None:
    key_files = _load_key_file_map()
    for env_name in ('OPENAI_API_KEY', 'GEMINI_API_KEY', 'ANTHROPIC_API_KEY'):
        if command_env.get(env_name):
            continue
        configured = key_files.get(env_name)
        if not configured:
            continue
        key_path = Path(configured).expanduser()
        if key_path.is_file() and key_path.stat().st_size > 0:
            value = _extract_key_value(env_name, key_path)
            if value:
                command_env[env_name] = value


def _provider_failure_kind(note: str) -> str | None:
    lowered = note.lower()
    if 'credit balance is too low' in lowered or 'billing' in lowered:
        return 'provider_billing_blocked'
    if 'does not exist' in lowered and 'requested model' in lowered:
        return 'provider_model_unavailable'
    if "'openai_api_key'" in lowered or "'gemini_api_key'" in lowered or "'anthropic_api_key'" in lowered:
        return 'provider_auth_missing'
    if 'missing required environment variables' in lowered:
        return 'provider_auth_missing'
    if 'timed out' in lowered:
        return 'provider_timeout'
    if 'provider completion failed:' in lowered:
        return 'provider_error'
    return None


def _summarize_provider_failure(cases: list[dict]) -> tuple[str, str] | None:
    if not cases:
        return None
    kinds = []
    notes = []
    for case in cases:
        classification = case.get('classification')
        note = str(case.get('note', ''))
        kind = _provider_failure_kind(note)
        if classification != 'unusable' or not kind:
            return None
        kinds.append(kind)
        notes.append(note)
    unique_kinds = sorted(set(kinds))
    if len(unique_kinds) != 1:
        return None
    first_note = notes[0]
    return unique_kinds[0], first_note


@contextmanager
def hosted_adapter(selected, timeout, env):
    port = _find_free_port()
    with tempfile.TemporaryDirectory(prefix='adl-live-provider-') as tmp_dir:
        tmp = Path(tmp_dir)
        metadata = tmp / 'provider_invocations.json'
        port_file = tmp / 'port.txt'
        command = [
            'python3',
            str(CURRENT_DIR / 'real_chatgpt_gemini_claude_provider_adapter.py'),
            '--port',
            str(port),
            '--port-file',
            str(port_file),
            '--metadata',
            str(metadata),
            '--timeout',
            str(timeout),
        ]
        process = subprocess.Popen(
            command,
            env=env,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            text=True,
        )
        try:
            deadline = time.time() + 15
            while time.time() < deadline:
                if process.poll() is not None:
                    raise RuntimeError('hosted provider adapter exited early')
                if port_file.exists():
                    break
                time.sleep(0.1)
            if not port_file.exists():
                raise RuntimeError('timed out waiting for hosted provider adapter startup')
            yield {
                'host': f'http://127.0.0.1:{port}',
                'metadata': metadata,
            }
        finally:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=5)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--panel-file', default=str(default_panel_path()))
    parser.add_argument('--task-panel-file', default=str(default_task_panel_path()))
    parser.add_argument('--model')
    parser.add_argument('--tier')
    parser.add_argument('--provider-kind', default='local')
    parser.add_argument('--list-models', action='store_true')
    parser.add_argument('--out')
    args = parser.parse_args()

    panel = load_panel(args.panel_file)
    selected = select_models(panel, model=args.model, tier=args.tier, provider_kind=args.provider_kind)

    if args.list_models:
        rows = selected if selected else [entry for entry in panel.get('models', []) if entry.get('provider_kind') == args.provider_kind]
        for entry in rows:
            print(f"{entry['id']}\t{entry['tier']}\t{entry['provider_kind']}\t{entry['provider']}")
        return

    if not args.out:
        raise SystemExit('--out is required unless --list-models is used')
    if not selected:
        raise SystemExit(f'no {args.provider_kind} models matched the requested selection')

    repo_root = CURRENT_DIR.parent
    model_csv = ','.join(entry['model_id'] for entry in selected)
    append_governed_progress(
        f"governed_runner_start selection_models={','.join(entry['id'] for entry in selected)} panel={display_path(args.panel_file)} task_panel={display_task_panel_path(args.task_panel_file)}"
    )
    command = [
        'cargo',
        'run',
        '--manifest-path',
        str(repo_root / 'Cargo.toml'),
        '--bin',
        'demo_v0912_uts_acc_multi_model_benchmark',
        '--',
        args.out,
        model_csv,
        args.task_panel_file,
    ]
    command_env = os.environ.copy()
    if args.provider_kind == 'hosted':
        _populate_hosted_env(command_env)
    adapter_context = hosted_adapter(
        selected,
        timeout=int(command_env.get('ADL_LIVE_PROVIDER_TIMEOUT_SECS', '240')),
        env=command_env,
    ) if args.provider_kind == 'hosted' else nullcontext(None)
    with adapter_context as adapter:
        if adapter:
            command_env['OLLAMA_HOST'] = adapter['host']
            append_governed_progress(f"hosted_adapter_start host={adapter['host']}")
        append_governed_progress(f"cargo_start command={' '.join(command)}")
        completed = subprocess.run(command, check=True, capture_output=True, text=True, env=command_env)
    if completed.stdout:
        for line in completed.stdout.splitlines():
            append_governed_progress(f"cargo_stdout {line}")
    if completed.stderr:
        for line in completed.stderr.splitlines():
            append_governed_progress(f"cargo_stderr {line}")
    out_path = Path(args.out)
    report = json.loads(out_path.read_text(encoding='utf-8'))
    results = []
    for result in report.get('models', []):
        cases = [
            {
                'task_id': case['task_id'],
                'classification': case['classification'],
                'passed': case['passed'],
                'duration_ms': case.get('duration_ms'),
                'note': '; '.join(case.get('notes', [])),
            }
            for case in result.get('cases', [])
        ]
        simplified_result = {
            'candidate_id': result['candidate_id'],
            'tier': next((entry.get('tier') for entry in selected if entry['model_id'] in result['candidate_id'] or entry['id'] in result['candidate_id']), ''),
            'provider': next((entry.get('provider') for entry in selected if entry['model_id'] == result['conditions']['model_id']), result['conditions']['provider_id']),
            'model_id': result['conditions']['model_id'],
            'passed_count': (result.get('scorecard') or {}).get('passed_count', 0),
            'total_cases': (result.get('scorecard') or {}).get('total_cases', len(result.get('cases', []))),
            'supports_governed_tool_use': (result.get('scorecard') or {}).get('supports_governed_tool_use', False),
            'cases': cases,
        }
        provider_failure = _summarize_provider_failure(cases)
        if provider_failure is not None:
            failure_kind, failure_note = provider_failure
            simplified_result['passed_count'] = 0
            simplified_result['total_cases'] = 0
            simplified_result['supports_governed_tool_use'] = False
            simplified_result['cases'] = []
            simplified_result['note'] = f'{failure_kind}: {failure_note}'
            simplified_result['provider_failure_kind'] = failure_kind
        results.append(simplified_result)

    simplified = {
        'selection': {
            'model': args.model,
            'tier': args.tier,
            'provider_kind': args.provider_kind,
            'panel_file': display_path(args.panel_file),
            'task_panel_file': display_task_panel_path(args.task_panel_file),
        },
        'results': results,
    }
    for result in simplified['results']:
        append_benchmark_log(
            f"uts_acc summarized model={result['candidate_id']} passed={result['passed_count']}/{result['total_cases']}",
        )
        for case in result.get('cases', []):
            append_governed_progress(
                f"case model={result['candidate_id']} task={case['task_id']} classification={case['classification']} passed={str(case['passed']).lower()} duration_ms={case.get('duration_ms') if case.get('duration_ms') is not None else 'n/a'} note={case.get('note','')}"
            )
    out_path.write_text(json.dumps(simplified, indent=2) + '\n', encoding='utf-8')
    write_markdown_reports(simplified, 'UTS + ACC Governed Tool Proposal', 'supports_governed_tool_use', out_path)
    append_governed_progress(f"governed_runner_complete out={out_path}")
    print_console_table(simplified, 'supports_governed_tool_use')


if __name__ == '__main__':
    main()
