# Issue 5795 Design: Governed Local Gemma/MLX Shepherd MVP

## Outcome And Boundary

Issue 5795 lets an operator send one bounded Shepherd message from the separate
HTML Observatory through Runtime v3 governed ingress to an explicitly
configured local MLX/Gemma adapter and receive the real response plus execution
evidence. The MVP is local-only and optional. It must distinguish unavailable,
deterministic test-double, and real-model states and may not present fake,
cached, or retained responses as live local inference.

The issue does not implement the v0.95 Shepherd training/evaluator program,
change the global default model, use cloud providers, or redefine Runtime,
Observatory, or WP-14 protocol contracts.

## Source Baseline

- `adl-runtime-kernel/src/operations.rs` already declares Shepherd admission
  and its governed dependency chain.
- `adl-runtime-kernel/src/ingress.rs`, `control.rs`, `governed_operations.rs`,
  and `protocol_adapters.rs` own signed command admission, authorization, and
  adapter execution boundaries.
- `adl-runtime/src/runtime_api.rs`, `runtime_api_auth.rs`, and
  `tests/runtime_api_wss.rs` own authenticated HTTP/WSS transport.
- `demos/html-observatory/app.js` and `runtime-v3.config.json` own the separate
  client projection and operator channel.
- Existing provider profiles under `adl/src/provider/` are evidence inputs,
  not permission to route this MVP around Runtime v3.

## Design

Add a Runtime-owned local Shepherd adapter contract selected only by explicit
configuration. Admission validates the signed command, principal, capability,
runtime identity, message bounds, and operation policy before provider work.
The adapter launches or connects to the configured local MLX boundary with a
bounded request, timeout, output limit, cancellation, and redacted metadata.
No model availability is inferred from configuration alone.

The response envelope includes correlation identity and a truthful execution
classification: `unavailable`, `deterministic_test_double`, or
`real_local_model`. Observatory renders that classification and response but
does not hold signing keys, launch providers, or gain direct filesystem/model
authority. Provider failure returns a bounded error and leaves Runtime and the
public read stream usable.

## Owned Paths

- `adl-runtime-kernel/src/shepherd.rs`
- `adl-runtime-kernel/tests/shepherd.rs`
- `adl-runtime/tests/shepherd_local_model.rs`
- `demos/html-observatory/shepherd.js`
- `demos/html-observatory/index.html`
- `adl/tools/validate_v092_shepherd_browser_roundtrip.mjs`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Invariants And Failure Semantics

- Unsigned, unauthorized, malformed, oversized, or wrong-runtime messages are
  rejected before local provider invocation.
- No cloud fallback, silent model substitution, or global-default change.
- Model path, prompt content, tokens, and private response data are not logged
  beyond the declared redacted evidence policy.
- Timeouts and cancellation release permits and preserve Runtime usability.
- Deterministic fakes prove adapter logic only; they cannot satisfy the real
  local-model acceptance criterion.
- Observatory status never upgrades retained/mock evidence to live proof.

## Dependencies And Coordination

WP-03 issue 5820 and TLS issue 5800 establish the launch path. WP-14 issue 5832
must freeze the command/WSS contract before final Observatory integration.
Preparation may proceed now, but implementation cannot cross those serial
gates or claim their surfaces.

## Validation Boundary

Deterministic tests cover admission, fake adapter behavior, timeout,
cancellation, malformed commands, status classification, redaction, and
unauthorized mutation. A local macOS Apple Metal/MLX lane must invoke the
explicitly configured model and retain a real response with correlation proof.
Missing hardware/model is a truthful deferred or blocked lane, never a pass.
Browser proof verifies the complete Observatory-to-Runtime round trip.
The implementation must add
`adl/tools/validate_v092_shepherd_browser_roundtrip.mjs`. That live validator
opens the real Observatory in Chrome, submits one uniquely correlated governed
message, proves Runtime admission invoked the configured MLX/Gemma adapter,
waits for the non-retained `real_local_model` response, verifies the browser
renders the same correlation and classification, and retains redacted Runtime,
adapter, WSS, and browser evidence. Legacy Observatory scripts, deterministic
adapters, and direct model invocation cannot satisfy this lane.

## Rollback

Rollback disables the optional adapter in configuration, removes the new
operation route and projection fields only if compatibility permits, reconnects
the Observatory in read-only mode, and verifies that Runtime health and WSS
remain usable. It does not switch to cloud or label a fake as production.

## Non-Goals

- Full v0.95 Shepherd/Gemma training, Aptitude Atlas, or evaluator buildout.
- AWS, hosted inference, or provider billing work.
- Global default model selection or broad intelligence/safety claims.
- Runtime/API/protocol redesign owned by 5820 or 5832.
- Observatory visual redesign or Unity consumer work.
