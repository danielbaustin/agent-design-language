#!/usr/bin/env python3
import json
import os
import time
import urllib.error
import urllib.request
from contextlib import contextmanager
from pathlib import Path

import fcntl

HOSTED_BASE_URL = 'http://127.0.0.1:8796'
OLLAMA_KEEP_ALIVE = '30m'
LOCAL_TEST_TIMEOUT = int(os.getenv('ADL_UTS_LOCAL_TEST_TIMEOUT_SECONDS', '20'))
LOCAL_NUM_PREDICT = int(os.getenv('ADL_UTS_LOCAL_NUM_PREDICT', '96'))
LOCAL_NUM_CTX = int(os.getenv('ADL_UTS_LOCAL_NUM_CTX', '8192'))
LOCAL_TEMPERATURE = float(os.getenv('ADL_UTS_LOCAL_TEMPERATURE', '0'))


def append_benchmark_log(message, log_path=None):
    target = log_path or os.environ.get('BENCHMARK_LOG_PATH')
    if not target:
        return
    path = Path(target)
    path.parent.mkdir(parents=True, exist_ok=True)
    timestamp = time.strftime('%Y-%m-%dT%H:%M:%S%z')
    with path.open('a', encoding='utf-8') as handle:
        handle.write(f'[{timestamp}] {message}\n')


def current_ollama_base_url() -> str:
    return os.getenv('OLLAMA_HOST', 'http://127.0.0.1:11434').rstrip('/')


def is_remote_ollama_target() -> bool:
    normalized = current_ollama_base_url()
    return normalized not in {'http://127.0.0.1:11434', 'http://localhost:11434'}


def extract_json_object(text):
    text = text.strip()
    if not text:
        raise ValueError('empty response')
    try:
        parsed = json.loads(text)
        if isinstance(parsed, dict) and isinstance(parsed.get('output'), str):
            return extract_json_object(parsed['output'])
        return parsed
    except json.JSONDecodeError:
        pass
    start = text.find('{')
    while start != -1:
        depth = 0
        for index in range(start, len(text)):
            char = text[index]
            if char == '{':
                depth += 1
            elif char == '}':
                depth -= 1
                if depth == 0:
                    candidate = text[start:index + 1]
                    try:
                        parsed = json.loads(candidate)
                        if isinstance(parsed, dict) and isinstance(parsed.get('output'), str):
                            return extract_json_object(parsed['output'])
                        return parsed
                    except json.JSONDecodeError:
                        break
        start = text.find('{', start + 1)
    raise ValueError('no parseable json object found')


def normalize_tool_call(tool_call):
    normalized = dict(tool_call)
    name = normalized.get('name')
    arguments = dict(normalized.get('arguments') or {})
    if name == 'query_table':
        normalized['name'] = 'query_database'
        if 'filter' in arguments and 'filters' not in arguments:
            arguments['filters'] = arguments.pop('filter')
    if name == 'decrement_inventory':
        normalized['name'] = 'update_inventory'
        if 'quantity' in arguments and 'delta' not in arguments:
            arguments['delta'] = -int(arguments.pop('quantity'))
    if 'document' in arguments and 'document_id' not in arguments:
        arguments['document_id'] = arguments.pop('document')
    if 'line' in arguments and 'log_line' not in arguments:
        arguments['log_line'] = arguments.pop('line')
    normalized['arguments'] = arguments
    return normalized


def matches_expected_arguments(args, task):
    expected = task.get('expected_arguments', {})
    for key, value in expected.items():
        if args.get(key) != value:
            return False
    optional_enums = task.get('optional_enum_arguments', {})
    for key, allowed in optional_enums.items():
        if key in args and args.get(key) not in allowed:
            return False
    if task.get('require_exact_arguments'):
        allowed_keys = set(expected.keys()) | set(optional_enums.keys())
        if set(args.keys()) != allowed_keys:
            return False
    return True


def _post_json(url, payload, timeout=900):
    body = json.dumps(payload).encode('utf-8')
    request = urllib.request.Request(
        url,
        data=body,
        headers={'content-type': 'application/json'},
        method='POST',
    )
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return response.read().decode('utf-8')


def ensure_local_model_loaded(model_id, timeout=900):
    payload = {
        'model': model_id,
        'prompt': '',
        'stream': False,
        'keep_alive': OLLAMA_KEEP_ALIVE,
        'options': {
            'num_predict': 1,
            'temperature': 0,
            'num_ctx': LOCAL_NUM_CTX,
        },
    }
    append_benchmark_log(
        f"local warm_load_start model={model_id} keep_alive={OLLAMA_KEEP_ALIVE} num_ctx={LOCAL_NUM_CTX}"
    )
    try:
        _post_json(f'{current_ollama_base_url()}/api/generate', payload, timeout=timeout)
    except urllib.error.HTTPError as exc:
        raise RuntimeError(f'ollama warm load failed: status={exc.code}') from exc
    append_benchmark_log(f"local warm_load_done model={model_id}")


