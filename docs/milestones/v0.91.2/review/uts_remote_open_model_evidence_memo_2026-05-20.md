# UTS Remote Open-Model Evidence Memo (2026-05-20)

## Status

This memo records the strongest source-backed benchmark evidence currently available for UTS on remote Ollama-served open models.

This is an interim evidence memo, not a publication-ready final benchmark report.

## Why this memo exists

The benchmark campaign spent substantial time stabilizing the harness. Earlier results mixed real model behavior with harness bugs, stale registry fixtures, and runtime-path differences. The current goal is to preserve only the rows that still hold after the runner and governed fixture repairs.

## Methodology currently supported

All rows in the main table below were run with the same top-level method:

1. one model at a time
2. one warm load per model
3. sequential lane order:
   - `regular`
   - `UTS-only`
   - `UTS+ACC`
4. one unload after the full per-model matrix
5. fixed task panel of 11 benchmark tasks
6. remote Ollama host via `OLLAMA_HOST=http://192.168.68.77:11434`
7. short per-test timeout budget and capped output
8. deterministic self-check before the live run

Important caveat:
- two rows still use mixed-source evidence because the governed lane had to be rerun after fixture repairs:
  - `gemma4:31b`
  - `deepseek-r1:32b`
- for those rows, the `regular` and `UTS-only` values come from the unified run artifact, while the corrected `UTS+ACC` value comes from the later governed rerun artifact.

## Main evidence table

| Model | Regular | UTS-only | UTS+ACC | Evidence state |
|---|---:|---:|---:|---|
| `gemma4:31b` | `9/11` | `11/11` | `11/11` | strong after governed rerun |
| `gemma4:26b` | `7/11` | `11/11` | `11/11` | strong |
| `gemma3:27b` | `8/11` | `11/11` | `11/11` | strong exploratory |
| `qwen3-coder:30b` | `7/11` | `11/11` | `11/11` | strong |
| `gemma4:e4b` | `6/11` | `11/11` | `11/11` | strong small-model governed support |
| `gemma4:e2b` | `4/11` | `11/11` | `9/11` | near-complete small-model governed support |
| `deepseek-r1:32b` | `6/11` | `11/11` | `1/11` | governed failure outlier |
| `gpt-oss:latest` | `5/11` | `2/11` | `1/11` | weak |
| `qwen3:30b` | `1/11` | `1/11` | `0/11` | non-viable on current benchmark |

## Strongest supported claims

The following claims are currently supported by source evidence:

1. `UTS-only` is not a failure mode on strong remote open models.
2. `UTS+ACC` is not merely theoretical; it reaches `11/11` on multiple serious remote open models.
3. support is sharply model-dependent.
4. coder-tuned and family-specific differences matter a lot:
   - `qwen3-coder:30b` is strong
   - `qwen3:30b` is very weak
5. `UTS+ACC` can still be almost viable even on a smaller model:
   - `gemma4:e2b` reached `9/11`

## What this evidence does **not** support

This memo does **not** support the following claims:

1. that UTS is universally supported across open models
2. that UTS is already a mature universal standard
3. that ADL or UTS is broadly better than OpenAI or Anthropic overall
4. that hosted and remote-open results are yet directly comparable under one fully unified final evidence pass
5. that every earlier negative governed result was invalid

## Why this is still meaningful

The strongest positive pattern is now hard to dismiss as pure noise:

- several independent remote open models reached `11/11` on `UTS-only`
- several independent remote open models reached `11/11` on `UTS+ACC`
- weaker models did not
- the harness now separates strong and weak models in a believable way

That is the first real evidence that UTS is useful as a discriminating contract rather than a decorative wrapper.

## Why this is still not enough for a bold public claim

A skeptical reviewer would still push back on at least these points:

1. the sample is still too small
2. the hosted frontier lane has not yet been rerun end-to-end under the final stabilized methodology surface
3. two key rows still depend on corrected governed reruns rather than one fresh all-lane rerun artifact
4. several important panel models are still missing from the remote evidence set
5. the current strongest evidence comes disproportionately from Gemma-family models

Those criticisms would be fair.

## Minimum additional evidence needed

Before making a stronger public claim, the benchmark should add more remote rows from the same fixed methodology, especially:

1. one or more additional non-Gemma strong models
2. one or more additional weaker or mixed models
3. fresh governed reruns for any row collected before the final fixture repair
4. a fresh hosted evidence pass under the final runner surface if frontier comparisons are going to be presented alongside the open-model table

## Evidence sources used

Main remote row artifacts:

- `/private/tmp/uts-toolkit-gemma431-remote-v1.json`
- `/private/tmp/uts-acc-gemma431-remote-v3.json`
- `/private/tmp/uts-toolkit-gemma426-remote-v1.json`
- `/private/tmp/uts-toolkit-gemma327b-remote-v1.json`
- `/private/tmp/uts-toolkit-qwen3coder30b-remote-v1.json`
- `/private/tmp/uts-toolkit-gemma4e4b-remote-v1.json`
- `/private/tmp/uts-toolkit-gemma4e2b-remote-v1.json`
- `/private/tmp/uts-toolkit-deepseek32-remote-v3.json`
- `/private/tmp/uts-acc-deepseek32-remote-v4.json`
- `/private/tmp/uts-toolkit-gptoss-remote-v1.json`
- `/private/tmp/uts-toolkit-qwen330b-remote-v1.json`

## Bottom line

The evidence no longer supports the claim that "UTS is a total failure."

The evidence now supports a narrower, stronger claim:

- UTS is useful on multiple serious remote open models.
- UTS+ACC is viable on multiple serious remote open models.
- support varies sharply by family and tuning.
- more data is still needed before claiming broad standard-level success.
