# Structured Task Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Review and route or fix accepted #5791 findings only.

## Deliverables

- Exact-revision review corpus
- Specialist lane notes
- Deduplicated findings register
- Synthesis and validation record
- PR with Closes #5791

## Acceptance

1. AC-1 exact current origin/main revision is recorded.
2. AC-2 residual coding issue closure and merged PR truth is verified.
3. AC-3 review corpus emphasizes issues closed since the prior review.
4. AC-4 actual code/test/CI/docs/evidence/lifecycle surfaces are reviewed.
5. AC-5 specialist findings are deduplicated and severity ranked.
6. AC-6 accepted in-scope findings are fixed or routed truthfully.
7. AC-7 exact-head re-review and focused validation are recorded.
8. AC-8 PR is published with Closes #5791 and closed out truthfully after merge.

## Dependencies

- Residual coding issues required by #5791 are closed through merged PRs or truthfully blocked before review.

## Inputs

- GitHub issue #5791
- v0.91.8 milestone docs and issue wave
- closed issue and merged PR truth since the prior WP-18 review
- current origin/main source tree

## Non Goals

- v0.92 implementation
- AWS operations
- root main checkout mutation
