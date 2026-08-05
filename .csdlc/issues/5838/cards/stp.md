# Structured Task Prompt

Template: 1.0.0

Issue: 5838

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the WP-18B provider-neutral harness, validator, proof matrix, traces, and failure cases over landed #5832, #5834, and #5836 contracts.

## Deliverables

- Real-provider scenario harness
- Provider-neutral matrix with at least two positive columns
- Redacted ACIP traces and artifact digests
- Malformed, denied, interrupted, unavailable, loss, and substitution proof

## Acceptance

1. AC-1: At least two real providers complete the identical versioned scenario through equivalent ACIP operations.
2. AC-2: Provider identity/capability truth and bounded semantic differences are retained without credentials or private payloads.
3. AC-3: Malformed, denied, interrupted, unavailable, provider-loss, and substitution cases have visible non-pass outcomes.
4. AC-4: One provider failure leaves Runtime and unrelated agents available, with macOS/Linux tooling posture recorded.
5. AC-5: Exact-head review has no unresolved actionable finding.

## Dependencies

- #5832 / WP-14 complete
- #5834 / WP-16 complete
- #5836 / WP-18 complete
- Two approved real-provider credential sources available at execution time

## Inputs

- docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md
- docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md
- adl/tools/real_multi_agent_provider_adapter.py
- adl/tools/provider_demo_common.sh

## Non Goals

- Identical prose or token usage
- Every possible provider
- Changing ACIP or the birthday scenario
- Publishing credentials, private prompts, or raw payloads
