# UTS Benchmark Runbook

This runbook is the supported execution contract for the `#3121` benchmark harness.

## Supported entrypoint

Use exactly one Python runner for benchmark execution:

```bash
python3 adl/tools/uts_benchmark_runner.py <provider-kind> <models-file> <out-json> [options]
```

`adl/tools/run_uts_benchmark.sh` is only a convenience wrapper around the canonical runner. The old lane-specific Python scripts are retired and are not supported entrypoints.

## Hosted provider credentials

Use provider environment variables directly when possible:

- `OPENAI_API_KEY`
- `GEMINI_API_KEY`
- `ANTHROPIC_API_KEY`

If a file-backed setup is needed, use the provider-specific file-path
environment variables referenced by the canonical template:

- `ADL_OPENAI_API_KEY_FILE`
- `ADL_GEMINI_API_KEY_FILE`
- `ADL_ANTHROPIC_API_KEY_FILE`

The canonical template file is:

```text
adl/tools/benchmark/hosted_provider_key_files.json
```

That template is intentionally portable and must not contain operator-local
absolute paths.

## Required panels

The canonical model and task panels are:

```text
adl/tools/benchmark/uts_33_model_panel.json
adl/tools/benchmark/uts_33_task_panel.json
```

The current canonical task panel contains 11 tasks. A full three-lane run produces 33 evaluated fixtures per model: `regular`, `uts_only`, and `uts_acc`.

Requested model IDs must exist in the canonical panel before any run is claimed. A profile or model list that references an absent ID should fail immediately in preflight rather than producing a partial artifact.

## Deterministic self-check

Run this before relying on benchmark output:

```bash
python3 adl/tools/benchmark/deterministic_self_check.py
```

Expected result:

```text
"passed": true
"fixture_count": 33
"task_count": 11
```

The canonical runner also writes a sibling self-check artifact for each run.

## Smoke runs

Hosted smoke without the governed Rust lane:

```bash
python3 adl/tools/uts_benchmark_runner.py \
  hosted \
  adl/tools/benchmark/hosted_smoke_models.txt \
  artifacts/uts_runs/utsbench_smoke_hosted.json \
  --no-resume
```

Hosted Claude-only smoke:

```bash
python3 adl/tools/uts_benchmark_runner.py \
  hosted \
  adl/tools/benchmark/hosted_claude_smoke_models.txt \
  artifacts/uts_runs/utsbench_smoke_claude.json \
  --no-resume
```

Local smoke should use a one-model file and must run when `ollama ps` is clean:

```bash
mkdir -p artifacts/uts_runs/model_lists
printf 'Qwen3.5:35b-a3b\n' > artifacts/uts_runs/model_lists/uts_one_local_model.txt
python3 adl/tools/uts_benchmark_runner.py \
  local \
  artifacts/uts_runs/model_lists/uts_one_local_model.txt \
  artifacts/uts_runs/utsbench_smoke_local_qwen35.json \
  --no-resume
```

## Governed UTS+ACC runs

Add `--include-governed` to include the Rust-backed `UTS+ACC` lane:

```bash
python3 adl/tools/uts_benchmark_runner.py \
  hosted \
  adl/tools/benchmark/hosted_core_models.txt \
  artifacts/uts_runs/utsbench_hosted_core_governed.json \
  --include-governed \
  --no-resume
```

The governed lane requires Rust/Cargo because it exercises the ADL UTS+ACC compiler path. Regular and UTS-only lanes remain Python-only.

Governed scoring rules that matter for proof:

- must-compile tasks must match the canonical expected arguments, not just the tool name and wrapper shape
- fail-closed tasks pass only on explicit refusal
- a forbidden proposal that ACC later rejects is still a benchmark failure, not a refusal success

Hosted governed runs now use an ephemeral token-guarded localhost adapter
bridge. The adapter still binds to `127.0.0.1`, but requests without the
randomized per-run path token are rejected instead of being forwarded to the
backing hosted provider.

## Full hosted run

```bash
python3 adl/tools/uts_benchmark_runner.py \
  hosted \
  adl/tools/benchmark/hosted_core_models.txt \
  artifacts/uts_runs/utsbench_hosted_core.json \
  --include-governed \
  --no-resume
```

Hosted runs may be run as one sequential benchmark process.

## Full local run

Local models must be run one model at a time. Do not run multiple local Ollama models concurrently. Before each model, confirm:

```bash
ollama ps
```

Expected clean output has no resident model rows.

Use a one-model file per run:

```bash
mkdir -p artifacts/uts_runs/model_lists
printf 'Qwen3.5:35b-a3b\n' > artifacts/uts_runs/model_lists/uts_one_local_model.txt
python3 adl/tools/uts_benchmark_runner.py \
  local \
  artifacts/uts_runs/model_lists/uts_one_local_model.txt \
  artifacts/uts_runs/utsbench_local_qwen35.json \
  --include-governed \
  --no-resume
ollama stop 'Qwen3.5:35b-a3b' || true
ollama ps
```

Repeat the same pattern for each local model. Never run `deepseek-r1:32b` on wuji; use the remote host for that model.

## Remote Ollama run

Use `OLLAMA_HOST` for the remote AI node. DeepSeek is the required remote model:

```bash
mkdir -p artifacts/uts_runs/model_lists
printf 'deepseek-r1:32b\n' > artifacts/uts_runs/model_lists/uts_remote_deepseek_only.txt
OLLAMA_HOST=http://192.168.68.77:11434 \
python3 adl/tools/uts_benchmark_runner.py \
  local \
  artifacts/uts_runs/model_lists/uts_remote_deepseek_only.txt \
  artifacts/uts_runs/utsbench_remote_deepseek.json \
  --include-governed \
  --no-resume
OLLAMA_HOST=http://192.168.68.77:11434 ollama stop 'deepseek-r1:32b' || true
OLLAMA_HOST=http://192.168.68.77:11434 ollama ps
```

Remote background pulls do not invalidate a benchmark row by themselves. Treat a row as invalid only if the artifact or run log records provider failure, timeout, non-target model residency, or another execution error.

## Expected outputs

For an output path like:

```text
artifacts/uts_runs/utsbench_hosted_core.json
```

The runner writes:

```text
artifacts/uts_runs/utsbench_hosted_core.json
artifacts/uts_runs/utsbench_hosted_core_summary.md
artifacts/uts_runs/utsbench_hosted_core_details.md
artifacts/uts_runs/utsbench_hosted_core_provider_status.json
artifacts/uts_runs/utsbench_hosted_core_self_check.json
```

When `--include-governed` is used, governed raw artifacts are also written under a sibling directory named like:

```text
artifacts/uts_runs/utsbench_hosted_core_governed_raw/
```

Current runner artifacts are hardened for portability and review:

- non-repo paths are serialized as bounded external markers instead of absolute
  host paths
- raw model output is replaced with redacted response markers
- provider failure details are reduced to bounded failure summaries

## Validity checks

A run is usable evidence when:

```text
self_check.passed == true
all intended lane statuses are evaluated
run log has no provider/runtime failure
local/remote Ollama residency is clean before and after each model
blocked/skipped/provider_failed states are not counted as evaluated successes
```

The canonical Python runner now exits nonzero when any required lane remains blocked, skipped, or provider-failed. A zero exit code means the intended lanes actually evaluated; it does not mean the model passed every benchmark case.

A run is not publication proof just because JSON exists. Publish only source-backed rows and keep historical/provisional rows separate from the main proof path.
