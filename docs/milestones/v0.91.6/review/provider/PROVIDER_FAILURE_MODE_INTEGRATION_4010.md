# Provider Failure-Mode And Resilience Integration Proof Note for #4010

## Scope

This note records the bounded WP-05 integration surface between:

- the shared WP-02 resilience substrate
- the WP-03 provider/logging diagnostic floor
- the currently proven WP-05 provider reliability lanes

It does not claim broad runtime policy execution across every provider path. It
defines the failure-mode matrix and the integration contract that `v0.91.6` may
consume safely.

## Source evidence

- `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`
- `adl/src/resilience.rs`
- `adl/src/provider_communication.rs`
- `docs/milestones/v0.91.6/review/logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3997.md`
- external PR-backed context only:
  - `GEMMA_OPENROUTER_RELIABILITY_4009.md` from PR `#4070`
  - `PRIVATE_ENDPOINT_FIXTURE_SANITATION_4011.md` from PR `#4068`

## Shared integration boundary

The WP-02 resilience substrate is the source of truth for:

- provider-facing fault classes
- fault dispositions
- recovery dispositions
- resilience telemetry event kinds
- retry / timeout / circuit-breaker / rate-limit / bulkhead / fallback policy
  shapes

The provider communication surface already consumes the same shared vocabulary
rather than inventing a provider-local taxonomy. That means `v0.91.6` can use
one bounded failure-language surface across provider reliability, logging, and
recovery routing.

## Failure-mode matrix

| Failure mode | Shared fault class / status | Resilience disposition | Current routing expectation | Evidence boundary |
| --- | --- | --- | --- | --- |
| Missing credential | `provider_auth_missing` | `operator_gated` | Fail closed; do not silently fall back; surface credential/operator action requirement | OpenRouter negative control in `OPENROUTER_MATRIX_PROOF_2026-06-14.md`; provider/logging proof `#3997`; shared resilience/provider vocab in `adl/src/resilience.rs` and `adl/src/provider_communication.rs` |
| Auth error | `provider_auth_error` | `operator_gated` | Surface explicit auth failure; no hidden retry loop claimed here | Shared vocab in `adl/src/resilience.rs` and `adl/src/provider_communication.rs`; no separate live proof note for every provider |
| Timeout | `provider_timeout` | `retryable` | Timeout class must be represented and routable through shared policy rather than provider-local ad hoc handling | Shared timeout policy and telemetry event kinds in `adl/src/resilience.rs`; provider/logging proof `#3997` |
| Rate limit | `provider_rate_limited` | `retryable` | Rate-limit handling must route through shared retry/rate-limit policy vocabulary | Shared rate-limit policy and telemetry event kinds in `adl/src/resilience.rs`; provider/logging proof `#3997` |
| Transient HTTP / provider transient | `provider_transient_http` | `retryable` | Retry/backoff path belongs to the resilience substrate rather than a provider-specific shadow policy | Shared retry policy in `adl/src/resilience.rs`; provider communication fault kinds |
| Provider/model unavailable | `provider_model_unavailable` | `terminal` | Do not overclaim automatic substitute routing; fallback must be explicit policy, not hidden behavior | Shared fault/disposition and fallback policy shapes in `adl/src/resilience.rs`; broader substitution remains a later/provider-specific decision |
| Billing blocked | `provider_billing_blocked` | `operator_gated` | Surface hard blocking state rather than retry theater | Shared provider fault kinds in `adl/src/provider_communication.rs` |
| Empty or malformed model output | `provider_empty_text_output`; result-level malformed classifications remain bounded by consumer contract | `terminal` for explicit empty-text classification; broader malformed handling remains consumer-contract bounded | Treat malformed/empty output as a first-class failure class; do not silently promote low-quality output to success | Shared fault classes in `adl/src/resilience.rs`; provider invocation result structures in `adl/src/provider_communication.rs` |
| Local runtime unavailable | `local_runtime_unavailable` | `retryable` | Surface runtime unavailability explicitly; local fallback claims remain bounded | Shared/provider fault kinds in `adl/src/resilience.rs` and `adl/src/provider_communication.rs`; historical local baseline in the provider matrix |
| Local runtime busy / hung | `local_runtime_busy`, `local_runtime_hung` | `retryable` | Busy/hung local runtimes must route through bulkhead/retry policy rather than ad hoc queue pressure handling | Shared bulkhead / retry / timeout vocabulary in `adl/src/resilience.rs`; provider/logging proof `#3997` |
| Generic provider error | `provider_error` | `terminal` | Preserve classification and evidence instead of flattening into “unknown failure” | Shared/provider vocab in `adl/src/resilience.rs` and `adl/src/provider_communication.rs` |

