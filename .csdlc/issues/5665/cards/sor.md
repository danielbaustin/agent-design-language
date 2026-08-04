# Structured Output Record

Template: 1.0.0

Issue: 5665

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Completed an API-only Runtime v3 Rustls WSS proof surface with authenticated upgrade, bidirectional frames, rotation/revocation rechecks, shutdown acknowledgement, health-state distinctions, sink-bounded telemetry, port 20997 init truth, and a feature/adapter matrix.

## Artifacts

- adl-runtime/src/runtime_api.rs
- adl-runtime/tests/runtime_api_wss.rs
- infra/runtime-v3/runtime-api-5665.toml
- docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json
- .csdlc/prepared/issues/5665/amend-obsolete-wp12-wrapper-scope.json

## Execution

- Added Axum/Rustls Runtime API helpers in adl-runtime for /health, /metrics, and /acip/ws.
- Reused the existing RuntimeApiCredentialStore for WSS handshake authentication and live revocation checks.
- Added Runtime API health-state, telemetry sink-capability, and feature-matrix contracts.
- Added a real TLS/WSS integration test that exercises auth failure, successful bidirectional frames, credential rotation overlap, revocation closeout, and shutdown acknowledgement.
- Added the clean-checkout Runtime API init file for port 20997 and the #5665 feature/adapter matrix artifact.
- Recorded the blocked obsolete-wrapper deletion attempt: #5587 still protects adl/Cargo.toml.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the #5665 Runtime API implementation is warning-clean under strict Clippy.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Prove #5665 Runtime API WSS and Observatory truth without URL-only, fixture-only, metadata-only, Python, AWS, or degraded proof.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-wss-focused.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "run_wp12_acip_websocket_transport_proof"
    ],
    "purpose": "Prove the retired WP-12 wrapper source still compiles as a fail-closed tombstone while removing the duplicate proof generator path.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5665/wp12-wrapper-tombstone-check.log"
  },
  {
    "command": [
      "bash",
      "-lc",
      "for bin in run_wp12_acip_websocket_transport_proof run_v0916_acip_aee_memory_integration run_v0916_integrated_runtime_soak run_v0916_runtime_failure_injection run_v0917_integrated_resilience_failure_injection; do cargo check --locked --manifest-path adl/Cargo.toml --bin \"$bin\"; done"
    ],
    "purpose": "Prove each retired executable Runtime v2 or duplicate WSS proof wrapper compiles as a fail-closed tombstone while #5665 owns the real Runtime v3 API WSS proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5665/runtime-v2-wrapper-tombstone-checks.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--numstat",
      "main"
    ],
    "purpose": "Measure physical added and deleted lines against main and preserve the net-negative #5665 proof after replacing obsolete wrappers with fail-closed tombstones.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5665/runtime-v3-loc-measurement.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "purpose": "Prove the authenticated Runtime API WSS feature_matrix frame serves the committed feature/adapter matrix artifact, including health-state and telemetry rows.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5665/runtime-v3-wss-focused.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Prove the committed matrix-artifact WSS feature_matrix path as part of the full adl-runtime test suite retained in the evidence log.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5665/runtime-v3-wss-focused.log"
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
