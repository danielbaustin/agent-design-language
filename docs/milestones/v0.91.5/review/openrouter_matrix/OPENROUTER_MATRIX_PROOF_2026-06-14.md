# v0.91.5 OpenRouter Matrix Proof Packet

Date: 2026-06-14

Issue: `#3723`

Status: `supported_with_limits`

## Purpose

Record a real bounded native OpenRouter proof lane that goes beyond one smoke request by exercising provider setup, live catalog discovery, five requested route IDs, and one fail-closed missing-credential path.

## What Ran

- `adl provider setup openrouter` into the tracked packet directory.
- `GET https://openrouter.ai/api/v1/models` to snapshot the live catalog.
- Five native OpenRouter provider invocations through `adl-provider-adapter` using requested model IDs, while preserving provider-returned identity metadata in the state packet when present.
- One negative control that omits the required credential and must fail closed.

## Lane Summary

| Lane | Role | Model | Result | Notes |
| --- | --- | --- | --- | --- |
| planner_openrouter_deepseek_v4_flash | planner | `deepseek/deepseek-v4-flash` | supported | structured planner note satisfied contract |
| worker_openrouter_gpt4o_mini | worker | `openai/gpt-4o-mini` | supported | worker lane returned bounded JSON proof step |
| reviewer_openrouter_claude_3_5_haiku | reviewer | `anthropic/claude-3.5-haiku` | supported | reviewer lane returned a bounded finding surface |
| watcher_openrouter_gemini_2_5_flash_lite | watcher | `google/gemini-2.5-flash-lite` | supported | watcher lane returned bounded status/signal/next-step text |
| worker_openrouter_qwen3_6_flash | worker | `qwen/qwen3.6-flash` | supported | worker lane returned bounded JSON proof step |
| negative_missing_credential | negative control | `deepseek/deepseek-v4-flash` | blocked_missing_credential | `provider_auth_missing` |

## Supported Paths

- The native OpenRouter lane completed successfully for all five requested route IDs recorded in the state packet.
- `planner_openrouter_deepseek_v4_flash` satisfied the stricter bounded role contract for requested route `deepseek/deepseek-v4-flash`.
- `worker_openrouter_gpt4o_mini` satisfied the stricter bounded role contract for requested route `openai/gpt-4o-mini`.
- `reviewer_openrouter_claude_3_5_haiku` satisfied the stricter bounded role contract for requested route `anthropic/claude-3.5-haiku`.
- `watcher_openrouter_gemini_2_5_flash_lite` satisfied the stricter bounded role contract for requested route `google/gemini-2.5-flash-lite`.
- `worker_openrouter_qwen3_6_flash` satisfied the stricter bounded role contract for requested route `qwen/qwen3.6-flash`.
- Provider setup is now scaffoldable through `adl provider setup openrouter` and the generated tracked `provider_setup/` bundle.
- Provider-returned identity metadata is recorded in the state packet and may be more specific or normalized relative to the requested route ID.

## Blocked Paths

- The missing-credential negative control normalized to `provider_auth_missing`, proving fail-closed auth behavior rather than hidden fallback.

## Flaky Paths

- No flaky retry or timeout behavior was observed in this bounded run.
- This packet does not elevate that absence into a broad stability claim for untested models or longer prompts.

## Non-Proving Paths

- Prior evidence from `#3415` remains binding for broad planner usefulness: `docs/milestones/v0.91.5/review/multi_agent_workcell/lane_outputs/planner_openrouter_deepseek_v4_flash.md` recorded OpenRouter planner output as useful but generic/off-target in places.
- This packet proves structured route execution, not broad role usefulness across all OpenRouter-backed models.

## Non-Claims

- no universal OpenRouter compatibility claim
- tool-call capability still unproven
- JSON-mode capability still unproven
- no claim that the five tested requested routes generalize to all OpenRouter models or all prompt shapes

## Validation

- `python3 adl/tools/validate_v0915_openrouter_matrix.py docs/milestones/v0.91.5/review/openrouter_matrix`
- `bash adl/tools/test_v0915_openrouter_matrix.sh`

