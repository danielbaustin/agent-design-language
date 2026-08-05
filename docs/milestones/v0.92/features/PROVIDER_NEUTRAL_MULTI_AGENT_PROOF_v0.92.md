# Provider-Neutral Multi-Agent Proof

## Status

Planned for v0.92 under WP-18B (`#5838`). This document defines the required
feature and proof boundary; it does not claim that the proof has run.

## Purpose

Prove that a governed multi-agent interaction is an ADL and ACIP capability,
not behavior that depends on one model provider, one provider-specific prompt,
or substituted fixture output.

## Required Behavior

- Run the same versioned multi-agent scenario through at least two real,
  independently configured providers.
- Route agent communication and invocation through the reconciled ACIP
  contract and retain inspectable protocol traces.
- Preserve stable agent identity, capability limits, ordering, and result
  semantics across provider changes.
- Record provider identity and capability truth without exposing credentials.
- Fail visibly when a required provider or capability is unavailable; do not
  replace it with a fixture, receipt, cached answer, or synthetic success.
- Include malformed, denied, interrupted, and provider-loss cases while
  keeping the Runtime and unaffected agents available.

## Ownership

- WP-14 owns the ACIP/A2A protocol and authenticated transport substrate.
- WP-16 owns the integrated birthday review packet consumed by this proof.
- WP-18 owns the runnable birthday scenario.
- WP-18B owns provider-neutral execution, comparison, negative cases, and the
  final proof matrix.

## Validation

- Real provider invocations complete through the same scenario contract.
- Protocol traces identify equivalent ACIP operations and bounded differences.
- Negative cases prove that provider substitution cannot be mistaken for a
  successful run.
- Artifacts are redacted, deterministic where required, and linked to exact
  source and provider-capability revisions.

## Acceptance Criteria

- The provider-neutral matrix covers every required scenario and provider.
- At least two real providers complete the positive path.
- No fixture, demo mode, receipt-only adapter, or synthetic success receives
  proof credit.
- Failure of one provider does not terminate the Runtime or unrelated agents.
- Exact-revision review accepts the proof packet and its non-claims.

## Non-Goals

- Claiming identical prose or token usage across providers.
- Requiring every possible provider in v0.92.
- Publishing credentials, private prompts, or unredacted provider payloads.
