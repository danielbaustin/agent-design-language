# v0.91.5 Parallel C-SDLC Workcell Proof Packet

Date: 2026-06-14

Issue: `#3415`

Run ID: `v0915-parallel-csdlc-workcell-20260614`

Status: `useful_with_limits`

## Purpose

This packet records a bounded live multi-agent C-SDLC workcell proof for
v0.91.5 Sprint 2. It consumes the provider/model matrix from `#3501` and the
usefulness checklist from `#3504`.

The proof tests whether multiple model/provider lanes can run concurrently,
write disjoint artifacts, and produce reviewable C-SDLC evidence while
conductor, PR publication, review, merge, and closeout authority remain
serialized.

## Proof Summary

Three lanes were started concurrently from the `#3415` worktree:

| Lane | Provider | Model | Role | Output | Result |
| --- | --- | --- | --- | --- | --- |
| planner/critic | OpenRouter | `deepseek/deepseek-v4-flash` | planner/critic | `lane_outputs/planner_openrouter_deepseek_v4_flash.md` | useful but generic/off-target in places |
| worker | local Ollama | `qwen3-coder:30b` | worker | `lane_outputs/worker_local_qwen3_coder_30b.md` | useful output |
| watcher | remote Ollama | `gemma4:e2b` | watcher | `lane_outputs/watcher_remote_gemma4_e2b.md` | completed with empty output |

The run completed in `54.495` seconds wall time. The machine-readable state
packet is:

- `docs/milestones/v0.91.5/review/multi_agent_workcell/v0915_parallel_csdlc_workcell_state_2026-06-14.json`

## What Was Proven

- The workcell can start multiple provider/model lanes concurrently from one
  bounded issue execution.
- Each lane can be assigned a disjoint write path under
  `docs/milestones/v0.91.5/review/multi_agent_workcell/lane_outputs/`.
- The local Qwen coder lane produced useful reviewer-facing evidence
  expectations for multi-agent proof.
- The OpenRouter DeepSeek lane returned quickly and produced partially useful
  safety/gating concepts, but it also drifted into generic systems language.
- The remote Ollama Gemma watcher lane was reachable and completed, but did not
  produce useful watcher text for this prompt.
- Serialized gates remained outside the model lanes:
  conductor admission, pre-PR review, PR publication, merge, and issue closeout.

## Usefulness Checklist Result

Using
`docs/milestones/v0.91.5/review/multi_agent_usefulness/MULTI_AGENT_USEFULNESS_REVIEW_CHECKLIST_2026-06-14.md`,
the run classifies as `useful_with_limits`.

| Checklist area | Result | Evidence |
| --- | --- | --- |
| Issue binding | pass | Work ran under issue `#3415` in a bound v0.91.5 branch/worktree. |
| Role identity | pass | Every lane records provider, model, role, output path, and timing in JSON. |
| Write ownership | pass | Each lane wrote a distinct output file. |
| Serialized gates | pass | No model lane was allowed to review, merge, close, or approve. |
| Evidence packet | pass | Raw lane outputs plus JSON state and this packet are tracked. |
| Non-claims | pass | This packet does not claim broad multi-agent usefulness or autonomous closeout. |
| Parallel throughput | partial | Lanes overlapped, but remote watcher latency dominated wall time. |
| Quality improvement | partial | Qwen worker output was useful; planner output was partially useful; watcher output was empty. |
| Cognitive diversity | partial | Different providers behaved differently enough to be informative. |
| Lower operator burden | not proven | Human synthesis was still required to classify usefulness and limitations. |
| Bounded autonomy | pass | Lanes stayed inside assigned output files. |
| Reviewability | pass | Artifacts are durable and reconstructable from tracked state. |

## Timing And Overlap

The concurrent run started all lanes at `2026-06-14T09:03:10Z`.

| Lane | End time | Duration |
| --- | --- | --- |
| OpenRouter DeepSeek planner/critic | `2026-06-14T09:03:13Z` | `3.404s` |
| local Qwen worker | `2026-06-14T09:03:30Z` | `20.448s` |
| remote Gemma watcher | `2026-06-14T09:04:05Z` | `54.490s` |

This proves scheduling overlap, but it also shows that slow or unhelpful lanes
can dominate operator wait time. `#3503` must compare this against a
single-agent path before Sprint 2 claims net speedup.

## Findings

- Local Qwen coder is a plausible worker lane for evidence/checklist drafting.
- OpenRouter DeepSeek is a plausible fast planner/critic lane, but role prompts
  need tighter ADL grounding to avoid generic drift.
- Remote Gemma is not yet a useful watcher lane for this prompt shape despite
  successful remote Ollama availability.
- The disjoint-output model is workable and reviewable.
- Multi-agent proof is not yet strong enough to remove the single-agent
  fallback rule.

## Non-Claims

- This packet does not prove that multi-agent execution is faster than
  single-agent execution.
- This packet does not prove remote Qwen or remote DeepSeek role usefulness.
- This packet does not prove OpenRouter tool-call or JSON-mode usefulness.
- This packet does not grant any model lane merge, review-approval, or closeout
  authority.
- This packet does not replace `#3503` overhead comparison.

## Required Follow-Up

- `#3503` must compare this proof against a single-agent path for time,
  quality, and operator overhead.
- Remote watcher prompts or model selection need refinement before remote
  Gemma is reused for janitor/watcher duties.
- OpenRouter planner prompts need stronger ADL-specific anchoring before
  planner usefulness is claimed broadly.
- Sprint 2 closeout must classify this as `useful_with_limits`, not fully
  proven multi-agent usefulness.

## Validation

Focused validation for this packet should include:

- JSON parse of
  `docs/milestones/v0.91.5/review/multi_agent_workcell/v0915_parallel_csdlc_workcell_state_2026-06-14.json`.
- Path existence for all lane output files.
- Leakage scan over the proof packet, lane outputs, and state JSON.
- Overclaim scan confirming no broad autonomous or fully-proven claims.
- `git diff --check`.
