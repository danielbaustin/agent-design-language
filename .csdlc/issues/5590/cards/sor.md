# Structured Output Record

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Implemented secure Runtime v3 local and remote Observatory access through configured HTTPS and authenticated WebSocket transport, an external guardian executable, actual-address discovery, Vector-owned telemetry boundaries, operational selector rollback, and signed continuity proof for candidate and restored prior runtimes.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl/tools/runtime_v3_operational_selector.sh
- adl/tools/test_runtime_v3_operational_selector.sh
- adl/tools/run_runtime_v3_operational_proof.sh
- demos/v0.91.7/html-observatory/app.js

## Execution

- Expose actual bound control address, configured public HTTPS base, and WSS discovery in Runtime-owned readiness and Observatory feeds
- Require configured-origin WebSocket upgrades and bounded session bearer authentication before read-only live feed delivery
- Run the HTML Observatory over authenticated HTTP fallback and authenticated WSS live updates without hard-coded deployment addresses
- Add the standalone external guardian binary over the existing guardian library with bounded restart, backoff, and shutdown controls
- Add an operational process selector that activates candidate or prior Runtime v3 launch contracts and performs bounded graceful replacement
- Prove candidate-to-prior rollback over real TLS processes with signed generation-1 continuity for both shutdowns
- Align the WebSocket test dependency with Axum's protocol version and retain exact lockfile truth

## Validation

[
  {
    "command": [
      "ADL_RUNTIME_V3_PROOF_ROOT=/Volumes/FastWork/adl-5590 CARGO_HOME=/Volumes/FastWork/adl-5590/cargo-home CARGO_TARGET_DIR=/Volumes/FastWork/adl-5590/runtime-target bash adl/tools/run_runtime_v3_operational_proof.sh"
    ],
    "purpose": "Prove the landed Runtime v3 Parity-D path through an external guardian, HTTPS bearer authorization, WSS discovery, operational rollback restoration, and cryptographically restored candidate and prior continuity.",
    "outcome": "passed",
    "evidence_ref": "merge:8ba224027a7ebd410aa4596c9edce571758f8a0a:/Volumes/FastWork/adl-5590/operational-proof.*; runtime_v3_operational_proof=pass"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
