# Structured Output Record

Template: 1.0.0

Issue: 5413

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Added authenticated live local HTTPS Observatory consumption with freshness truth and corrected Runtime v3 parity/release evidence without authorizing default cutover.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/parity.rs
- demos/v0.91.7/html-observatory/app.js
- docs/architecture/runtime_v3_live_parity_remediation_5413.v1.json

## Execution

- Require bounded bearer authentication for the Observatory and fail closed in production
- Serve schema-v2 no-store feeds with atomic weather freshness and configured-origin CORS preflight
- Update the browser client and integrated proof to authenticate and reject stale or legacy feeds
- Reclassify fixture-only parity honestly and retain one proven live equivalence
- Record the full Runtime v3 release wave while keeping default cutover false
- Accept the new live_local_https evidence class only as live evidence

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Prove authenticated Observatory behavior, weather freshness, parity classification, release evidence, and full Runtime v3 regression safety",
    "outcome": "passed",
    "evidence_ref": "local:5413-full-runtime-clippy-fmt-local-https-integrated-observatory-proof"
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
