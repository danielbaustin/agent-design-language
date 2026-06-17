# v0.91.5 Provider / Model Role Matrix

Date: 2026-06-05

Issue: `#3501`

Related issues: `#3505`, `#3504`, `#3415`, `#3503`

Status: `bounded_matrix_ready_live_proof_partial`

OpenRouter follow-on: `#3723` now upgrades the hosted gateway evidence with a
dedicated packet at
`docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md`.
This June 5 packet remains the historical baseline rather than the current
strongest OpenRouter proof surface.

Redaction note: private LAN host/IP/port details in public evidence are
normalized to `remote_ollama_private_lan` or `remote_ollama_private_lan_stale`
to preserve proof semantics without publishing local network coordinates.

## Purpose

This packet records the provider/model readiness matrix for the v0.91.5
multi-agent C-SDLC proof lane. It separates:

- implemented provider substrate;
- locally available models;
- live provider lanes that were executable or blocked in this shell;
- blocked lanes that require operator repair or credentials;
- downstream issues that must still prove usefulness and parallel workcell
  behavior.

It does not claim that multi-agent execution is complete.

## Current Evidence Snapshot

The following checks were run from the `#3501` worktree.

| Surface | Command / check | Result | Evidence boundary |
| --- | --- | --- | --- |
| Hosted credentials | Env-presence check for `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GEMINI_API_KEY`, `DEEPSEEK_API_KEY`, `OPENROUTER_API_KEY` | Direct hosted env vars missing in this shell | Presence only; no secret values printed. OpenRouter was later provided through an operator-supplied credential loaded ephemerally for live smoke. |
| OpenRouter catalogue | `GET https://openrouter.ai/api/v1/models` | Available | Returned 337 models, including DeepSeek V4 candidates. Credential value was not printed or recorded. |
| OpenRouter live smoke | `POST https://openrouter.ai/api/v1/chat/completions` with `deepseek/deepseek-v4-flash` | Passed | Response model `deepseek/deepseek-v4-flash-20260423`; sentinel `ADL_OPENROUTER_SMOKE_OK`; usage 80 tokens; reported cost `0.00001955` USD. |
| Local Ollama | `ollama list` | Available | Model names and sizes only. |
| Remote Ollama stale node | `ssh -o BatchMode=yes -o ConnectTimeout=5 remote_ollama_private_lan_stale 'ollama list'`; `GET remote_ollama_private_lan_stale/api/tags` | Blocked | SSH host-key verification changed and Ollama HTTP refused; no bypass attempted. |
| Remote Ollama current node | `GET remote_ollama_private_lan/api/tags` | Available | Returned 18 models on the current remote Ollama node. |
| Remote Ollama generation smoke | `POST remote_ollama_private_lan/api/generate` | Partial pass | `gemma4:e2b` returned sentinel `ADL_REMOTE_OLLAMA_SMOKE_OK`; `qwen3:30b` and `deepseek-r1:8b` completed but returned empty sentinel responses. |
| OpenRouter substrate | `#3505` / PR `#3686` | Implemented and merged | Native provider substrate exists; this packet also records one direct OpenRouter API smoke for matrix readiness. |

## Available Local Model Panel

Local Ollama returned the following role-relevant families:

| Family | Observed local model(s) | Candidate role fit | Status |
| --- | --- | --- | --- |
| Qwen general / reasoning | `qwen3.6:27b`, `Qwen3.5:35b-a3b`, `Qwen3.5:9b`, `qwq:32b`, `qwen3:14b` | planner, author, reviewer, watcher | available_local |
| Qwen coder | `qwen3-coder:30b`, `qwen2.5-coder:32b` | worker, code-review helper | available_local |
| DeepSeek reasoning | `deepseek-r1:8b`, `deepseek-r1:32b`, `deepseek-r1:latest` | reviewer, critic, hard-case reasoner | available_local |
| Gemma family | `gemma4:31b`, `gemma4:26b`, `gemma4:e4b`, `gemma4:e2b`, `gemma3:latest`, `gemma2:2b` | watcher, summarizer, secondary reviewer | available_local |
| Mistral family | `mistral-small3.2:24b`, `mistral-nemo:latest` | reviewer, summarizer | available_local |
| Llama / Phi / Granite | `llama3.3:70b`, `llama3.1:8b`, `phi4-mini:latest`, `phi4-reasoning:latest`, `granite3.1-moe:latest` | fallback worker/reviewer/watcher | available_local |
| Embeddings | `nomic-embed-text:latest` | retrieval/indexing support | available_local |

## Provider Lane Status

| Lane | Provider / substrate | Candidate role use | Status | Required next action |
| --- | --- | --- | --- | --- |
| Hosted Codex / OpenAI | Native `openai` provider | planner, worker, reviewer | blocked_missing_credential | Re-run with `OPENAI_API_KEY` when live hosted proof is required. |
| Anthropic / Claude | Native `anthropic` provider | reviewer, critic, synthesis | blocked_missing_credential | Re-run with `ANTHROPIC_API_KEY`. |
| Gemini | Compatibility/profile lane | reviewer, alternate synthesis | blocked_missing_credential | Re-run with `GEMINI_API_KEY`; do not treat as native provider until implemented. |
| DeepSeek hosted API | Native `deepseek` provider from `#3549` | critic, reviewer, reasoning worker | blocked_missing_credential | Re-run with `DEEPSEEK_API_KEY`; keep distinct from Ollama DeepSeek models. |
| OpenRouter | Native `openrouter` provider from `#3505` | broad hosted model routing | follow_on_role_proof_recorded | DeepSeek V4 Flash routed successfully here as smoke; `#3723` later added four exact native routed role probes and a fail-closed missing-credential control. Broad role usefulness and capability details still require `#3504`, `#3415`, and `#3503`. |
| Local Ollama | Local `ollama list` | Qwen + DeepSeek local team | available_for_bounded_local_probe | Use `#3504`, `#3415`, and `#3503` to test actual usefulness and overhead. |
| Remote Ollama | `remote_ollama_private_lan` | remote local-model capacity | live_inventory_and_watcher_smoke_passed | Current node inventory passed and `gemma4:e2b` sentinel generation passed; Qwen/DeepSeek remote role probes still need prompt/model-specific follow-up. |

