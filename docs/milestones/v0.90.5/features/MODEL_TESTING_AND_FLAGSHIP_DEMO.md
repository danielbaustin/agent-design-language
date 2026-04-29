# Model Testing And Flagship Demo

## Purpose

Governed Tools v1.0 must be tested against real model behavior. A schema is not
enough if models misunderstand authority, leak arguments, or try to bypass the
runtime.

This feature inherits the WP-02 proposal/action boundary from
`TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`: model evaluation should score whether
models preserve proposal humility and whether dangerous categories fail closed
before action.

## Model Testing

The model benchmark should test:

- UTS comprehension
- tool proposal generation
- unsafe proposal resistance
- authority reasoning
- privacy and visibility discipline
- prompt/tool-argument leakage avoidance
- ambiguity handling
- injection and jailbreak resistance
- correction after feedback

For `v0.90.5`, WP-16 lands the deterministic fixture-backed harness and scored
benchmark report, while WP-17 reuses that harness for the smaller live
local/Gemma-focused evaluation or an explicit model-availability skip.

The full comparison panel should eventually include:

- local house-model candidate, especially Gemma-family models when available
- at least one additional local model
- at least one strong hosted model when credentials and budget permit
- one weaker/smaller model to expose failure modes

For `v0.90.5`, keep WP-17 intentionally smaller:

- run a simple bounded local/Gemma-focused evaluation demo when a model is
  available
- record an explicit skip rationale when local/Gemma models are not available
- produce a small scorecard and failure-note packet
- defer full local-vs-remote and multi-model comparison reporting to `v0.91`

The WP-17 demo may include governed fixture-backed execution or refusal when
the harness path supports it. It must not expand into the whole benchmark test
suite.

The WP-16 harness report is written by the bounded runner entrypoint:

`cargo run --manifest-path adl/Cargo.toml --bin demo_v0905_model_proposal_benchmark`

The WP-17 live local/Gemma evaluation is written by the bounded runner
entrypoint:

`cargo run --manifest-path adl/Cargo.toml --bin demo_v0905_local_gemma_model_evaluation -- --out docs/milestones/v0.90.5/review/local-gemma-model-evaluation-report.json --model gemma4:e2b --model gemma4:e4b --model llama3.1:8b`

The tracked review artifact is
`docs/milestones/v0.90.5/review/local-gemma-model-evaluation-report.json`.
That bounded live report is intentionally not a ranking claim: it records the
current model-specific pass/fail mix for a small local Ollama comparison panel
covering `gemma4:e2b`, `gemma4:e4b`, and `llama3.1:8b`, including safe-read
proposal shape, privacy discipline, execution humility, jailbreak resistance,
and feedback responsiveness, plus any governed fixture-backed execution or
refusal path the run actually demonstrated.

The placement note lives in
`ideas/GEMMA4_UTS_ACC_MODEL_BENCHMARK_PLAN.md`.

## Flagship Demo

The flagship demo should show:

- an allowed read proposal
- a denied low-authority proposal
- a delegated local-write proposal
- a destructive or exfiltrating proposal that fails closed
- UTS validation
- ACC compilation
- policy and Freedom Gate mediation
- governed execution or refusal
- trace and redacted report output

The demo should make Governed Tools v1.0 visibly better than current industry
tool calling.

The flagship demo may use model benchmark artifacts as supporting context, but
it must not depend on completing the full `v0.91` model comparison report.

The canonical bounded entrypoint is:

`adl runtime-v2 governed-tools-flagship-demo --out artifacts/v0905/demo-d11-governed-tools-flagship`

The reviewer-facing bundle should include:

- `runtime_v2/governed_tools/flagship_proof_packet.json`
- `runtime_v2/governed_tools/flagship_operator_report.md`
- `runtime_v2/governed_tools/flagship_public_report.md`
- four named case artifacts for allowed read, delegated local write, denied low authority, and denied exfiltration
- trace plus governed proposal/result-or-refusal redaction artifacts for the cases that reach gate/executor review
- supporting benchmark and dangerous-negative-suite reports as bounded context only

## ACIP Proof Demo

The Comms-08 ACIP proof surface is intentionally smaller than the flagship
governed-tools demo. It exists so reviewers can see that ACIP is real
communication-plus-invocation infrastructure before it is absorbed into larger
tool-governance stories.

The canonical proof path is:

- consultation message
- capability-negotiation message
- coding-request message
- proposal-only coding invocation contract
- proposal-ready coding outcome
- reviewer/public trace views with deterministic redaction

The proof packet is fixture-backed and provider-optional. It must not require
live model execution, paid APIs, or encrypted transport claims.

The proof packet is also explicitly non-proving for:

- live transport
- encrypted external transport
- reputation systems
- cross-polis federation
