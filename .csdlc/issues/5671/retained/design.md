# Issue #5671 — Anthropic Claude Opus 5 provider

## Goal

Add a first-class Claude Opus 5 provider profile and setup route while reusing
the existing Rust-native Anthropic Messages adapter.

## Design

The profile `claude:claude-opus-5` expands to the Anthropic provider kind,
the canonical model id `claude-opus-5`, the Messages API endpoint, and the
`ANTHROPIC_API_KEY` credential boundary. A `provider setup claude-opus-5`
template emits that profile and a reproducible local setup bundle.

No second transport, shell wrapper, live credential, or live provider call is
introduced. Focused tests use the existing mock HTTP harness to prove request
headers, model selection, response extraction, and setup rendering.

## Acceptance

1. The profile registry contains `claude:claude-opus-5` with Anthropic kind,
   endpoint, default model, and provider model id.
2. Profile expansion produces an Anthropic provider without changing existing
   Claude profiles.
3. The setup command accepts `claude-opus-5` and emits the canonical model and
   credential variable.
4. Mocked adapter proof verifies the Opus 5 model and Anthropic version header.
5. Focused Rust validation and exact-head review pass without a live API call.

## Scope boundary

Only provider profile expansion, setup generation, and focused tests are in
scope. Pricing, account provisioning, credentials, fallback policy, and model
benchmarking are follow-on work.
