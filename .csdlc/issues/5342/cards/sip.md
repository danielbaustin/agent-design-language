# Structured Intent Prompt

Template: 1.0.0

Issue: 5342

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement a small portable adl-records crate with stable bounded record contracts, deterministic canonical bytes, real Ed25519 signatures, explicit trust policy, and fail-closed verification.

## Required Outcome

Versioned errors, events, traces, results, and artifact descriptors cross byte channels as deterministic signed envelopes and are accepted only after exact canonical-byte, signature, trust-policy, bounds, and tamper verification.

## Scope

- adl-v2/crates/adl-records versioned bounded record contracts
- deterministic canonical bytes and SHA-256 payload identity
- real Ed25519 signing and verification through ed25519-dalek
- external immutable trust policy with kind, profile, validity, and revocation decisions
- tamper, channel, fresh-process, schema, bounds, COTS, and budget proof
- issue-local lifecycle, design, review, validation, and evidence records

## Authority

- Issue #5342 owns only adl-v2/crates/adl-records and issue-local C-SDLC records
- Issues #5339 and #5340 are merged read-only language and engine dependencies
- Trust policy is external verifier input and cannot be self-issued by a signed envelope
- The crate owns no key generation/storage, filesystem/network/process IO, telemetry backend, provider/tool adapter, Runtime v2/v3, C-SDLC, CLI, or cloud authority
- Incumbent ADL and Runtime code are behavioral evidence only and cannot be copied, adapted, imported, linked, or changed

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and semantic card operations only
- Never edit tracked issue work on root main
- Never use raw gh, AWS, hard-coded IPs, credentials, or provider network calls
- Use /Volumes/FastWork for all Cargo build, cache, and temporary output
- Coordinate product paths with active #5589, #5590, and #5615 claims
- Run mandatory bounded exact-revision subagent review and fix every actionable finding before publication