def unload_local_model(model_id, timeout=900):
    payload = {
        'model': model_id,
        'prompt': '',
        'stream': False,
        'keep_alive': 0,
    }
    try:
        _post_json(f'{current_ollama_base_url()}/api/generate', payload, timeout=timeout)
    except urllib.error.HTTPError as exc:
        raise RuntimeError(f'ollama unload failed: status={exc.code}') from exc
    append_benchmark_log(f"local unload_done model={model_id}")


@contextmanager
def local_model_execution_lock(lock_path='/private/tmp/uts_local_model_execution.lock'):
    path = Path(lock_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('w') as handle:
        fcntl.flock(handle.fileno(), fcntl.LOCK_EX)
        try:
            yield
        finally:
            fcntl.flock(handle.fileno(), fcntl.LOCK_UN)


def invoke_local(model_id, prompt, timeout=LOCAL_TEST_TIMEOUT):
    payload = {
        'model': model_id,
        'prompt': prompt,
        'stream': False,
        'think': False,
        'keep_alive': OLLAMA_KEEP_ALIVE,
        'options': {
            'num_predict': LOCAL_NUM_PREDICT,
            'temperature': LOCAL_TEMPERATURE,
            'num_ctx': LOCAL_NUM_CTX,
        },
    }
    append_benchmark_log(
        f"local invoke_start model={model_id} timeout_s={timeout} num_predict={LOCAL_NUM_PREDICT} num_ctx={LOCAL_NUM_CTX}"
    )
    start = time.time()
    try:
        raw = _post_json(f'{current_ollama_base_url()}/api/generate', payload, timeout=timeout)
    except TimeoutError as exc:
        append_benchmark_log(f"local invoke_timeout model={model_id} timeout_s={timeout}")
        raise RuntimeError('timed out') from exc
    except urllib.error.URLError as exc:
        if 'timed out' in str(exc).lower():
            append_benchmark_log(f"local invoke_timeout model={model_id} timeout_s={timeout}")
            raise RuntimeError('timed out') from exc
        raise RuntimeError(f'ollama generate failed: {exc}') from exc
    except urllib.error.HTTPError as exc:
        raise RuntimeError(f'ollama generate failed: status={exc.code}') from exc
    duration_ms = int((time.time() - start) * 1000)
    parsed = json.loads(raw)
    response_text = parsed.get('response')
    if not isinstance(response_text, str):
        raise RuntimeError('ollama generate returned no response text')
    append_benchmark_log(
        f"local invoke_done model={model_id} duration_ms={duration_ms} response_chars={len(response_text)}"
    )
    return response_text, duration_ms


def invoke_hosted(route, model_id, prompt, timeout=900):
    base_url = os.getenv('ADL_HOSTED_BASE_URL', HOSTED_BASE_URL).rstrip('/')
    endpoint = f'{base_url}/{route}'
    body = json.dumps({'prompt': prompt, 'model': model_id}).encode('utf-8')
    request = urllib.request.Request(
        endpoint,
        data=body,
        headers={'content-type': 'application/json'},
        method='POST',
    )
    start = time.time()
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            payload = response.read().decode('utf-8')
    except urllib.error.HTTPError as exc:
        raise RuntimeError(f'hosted request failed: status={exc.code}') from exc
    duration_ms = int((time.time() - start) * 1000)
    return payload, duration_ms, model_id


def summary_path_for(out_path: Path) -> Path:
    return out_path.with_name(f'{out_path.stem}_summary.md')


def verbose_path_for(out_path: Path) -> Path:
    return out_path.with_name(f'{out_path.stem}_verbose.md')


def _durations(cases):
    return [case.get('duration_ms') for case in cases if case.get('duration_ms') is not None]


def _duration_stats(cases):
    values = _durations(cases)
    if not values:
        return None, None
    return int(sum(values) / len(values)), int(sum(values))


def print_console_table(report, support_key):
    results = report['results']
    print('Model\tTier\tProvider\tPassed\tTotal\tSupport\tAvg ms\tTotal ms')
    for result in results:
        avg_ms, total_ms = _duration_stats(result.get('cases', []))
        avg_text = '' if avg_ms is None else str(avg_ms)
        total_text = '' if total_ms is None else str(total_ms)
        print(
            f"{result['candidate_id']}\t{result.get('tier','')}\t{result.get('provider','')}\t{result['passed_count']}\t{result['total_cases']}\t{result.get(support_key)}\t{avg_text}\t{total_text}"
        )


def write_markdown_reports(report, lane_title, support_key, out_path: Path):
    results = report['results']
    total_models = len(results)
    full_support = sum(1 for result in results if result.get(support_key))

    summary_lines = [
        f'# {lane_title} Benchmark Summary',
        '',
        '## Executive Summary',
        '',
        f"- Models evaluated: `{total_models}`",
        f"- Models with full support: `{full_support}`",
        f"- Selection model: `{report['selection'].get('model') or 'panel-driven'}`",
        f"- Selection tier: `{report['selection'].get('tier') or 'all matched tiers'}`",
        f"- Model panel: `{report['selection'].get('panel_file')}`",
        f"- Task panel: `{report['selection'].get('task_panel_file')}`",
        '',
        '## Overview Table',
        '',
        '| Model | Tier | Provider | Passed | Total | Full Support | Avg ms/test | Total ms |',
        '|---|---|---|---:|---:|---|---:|---:|',
    ]
    for result in results:
        avg_ms, total_ms = _duration_stats(result.get('cases', []))
        summary_lines.append(
            f"| `{result['candidate_id']}` | `{result.get('tier','')}` | `{result.get('provider','')}` | `{result['passed_count']}` | `{result['total_cases']}` | `{str(result.get(support_key)).lower()}` | `{avg_ms if avg_ms is not None else 'n/a'}` | `{total_ms if total_ms is not None else 'n/a'}` |"
        )
    summary_lines.append('')
    summary_lines.append('This summary is intentionally compact for comparison and slide use.')
    summary_path = summary_path_for(out_path)
    summary_path.write_text('\n'.join(summary_lines) + '\n', encoding='utf-8')

    verbose_lines = [
        f'# {lane_title} Benchmark Verbose Report',
        '',
        '## Executive Summary',
        '',
        f"- Models evaluated: `{total_models}`",
        f"- Models with full support: `{full_support}`",
        f"- Selection model: `{report['selection'].get('model') or 'panel-driven'}`",
        f"- Selection tier: `{report['selection'].get('tier') or 'all matched tiers'}`",
        f"- Model panel: `{report['selection'].get('panel_file')}`",
        f"- Task panel: `{report['selection'].get('task_panel_file')}`",
        '',
        '## Overview Table',
        '',
        '| Model | Tier | Provider | Passed | Total | Full Support | Avg ms/test | Total ms |',
        '|---|---|---|---:|---:|---|---:|---:|',
    ]
    for result in results:
        avg_ms, total_ms = _duration_stats(result.get('cases', []))
        verbose_lines.append(
            f"| `{result['candidate_id']}` | `{result.get('tier','')}` | `{result.get('provider','')}` | `{result['passed_count']}` | `{result['total_cases']}` | `{str(result.get(support_key)).lower()}` | `{avg_ms if avg_ms is not None else 'n/a'}` | `{total_ms if total_ms is not None else 'n/a'}` |"
        )
    verbose_lines.append('')
    for result in results:
        avg_ms, total_ms = _duration_stats(result.get('cases', []))
        verbose_lines.extend([
            f"## {result['candidate_id']}",
            '',
            f"- Provider: `{result.get('provider','')}`",
            f"- Runtime model id: `{result.get('model_id','')}`",
            f"- Passed: `{result['passed_count']}` / `{result['total_cases']}`",
            f"- Full support: `{str(result.get(support_key)).lower()}`",
            f"- Average duration per test: `{avg_ms if avg_ms is not None else 'n/a'}` ms",
            f"- Total known duration: `{total_ms if total_ms is not None else 'n/a'}` ms",
            '',
            '| Task | Classification | Passed | Duration ms | Note |',
            '|---|---|---:|---:|---|',
        ])
        for case in result['cases']:
            note = str(case.get('note', '')).replace('|', '/').replace('\n', ' ')
            duration = case.get('duration_ms')
            verbose_lines.append(
                f"| `{case['task_id']}` | `{case['classification']}` | `{str(case['passed']).lower()}` | `{duration if duration is not None else 'n/a'}` | {note} |"
            )
        verbose_lines.append('')
    verbose_path = verbose_path_for(out_path)
    verbose_path.write_text('\n'.join(verbose_lines) + '\n', encoding='utf-8')
    return summary_path, verbose_path


def write_markdown_report(report, lane_title, support_key, out_path: Path):
    summary_path, _ = write_markdown_reports(report, lane_title, support_key, out_path)
    return summary_path
