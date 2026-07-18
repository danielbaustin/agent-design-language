# Structured Review Prompt

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final proof that the focused llvm-cov route interoperates with the GitHub runner toolchain.

## Review Result

Revision: Some("git-blake3:209a2c14f7db7c977ffb79b0864c0a11807415c9:3d42c4164fd38abe0faf62a31ef9ba3cb7b387b9a08932e92dbb84654e2161dc")

Reviewer: Some("subagent:019f751d-2d23-70d1-b40f-69c756ed58d0")

Result: pass
