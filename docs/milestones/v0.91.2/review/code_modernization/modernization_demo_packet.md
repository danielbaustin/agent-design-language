# Moderne / OpenRewrite Modernization Demo Packet

## Demo Thesis

`WP-10` demonstrates how ADL can govern deterministic modernization more
intelligently than a simplistic tool-calling agent by separating:

- planning and scoping intelligence
- deterministic recipe execution
- explicit review and residual classification

## What This Packet Contains

- [modernization_interaction_plan.md](modernization_interaction_plan.md)
- [modernization_dry_run_evidence.md](modernization_dry_run_evidence.md)
- [modernization_reversibility_and_review_policy.md](modernization_reversibility_and_review_policy.md)
- [modernization_execution_command.md](modernization_execution_command.md)
- [modernization_execution_log.txt](modernization_execution_log.txt)
- [modernization_rewrite.patch](modernization_rewrite.patch)

## Proved Here

- the modernization lane is framed around Moderne, OpenRewrite, LST, and
  recipes with correct terminology
- the first proving path is bounded, dry-run-first, and review-gated
- ADL is positioned as the governed planning/authority layer rather than as a
  direct source editor
- reversibility and residual honesty are first-class requirements
- a real OpenRewrite Maven dry-run was executed against a small tracked Java
  fixture
- the dry-run produced one concrete modernization diff for reviewer inspection

## Why The Packet Uses A Fixture Instead Of The Main Repo

This repository is not a Java/Maven target repo, so a live OpenRewrite run
against the main ADL source tree would not be a truthful proof surface.

The packet therefore proves:

- command posture
- authority posture
- review posture
- non-claim posture
- a real deterministic dry-run on a bounded fixture
- a concrete rewrite artifact that can be inspected before any mutation would be
  accepted elsewhere

without pretending the main ADL repository itself is a Java modernization
target.

## Intended Next Real Execution Context

The next stronger proof would occur in a bounded Java/Maven target repository
where:

- a small recipe family is selected explicitly
- dry-run output is captured for a real target
- resulting diffs are reviewed and classified honestly
- mutation acceptance remains gated by explicit human review

## Non-Claims

- This packet does not claim live Moderne SaaS orchestration.
- This packet does not claim the main ADL repository itself was modernized.
- This packet does not claim mass rewrite readiness.
