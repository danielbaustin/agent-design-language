# v0.91.6 Current-Model Suitability Mini-Sprint Review

Status: `retained_review_packet`
Date: 2026-06-20
Sprint umbrella: `#4158`
Merged PR: `#4289`
Closeout packet:
`docs/milestones/v0.91.6/review/provider/CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`

This review summarizes the bounded current-model suitability mini-sprint after
PR `#4289` merged and the nested umbrellas closed.

## Findings

No P1/P2/P3 findings remain in the retained review surface.

The pre-publication review for PR `#4289` found two P3 wording issues:

- stale `blocked_missing_credential` wording in the role-suitability matrix;
- an imprecise closeout precondition that said the nested umbrellas were ready
  after the summary merged, while the actual gate also required the matrix
  truth updates.

Both were fixed before PR `#4289` merged.

## Issue Closure Truth

| Issue | Role | State after review |
| --- | --- | --- |
| `#4095` | Reusable suitability umbrella | closed at 2026-06-20T08:48:20Z |
| `#4096` | DeepSeek suitability proof | closed before this retained review |
| `#4154` | Direct-hosted frontier-model umbrella | closed at 2026-06-20T08:48:22Z |
| `#4155` | Direct-hosted OpenAI/Codex proof | closed before this retained review |
| `#4156` | Direct-hosted Anthropic proof | closed before this retained review |
| `#4157` | Direct-hosted Gemini proof | closed before this retained review |
| `#4158` | Current-model suitability mini-sprint umbrella | closed by PR `#4289` at 2026-06-20T08:48:02Z |
| `#4034` | Observatory logging/OTel/security proof | open; explicitly outside this sprint review |

## Scope Check

The reviewed mini-sprint covers:

- reusable panel and DeepSeek baseline truth from `#4095`, `#4096`, and
  `#4097`;
- direct-hosted current-model proof truth from `#4154`, `#4155`, `#4156`, and
  `#4157`;
- the final rollup and provider-matrix truth repair from `#4158` / PR `#4289`.

It does not include `#4034`, Observatory logging/OTel/security consumption, or
Unity Observatory readiness.

## Validation Evidence

The six retained packet directories were revalidated after PR `#4289` merged:

```text
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/deepseek_suitability
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/openai_current_models
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/openai_current_models_rerun
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/openai_gpt53_codex_rerun
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/anthropic_current_models_adl_demo_key
python3 adl/tools/validate_v0916_agent_suitability_panel.py docs/milestones/v0.91.6/review/provider/gemini_current_models
```

Result: `PASS` for all six packet directories.

PR `#4289` CI:

- `adl-ci`: passed after the PR body was updated with `Closes #4158`;
- `adl-coverage`: passed;
- `adl-slow-proof`: skipped by CI policy.

## Evidence Summary

| Packet | Review conclusion |
| --- | --- |
| `deepseek_suitability/` | Hosted native `deepseek-chat` is `useful_with_limits`; local Ollama DeepSeek lanes remain candidate-only because they overclaimed closeout truth. |
| `openai_current_models/` | Direct-hosted `gpt-5.5` and `gpt-5.4` are `useful_with_limits` for the bounded panel. |
| `openai_current_models_rerun/` | Direct-hosted `gpt-5-codex` is `useful_with_limits`; `gpt-5.3-codex-spark` is `runtime_unsuitable_for_this_panel` because the provider returned `model_not_found`. |
| `openai_gpt53_codex_rerun/` | Direct-hosted `gpt-5.3-codex` is `useful_with_limits`. |
| `anthropic_current_models_adl_demo_key/` | `claude-sonnet-4-6` and `claude-haiku-4-5` are `useful_with_limits`; `claude-opus-4-8` remains `candidate_only_format_repair_needed`. |
| `gemini_current_models/` | `gemini-2.5-pro`, `gemini-2.5-flash`, and `gemini-2.5-flash-lite` are `useful_with_limits`. |

## Non-Claims

- No tested model lane gains merge, release, issue-close, file-write, or
  workflow authority.
- The sprint does not benchmark general model intelligence or broad coding
  quality.
- Direct-hosted, OpenRouter, remote Ollama, and local Ollama evidence remain
  distinct provider surfaces.
- Provider output is not claimed byte-stable across reruns.
- `#4034` remains open and outside this review.

## Closeout Position

The current-model suitability mini-sprint is reviewed and closed. The retained
review surface for future reviewers is this file plus:

- `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`;
- `PROVIDER_MODEL_RELIABILITY_v0.91.6.md`;
- `PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md`;
- the six packet directories listed above.
