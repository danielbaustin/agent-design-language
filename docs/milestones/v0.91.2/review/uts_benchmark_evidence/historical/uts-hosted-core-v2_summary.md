# Unified UTS Benchmark Summary

## Executive Summary

- Models evaluated: `6`
- Governed lane included: `true`
- Model panel: `adl/tools/benchmark/uts_33_model_panel.json`
- Task panel: `adl/tools/benchmark/uts_33_task_panel.json`

## Overview Table

| Model | Tier | Provider | Regular | UTS-only | UTS+ACC | Regular avg ms | UTS avg ms | Governed avg ms |
|---|---|---|---:|---:|---:|---:|---:|---:|
| `gpt-5.4` | `hosted` | `openai-hosted` | `3/11` | `3/11` | `11/11` | `2623` | `2900` | `2465` |
| `gpt-5.5` | `hosted` | `openai-hosted` | `7/11` | `2/11` | `11/11` | `4966` | `5948` | `3202` |
| `gpt-5.3-codex` | `hosted` | `openai-hosted` | `5/11` | `3/11` | `11/11` | `2017` | `2729` | `2569` |
| `gpt-5.3-codex-spark` | `hosted` | `openai-hosted` | `0/11` | `0/11` | `skipped` | `n/a` | `n/a` | `n/a` |
| `gemini-2.5-pro` | `hosted` | `google-hosted` | `4/11` | `2/11` | `11/11` | `2616` | `2570` | `3041` |
| `claude-opus-4-1-20250805` | `hosted` | `anthropic-hosted` | `4/11` | `3/11` | `11/11` | `2549` | `2711` | `3771` |

This summary is intentionally compact for comparison and presentation use.
