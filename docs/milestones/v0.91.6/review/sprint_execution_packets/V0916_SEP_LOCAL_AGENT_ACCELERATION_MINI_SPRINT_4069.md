# v0.91.6 SEP Local-Agent Acceleration Mini-Sprint

Status: `closed_retained_packet`
Date: 2026-06-18
Sprint umbrella: `#4069`

This packet records the retained umbrella closeout refresh for the `#4069`
mini-sprint after the child wave completed and the umbrella later closed. It
does not replace issue-local
`SIP -> STP -> SPP -> SRP -> SOR` truth, and it does not claim local models are
authoritative implementers or reviewers.

## Final Outcome

The SEP local-agent acceleration mini-sprint completed its child wave, and this
packet is the retained umbrella closeout packet for the now-closed sprint:

- `#4076` landed deterministic sprint readiness sweep support.
- `#4077` landed deterministic sprint closeout support.
- `#3927` landed the reusable sprint-review skill.
- `#4074` landed the bounded agent-per-task simulation proof.
- `#4069` established the umbrella packet, watcher rule, local-model inventory,
  role-fit matrix, and bounded local-model proof posture that the child wave
  consumed.

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
| `#4069` | umbrella setup, local-model inventory, role-fit matrix, proof packet | closed | Owns the retained mini-sprint packet and final umbrella closeout truth refresh. |
| `#4076` | readiness sweep | closed | Landed deterministic watcher/card/SEP pre-execution readiness support. |
| `#4077` | deterministic sprint closeout | closed | Landed sprint closeout gating and deterministic closeout support. |
| `#3927` | sprint-review skill | closed | Landed reusable sprint/mini-sprint review skill coverage. |
| `#4074` | agent-per-task simulation proof | closed | Landed the bounded proof showing delegated watcher/reviewer roles can be useful with main-conductor verification. |

## Recommended Execution Order

1. Treated `#4078` as the completed enabling substrate.
2. Landed `#4069` first so the local-model inventory, role-fit matrix, watcher policy, and sprint packet were durable.
3. Landed `#4076` to make readiness deterministic before future sprint execution.
4. Landed `#4077` to codify deterministic sprint closeout.
5. Landed `#3927` once the sprint-review input/closeout expectations were stable enough to encode.
6. Landed `#4074` after the setup packet and role limitations were durable so the simulation consumed real contracts rather than chat assumptions.
7. The umbrella later closed after the child-wave routing was durable and this retained umbrella truth refresh existed.

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

## Sprint-Level Review

- The biggest sprint win was operational visibility: active child issues were
  treated as watcher-owned states rather than silent waiting states.
- The most useful bounded local-model result was still advisory-only. `gemma:2b`
  proved a tiny local watcher-style output can be produced quickly, but the
  output still required main-conductor verification.
- The larger local models remained too slow or unstable for the tiny fast-loop
  watcher proof window used here, so the sprint did not prove reliable
  larger-model watcher loops.
- The durable sprint-process outcome is stronger than the local-model proof
  itself: readiness sweep, deterministic closeout, sprint-review skill, and the
  bounded simulation proof all landed as tracked repo truth.

## Closeout Recommendation

The sprint umbrella is already closed. No additional child implementation
remains in scope for the original `#4069` mini-sprint, and this packet should
be read as retained reviewer-facing evidence rather than a still-pending
closeout proposal.

## Non-Claims

- This packet does not claim local models can implement issues autonomously.
- This packet does not claim local models are authoritative reviewers.
- This packet does not claim fast local watcher loops are solved for larger models.
- This packet does not claim markdown.rs/AST-backed SEP editing is implemented here.
