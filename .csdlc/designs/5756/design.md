# Issue 5756 design

## Scope

Correct the Runtime v3 hosted provider HTTP failure classifier so MiniMax
billing-blocked handling remains MiniMax-specific. The fix is limited to
provider error classification and focused provider adapter regressions.

## Intended change

The shared non-2xx hosted-provider classifier must not treat a bare `1008`
substring as billing evidence for every provider. MiniMax-specific structured
response handling already maps `base_resp.status_code == 1008` to
`ProviderBillingBlocked`; the corrective change keeps that behavior while
removing cross-provider bare-substring classification.

## Validation

Run focused provider adapter tests for MiniMax positive and cross-provider
negative cases, then strict Clippy for the touched crate surface.
