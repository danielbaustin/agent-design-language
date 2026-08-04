# Structured Review Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5791
adl/tools/run_cargo_validation.sh
adl/tools/test_run_cargo_validation.sh
csdlc-v2/tests/gate_terminal_authority_deletion.rs
docs/reviews/v0.91.8/internal-review-5791

## Prompts

- Does the review corpus include issues closed since the prior WP-18 review?
- Does the review inspect actual code and validation surfaces?
- Are findings deduplicated and evidence-bound?
- Are release-readiness claims supported by exact current evidence?

## Findings

[
  {
    "id": "IR5791-04",
    "severity": "p1",
    "summary": "PR #5799 CI failed because run_cargo_validation required --locked for cargo fmt, but cargo fmt does not support --locked.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:3f26b4db01d3cba21c6e13708b8a5d6d8ced188c:bfe7b202ddee6cf4b38ae0179edcb6cb3fc1cd9ee45b9f3df8beb9d9a6a01b3c",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The PR still needs exact-head GitHub CI after republishing.

## Review Result

Revision: Some("git-blake3:3f26b4db01d3cba21c6e13708b8a5d6d8ced188c:bfe7b202ddee6cf4b38ae0179edcb6cb3fc1cd9ee45b9f3df8beb9d9a6a01b3c")

Reviewer: Some("codex-current-head-ci-recovery-review")

Result: pass
