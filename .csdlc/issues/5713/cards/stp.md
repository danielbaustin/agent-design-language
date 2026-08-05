# Structured Task Prompt

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement real Runtime v3 local TLS bootstrap with rcgen and rustls in the issue worktree, then validate, review, publish, shepherd, merge, and verify GitHub closure; typed closeout is out of scope.

## Deliverables

- TLS bootstrap mode configuration and validation
- Rust rcgen local certificate bootstrap module and repo-native binary
- Focused tests for bootstrap/reuse/SAN/permissions/concurrency/replacement/external preservation
- Runtime TLS local trust documentation
- Ready PR with Closes #5713

## Acceptance

1. AC-1: A repo-native Rust Runtime v3 operation creates a stable self-signed development server certificate with rcgen under an explicit absolute configured state root
2. AC-2: Private key and certificate are created once with restrictive permissions and are never logged, committed, copied into evidence, or regenerated on ordinary restart
3. AC-3: The public certificate is persisted separately for one-time macOS, Windows, or Linux trust
4. AC-4: The certificate includes configured DNS/IP SANs and server-auth usage and is accepted by rustls and standard browsers after trust
5. AC-5: Runtime restart reuses the same certificate identity; replacement is explicit and atomic, and failed replacement preserves the last valid certificate with a structured event
6. AC-6: Existing externally managed certificate paths remain supported without mutation
7. AC-7: Configuration explicitly selects managed_external or local_self_signed and production defaults fail closed rather than creating an implicit development certificate
8. AC-8: Focused tests prove bootstrap, reuse, SAN validation, restrictive permissions, concurrent exclusion, replacement, failed replacement preservation, and external preservation
9. AC-9: macOS, Linux, and Windows use the same Rust implementation and configuration schema
10. AC-10: Documentation explains one-time trust and replacement/re-trust without requiring a private CA

## Dependencies

- Current origin/main at ab4e9e2217c152df47b1754b66b01febb4a59549
- Existing Runtime v3 rustls TLS loading
- Existing adl-runtime rcgen/rustls dependencies
- Existing rcgen dependency policy

## Inputs

- GitHub issue #5713
- adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs
- adl-runtime/src/local_tls.rs
- adl-runtime/tests/local_tls.rs
- docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md

## Non Goals

- No AWS Private CA or local CA hierarchy
- No automatic OS trust-store mutation
- No hard-coded ports, hostnames, certificate paths, or production credentials
- No Runtime or Observatory public API behavior changes beyond certificate deployment documentation
- No #5733 or WP-21 edits
