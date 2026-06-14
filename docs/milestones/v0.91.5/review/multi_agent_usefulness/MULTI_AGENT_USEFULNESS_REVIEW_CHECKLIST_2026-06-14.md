# Multi-Agent Usefulness Review Checklist

Date: 2026-06-14

Issue: `#3504`

Consumes: `#3501`, `#3505`, `#3415`, `#3503`, prior `#3484` evidence

Status: `ready_for_sprint_2_use`

## Purpose

Use this checklist to decide whether a multi-agent C-SDLC run is genuinely
useful or merely adding coordination overhead. It is intentionally small,
evidence-bound, and conservative.

The checklist is calibrated for the v0.91.5 Sprint 2 lane: local Qwen and
DeepSeek-family models, OpenRouter-routed hosted models, remote Ollama capacity,
and serialized human/tooling authority for review, merge, and closeout.

## Review Inputs

A reviewer should inspect these inputs before scoring usefulness:

- `docs/milestones/v0.91.5/review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md`
- `docs/milestones/v0.91.4/review/multi_agent_workcell/MULTI_AGENT_CSDLC_WORKCELL_PROOF_PACKET_2026-05-28.md`
- `docs/milestones/v0.91.4/review/multi_agent_workcell/CODEX_ONLY_COMPLETE_ISSUE_WORKCELL_PROOF_PACKET_2026-05-29.md`
- issue-local `SIP`, `STP`, `SPP`, `SRP`, and `SOR` for the workcell under review
- PR diff, checks, review comments, and closeout truth for the workcell under review

## Hard Gates

If any hard gate fails, classify the run as `not_useful_yet` or `blocked`.

| Gate | Pass condition | Fail condition |
| --- | --- | --- |
| Issue binding | Every role is tied to a concrete issue, branch/worktree, card bundle, and PR or no-PR disposition. | Work exists only in chat, an unbound checkout, or an untracked scratch area. |
| Role identity | Every role records provider, model, substrate, and authority boundary. | A role is described only as "agent", "AI", or "reviewer" without model/provider identity. |
| Write ownership | Parallel workers have disjoint file ownership or an explicit merge protocol. | Multiple workers edit the same surface without a planned handoff. |
| Serialized gates | Review, merge, issue closeout, and authority decisions remain serialized unless a tracked issue explicitly changes policy. | A model lane self-approves, self-merges, or silently closes work. |
| Evidence packet | The run produces durable docs/JSON/SOR evidence that a reviewer can inspect later. | The claimed proof depends on session memory or unreviewable local state. |
| Non-claims | The record names what was not proven. | The run claims general multi-agent usefulness from a smoke test or partial shard. |

## Usefulness Signals

Score these signals as `pass`, `partial`, `fail`, or `not_applicable`.

| Signal | Useful evidence | Overhead evidence |
| --- | --- | --- |
| Parallel throughput | Two or more roles produce independent useful artifacts faster than one serialized agent would. | Coordination takes longer than a single agent completing the same slice. |
| Quality improvement | Reviewer/critic role finds issues the worker would likely miss, or improves acceptance clarity. | Reviewer role repeats the worker summary or rubber-stamps without findings/non-findings. |
| Cognitive diversity | Different model families contribute different strengths, such as Qwen planning/coding and DeepSeek critique. | Models produce interchangeable summaries or require heavy human repair. |
| Lower operator burden | The human spends less time coordinating than they would doing the work directly. | The human spends most of the time fixing cards, prompts, branches, or conflicting edits. |
| Bounded autonomy | Worker lanes complete assigned slices without crossing authority boundaries. | Worker lanes need repeated rescue or drift into unrelated work. |
| Reviewability | The final packet makes it easy to reconstruct roles, decisions, commands, and residual risks. | The result requires chat archaeology to understand what happened. |

## Role-Specific Checks

