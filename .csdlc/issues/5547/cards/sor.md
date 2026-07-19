# Structured Output Record

Template: 1.0.0

Issue: 5547

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented scope-aware C-SDLC substantive revision identity and recorded the IR-4645-012 ownership-first split plan.

## Artifacts

- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate5.rs
- docs/reviews/v0.91.7/review-fixes-5547/CSDLC_IDENTITY_AND_OWNERSHIP_RESIDUALS_5547.md

## Execution

- csdlc-v2/src/git.rs now applies the declared review scope pathspec to tracked diff and untracked-file hashing.
- csdlc-v2/tests/gate5.rs adds a regression proving out-of-scope dirty files do not stale scoped review identity while in-scope changes do.
- docs/reviews/v0.91.7/review-fixes-5547/CSDLC_IDENTITY_AND_OWNERSHIP_RESIDUALS_5547.md records the #5547 disposition and v0.91.8 ownership split routing.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "substantive_revision_honors_review_scope_pathspecs"
    ],
    "purpose": "Focused proof that C-SDLC substantive review identity honors declared review scope pathspecs.",
    "outcome": "passed",
    "evidence_ref": "Local run in .worktrees/adl-wp-5547 with TMPDIR=.adl/tmp/5547 and CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5547/csdlc-v2-target; 1 matching test passed."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "purpose": "Validate the surrounding C-SDLC review/publication guard behavior after scoped revision identity change.",
    "outcome": "passed",
    "evidence_ref": "Local run in .worktrees/adl-wp-5547 with TMPDIR=.adl/tmp/5547 and CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5547/csdlc-v2-target; gate5 passed 13 tests."
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
