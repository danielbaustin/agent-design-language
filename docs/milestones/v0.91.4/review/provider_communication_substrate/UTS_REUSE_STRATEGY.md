# UTS Reuse Strategy For ADL Provider Communication Substrate

Issue: #3481
Status: proposed

## Summary

ADL should own the canonical Rust provider communication substrate. UTS should reuse the substrate semantics without becoming hard to install or run.

The near-term reuse path is generated schema plus a small CLI boundary, not a direct UTS dependency on ADL internals.

## Decision

Use this sequence:

1. ADL defines canonical Rust structs and offline fixture tests for provider requests, invocation results, failure kinds, model identity helpers, and tail-friendly provider run logs.
2. ADL exposes generated JSON schema/fixtures for the provider communication contract.
3. UTS consumes the contract shape in its standalone Python runner and validates its artifacts against the shared fixtures.
4. ADL later adds a Rust CLI provider adapter that UTS may call optionally for live provider execution.
5. Only after the CLI boundary is stable should we consider vendoring a Rust crate or extracting a shared package.

## Why Not Direct Rust Crate Dependency Yet

UTS must remain easy for non-ADL users to run. A mandatory Rust dependency would make the standalone UTS benchmark less accessible and would recreate the installation problem that the Python runner intentionally avoided.

## Why A CLI Boundary Is The Safer Shared Runtime Path

A CLI can provide shared provider behavior while preserving UTS usability:

- UTS can keep its Python entrypoint.
- ADL can own hosted/Ollama communication and normalized provider errors.
- The boundary can stream JSONL events for `tail -f` observability.
- UTS can opt into the CLI once it is stable, while keeping a fallback path during migration.

## Required Shared Semantics

UTS should match these ADL-owned semantics:

- `ProviderInvocationRequestV1`
- `ProviderInvocationResultV1`
- `ProviderFailureKindV1`
- `ProviderAttemptV1`
- `ProviderRunLogEventV1`
- hosted identity as provider-asserted unless immutable provider revision metadata exists
- Ollama identity as pinned only when a digest is observed
- Ollama identity as tag-only when no digest is available
- provider/runtime failures as operational failures, not scored benchmark failures
- logs as line-oriented JSONL with immediate flushing and no secrets/prompts

## Follow-On Work

Recommended follow-on issues:

1. Generate provider communication JSON schema and fixture artifacts from ADL Rust types.
2. Add an ADL provider CLI that accepts JSON requests and emits JSONL events plus a final JSON result.
3. Add UTS fixture validation against ADL-generated schemas.
4. Add optional UTS execution mode that calls the ADL provider CLI when available.
5. Decide later whether extraction into a shared package is warranted.

## Non-Claims

This document does not claim UTS already consumes the Rust substrate. It records the concrete reuse strategy and migration boundary for the next implementation wave.
