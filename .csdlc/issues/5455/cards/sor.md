# Structured Output Record

Template: 1.0.0

Issue: 5455

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Remediated stale owner-binary provenance and replaced the editor help smoke test with an implemented-phase semantic mutation proof.

## Artifacts

- csdlc-v2/src/operator.rs
- csdlc-v2/tests/gate10a.rs
- https://github.com/danielbaustin/agent-design-language/pull/5460
- csdlc-v2/src/operator.rs
- csdlc-v2/src/bin/csdlc-install.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests/gate10a.rs
- #5540

## Execution

- Install receipts record source checkout revision or content provenance fallback
- Coexistence verification rejects stale owner-binary provenance explicitly
- Gate 10A executes a freshly installed stable editor
- Repository coexistence now rejects every install receipt that does not exactly match the current Git revision.
- Trusted Git provenance is issued only by a clean, unchanged repository build performed by csdlc-install.
- Direct artifact installation remains content provenance and cannot pass repository coexistence verification.
- Gate 10A proves stale artifacts under an advanced checkout fail closed and the installed editor performs implemented-phase approve-design.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Prove current merged-head install provenance, stale rejection, atomic install, and stable editor execution behavior.",
    "outcome": "passed",
    "evidence_ref": "Fresh recovery run at PR #5460 head fb7d09a56: 9 passed, 0 failed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove trusted clean-source build installation, fail-closed stale/content provenance, atomic installation, and implemented-phase editor mutation.",
    "outcome": "passed",
    "evidence_ref": "Head e496f0af5: Gate 10A 9 passed, 0 failed; strict all-target all-feature Clippy also passed. Broad owner lane is independently blocked by stale v1 guidance tracked in #5558."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove exact clean-source build provenance including untracked build inputs, fail-closed artifact provenance, atomic installation, and implemented-phase editor mutation.",
    "outcome": "passed",
    "evidence_ref": "Head 7d314fded: Gate 10A 10 passed, 0 failed; strict all-target all-feature Clippy passed. Broad owner lane remains independently blocked by #5558."
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
