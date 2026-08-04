# #5354 Exact-Head GPT-5.5 Review

- Issue: #5354
- PR: #5731
- Reviewer: `codex review --model gpt-5.5`
- Reviewed revision: `d2d6ec60e7c76c254e36435fc0b83a6af9cec32b`
- Scope: #5354 repair diff against `origin/main`, focused on live Runtime v3 ingress proof, ADL v2 binary provenance, evidence substitution freeze, and live #5384 dependency gating.

## Findings

1. P2: The WSS proof treated `ADL_RUNTIME_V3_CA_CERT` as the exact peer leaf certificate while the HTTPS probes used it as a CA bundle. Valid Runtime v3 deployments whose leaf certificate chains to the configured CA would pass curl and fail the Ruby WSS probe.

## Disposition

- Fixed by changing the WSS TLS setup to load `ADL_RUNTIME_V3_CA_CERT` into an `OpenSSL::X509::Store`, use `VERIFY_PEER`, and retain hostname identity verification.

## Residual Risk

- No additional material findings were reported by the bounded exact-head review.
