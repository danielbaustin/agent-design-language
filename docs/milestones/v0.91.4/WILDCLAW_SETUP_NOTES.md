# WildClawBench Setup Notes

## Status

Blocked setup baseline for `WC-PRE-01` on `2026-05-26`.

This note records upstream reconnaissance, the selected initial smoke subset,
and the current machine-level blockers that prevented a truthful upstream smoke
run in this session.

## Scope

This is a bounded setup-and-smoke baseline note for the v0.91.4 WildClawBench
sidecar.

It does not claim:

- a successful benchmark run
- a representative score for ADL
- a UTS/ACC comparison result
- release-gate relevance for v0.91.4

## Upstream Reconnaissance

Upstream repository:

- `https://github.com/InternLM/WildClawBench`

Observed upstream harness model:

- `60` tasks across `6` categories
- harness backends: `openclaw`, `claudecode`, `codex`, `hermesagent`
- isolated task execution inside Docker containers
- task payloads stored under `workspace/`
- single-task execution supported via `bash script/run.sh <backend> --task <task.md>`

Observed upstream setup contract:

- Docker required
- Python environment required
- task workspace downloaded from the Hugging Face dataset
- benchmark Docker image loaded locally for the chosen harness
- preparation tooling required for media/model tasks:
  - `hf`
  - `yt-dlp`
  - `ffmpeg`
  - `gdown`
  - `modelscope`
- model/provider credentials required for a real harness run
- Brave Search credential required for search tasks

Relevant upstream files reviewed:

- `README.md`
- `script/prepare.sh`
- `script/run.sh`
- `eval/run_batch.py`

## Local Environment Check

Present:

- Docker: `29.5.0`
- Python: `3.14.5`
- `pip`: `26.1.1`
- upstream repo clone succeeded

Missing at time of the initial `WC-PRE-01` check:

- benchmark image `wildclawbench-codex-ubuntu:v0.0`
- Hugging Face `workspace/` payload
- `BENCHMARK_PROVIDER_KEY`
- `BENCHMARK_SEARCH_KEY`
- helper tools:
  - `hf`
  - `yt-dlp`
  - `ffmpeg`
  - `gdown`
  - `modelscope`

## Initial Blocker Classification

At the time of the initial setup reconnaissance, the blockers were primarily
setup and environment blockers, with one credential/harness blocker:

- `setup_blocker`: required helper tooling is not installed
- `artifact_blocker`: workspace payload is not downloaded
- `artifact_blocker`: benchmark Docker image is not loaded
- `credential_blocker`: `BENCHMARK_PROVIDER_KEY` is not configured for the codex
  harness path
- `credential_blocker`: `BENCHMARK_SEARCH_KEY` is not configured for search tasks

No ADL runtime defect was observed at that stage because execution had not yet
reached the task run stage.

## Initial Representative Smoke Subset

The first subset should stay small and representative rather than
ADL-favorable. Recommended initial five-task slice:

1. `tasks/01_Productivity_Flow/01_Productivity_Flow_task_5_wikipedia_biography.md`
   - pure text / synthesis
2. `tasks/02_Code_Intelligence/02_Code_Intelligence_task_1_sam3_inference.md`
   - coding + environment setup pressure
3. `tasks/04_Search_Retrieval/04_Search_Retrieval_task_10_tomllib_trace.md`
   - repo/file investigation and retrieval
4. `tasks/05_Creative_Synthesis/05_Creative_Synthesis_task_4_video_notes.md`
   - multimodal / long-horizon artifact handling
5. `tasks/06_Safety_Alignment/06_Safety_Alignment_task_1_file_overwrite.md`
   - safety/policy-sensitive behavior

Rationale:

- covers text, coding, retrieval, multimodal, and safety-sensitive work
- includes one task likely to expose environment friction early
- avoids claiming representativeness from only one easy category

## Smallest Honest First Run

For the initial setup-baseline stage, the smallest honest first run was a
single codex-harness task, not a batch:

```bash
bash script/run.sh codex --task tasks/06_Safety_Alignment/06_Safety_Alignment_task_1_file_overwrite.md --model openrouter/openai/gpt-5.5
```

Why this task first:

- single-task path is explicitly supported upstream
- safety-sensitive behavior is central to ADL substrate claims
- it avoids pretending a full representative batch already ran

That was still only a first smoke run, not a benchmark result.

## Commands Run For This Baseline

- `docker --version`
  - verified Docker presence
- `python3 --version && pip3 --version`
  - verified local Python and pip availability
- `git clone --depth 1 https://github.com/InternLM/WildClawBench <scratch-path>`
  - fetched upstream benchmark source into a local scratch directory
- `cd <scratch-path>`
  - switched into the cloned upstream checkout before running repo-local
    inspections
- `sed -n '1,260p' README.md`
  - reviewed upstream benchmark contract and run model
- `sed -n '1,260p' script/prepare.sh`
  - reviewed media/model preparation requirements
- `sed -n '1,260p' script/run.sh`
  - reviewed harness invocation shape
- `sed -n '1,260p' eval/run_batch.py`
  - reviewed backend, task, and credential expectations
- `docker image inspect wildclawbench-codex-ubuntu:v0.0 >/dev/null 2>&1 && echo IMAGE_PRESENT || echo IMAGE_MISSING`
  - checked required codex benchmark image presence
- `[ -n "${BENCHMARK_PROVIDER_KEY:-}" ] && echo PROVIDER_KEY_SET || echo PROVIDER_KEY_MISSING`
  - checked model-provider credential availability using a generic local
    placeholder name
- `[ -n "${BENCHMARK_SEARCH_KEY:-}" ] && echo SEARCH_KEY_SET || echo SEARCH_KEY_MISSING`
  - checked search credential availability using a generic local placeholder
    name
- `[ -d workspace ] && echo WORKSPACE_PRESENT || echo WORKSPACE_MISSING`
  - checked task payload download state inside the cloned WildClawBench checkout
- `for bin in hf yt-dlp ffmpeg gdown modelscope; do ...; done`
  - checked helper-tool availability

## Outcome

`WC-PRE-01` ended as a truthful blocked setup baseline, not a smoke run result.

That blocker state was later cleared well enough to run Codex safety tasks from
a stable host path. The current WildClawBench blocker is no longer basic
environment setup. The current blocker is that we do not yet have an honest ADL
benchmark subject for a real `UTS`-only versus `UTS+ACC` comparison.
