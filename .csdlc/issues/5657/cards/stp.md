# Structured Task Prompt

Template: 1.0.0

Issue: 5657

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and validate the bounded Runtime v3 launch recovery slice, then review, publish, merge, and close it.

## Deliverables

- single Guardian-owned serve/readiness path
- coherent configured endpoint and Observatory routes
- real adapter readiness gate
- continuity identity correction
- authenticated WebSocket regression
- focused launch proof and operator documentation

## Acceptance

1. AC-1: clean checkout launch resolves one endpoint and reports actual readiness
2. AC-2: required production adapters are real or startup fails before readiness
3. AC-3: health, root Observatory, feed, and WebSocket routes are coherent and authenticated
4. AC-4: continuity identity excludes TLS private-key material and launch artifacts contain no plaintext secrets
5. AC-5: Guardian shutdown reaps the child and the second launch succeeds without manual cleanup
6. AC-6: focused Rust launch proof is fast and unrelated slow suites are not part of its gate

## Dependencies

- existing Runtime v3 kernel and Guardian proof surfaces
- GitHub issue #5657

## Inputs

- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- infra/runtime-v3/runtime-init.toml
- docs/architecture/RUNTIME_V3_CONTROL_OBSERVABILITY_ARCHITECTURE.md
- docs/architecture/RUNTIME_V3_FINAL_REVIEW_5175.md

## Non Goals

- Runtime v2 deletion or cutover
- distributed Guardian mesh or PKI issuance
- AWS/EC2/Spot/Windows qualification
- new custom HTTP/WebSocket/TLS/supervisor implementation
- placeholder or fixture-only production credit