## Available Remote Model Panel

Remote Ollama on the current `remote_ollama_private_lan` node returned the following
role-relevant families:

| Family | Observed remote model(s) | Candidate role fit | Status |
| --- | --- | --- | --- |
| Qwen general / coder | `qwen2.5:32b`, `qwen2.5-coder:32b`, `qwen3-coder:30b`, `qwen3:30b` | planner, author, worker | inventory_available_generation_not_yet_useful |
| DeepSeek reasoning | `deepseek-r1:32b`, `deepseek-r1:8b` | reviewer, critic | inventory_available_generation_not_yet_useful |
| Gemma family | `gemma4:31b`, `gemma4:26b`, `gemma4:e4b`, `gemma4:e2b`, `gemma4:latest`, `gemma3:4b`, `gemma3:27b` | watcher, summarizer, secondary reviewer | watcher_smoke_passed |
| GPT-OSS / Llama | `gpt-oss:latest`, `gpt-oss:120b`, `llama4:16x17b`, `llama2-chinese:latest` | fallback worker/reviewer/watcher | inventory_available |

## Role Assignment Matrix

| C-SDLC role | Primary candidate | Secondary candidate | Status | Notes |
| --- | --- | --- | --- | --- |
| Planner | Hosted Codex/OpenAI if credentialed; otherwise local Qwen general | OpenRouter `deepseek/deepseek-v4-flash`; `qwen3.6:27b`, `Qwen3.5:35b-a3b` | partial_ready | OpenRouter live smoke is available, but planner aptitude is not yet proven. |
| Author / issue creator | Hosted Codex/OpenAI if credentialed; otherwise local Qwen general | OpenRouter `deepseek/deepseek-v4-flash`; `qwen3-coder:30b` for code-heavy cards | partial_ready | Must use current prompt templates and `pr.sh init`; no hand-rolled cards. |
| Worker | Local Qwen coder or hosted Codex | OpenRouter `deepseek/deepseek-v4-flash`; `qwen2.5-coder:32b`, `qwen3-coder:30b` | partial_ready | Needs bounded issue with disjoint write surface. |
| Reviewer | DeepSeek reasoning or Claude if credentialed | OpenRouter `deepseek/deepseek-v4-flash`; `deepseek-r1:32b`, `deepseek-r1:8b` | partial_ready | Review must produce severity-ranked findings, not approval theater. |
| Watcher / janitor | Gemma/Mistral small-to-mid local or remote model | remote `gemma4:e2b`, local `gemma4:e4b`, `mistral-nemo:latest` | partial_ready | Remote Gemma sentinel smoke passed; watcher usefulness still needs real issue-status prompt proof. |
| Closeout authority | Human/team plus ADL closeout tool | n/a | serialized_authority | No model lane may self-merge or self-close. |

## Downstream Proof Routing

| Issue | Purpose | How it consumes this matrix |
| --- | --- | --- |
| `#3504` | Reviewer checklist for multi-agent usefulness | Converts the role matrix into go/no-go review criteria. |
| `#3415` | Parallel C-SDLC workcell proof | Uses role candidates and lane statuses to bound workcell proof. |
| `#3503` | Single-agent vs multi-agent overhead comparison | Measures whether the multi-agent lane is useful on one small docs audit. |
| `#3572` | Sprint 2 umbrella | Tracks whether provider matrix and multi-agent proof are complete, blocked, or deferred. |

## Non-Claims

- This packet proves only one OpenRouter live smoke, not broad hosted model
  execution.
- This packet does not prove OpenRouter role usefulness, tool calls, JSON mode,
  or multi-model routing quality.
- This packet does not prove broad remote Ollama execution.
- This packet proves only one remote Ollama watcher-class smoke, not Qwen or
  DeepSeek role usefulness on the remote node.
- This packet does not prove multi-agent usefulness.
- This packet does not authorize any agent to merge, close issues, or bypass
  the C-SDLC card lifecycle.

## Required Follow-Up Before Sprint 2 Closeout

- `#3504` must define the usefulness review checklist.
- `#3415` must record actual workcell proof or a truthful blocker.
- `#3503` must record single-agent vs multi-agent overhead and quality.
- Remote Ollama Qwen/DeepSeek role probes must be repeated with role-specific
  prompts before planner/worker/reviewer usefulness is claimed.
- Direct hosted lanes must either be re-run with credentials or marked
  skipped/blocked in the Sprint 2 closeout evidence.
- OpenRouter smoke-only proof has now been upgraded by `#3723`, but broad role
  usefulness still depends on the later multi-agent packets rather than this
  provider-only lane.
