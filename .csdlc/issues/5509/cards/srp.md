# Structured Review Prompt

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl/tools/ci_path_policy.sh
adl/tools/run_pr_fast_test_lane.sh
adl/tools/test_ci_path_policy.sh
adl/tools/test_run_pr_fast_coverage_lane.sh
adl/tools/test_run_pr_fast_test_lane.sh

## Prompts

- Can an unrelated mixed-crate change enter the bounded route?
- Does each changed crate execute its own tests?
- Can coverage omit one crate while still reporting success?
- Are Runtime v2 tests excluded by construction?

## Findings

[
  {
    "id": "F-5509-1",
    "severity": "p1",
    "summary": "The committed review metadata changed HEAD without an exact non-substantive proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:23fbe8a6de84734c2b517c3d36220b3fd960a4db:05b5b27993b947dbf821abcb156dd5f22a6b251761c2a9eefd485fa5a7476afd",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final proof that the focused llvm-cov route interoperates with the GitHub runner toolchain.

## Review Result

Revision: Some("git-blake3:23fbe8a6de84734c2b517c3d36220b3fd960a4db:05b5b27993b947dbf821abcb156dd5f22a6b251761c2a9eefd485fa5a7476afd")

Reviewer: Some("subagent:019f751d-2d23-70d1-b40f-69c756ed58d0")

Result: pass
