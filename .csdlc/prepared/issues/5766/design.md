# Design: Reconcile CSM Runtime Endpoint Inventory

## Context

Issue #5766 tracks a Runtime v3 / Observatory defect found during the overnight probe pass: advertised CSM runtime endpoints can drift from the routes actually served by the runtime API.

Current source evidence shows two relevant surfaces:

- `adl-runtime/src/runtime_api.rs` advertises and serves the bounded runtime API routes.
- `adl/src/csm_runtime_api.rs` still exposes a broader CSM runtime API inventory used by CLI/observatory-related surfaces.

The repair must make advertised endpoint truth match mounted/served behavior for the issue's CSM runtime API scope, without confusing the Runtime v3 kernel `/v1/ready` surface with the CSM runtime API surface.

## Approach

1. Inventory every source and test surface that declares or consumes `CSM_RUNTIME_API_ENDPOINTS`.
2. Decide endpoint truth per surface:
   - keep only served endpoints in an availability inventory; or
   - mount truthful bounded handlers for every advertised endpoint.
3. Prefer the smallest truthful fix for v0.91.8: advertised paths must not claim availability unless they are actually routed.
4. Add focused tests that compare inventory to mounted routes or explicit response metadata.
5. Preserve a clear distinction between:
   - CSM runtime API routes; and
   - Runtime v3 kernel readiness/control surfaces.

## Invariants

- No AWS.
- No broad product claims for unimplemented endpoints.
- No fake route success for features that remain unimplemented.
- Operator-facing inventories must distinguish implemented routes from planned/future routes.
- Tests must fail if advertised availability and router/mount truth diverge again.

## Expected Files

- `adl-runtime/src/runtime_api.rs`
- `adl/src/csm_runtime_api.rs`
- `adl/src/csm_api_gateway_bridge.rs`
- Focused Rust tests adjacent to touched runtime/API modules.
- v0.91.8 documentation surfaces only if source truth requires documentation reconciliation.
