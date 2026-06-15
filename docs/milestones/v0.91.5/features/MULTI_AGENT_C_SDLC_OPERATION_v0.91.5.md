# Multi-Agent C-SDLC Operation v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-06-14`
- Owner: ADL maintainers
- Status: `active_sprint_2_evidence`
- Related issues: `#3415`, `#3501`, `#3503`, `#3504`
- Prior satisfied evidence: `#3484`
- Current provider/model evidence:
  [OPENROUTER_MATRIX_PROOF_2026-06-14.md](../review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md)
- Historical provider/model baseline:
  [PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md](../review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md)
- Current usefulness checklist:
  [MULTI_AGENT_USEFULNESS_REVIEW_CHECKLIST_2026-06-14.md](../review/multi_agent_usefulness/MULTI_AGENT_USEFULNESS_REVIEW_CHECKLIST_2026-06-14.md)
- Current workcell proof:
  [V0915_PARALLEL_C_SDLC_WORKCELL_PROOF_PACKET_2026-06-14.md](../review/multi_agent_workcell/V0915_PARALLEL_C_SDLC_WORKCELL_PROOF_PACKET_2026-06-14.md)
- Current overhead comparison:
  [MULTI_AGENT_OVERHEAD_COMPARISON_2026-06-14.md](../review/multi_agent_overhead/MULTI_AGENT_OVERHEAD_COMPARISON_2026-06-14.md)

## Template Rules

This is a planning feature doc, not implementation evidence.

## Purpose

Define the bounded multi-agent operating surface that must work before v0.91.5
closes.

## Context

Single-threaded sprints are useful but too slow for the long-term C-SDLC. The
multi-agent lane must prove governed parallelism without hiding state or
weakening review.

## Coverage / Ownership

This feature owns role, shard, coordination, review, and closeout expectations
for multi-agent C-SDLC execution.

## Overview

The target workcell includes planner, worker, reviewer, janitor/closeout, and
watcher roles. Roles may use different models based on aptitude, but all work
must remain bound to issues, cards, branches, reviews, and PR truth.

## Design

- Each role records provider/model identity.
- Shards have explicit ownership and interface boundaries.
- Review and merge/closeout gates remain serialized where needed.
- Multi-agent benefit is measured against single-agent overhead.

## Execution Flow

1. Build or verify role/shard planning.
2. Select models from the provider/model matrix.
3. Execute a bounded workcell proof.
4. Review usefulness and overhead.
5. Record completion or blocker truth.

## Determinism and Constraints

No agent gets hidden authority. No role bypasses card, review, branch, or
closeout truth.

## Integration Points

- [../SPRINT_v0.91.5.md](../SPRINT_v0.91.5.md)
- [../DEMO_MATRIX_v0.91.5.md](../DEMO_MATRIX_v0.91.5.md)
- [PROVIDER_MODEL_MATRIX_v0.91.5.md](PROVIDER_MODEL_MATRIX_v0.91.5.md)

## Validation

Validation should include a bounded proof packet, role records, shard records,
timing/overhead comparison, and reviewer checklist.

The `#3501` provider/model role matrix remains the baseline availability packet:
local Ollama role candidates are available, direct hosted provider lanes were
credential-blocked in that shell, and the current remote Ollama node passed
inventory plus one Gemma watcher-class smoke. `#3723` upgrades the OpenRouter
lane from one smoke to five requested native OpenRouter route probes
(`deepseek/deepseek-v4-flash`, `openai/gpt-4o-mini`,
`anthropic/claude-3.5-haiku`, `google/gemini-2.5-flash-lite`,
`qwen/qwen3.6-flash`) plus a
fail-closed missing-credential negative control. Broad role usefulness,
tool-call capability, and JSON-mode capability across OpenRouter models remain
explicit non-claims.

The `#3504` reviewer checklist defines hard gates, usefulness signals,
role-specific warning signs, and single-agent fallback rules for evaluating
`#3415` and `#3503`.

The `#3415` live workcell proof classifies the current lane as
`useful_with_limits`: local Qwen produced useful worker evidence, OpenRouter
DeepSeek produced partially useful planner/critic evidence, and remote Gemma
completed with empty watcher output. The `#3503` overhead comparison records
that single-agent execution is preferred for tiny docs audits, while
multi-agent remains useful for disjoint surfaces or review-quality tasks when
the extra coordination cost is justified.

Follow-on issue `#3724` now adds a bounded recovery proof for the remote watcher
lane at
`docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`.
That packet keeps the historical `#3415` empty-output fact intact while proving
that larger remote Gemma4 routes (`gemma4:31b`, `gemma4:26b`, and `gemma4:e4b`)
can return useful short watcher summaries, with `gemma4:31b` proven through the
real `adl-provider-adapter` surface. This still does not prove broad
multi-agent quality; `#3725` remains the comparative baseline issue.

## Acceptance Criteria

- Multi-agent C-SDLC executes a bounded issue/sprint slice or blocks truthfully.
- Role and model identity are visible.
- Review and closeout truth are preserved.

## Risks

- Multi-agent overhead may exceed benefit on small tasks.
- Local models may be weak for some roles.

## Future Work

Future milestones can expand from bounded workcells to richer Software
Development Polis operation.

## Notes

This feature does not claim unbounded autonomous development.
