# Operator Review Surfaces

## Purpose

Give operators and reviewers a compact way to inspect Runtime v2 hardening
evidence.

## Required Surfaces

- first CSM run summary
- manifold boot and citizen admission evidence
- governed episode and scheduling evidence
- snapshot rehydrate wake continuity evidence
- CSM Observatory packet and operator report
- current manifold hardening status
- invariant coverage summary
- recent violation artifacts
- recovery eligibility decisions
- quarantine rationale
- security-boundary evidence
- release-readiness caveats

## WP-10 D7 Surface

WP-10 lands the first code-backed CSM Observatory projection for the v0.90.2
bounded run. The packet uses the inherited `adl.csm_visibility_packet.v1`
schema and is generated from Runtime v2 run artifacts instead of a standalone
parallel format.

Primary artifacts:

- `adl/src/runtime_v2/observatory.rs`
- `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`
- `adl/tests/fixtures/runtime_v2/observatory/operator_report.md`

Validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture
```

The report is rendered from the packet and checked for packet-truth alignment:
packet identity, manifold identity, event sequence 9, Freedom Gate allow/refuse
counts, wake-continuity evidence, and the explicit no-live-run/no-birthday
boundary.

## Non-Goals

- full dashboard product
- live long-running control plane
- replacing internal review
