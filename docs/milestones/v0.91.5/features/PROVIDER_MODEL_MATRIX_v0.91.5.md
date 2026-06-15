# Provider Model Matrix v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `openrouter_follow_on_evidence_active`
- Related issues: `#3501`, `#3505`
- Current evidence packet:
  [OPENROUTER_MATRIX_PROOF_2026-06-14.md](../review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md)
- Historical baseline packet:
  [PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md](../review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md)

## Template Rules

This is a planning feature doc, not a provider benchmark result.

## Purpose

Define the provider/model matrix needed to test multi-agent C-SDLC roles.

## Context

ADL needs to test hosted models, local Ollama models, remote Ollama models, and
OpenRouter-backed models without blurring provider identity or authority.

## Coverage / Ownership

This feature owns matrix planning and evidence expectations for model-role
testing.

## Overview

The matrix should cover planner, worker, reviewer, janitor, and watcher roles
across direct hosted providers, local Ollama, remote AI node Ollama, and
OpenRouter.

## Native Provider Status

| Provider family | Native ADL type | Credential env | Default endpoint | Status |
| --- | --- | --- | --- | --- |
| OpenAI | `openai` | `OPENAI_API_KEY` | `https://api.openai.com/v1/responses` | implemented |
| Anthropic | `anthropic` | `ANTHROPIC_API_KEY` | `https://api.anthropic.com/v1/messages` | implemented |
| DeepSeek | `deepseek` | `DEEPSEEK_API_KEY` | `https://api.deepseek.com/chat/completions` | implemented in `#3549` |
| OpenRouter | `openrouter` | `OPENROUTER_API_KEY` | `https://openrouter.ai/api/v1/chat/completions` | implemented in `#3505`; `#3723` now proves five requested native route probes plus fail-closed missing-credential behavior |
| Gemini | none yet | `GEMINI_API_KEY` | n/a | compatibility/profile lane only |

Compatibility profiles such as `http:deepseek-chat` remain useful for gateway
tests, but they must not be confused with the native `deepseek` or
`openrouter` provider paths.

## Design

- Record provider, model, transport, credentials posture, and role.
- Keep skipped and blocked tests explicit.
- Do not treat tool availability as authority.
- Prefer aptitude evidence over general model reputation.

## Execution Flow

1. Inventory available local and remote models.
2. Use the native OpenRouter provider for broad hosted model-lane probes.
3. Test small role-specific prompts.
4. Feed results into multi-agent execution planning.

## Determinism and Constraints

Results must be reproducible enough for review and must not expose secrets or
private prompt content.

## Integration Points

- [MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md](MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md)
- [../V092_ACTIVATION_TEST_MAP_v0.91.5.md](../V092_ACTIVATION_TEST_MAP_v0.91.5.md)

## Validation

Validation should include provider smoke tests, role probes, skipped-state
records, and explicit separation between OpenRouter substrate proof and live
model-matrix evidence.

OpenRouter capability support is model-dependent. The native provider preserves
the OpenRouter lane and requested route identifier, but role probes must
record any native tool/JSON capability as explicit lane evidence or a configured
capability override rather than assuming gateway-wide support.

The first v0.91.5 matrix packet recorded local Ollama availability, one
OpenRouter live smoke through `deepseek/deepseek-v4-flash`, direct hosted lanes
blocked where provider credentials were unavailable in the `#3501` shell, and
remote Ollama inventory plus one Gemma watcher-class smoke on the current
`nessus.local` node. The follow-on `#3723` packet upgrades the OpenRouter lane
to a bounded native requested-route proof across:

- `deepseek/deepseek-v4-flash`
- `openai/gpt-4o-mini`
- `anthropic/claude-3.5-haiku`
- `google/gemini-2.5-flash-lite`
- `qwen/qwen3.6-flash`

and also proves fail-closed auth behavior through a missing-credential negative
control. See
[OPENROUTER_MATRIX_PROOF_2026-06-14.md](../review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md).

## Acceptance Criteria

- Hosted, local Ollama, remote Ollama, and OpenRouter lanes are separately
  represented.
- At least one useful candidate per C-SDLC role is identified or blocked.

## Risks

- Provider access may be unavailable.
- Model behavior may be too unstable for a role.

## Future Work

Future milestones can automate aptitude selection and cost/latency-aware model
routing.

## Notes

The matrix informs role selection; it does not grant execution authority.