## Mapping to the six resilience patterns

The shared resilience substrate defines six policy families that provider
reliability must consume:

1. `retry`
2. `timeout`
3. `circuit_breaker`
4. `rate_limit`
5. `bulkhead`
6. `fallback`

Current bounded integration truth:

- `retry`:
  - shared retryable fault classes exist
  - retry policy shape includes `max_attempts`, optional `backoff_ms`,
    optional `jitter_ms`, and explicit retryable classes
- `timeout`:
  - timeout policy shape exists
  - provider timeout is a named fault class
- `circuit_breaker`:
  - shared circuit-breaker policy and telemetry event shape exist
  - this issue does not claim every provider path actively emits a circuit
    breaker decision artifact yet
- `rate_limit`:
  - shared rate-limit policy and named `provider_rate_limited` fault exist
- `bulkhead`:
  - shared bulkhead policy shape exists for concurrency/fault-domain isolation
  - local busy/hung runtime classes are the strongest bounded consumers today
- `fallback`:
  - shared fallback policy shape exists
  - fallback must remain explicit and policy-driven, not hidden provider
    substitution theater

## Logging / diagnostic expectations from WP-03

The bounded provider/logging proof adds the following failure-routing floor for
documented provider/runtime paths:

- provider route identity
- provider model identity
- optional run/request correlation
- stable failure classifications suitable for downstream routing
- redacted output posture instead of raw provider payload publication

This is enough for bounded provider failure-mode accounting in `v0.91.6`.

Explicit limits:

- `#3997` does not claim every provider path writes the same durable artifact
  family
- raw remote probe lanes, especially non-adapter HTTP lanes, must not be
  upgraded to full correlated action-log coverage by implication alone

## Malformed-output and partial-response handling

Current bounded handling expectations:

- empty text output is an explicit fault class
- malformed or partial provider results must be treated as contract-level
  degradation or failure rather than implicit success
- degraded output may be allowed only when the relevant resilience/fallback
  policy marks degraded output as acceptable and the consuming issue or lane is
  truthful about that downgrade

This issue does not claim a full repository-wide malformed-output harness. It
defines the routing contract that later slices may rely on.

## Relationship to adjacent provider proofs

Adjacent provider packets exist in draft PRs and informed the intended
consumption order for the sprint, but they are not locally merged evidence in
this worktree.

- `#4009` is the adjacent PR-backed reliability classification note for current
  OpenRouter and remote Gemma lane status.
- `#4011` is the adjacent PR-backed sanitation note for bounded durable packet
  roots.

`#4010` does not depend on those packets for the local failure-classification
matrix itself. It states how provider failures must be classified and routed
when provider lanes are consumed, while keeping PR-backed provider specifics as
external sprint context rather than locally merged proof.

## Multi-agent consumption boundary

`v0.91.6` may consume this note as:

- the bounded provider failure-classification matrix
- the mapping from provider failures to the six shared resilience policy
  families
- the explicit logging/diagnostic floor for documented paths

`v0.91.6` may not consume this note as:

- proof that every provider path already executes every resilience policy
- proof that all fallback behavior is implemented automatically
- proof that raw remote HTTP probe paths have full correlated telemetry parity
- a reason to bypass role/provider limits or human/team authority boundaries

## Non-Claims

- This note does not claim new runtime refactors were implemented here.
- This note does not claim repository-wide failure-policy execution proof.
- This note does not claim every provider or model lane is equally recoverable.
- This note does not replace the final provider closeout matrix in `#4012`.
