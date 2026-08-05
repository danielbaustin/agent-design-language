# WP-02A CI And Coverage Reliability Design

## Outcome And Current Substrate

Issue #5801 simplifies the existing CI decision layer without weakening source
merge proof. The current substrate is `.github/workflows/ci.yaml`,
`adl/tools/ci_path_policy.sh`, `adl/tools/run_pr_fast_test_lane.sh`,
`adl/tools/check_coverage_impact.sh`, the authoritative coverage runner, and
their contract tests. CI already cancels superseded runs and has separate fast,
slow, runtime, demo, and sharded coverage jobs; the task is to make their
selection deterministic, nonduplicative, and reviewable.

## Classification Contract

One path-policy decision must classify a change as current docs/review,
lifecycle metadata, workflow/tooling, ordinary product source, runtime-critical
source, or explicit slow-proof/coverage authority. Every class declares its PVF
lane, release-gate role, and why omitted heavy lanes are not required.

Metadata-only reconciliation may reuse exact-head source proof only when typed
lineage proves the source candidate is unchanged and current review evidence is
complete. Any product/source drift retains exact-head validation.

## Execution Design

1. Retain the required Gemini 3.1 Pro review packet of the current topology,
   including model identity, exact reviewed revision, prompt digest, response
   digest, findings, and dispositions, then convert actionable findings into a
   source-grounded lane map.
2. Centralize path classification and remove duplicate heavy lane triggers.
3. Keep fast and slow test families explicit; preserve fail-closed handling for
   unknown source paths and coverage-impact mappings.
4. Aggregate each coverage authority once per PR class and preserve shard
   provenance, exact SHA, and the authoritative final gate.
5. Preserve superseded-run cancellation while ensuring required final-state
   checks cannot be cancelled into a false success.
6. Wrap metadata-only lifecycle reuse behind typed lineage and negative tests.

## Invariants, Non-Goals, And Rollback

- Required-check names and source/product exact-head safety remain stable.
- No AWS, no test deletion, no coverage threshold reduction, and no broad
  workflow rewrite without compatibility proof.
- Unknown or mixed changes select the stronger proving lane.
- Rollback restores the previous path-policy/workflow routing as one coherent
  set; partial rollback of workflow and selector logic is forbidden.

## Proof Design

Contract fixtures must cover docs-only, lifecycle-only, tooling, ordinary Rust,
runtime-critical, unknown, mixed, stale-run, and metadata-lineage cases.
Coverage tests prove one authoritative aggregation with complete shard
provenance. Platform syntax and path handling are checked for Linux, macOS, and
Windows-facing contracts; required GitHub checks prove the final exact head.
The retained Gemini 3.1 Pro packet is a required review input, not replaceable
by an unnamed generic external review.

That packet names repository-relative prompt, response, and topology artifact
paths. The issue-local validator recomputes each SHA-256 digest and verifies the
topology file's Git blob at the exact 40-hex reviewed revision. Free-standing
digest strings or an untracked model response do not satisfy the review gate.
