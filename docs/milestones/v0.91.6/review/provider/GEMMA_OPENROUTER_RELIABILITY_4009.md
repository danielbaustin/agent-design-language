# Gemma And OpenRouter Reliability Proof Note for #4009

## Scope

This note records the bounded WP-05 reliability proof surface for OpenRouter,
remote Gemma watcher lanes, and the adjacent hosted/local comparison truth that
`v0.91.6` may consume safely.

It does not create new live probes. It classifies the current tracked evidence
and preserves blocked or limited states where proof is incomplete.

## Source evidence

- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md`
- `docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`
- `docs/milestones/v0.91.5/review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md`
- `docs/milestones/v0.91.5/review/multi_agent_quality_comparison/MULTI_AGENT_QUALITY_COMPARISON_2026-06-15.md`
- `docs/milestones/v0.91.6/review/logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3997.md`

## Current reliability classification

### OpenRouter

Classification: `supported_with_limits`

Why:

- five requested native route IDs completed successfully in the tracked packet
- one missing-credential negative control failed closed as
  `provider_auth_missing`
- the packet is self-validating through its tracked validator/test pair

What is proven:

- native requested-route execution for:
  - `deepseek/deepseek-v4-flash`
  - `openai/gpt-4o-mini`
  - `anthropic/claude-3.5-haiku`
  - `google/gemini-2.5-flash-lite`
  - `qwen/qwen3.6-flash`
- fail-closed auth behavior for a missing credential
- route identity retention through the provider packet/state artifacts

What is not proven:

- universal OpenRouter compatibility
- broad tool-call capability
- broad JSON-mode capability
- broad role usefulness across all OpenRouter-backed models or prompt shapes

### Remote Gemma watcher lanes

Classification: `useful_with_limits`

Why:

- larger Gemma4 routes returned useful structured watcher output
- the strongest proving lane used the real `adl-provider-adapter` surface on
  `gemma4:31b`
- the packet is self-validating through its tracked validator/test pair

What is proven:

- `adapter_gemma4_31b` returned useful output through `adl-provider-adapter`
- `raw_gemma4_26b` and `raw_gemma4_e4b` also returned useful structured watcher
  output
- the older empty-output watcher result is no longer the only observed watcher
  outcome

What remains limited:

- proof covers one bounded short structured prompt shape
- proof does not establish broad multi-agent planning or janitor usefulness
- `gemma4:e2b` is still not promoted as reliable from the historical packet

### Hosted / local / remote comparison truth

Current comparison status from the tracked baseline:

- direct hosted OpenAI, Anthropic, Gemini, and DeepSeek hosted lanes were
  credential-blocked in the cited baseline shell
- local Ollama lanes are inventory-known and bounded-candidate only
- remote Ollama inventory is available, but only the larger Gemma4 watcher
  routes have current useful-output proof

This means `v0.91.6` may treat:

- OpenRouter as the strongest current hosted-route proof lane
- larger remote Gemma4 watcher routes as the strongest current Gemma proof lane
- direct hosted and broad local/remote lanes as blocked or candidate-only until
  later issue-specific proof is added

## Diagnostic and failure-routing posture

The adjacent logging proof for `#3997` adds the provider-diagnostic floor that
these reliability claims depend on.

Current bounded diagnostic posture includes:

- provider route identity
- provider model identity
- optional request/run correlation
- stable failure kinds such as:
  - `provider_auth_missing`
  - `provider_timeout`
  - `provider_rate_limited`
  - `local_runtime_busy`
  - `provider_error`
- redacted output/excerpt posture instead of raw prompts or secret material

This is enough to support bounded reliability routing for the adapter-backed and
documented provider/runtime paths consumed here without claiming full telemetry
unification.

Explicit limit:

- the `#3997` logging proof does not upgrade every raw remote probe surface to
  the same durable action-log coverage as adapter-backed runs
- the raw `raw_ollama_http` Gemma lanes remain useful-output evidence only, not
  a proof that every remote probe path shares the full runtime/provider logging
  floor

## Multi-agent consumption boundary

`v0.91.6` may consume this proof only as:

- a bounded hosted-route reliability surface for OpenRouter
- a bounded watcher-lane reliability surface for larger remote Gemma4 routes
- a truthful blocked/candidate status for direct hosted and broad local lanes
- a diagnostic floor for provider failure classification

`v0.91.6` may not consume this proof as:

- universal hosted reliability
- broad Gemma autonomy
- broad local-model reliability
- merge, closeout, or authority-bearing agent routing

## Non-Claims

- This note does not prove every OpenRouter model or prompt shape.
- This note does not prove every Gemma size or route.
- This note does not prove direct hosted-provider readiness in the absence of
  credentialed live proof.
- This note does not prove repository-wide provider portability; adjacent
  sanitation proof remains a separate bounded surface.
- This note does not replace the broader role-routing work in `#4008`, the
  failure-mode integration work in `#4010`, or the final closeout matrix in
  `#4012`.
