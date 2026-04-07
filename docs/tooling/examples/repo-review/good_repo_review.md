## Metadata
- Review Type: repo_review
- Subject: adl/
- Reviewer: repo-code-review
- Date: 2026-04-06
- Input Surfaces:
  - adl/src/provider.rs
  - Cargo.toml
- Output Location: .adl/reviews/20260406-120000-repo-review.md

## Scope
- Reviewed: provider selection, retry classification, top-level Rust manifests
- Not Reviewed: external service integrations and non-Rust tooling
- Review Mode: code
- Gate: pre-merge

## Findings
1. [P2] Provider retry policy treats deterministic schema failures as retriable
Location: adl/src/provider.rs:161
Impact: wastes retry budget and obscures the real configuration failure mode
Trigger: malformed or unsupported provider policy response
Evidence: retry classification path still maps deterministic policy/schema failures into the retryable bucket
Fix Direction: classify deterministic provider schema/policy failures separately and stop retrying them

2. [P4] Provider tests are concentrated in one large surface
Location: adl/src/provider.rs
Impact: slows safe review and makes targeted regression additions harder
Trigger: adding or auditing provider-edge behavior
Evidence: provider logic and many edge tests are still coupled in one large review surface
Fix Direction: split focused provider-edge tests into narrower modules over time

## System-Level Assessment
The reviewed surface is directionally sound, but failure-classification behavior is still doing too much in one place. The dominant risk is not missing functionality; it is that deterministic provider errors can still look transient and mislead both operators and reviewers.

## Recommended Action Plan
- Fix now: stop retrying deterministic provider schema/policy failures
- Fix before milestone closeout: add focused regression coverage for deterministic provider failure classes
- Defer: split remaining oversized provider review surfaces if they continue to slow safe iteration

## Follow-ups / Deferred Work
- Consider a later cleanup issue if provider-edge tests continue growing in one file

## Final Assessment
This reviewed surface is not yet trustworthy enough for merge without the retry-classification fix, but it is bounded and ready for a targeted repair.
