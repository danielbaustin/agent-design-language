# CodeBuddy Multi-Agent Review Showcase Demo

This `v0.90` demo shows how the CodeBuddy review-engine skill family can turn
one bounded repository packet into a serious, product-style review package
without building the future `codebuddy.ai` web app or mutating a customer
repository.

The demo is fixture-backed and deterministic. It is a showcase/rehearsal packet,
not a live customer review. That boundary is intentional: `#2070`
(`review-quality-evaluator`) is still underway, so the quality-evaluation lane
is marked as staged instead of pretending the lane is already available.

## Command

```bash
bash adl/tools/demo_v090_codebuddy_review_showcase.sh
```

## What It Builds

The demo writes a review packet under:

```text
artifacts/v090/codebuddy_review_showcase/
```

Primary proof surfaces:

- `run_manifest.json`
- `repo_scope.md`
- `repo_inventory.json`
- `specialist_reviews/code.md`
- `specialist_reviews/security.md`
- `specialist_reviews/tests.md`
- `specialist_reviews/docs.md`
- `specialist_reviews/architecture.md`
- `specialist_reviews/dependencies.md`
- `diagrams/system_map.mmd`
- `diagrams/diagram_manifest.md`
- `diagrams/diagram_review.md`
- `redaction_report.md`
- `test_recommendations/test_gap_report.md`
- `issue_planning/issue_candidates.md`
- `adr_candidates/adr_candidates.md`
- `fitness_functions/fitness_function_plan.md`
- `final_report.md`
- `quality_evaluation.md`
- `demo_operator_result.json`

## Agent Roles

- `repo-packet-builder` defines the bounded review packet.
- `repo-review-code` reviews correctness and maintainability.
- `repo-review-security` reviews trust boundaries and secret handling.
- `repo-review-tests` reviews coverage and proof gaps.
- `repo-review-docs` reviews onboarding and documentation truth.
- `repo-architecture-review` reviews boundaries, state, and coupling.
- `repo-dependency-review` reviews manifests and supply-chain posture.
- `repo-diagram-planner` and `diagram-author` produce a source-backed diagram
  brief and Mermaid source.
- `architecture-diagram-reviewer` reviews diagram truth boundaries.
- `redaction-and-evidence-auditor` blocks publication until privacy gates pass.
- `review-to-test-planner` maps findings to test follow-through.
- `finding-to-issue-planner` creates human-approved issue candidates.
- `adr-curator` drafts ADR candidates without accepting decisions.
- `architecture-fitness-function-author` proposes enforceable architecture
  invariants without installing CI policy.
- `product-report-writer` produces the customer-grade final report.
- `review-quality-evaluator` is included as a staged lane pending `#2070`.

## Truth Classification

This demo classifies itself as `non_proving` because it is a deterministic
showcase packet, not a live multi-agent execution across all implemented skills.
It still proves the expected artifact shape, gates, role order, severity
preservation, redaction boundary, and final report compatibility with the
CodeBuddy review packet spec.

Run the test wrapper for a clean validation pass:

```bash
bash adl/tools/test_demo_v090_codebuddy_review_showcase.sh
```

## Non-Goals

- It does not build the `codebuddy.ai` product application.
- It does not call live providers.
- It does not create remediation PRs or tracker issues.
- It does not publish customer artifacts.
- It does not claim the staged `review-quality-evaluator` lane is complete.
