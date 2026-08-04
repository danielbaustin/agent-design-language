# Structured Output Record

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Reconciled the Runtime v3 CSM API advertised endpoint inventory with the mounted route constants, kept the broader local CSM dispatch inventory distinct, and added focused drift tests for both surfaces.

## Artifacts

- .csdlc/prepared/issues/5766/design.md
- .csdlc/prepared/issues/5766/diagram.mmd
- adl-runtime/src/runtime_api.rs
- adl/src/csm_runtime_api.rs
- .csdlc/evidence/5766

## Execution

- adl-runtime/src/runtime_api.rs
- adl/src/csm_runtime_api.rs
- Made adl-runtime CSM_RUNTIME_API_ENDPOINTS an alias of CSM_RUNTIME_API_MOUNTED_ROUTES and routed the Axum router from the same path constants.
- Kept /v1/ready out of the mounted Runtime v3 CSM API advertised endpoint list; the broader local dispatch surface remains unversioned /ready only.
- Added a local CSM dispatch test that walks every advertised local endpoint and proves unknown routes report the supported endpoint set.

## Validation

[
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-wp-5766/target",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "runtime_api_contract_advertises_only_served_routes"
    ],
    "purpose": "Prove adl-runtime advertised endpoints equal the mounted route authority.",
    "outcome": "passed",
    "evidence_ref": "local stdout: 1 passed, 0 failed"
  },
  {
    "command": [
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-wp-5766/target",
      "cargo",
      "test",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "runtime_api_advertised_endpoints_resolve_through_local_dispatch"
    ],
    "purpose": "Prove every local CSM advertised endpoint resolves through runtime API dispatch and unknown paths return not_found with supported endpoint metadata.",
    "outcome": "passed",
    "evidence_ref": "local stdout: 1 passed, 0 failed; transient adl/Cargo.lock resolution churn restored before commit"
  },
  {
    "command": [
      "/Users/daniel/.cargo/bin/cargo",
      "test",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "runtime_api_advertised_endpoints_resolve_through_local_dispatch"
    ],
    "purpose": "Prove every advertised local CSM dispatch endpoint resolves through runtime_api_response and unknown routes return supported endpoint metadata.",
    "outcome": "passed",
    "evidence_ref": "local-csm-dispatch-inventory.log"
  },
  {
    "command": [
      "/Users/daniel/.cargo/bin/cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "runtime_api_contract_advertises_only_served_routes"
    ],
    "purpose": "Prove the mounted Runtime v3 CSM API advertises exactly the mounted /v1 routes.",
    "outcome": "passed",
    "evidence_ref": "runtime-api-mounted-inventory.log"
  },
  {
    "command": [
      "/opt/homebrew/bin/git",
      "diff",
      "--check"
    ],
    "purpose": "Prove the bounded #5766 diff has no whitespace or patch-format defects.",
    "outcome": "passed",
    "evidence_ref": "runtime-endpoint-diff-hygiene.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
