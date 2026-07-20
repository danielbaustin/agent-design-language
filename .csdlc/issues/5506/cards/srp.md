# Structured Review Prompt

Template: 1.0.0

Issue: 5506

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl/tools/check_coverage_impact.sh
adl/tools/run_pr_fast_coverage_lane.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_run_pr_fast_coverage_lane.sh

## Prompts

- Can auth-only routing skip required ADL tests for a mixed change?
- Does the Runtime v3 expression select the intended tests?
- Did any runtime source enter the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Actual llvm-cov interoperability remains hosted CI proof.

## Review Result

Revision: Some("git-blake3:5620c6e920dcbb232c4160c171c5bf3f4e60f845:1fd30361d1d46debae0269d0d84fc878c6d136f704d2fcab8abc7f2b93e8bab0")

Reviewer: Some("subagent:019f74a4-d2d6-7e51-b69d-e92676e69394")

Result: pass
