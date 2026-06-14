# Multi-Agent Overhead Comparison

Date: 2026-06-14

Issue: `#3503`

Compared evidence: `#3415`

Status: `single_agent_preferred_for_tiny_docs_audit`

## Purpose

This packet compares a bounded single-agent docs audit against the live
multi-agent C-SDLC workcell proof recorded by `#3415`.

The comparison answers a narrow question: for one small docs-audit slice, did
multi-agent execution reduce time or improve quality enough to justify the
coordination overhead?

## Task Definition

Audit the `#3415` parallel workcell evidence for usefulness and overhead:

- proof packet:
  `docs/milestones/v0.91.5/review/multi_agent_workcell/V0915_PARALLEL_C_SDLC_WORKCELL_PROOF_PACKET_2026-06-14.md`
- machine state:
  `docs/milestones/v0.91.5/review/multi_agent_workcell/v0915_parallel_csdlc_workcell_state_2026-06-14.json`
- lane outputs:
  `docs/milestones/v0.91.5/review/multi_agent_workcell/lane_outputs/`
- usefulness checklist:
  `docs/milestones/v0.91.5/review/multi_agent_usefulness/MULTI_AGENT_USEFULNESS_REVIEW_CHECKLIST_2026-06-14.md`

Both paths evaluate the same evidence surface. The single-agent path did not
make live provider calls; it inspected the tracked `#3415` evidence and
produced this synthesis.

## Single-Agent Path

Topology: one Codex reviewer in the `#3503` issue worktree.

Measured window: `2026-06-14T09:52:10Z` to `2026-06-14T09:54:29Z`

Wall duration: `139s`

Work performed:

- inspected the `#3415` proof packet;
- inspected the `#3415` JSON state;
- inspected all three lane outputs;
- inspected the `#3504` usefulness checklist;
- synthesized findings and comparison outcome.

Findings:

- The `#3415` packet accurately records the workcell as
  `useful_with_limits`.
- The local Qwen worker lane produced useful checklist/evidence expectations.
- The OpenRouter DeepSeek planner/critic lane returned quickly, but the text
  drifted into generic systems language and was cut off mid-thought.
- The remote Gemma watcher lane completed but produced empty output.
- For this tiny docs audit, a single reviewer produced the same usefulness
  classification faster and with less coordination overhead.

## Multi-Agent Path

Topology: three concurrent lanes from the `#3415` worktree.

Measured window: `2026-06-14T09:03:10Z` to `2026-06-14T09:04:05Z`

Wall duration: `54.495s`

Lane summary:

| Lane | Provider / model | Role | Duration | Result |
| --- | --- | --- | --- | --- |
| planner/critic | OpenRouter `deepseek/deepseek-v4-flash` | planner/critic | `3.404s` | useful but generic/off-target in places |
| worker | local Ollama `qwen3-coder:30b` | worker | `20.448s` | useful output |
| watcher | remote Ollama `gemma4:e2b` | watcher | `54.490s` | completed with empty output |

The `#3415` packet correctly proves concurrent scheduling, disjoint write
paths, provider/model identity recording, and serialized authority gates.

## Overhead Comparison

| Dimension | Single-agent path | Multi-agent path | Result |
| --- | --- | --- | --- |
| Wall time | `139s` for inspection and synthesis | `54.495s` for live lane execution, plus human synthesis | multi-agent faster only for raw lane execution |
| Coordination cost | low; one reviewer, one synthesis surface | medium; three lane prompts, output files, JSON state, synthesis | single-agent simpler |
| Output quality | focused comparison with direct findings | mixed: one useful worker, one partial planner, one empty watcher | single-agent better for this tiny audit |
| Evidence diversity | one reviewer perspective | three provider/model lanes with different behavior | multi-agent better diversity |
| Operator burden | low after evidence existed | higher because empty/partial lanes required classification | single-agent lower burden |
| Reuse value | comparison packet only | provider/model lane evidence reusable by later work | multi-agent better reusable substrate |

## Net Usefulness

Classification: `single_agent_preferred_for_tiny_docs_audit`

Multi-agent was useful as a provider/model workcell proof, but not as the
preferred execution path for this small docs-audit task. The workcell produced
valuable reusable evidence about local Qwen, OpenRouter DeepSeek, and remote
Gemma behavior. It did not prove lower overhead for a small audit because one
lane was empty and human synthesis was still required.

The right Sprint 2 policy is:

- use single-agent execution for small, single-surface docs audits;
- use multi-agent execution when there are genuinely disjoint surfaces or a
  reviewer/critic lane is expected to find independent issues;
- keep fallback to single-agent mandatory until watcher and planner prompts are
  reliably useful.

## Findings

- `P2` Multi-agent is not yet a net speedup claim for tiny docs audits. The
  `#3415` proof should remain `useful_with_limits`, not `useful`.
- `P3` Remote Gemma watcher availability is proven, but role usefulness is not
  proven for this prompt shape because the output was empty.
- `P3` OpenRouter DeepSeek is promising as a fast planner/critic lane, but the
  prompt needs stronger ADL grounding before it is treated as reliable planning
  evidence.

## Non-Claims

- This packet does not prove multi-agent execution is broadly unhelpful.
- This packet does not compare against all hosted providers.
- This packet does not prove local Qwen, OpenRouter DeepSeek, or remote Gemma
  role fitness beyond the cited runs.
- This packet does not grant model lanes review, merge, approval, or closeout
  authority.

## Follow-Up

- Keep the `#3415` workcell proof as reusable provider/model evidence.
- Improve watcher prompts or model selection before reusing remote Gemma for
  janitor/watcher duty.
- Use multi-agent only on slices with disjoint ownership or expected review
  quality benefit.
- Record Sprint 2 closeout as mixed evidence unless later issues produce a
  stronger comparison.

## Validation

Focused validation for this packet should include:

- JSON parse of
  `docs/milestones/v0.91.5/review/multi_agent_overhead/multi_agent_overhead_comparison_2026-06-14.json`.
- Path existence for cited `#3415`, `#3504`, and `#3503` artifacts.
- Leakage scan over the comparison packet and JSON.
- Overclaim scan confirming this packet does not claim broad multi-agent
  usefulness.
- `git diff --check`.
