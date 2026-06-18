# v0.91.6 SEP Local-Agent Acceleration Mini-Sprint

Status: `active_setup`
Date: 2026-06-18
Sprint umbrella: `#4069`

This packet makes the `#4069` mini-sprint explicit and schedulable. It does not
replace issue-local `SIP -> STP -> SPP -> SRP -> SOR` truth, and it does not
claim local models are authoritative implementers or reviewers.

## Sprint Goal

Speed up future sprint execution by proving bounded, watchable local-agent
support roles and by routing the missing sprint-process hardening into tracked
implementation issues.

## Execution Mode

`hybrid`

## Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4078` | typed GitHub mutation transport | closed | Enabling dependency already landed; sprint setup no longer needs direct `gh` for covered issue mutation paths. |
| `#4069` | umbrella setup, local-model inventory, role-fit matrix, proof packet | active | Owns the mini-sprint packet and local-model proof posture. |
| `#4076` | readiness sweep | pending | Must make watcher, card, SEP, and pre-execution readiness deterministic before worker launch. |
| `#4077` | deterministic sprint closeout | pending | Must keep sprint umbrellas open until child truth, review, activity logs, and closeout artifacts are complete. |
| `#3927` | sprint-review skill | pending | Must standardize sprint/mini-sprint review lanes and findings-first output. |
| `#4074` | agent-per-task simulation proof | pending | Must consume the setup packet and role contracts without granting local mutation authority. |

## Recommended Execution Order

1. Treat `#4078` as completed enabling substrate.
2. Finish `#4069` first so the local-model inventory, role-fit matrix, watcher policy, and sprint packet are durable.
3. Run `#4076` next to make readiness deterministic before future sprint execution.
4. Run `#4077` after or alongside `#4076` where the touched surfaces remain disjoint.
5. Run `#3927` once the sprint-review input/closeout expectations from `#4069`/`#4077` are stable enough to encode.
6. Run `#4074` after the setup packet and role limitations are durable so the simulation consumes real contracts rather than chat assumptions.
7. Close `#4069` only after the child-wave routing is durable and any newly discovered sprint-process gaps are tracked explicitly.

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| process-contract lane | `#4076`, `#3927` | readiness and sprint-review can evolve in parallel once `#4069` fixes the umbrella packet and role boundaries | reconcile reviewer inputs, watcher terminology, and artifact paths before PR publication |
| closeout lane | `#4077` with `#3927` | closeout and review can progress in parallel when they reference the same child-truth model but do not overwrite the same scripts/docs | align closeout artifact fields and review packet expectations before merge |

## Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| setup gate | `#4076`, `#4077`, `#3927`, `#4074` | `#4069` records the child wave, watcher rule, local-model inventory, and proof posture durably. |
| readiness gate | `#4074` | `#4076` defines the deterministic readiness sweep for sprint execution. |
| closeout/review gate | final `#4069` closeout and any real-sprint adoption | `#4077` and `#3927` land enough deterministic closeout/review contract to avoid hidden wait states or hidden completed-but-open issues. |

## Watcher Policy

- Every active child issue must have a watcher or equivalent lifecycle monitor.
- Every wait state must record what it is waiting on, who is watching it, and the next handoff.
- Wait states without a watcher are invalid sprint state.
- Healthy PR-open states are watcher-owned states, not natural pause points.
- Sprint closeout must preserve the biggest waiting points and whether they were necessary.

## Local Model Inventory

| Model | Runtime | Suggested bounded roles | Current limitations |
|---|---|---|---|
| `qwen3.6:27b` | local Ollama | docs summarizer, issue-state summarizer, card validator candidate | too slow for the tiny watcher proof window used in this packet; use only when latency is acceptable |
| `Qwen3.5:35b-a3b` | local Ollama | richer docs/review synthesis candidate | larger local footprint; not yet proven in this mini-sprint packet |
| `deepseek-r1:8b` | local Ollama | watcher candidate, issue-state summarizer, bounded classification | timed out in the bounded HTTP proof window for this packet; needs more tuning before relying on it for fast watcher loops |
| `mistral-small3.2:24b` | local Ollama | concise summarizer / contract restater candidate | not yet proven in this mini-sprint packet |
| `gemma4:26b` | local Ollama | docs lint / bounded review candidate | not yet proven in this mini-sprint packet |
| `gemma:2b` | local Ollama | tiny watcher/status classification fallback | low-confidence output; useful only for non-authoritative, tightly-scoped support tasks |

## Role-Fit Matrix

| Role | Best current candidate | Why | Hard limits |
|---|---|---|---|
| watcher | `gemma:2b` as fallback, `deepseek-r1:8b` if latency improves | cheap bounded JSON classification is feasible for simple lifecycle states | output must be verified by the main agent; no mutation authority |
| card validator | `qwen3.6:27b` | stronger structured reading than tiny models | slower; do not use when a fast watch loop is required |
| closeout checker | `qwen3.6:27b` or hosted Codex | can compare artifact truth and residual routes | still non-authoritative until verified by the main agent |
| activity-log summarizer | `qwen3.6:27b` | summarization quality favored over latency | may be too slow for inline watch loops |
| docs lint reviewer | `gemma4:26b` or `qwen3.6:27b` | review-shaped bounded output is a plausible fit | not yet proven in this packet |

## Bounded Local-Model Proof

Proof method: direct Ollama HTTP API call, not CLI model invocation.

Prompt target:
- classify whether `#4076` is ready to execute given:
  - `#4076` open
  - parent `#4069` still being authored
  - `#4078` closed
  - no implementation PR yet

Model results:
- `qwen3.6:27b`: no bounded result returned during the initial proof window; not accepted as the fast watcher candidate for this packet.
- `deepseek-r1:8b`: HTTP call timed out in the bounded proof window; not accepted as the fast watcher candidate for this packet.
- `gemma:2b`: returned bounded JSON quickly enough for packet use.

Recorded `gemma:2b` output:

```json
{"issue":4076,"role":"watcher","status":"ready|pending|blocked","waiting_on":["4069","4078"],"next_action":"","confidence":"low|medium|high"}
```

Assessment:
- The model respected the JSON-only boundary.
- The model output is not yet trustworthy enough to drive automation directly because it left enumerated placeholders in `status` and `confidence`, and incorrectly kept `#4078` in `waiting_on` even though `#4078` is already closed.
- This still proves a useful narrow point: a tiny local model can participate in a bounded watcher-style support role, but only as advisory output checked by the main agent.

## Proof Classification

- local-model participation: `proven_for_non_authoritative_advisory_output_only`
- watcher-at-every-step automation: `routed_to_implementation`
- sprint readiness automation: `routed_to_implementation`
- deterministic closeout automation: `routed_to_implementation`
- sprint review standardization: `routed_to_implementation`

## Follow-On Implementation Map

- `#4076`: readiness sweep before execution
- `#4077`: deterministic sprint closeout
- `#3927`: sprint-review skill
- `#4074`: simulation proof that consumes the above contracts

## Non-Claims

- This packet does not claim local models can implement issues autonomously.
- This packet does not claim local models are authoritative reviewers.
- This packet does not claim fast local watcher loops are solved for larger models.
- This packet does not claim markdown.rs/AST-backed SEP editing is implemented here.
