# Structured Task Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver a browser-trusted local Observatory HTTPS flow with reproducible browser and health proof.

## Deliverables

- Supported certificate issuance, trust, renewal or reissue, and startup flow
- Chrome, curl, HTML, Runtime-feed, configuration, and documentation evidence

## Acceptance

1. Chrome loads the supported Observatory URL without certificate warnings or bypasses
2. The chosen certificate has correct localhost names and valid trust/expiry behavior
3. Startup, configuration, docs, URLs, and health checks consistently use HTTPS
4. Observatory HTML and Runtime feed succeed through verified TLS
5. Expired, mismatched, missing, or untrusted certificates fail truthfully without deleting the last valid pair
6. Certificate private material is not committed or logged
7. macOS local proof is retained and Windows/Linux setup is implemented or explicitly bounded by supported portable behavior
8. One exact-revision review has no unresolved actionable findings

## Dependencies

- WP-01
- WP-02A
- Runtime/Observatory sprint coordination before WP-03

## Inputs

- adl-runtime/src/local_tls.rs
- adl-runtime/tests/local_tls.rs
- demos/html-observatory
- infra/runtime-v3/runtime-init.toml

## Non Goals

- AWS work
- Let's Encrypt issuance for localhost
- Disabling TLS verification
- Production public-domain certificate automation
- Unrelated Observatory redesign
