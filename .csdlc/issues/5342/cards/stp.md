# Structured Task Prompt

Template: 1.0.0

Issue: 5342

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement complete portable record, signing, trust, tamper, and channel contracts only; do not absorb Runtime v2/v3, adapters, persistence, telemetry backends, CLI, deletion, or cloud work.

## Deliverables

- Versioned stable ErrorRecord, EventRecord, TraceRecord, ExecutionResult, and ArtifactDescriptor contracts
- Bounded deterministic canonical-byte and signed-envelope APIs
- Ed25519 signing and verification profile using ed25519-dalek
- External TrustPolicy with key id, profile, kind, validity, and revocation enforcement
- Generated checked schemas and positive/negative fixtures
- Tamper matrix plus byte-channel and fresh-process verification proof
- Exact COTS, forbidden dependency, scope, LoC, and validation-latency proof

## Acceptance

1. AC-1: Errors, events, traces, results, and artifacts are explicit versioned deny-unknown-fields contracts with stable identities and bounded fields
2. AC-2: Canonical bytes are deterministic, domain-separated, length-delimited, recursively key-sorted, exclude floating-point values, and yield stable SHA-256 identities
3. AC-3: Real Ed25519 signatures bind record kind, contract/profile version, key id, canonical payload digest, and canonical payload bytes without custom cryptography
4. AC-4: Verification uses an external immutable trust policy and fails closed for unknown keys, wrong kind/profile, logical expiry, revocation, malformed bytes, digest mismatch, or invalid signature
5. AC-5: Every signed field class, payload, key id, profile, signature, digest, kind, sequence, truncation, extension, unknown-field, duplicate-key, UTF-8, and bounds tamper case is rejected or exactly canonicalized as declared
6. AC-6: Byte-channel and fresh-process proof produces identical canonical bytes and verification outcomes across repeated runs without filesystem, network, clock, environment, provider, or runtime authority
7. AC-7: Generated checked schemas, Rust decoding, semantic validation, and fixtures remain aligned for every public record and envelope
8. AC-8: The crate uses only reviewed COTS and excludes Runtime v2/v3, incumbent ADL, C-SDLC, async, HTTP/TLS, cloud, database, telemetry-exporter, key-store, and workflow-engine dependencies
9. AC-9: Implementation stays within 3000 Rust implementation LoC and 3000 test/fixture LoC, with focused/quality proof under 120 seconds, channel/tamper proof under 300 seconds, and full proof under 600 seconds
10. AC-10: Exact #5339/#5340 terminal receipts, released claims, actual merge ancestry, disjoint protected paths, review truth, stable CI, post-merge proof, and typed closeout are retained

## Dependencies

- #5339 typed closed_out receipt with released claim and merge commit ancestry on current origin/main
- #5340 typed closed_out receipt with released claim and merge commit ancestry on current origin/main
- Landed adl-language and adl-engine contracts are read-only inputs

## Inputs

- AGENTS.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/DESIGN_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/features/ADL_V2_CORE_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md
- adl-v2/crates/adl-language
- adl-v2/crates/adl-engine

## Non Goals

- Runtime v2 or Runtime v3 source, control, supervision, ingress, continuity, networking, or Observatory changes
- Provider/tool adapters, persistence, object stores, databases, telemetry exporters, key stores, PKI, TLS, CLI, or selector work
- Key generation, credential discovery, environment scanning, trust-on-first-use, or self-authorized trust policy
- Compatibility with undocumented incumbent binary record formats
