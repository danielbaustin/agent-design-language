# WP-02 Resilience Layer Integration Proof Note for #3993

## Scope

This note records the bounded Phase 1 integration truth for the WP-02
resilience mini-sprint after `#3986` through `#3992`.

It does not claim that every workflow, tool, citizen-runtime, or provider path
in the repository already executes the full resilience layer in production. It
defines what is live, what is proven by representative fixtures, which bypasses
are currently allowed, and which continuity surfaces remain residual work.

## Source evidence

- `adl/src/resilience.rs`
- `adl/src/provider_communication.rs`
- `adl/src/provider_adapter.rs`
- `docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_FAILURE_MODE_INTEGRATION_4010.md`

## Live integration boundary

The strongest live `v0.91.6` runtime integration is the provider adapter path:

- `adl/src/provider_adapter.rs`
  - derives a shared `ResiliencePolicyV1` from provider attempt policy through
    `provider_attempt_policy_as_resilience_policy(...)`
  - executes that shared policy with `execute_retry_policy(...)`
  - preserves shared provider fault classification through
    `provider_failure_classification_from_failure(...)`
  - emits reviewable/redacted provider attempt logs without leaking prompt or
    raw provider output text

This means `v0.91.6` can truthfully claim:

- a live provider execution path using the shared retry substrate
- one shared provider failure vocabulary across provider communication,
  provider adapter execution, and resilience proof surfaces

It may not claim:

- that the provider adapter already executes every resilience policy family in
  production
- that workflow/tool/citizen-runtime live production surfaces all route through
  the same runtime layer today

## Representative proof surfaces

### Provider adapter proof

`adl/src/provider_adapter.rs` test coverage proves:

- retryable hosted-provider failures are retried and can recover successfully
- non-retryable hosted-provider failures stop without retry theater
- retry budget exhaustion is represented explicitly
- provider adapter run logs remain redacted and reviewable
- the retry log sequence stays observable from `run_start` through
  `attempt_failure` / `attempt_success` to `run_finish`

### Shared resilience substrate proof

`adl/src/resilience.rs` proves the shared Phase 1 policy families through
focused representative fixtures:

- retry
- timeout
- circuit breaker
- rate limit
- bulkhead
- fallback / degraded execution

The strongest composed proof in this sprint is the representative provider flow
fixture that combines:

- retry
- rate limit
- timeout
- circuit breaker
- degraded fallback

That fixture proves the policies can compose coherently without widening into a
false claim that every live provider call path already executes the whole stack
in production.

Additional bounded representative fixtures prove:

- workflow-labeled circuit-breaker and fallback behavior
- tool-labeled timeout/rate-limit/bulkhead behavior
- citizen-runtime-labeled bulkhead isolation and bounded artifact ids

These are valid proof surfaces for the shared layer, but they remain fixture
proof, not broad production-integration claims.

## Bypass exception policy

| Surface | Current expectation | Allowed exception |
| --- | --- | --- |
| provider adapter execution | Must use the shared retry substrate and shared provider fault vocabulary | none on the covered adapter path |
| provider communication mapping | Must use shared fault vocabulary and shared policy shape | mapping-only surface may normalize or classify without claiming full runtime execution |
| workflow/tool/citizen-runtime proof surfaces | Must use the shared resilience substrate when claiming Phase 1 proof | fixture-only proof may stand in for live integration, but must be labeled as proof-only |
| future continuity claims | Must not bypass checkpoint/restore/replay proof | none; these claims stay blocked until dedicated proof exists |

## Residual routing truth

The following areas remain outside the mini-sprint's proven completion boundary:

| Area | Current truth | Residual requirement |
| --- | --- | --- |
| checkpoint / restore | substrate vocabulary exists; broad runtime proof does not | later implementation and replay proof required |
| sleep / wake | planning doc exists; live resilience-layer sleep/wake bridge is not proven here | dedicated continuity slice required |
| hibernation / migration | not implemented in WP-02 | keep as explicit residual work |
| replay / durable continuity | not proven | keep `v0.92` continuity claims blocked or explicitly bounded |

## Review record shape

Any downstream review or closeout that consumes WP-02 should distinguish:

- `live_integration`
  - provider adapter retry path
- `representative_fixture_proof`
  - composed shared-layer tests in `adl/src/resilience.rs`
- `mapping_only_boundary`
  - provider communication normalization/classification surfaces
- `residual_continuity_work`
  - checkpoint/restore, sleep/wake, hibernation, migration, replay

Review should reject closeout language that collapses those four categories into
one generic “resilience completed” claim.

## Non-Claims

- This note does not claim repository-wide production use of every resilience
  policy family.
- This note does not claim checkpoint/restore or replay implementation.
- This note does not claim `v0.92` continuity readiness.
- This note does not replace the issue-local SOR closeout truth for `#3993` or
  the final WP-02 umbrella closeout.
