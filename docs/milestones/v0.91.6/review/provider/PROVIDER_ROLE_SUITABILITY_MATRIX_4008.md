# Provider Role Suitability Matrix Proof Note for #4008

## Scope

This note records the bounded proof surface for `#4008`. It turns the WP-05
provider/model evidence into a role-suitability matrix without claiming broad
model reliability, training readiness, or autonomous workflow authority.

## Source evidence

- `#4007` PR-pending proof note `PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`
  from PR `#4063`
- `docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md`
- `docs/milestones/v0.91.5/features/MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md`
- `docs/milestones/v0.91.5/review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md`
- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md`
- `docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`
- `docs/milestones/v0.91.6/review/provider/deepseek_suitability/DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md`

## What this issue establishes

`#4008` establishes a role-scoped suitability layer above the provider and
capability split from `#4007`.

It records:

- which lanes are `supported_with_limits`
- which lanes are `useful_with_limits`
- which lanes remain `blocked_missing_credential`
- which lanes are only inventory-known candidates
- which historical weak results must stay visible instead of being overwritten

## Key truth corrections captured here

1. OpenRouter route proof is stronger than the historical June 5 baseline.
   The five native requested-route proofs from `#3723` are now the strongest
   evidence for bounded planner/worker/reviewer/watcher route support.
2. Remote Gemma watcher truth has improved, but not universally.
   The historical `gemma4:e2b` watcher lane remains an empty-output fact, while
   larger Gemma4 routes now prove bounded useful watcher output.
3. Hosted direct-provider lanes remain shell-dependent.
   The v0.91.5 baseline cited here recorded direct hosted OpenAI, Anthropic,
   DeepSeek, and Gemini-native proof as credential-blocked in that run, so this
   issue does not silently upgrade them to reliable defaults.
4. DeepSeek now has a bounded direct-provider follow-on.
   The `#4096` panel shows hosted native DeepSeek as `useful_with_limits`, while
   local `deepseek-r1:8b` and `deepseek-r1:32b` remain candidate-only because
   closeout truth drift persisted in the local bounded panel.

## Strongest role recommendations from current evidence

| Role | Strongest current lane | Classification | Why |
| --- | --- | --- | --- |
| planning | OpenRouter `deepseek/deepseek-v4-flash` | `supported_with_limits` | Native requested-route proof exists, but broad planner usefulness remains task-shaped |
| coding | OpenRouter `openai/gpt-4o-mini` and `qwen/qwen3.6-flash` | `supported_with_limits` | Bounded worker route proofs exist without overclaiming broad code-quality reliability |
| review | OpenRouter `anthropic/claude-3.5-haiku` | `supported_with_limits` | Reviewer-class bounded finding surface is proven |
| summarization / watcher | Remote `gemma4:31b` via `adl-provider-adapter` | `useful_with_limits` | Strongest bounded watcher proof uses the real ADL provider path |
| orchestration | advisory combination only | `limited_advisory_only` | No provider/model lane gains execution authority or closeout authority |

## Non-claims preserved

- This issue does not prove universal provider/model reliability.
- This issue does not promote every Gemma size or prompt shape.
- This issue does not prove tool-call or JSON-mode support across all
  OpenRouter-backed models.
- This issue does not let role routing bypass human/team merge and closeout
  authority.
- This issue does not merge the `#4007` catalog work; that dependency remains
  PR-pending while this matrix cites it explicitly.

## Follow-on routing

- `#4009` should prove Gemma/OpenRouter reliability more directly where issue
  acceptance requires it.
- `#4010` should translate these lane limits into provider failure-mode and
  resilience routing.
- `#4053` should define durable C-SDLC role-provider profiles using this matrix
  as role evidence rather than re-deriving provider truth.
- `#4012` should use this issue as one input to the final closeout matrix.
