# Structured Output Record

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Preserve HasHooks as the clean merge target accepted by csdlc-merge.

## Artifacts

- csdlc-v2/src/github.rs
- .csdlc/issues/5695/cards/sor.md
- csdlc-v2/src/github.rs
- csdlc-v2/src/merge.rs
- .csdlc/issues/5695/cards/sor.md

## Execution

- Map Behind, Blocked, Clean, Dirty, Draft, HasHooks, Unstable, and Unknown explicitly
- Classify blocked, unstable, draft, and unknown as waiting; behind as stale_base; dirty as conflicted
- Add focused coverage for all supported mergeability variants and classification behavior
- Normalize HasHooks to clean to preserve merge-gate compatibility
- Add a regression test proving the normalized HasHooks packet is accepted by validate_remote

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--check"
    ],
    "purpose": "Verify formatting for the bounded Rust change.",
    "outcome": "passed",
    "evidence_ref": "local:cargo-fmt-check:passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "-p",
      "csdlc-v2",
      "github::tests"
    ],
    "purpose": "Exercise every supported mergeability variant and preserve fail-closed pending classification.",
    "outcome": "passed",
    "evidence_ref": "local:cargo-test-github-tests:3-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "-p",
      "csdlc-v2",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Verify strict lint for the touched Rust library.",
    "outcome": "passed",
    "evidence_ref": "local:cargo-clippy-csdlc-v2-lib:passed"
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
