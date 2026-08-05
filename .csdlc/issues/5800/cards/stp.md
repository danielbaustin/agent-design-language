# Structured Task Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver one supported browser-trusted localhost HTTPS flow for the separate Observatory and Runtime API, including atomic certificate handling, explicit operator trust, consistent configuration, and reproducible positive, negative, and platform proof.

## Deliverables

- Supported localhost certificate generation, validation, trust installation, and reissue flow
- Consistent Runtime API and separate Observatory HTTPS configuration and startup guidance
- Chrome, curl, HTML, health/readiness, Runtime-feed, and negative certificate evidence
- Platform disposition for macOS, Linux, and native Windows without inferred portability

## Acceptance

1. Chrome loads the supported Observatory URL without certificate warnings or bypasses
2. The committed certificate pair has correct localhost SANs, validity, Rustls compatibility, and private-key permissions
3. Runtime API, separate Observatory server, configured origins, docs, URLs, and probes consistently use HTTPS
4. Verified curl and browser paths reach Observatory HTML plus Runtime health, readiness, and feed
5. Missing, malformed, expired, mismatched, untrusted, or partial certificate states fail truthfully and preserve the last valid pair
6. No private key, token, trust-store export, or secret is committed or logged
7. macOS proof is retained and Linux/native Windows behavior is implemented or explicitly blocked with evidence
8. One exact-head review has no unresolved actionable findings

## Dependencies

- WP-01 issue 5817 merged and ancestral
- WP-02A proving substrate terminal before implementation
- Sprint 5855 coordination
- Serialization with issue 5820 for shared Runtime init or TLS paths

## Inputs

- adl-runtime/src/local_tls.rs
- adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs
- adl-runtime/tests/local_tls.rs
- infra/runtime-v3/runtime-init.toml
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/README.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml

## Non Goals

- AWS, ACM, Route53, or Let's Encrypt issuance for localhost
- Public production-domain certificate automation
- Disabling TLS verification, browser warnings, or CORS policy
- Serving Observatory HTML from Runtime or redesigning the Observatory
- Claiming WP-03, WP-14, or WP-18A completion
