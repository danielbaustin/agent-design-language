# Structured Review Prompt

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl-runtime/Cargo.toml
adl-runtime/Cargo.lock
adl-runtime/src/lib.rs
adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs
adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
adl-runtime/src/local_tls.rs
adl-runtime/tests/local_tls.rs
docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md
.csdlc/evidence/5713
.csdlc/issues/5713

## Prompts

- Does configuration fail closed unless TLS mode is explicitly managed_external or local_self_signed?
- Can local bootstrap ever mutate externally managed certificate/key paths?
- Does ordinary restart reuse the same certificate identity without regeneration?
- Is replacement explicit and atomic with last-valid preservation on failure?
- Do tests prove SANs, server-auth, rustls acceptance, restrictive permissions, and concurrent exclusion?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Native Windows execution remains a pre-merge integration residual because nessus.local did not resolve during pre-PR validation; the protected current-user-only DACL behavior is implemented and covered by a native-Windows-gated Rust test.

## Review Result

Revision: Some("git-blake3:00a261cb0c47ddcd6a2d42107f9ada74f3a62aea:f5bb750c78039327541f54205418d4db74c07a6b6fa2aab7d6c7f0e954741c66")

Reviewer: Some("subagent:gpt-5.5-sartre")

Result: pass
