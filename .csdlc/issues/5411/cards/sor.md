# Structured Output Record

Template: 1.0.0

Issue: 5411

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Resolved Runtime v3 selector truth, Unix guardian process-tree containment, scoped periodic resource-pressure monitoring, signed stateful checkpoint-before-stop behavior, reversible control admission, and release-evidence classification.

## Artifacts

- docs/reviews/v0.91.7/runtime-v3-5411/DESIGN.md
- docs/reviews/v0.91.7/runtime-v3-5411/DIAGRAM.mmd
- docs/architecture/runtime_v3_release_proof_gate_5220.v1.json
- docs/architecture/runtime_v3_current_inventory.v1.json

## Execution

- Clarified selector as a reporting contract without runtime invocation
- Contained Unix descendants with bounded capture and process-group liveness escalation
- Connected configured weather pressure to signed checkpoint and graceful stop with retry on checkpoint failure
- Retained stateful recorder snapshots with explicit current and legacy schema handling
- Classified release proof as executed, contract-only, ignored, or deferred

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "Full Runtime v3 kernel behavior across all targets",
    "outcome": "passed",
    "evidence_ref": "local FastWork validation: all non-ignored tests passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "Guardian containment, restart, signaling, and independence behavior",
    "outcome": "passed",
    "evidence_ref": "local FastWork validation: 117 tests passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Runtime v3 kernel warning-free quality gate",
    "outcome": "passed",
    "evidence_ref": "local FastWork strict Clippy"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Guardian library warning-free quality gate",
    "outcome": "passed",
    "evidence_ref": "local FastWork strict Clippy"
  },
  {
    "command": [
      "python3",
      "adl-runtime-kernel/tools/test_generate_runtime_inventory.py"
    ],
    "purpose": "Runtime v3 implementation budget and deterministic inventory truth",
    "outcome": "passed",
    "evidence_ref": "11,935 kernel LoC; 867-line shared guardian reported separately; deterministic inventory current"
  },
  {
    "command": [
      "python3",
      "adl-runtime-kernel/tools/test_generate_runtime_inventory.py"
    ],
    "purpose": "Final Runtime v3 implementation budget and deterministic inventory truth",
    "outcome": "passed",
    "evidence_ref": "11,964 kernel LoC; 867-line shared guardian auxiliary surface; 12,831 combined selected surface; 186 test attributes; inventory current"
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
