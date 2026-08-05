# Structured Task Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement a local Apple Metal/MLX Shepherd adapter behind explicit config and governed Runtime v3 admission, expose truthful response/status evidence to the separate Observatory, and prove real invocation plus unavailable, timeout, malformed, and unauthorized failures.

## Deliverables

- Explicitly configured bounded local MLX/Gemma adapter with timeout, cancellation, output limits, and redacted metadata
- Signed/capability-governed Runtime command route and correlated response envelope
- Observatory control/status integration that distinguishes real, fake, retained, and unavailable evidence
- Deterministic adapter negatives plus one real local-model smoke and browser round-trip proof

## Acceptance

1. An explicitly configured local MLX/Gemma adapter executes a real bounded Shepherd request when available
2. The Observatory sends the request through authenticated/signed governed Runtime v3 ingress and displays the correlated result
3. Runtime and Observatory distinguish unavailable, deterministic-test, retained, and real-local-model states
4. Missing model, timeout, cancellation, malformed or oversized input, and unsigned/unauthorized mutation fail truthfully without taking down Runtime
5. Deterministic tests cover adapter/admission/status behavior and cannot satisfy the real-model criterion
6. A real macOS Apple Metal/MLX smoke retains model identity, correlation, timing, and response evidence under redaction policy
7. No AWS, hosted inference, cloud fallback, global default switch, or v0.95 completion claim occurs
8. One exact-head review has no unresolved actionable findings

## Dependencies

- WP-03 issue 5820 stable Guardian/Runtime launch and readiness
- Issue 5800 trusted local Observatory HTTPS
- WP-14 issue 5832 stable command and WSS contracts before final integration
- Configured local Apple Metal/MLX runtime and model for the real smoke lane

## Inputs

- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/governed_operations.rs
- adl-runtime-kernel/src/protocol_adapters.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/tests/runtime_api_wss.rs
- demos/html-observatory/app.js
- demos/html-observatory/runtime-v3.config.json

## Non Goals

- Full v0.95 Shepherd/Gemma training, Aptitude Atlas, or evaluator program
- AWS, hosted inference, cloud fallback, or provider billing work
- Global default model switch or broad intelligence/safety claims
- Runtime launch or ACIP/A2A contract redesign
- Observatory visual redesign or Unity integration
