# Current-Model Suitability Mini-Sprint Closeout for #4158

Status: `closed_post_merge`
Merged PR: `#4289`
Post-merge review:
`docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_REVIEW_4158.md`

## Scope

This closeout summarizes the current-model suitability mini-sprint tracked by
`#4158`.

The mini-sprint exists to make provider/model suitability less confusing by
separating:

- the reusable C-SDLC suitability panel umbrella: `#4095`
- the DeepSeek baseline proof: `#4096`
- the direct-hosted frontier-model umbrella: `#4154`
- the direct-hosted provider-family proof issues: `#4155`, `#4156`, `#4157`

`#4034` is Observatory logging/OTel/security work and is intentionally outside
this closeout.

## Issue State Consumed

| Issue | Role | Closeout status |
| --- | --- | --- |
| `#4095` | Reusable suitability umbrella | Closed after this summary and the matrix truth updates merged. |
| `#4096` | DeepSeek suitability proof | Closed. |
| `#4154` | Direct-hosted frontier-model umbrella | Closed after this summary and the matrix truth updates merged. |
| `#4155` | Direct-hosted OpenAI/Codex proof | Closed. |
| `#4156` | Direct-hosted Anthropic proof | Closed. |
| `#4157` | Direct-hosted Gemini proof | Closed. |
| `#4158` | Current-model suitability mini-sprint umbrella | Closed by merged PR `#4289`. |

## What Was Built

The mini-sprint produced one reusable suitability-panel flow and six tracked
proof packet directories:

| Packet | Issues | Summary truth |
| --- | --- | --- |
| `docs/milestones/v0.91.6/review/provider/deepseek_suitability/` | `#4096`, `#4095` | Hosted native `deepseek-chat` is `useful_with_limits`; local Ollama `deepseek-r1:8b` and `deepseek-r1:32b` remain candidate-only because they overclaimed closeout truth in the bounded panel. |
| `docs/milestones/v0.91.6/review/provider/openai_current_models/` | `#4155`, `#4154` | Direct-hosted `gpt-5.5` and `gpt-5.4` are `useful_with_limits` for the bounded five-task panel. |
| `docs/milestones/v0.91.6/review/provider/openai_current_models_rerun/` | `#4155`, `#4154` | Direct-hosted `gpt-5-codex` is `useful_with_limits`; requested `gpt-5.3-codex-spark` returned `model_not_found` and is `runtime_unsuitable_for_this_panel`. |
| `docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun/` | `#4155`, `#4154` | Direct-hosted `gpt-5.3-codex` is `useful_with_limits`. |
| `docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key/` | `#4156`, `#4154` | Direct-hosted `claude-sonnet-4-6` and `claude-haiku-4-5` are `useful_with_limits`; `claude-opus-4-8` is `candidate_only_format_repair_needed` because one planner row missed the required heading contract. |
| `docs/milestones/v0.91.6/review/provider/gemini_current_models/` | `#4157`, `#4154` | Direct-hosted `gemini-2.5-pro`, `gemini-2.5-flash`, and `gemini-2.5-flash-lite` are `useful_with_limits`. |

## Role Summary

The strongest current-model suitability truth after the mini-sprint is:

| Role | Strongest bounded candidates | Limits |
| --- | --- | --- |
| Watcher | `gemini-2.5-flash`, `gemini-2.5-flash-lite`, `gpt-5-codex`, hosted `deepseek-chat`, remote Gemma watcher lanes | Advisory only; no janitor autonomy or closeout authority. |
| Card validator | `gpt-5.5`, `gpt-5.4`, `gpt-5-codex`, `gemini-2.5-*`, `claude-sonnet-4-6`, `claude-haiku-4-5`, hosted `deepseek-chat` | Validates supplied card facts; does not become the card editor. |
| Reviewer | Direct-hosted OpenAI/Codex, Gemini, Anthropic Sonnet/Haiku, hosted DeepSeek, and prior OpenRouter reviewer lanes | Findings remain advisory until synthesized and accepted through normal review. |
| Planner | Direct-hosted OpenAI/Codex, Gemini, Anthropic Sonnet/Haiku, hosted DeepSeek, and prior OpenRouter DeepSeek route | Plans are bounded issue-local inputs, not execution authority. |
| Closeout checker | Direct-hosted OpenAI/Codex, Gemini Flash/Flash-Lite, Anthropic Sonnet/Haiku, hosted DeepSeek | May check closure readiness; may not close issues, merge PRs, or declare release truth. |

## Non-Claims

- No tested model gains merge, release, issue-close, file-write, or workflow
  authority.
- The proof packets do not benchmark general intelligence or broad coding
  quality.
- The results are role- and prompt-shape-specific.
- Direct-hosted, OpenRouter, remote Ollama, and local Ollama evidence remain
  distinct provider surfaces.
- Provider-output bytes are not claimed to be replay-stable.
- Credential-source labels are evidence context only; no secret values are
  stored in the tracked packets.

## Validation

The packet directories were revalidated from the clean `#4158` worktree:

```text
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/deepseek_suitability
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/openai_current_models
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/openai_current_models_rerun
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/gemini_current_models
```

Result: `PASS` for all six packet directories.

## Closeout Result

After this closeout summary and the provider matrix truth updates merged in
PR `#4289`:

1. `#4158` closed through PR linkage;
2. `#4095` closed as satisfied by the reusable panel and completed child proof
   wave;
3. `#4154` closed as satisfied by the direct-hosted OpenAI/Codex, Anthropic,
   and Gemini proof packets.

Do not close or modify `#4034` as part of this provider closeout.
