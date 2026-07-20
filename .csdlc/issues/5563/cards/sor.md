# Structured Output Record

Template: 1.0.0

Issue: 5563

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Permit typed reapproval of previously approved initialized records only when current authored design inputs are demonstrably stale.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- #5563
- #5306

## Execution

- Approve-design validates CAS, active claim, card projections, and current authored digests before selecting the recovery path.
- Initialized approved reapproval is allowed only when SPP or VPP design/diagram digests differ from current authored files.
- Recovery records a distinct audit reason and retains normal pending, bound, and implemented approval behavior.
- Focused Gate 2 regression rejects redundant reapproval, proves stale recovery, and proves doctor readiness afterward.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "initialized_approved_stale_design_can_be_reapproved_before_readiness"
    ],
    "purpose": "Prove redundant initialized approval rejects, stale approved inputs recover atomically, and doctor readiness resumes.",
    "outcome": "passed",
    "evidence_ref": "Focused regression 1 passed, 0 failed; strict all-target all-feature Clippy passed. Existing non-Git Gate 2 fixture failure remains independently tracked in #5548."
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