| Role | Minimum useful behavior | Current v0.91.5 candidate lanes | Reviewer warning signs |
| --- | --- | --- | --- |
| Planner | Produces issue-local plan, dependencies, write ownership, and stop conditions. | Hosted Codex/OpenAI when credentialed; local Qwen; OpenRouter DeepSeek V4 Flash as smoke-proven only. | Plan is generic, misses issue graph, or ignores card lifecycle. |
| Author / issue creator | Creates or updates cards through templates/tools without hand-rolling locked prose. | Hosted Codex/OpenAI when credentialed; local Qwen; Qwen coder for code-heavy cards. | Cards are generic, stale, or edited outside renderer/editor discipline. |
| Worker | Produces bounded artifact changes on owned paths with focused validation. | Local Qwen coder; hosted Codex; OpenRouter as future role probe. | Changes overlap another lane, widen scope, or require heavy manual cleanup. |
| Reviewer / critic | Produces severity-ranked findings or explicit non-findings with evidence. | Local/remote DeepSeek inventory; Claude/OpenAI when credentialed; OpenRouter as future role probe. | Review praises instead of inspecting evidence, or reports unsupported scores. |
| Watcher / janitor | Reports status, blockers, and check outcomes without changing authority state. | Local Gemma/Mistral; remote `gemma4:e2b` smoke-proven watcher candidate. | Watcher closes, merges, approves, or edits implementation artifacts. |

## Provider / Model Evidence Rules

- Local Ollama inventory is enough to select a candidate, not enough to claim
  role usefulness.
- OpenRouter `deepseek/deepseek-v4-flash` has one live smoke in `#3501`; it
  still needs role-specific prompt evidence before planner/worker/reviewer
  usefulness is claimed.
- Remote `nessus.local` has inventory and one `gemma4:e2b` watcher-class smoke;
  remote Qwen and DeepSeek probes are inventory-only until useful non-empty
  role outputs are captured.
- Direct hosted OpenAI, Anthropic, Gemini, and native DeepSeek lanes remain
  blocked/skipped when credentials are unavailable and must not be inferred
  from OpenRouter evidence.

## Go / No-Go Classification

| Classification | When to use it | Required routing |
| --- | --- | --- |
| `useful` | Hard gates pass, at least three usefulness signals pass, and no high-severity review finding remains unresolved. | Continue multi-agent lane and record the evidence in `#3415` / `#3503`. |
| `useful_with_limits` | Hard gates pass, at least one role is clearly useful, but overhead or model quality remains mixed. | Continue only on bounded slices; name the limits and next probe. |
| `not_useful_yet` | Hard gates pass but quality, speed, or operator burden is worse than single-agent work. | Fall back to single-agent execution and route model/prompt improvements. |
| `blocked` | A hard gate fails or required provider/model evidence is missing. | Stop the multi-agent claim and open/fix the blocker before reuse. |

## Fallback Rules

Use a single-agent lane when:

- the work has one obvious owner and no independent review benefit;
- the expected output is smaller than the coordination packet;
- model lanes require repeated prompt repair to produce usable output;
- write ownership cannot be separated cleanly;
- closeout or review tooling is already the bottleneck.

Use multi-agent only when:

- disjoint work surfaces exist;
- review or critique quality is expected to improve the result;
- provider/model identity can be recorded;
- the human operator is not forced into constant rescue;
- the final SOR can explain the run without session memory.

## Minimum Evidence For Sprint 2 Closeout

Before v0.91.5 Sprint 2 claims multi-agent usefulness, it needs:

- a `#3415` workcell proof with role/model identity, shard ownership, review,
  validation, and closeout truth;
- a `#3503` comparison showing whether multi-agent overhead was worth it on a
  small docs audit;
- explicit blocked/skipped records for provider lanes that were not exercised;
- no unresolved hard-gate failure from this checklist.

## Non-Claims

- This checklist does not prove multi-agent usefulness by itself.
- This checklist does not grant merge, closeout, or approval authority to any
  model lane.
- This checklist does not claim remote Qwen or DeepSeek role usefulness yet.
- This checklist does not replace issue-local SRP/SOR review truth.
