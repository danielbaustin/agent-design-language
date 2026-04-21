# Internal Review - v0.90.2

## Metadata

- Milestone: `v0.90.2`
- Version: `v0.90.2`
- Canonical issue / WP: `#2260` / `WP-16`
- Date: 2026-04-21
- Scope: internal review of milestone truth, proof surfaces, demo coverage,
  review tooling, compression posture, and release-tail readiness

## Executive Summary

Recommendation: proceed to `WP-17` external / third-party review after this
`WP-16` record lands, but do not treat `v0.90.2` as release-ready until
accepted findings are remediated or explicitly deferred through `WP-18`.

The v0.90.2 tranche is substantial and well-supported. The milestone now has a
bounded first CSM run evidence spine, feature-by-feature proof coverage across
D1-D11, integrated CSM run validation, recovery/quarantine evidence, governed
adversarial hardening probes, and release-truth docs that correctly preserve the
non-claims around true birthday, v0.91 civilization scope, v0.92 identity
rebinding, and complete security ecology.

The internal review did not find a missing flagship implementation, hollow demo
matrix, or stale release-status contradiction in the tracked v0.90.2 milestone
package. The remaining findings are tooling and review-substrate truth gaps:
the multi-agent repo-review suite contract has drifted from the current
per-skill schema metadata, and the repo packet builder still names issue
worktrees as if they were the repository. Both should be fixed before release
closeout because they weaken reviewer confidence, but neither invalidates the
Runtime v2 / CSM proof tranche itself.

## Review Method

This pass used the WP-16 issue bundle, the tracked milestone docs, local
closeout records, the CodeBuddy-style repo packet builder, repo-review skill
contract tests, demo operator proof scripts, focused Runtime v2 cargo tests, and
manual claim-boundary review against the v0.90.2 milestone package.

The review explicitly did not remediate findings. Accepted findings should route
through `WP-18` or a named follow-up if the release owner chooses to defer them.

## Findings

### F1. P2 - Multi-agent repo-review suite contract test is stale

Location: `adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh`,
`adl/tools/skills/repo-review-code/adl-skill.yaml`

The suite-level contract test still expects each specialist skill metadata file
to reference the older shared `MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` document.
Current skill metadata instead points at per-skill schema documents such as
`REPO_REVIEW_CODE_SKILL_INPUT_SCHEMA.md`. As a result, the broader suite
contract test fails before it can exercise the rest of the contract, even
though the specialist contract test passes.

Impact: reviewers lose one high-level proof that the multi-agent review skill
suite is coherently wired. This is especially visible because v0.90.2 depends
on the review-skill family as part of WP-16/WP-17 quality posture.

Recommended action: update the suite contract test to accept the current
per-skill schema model, or update skill metadata to include both the shared
suite reference and the per-skill schema reference if both are intended to be
canonical.

### F2. P2 - Repo packet builder records worktree name as repository name

Location:
`.adl/reviews/v0.90.2/internal/codebuddy/repo-packet/run_manifest.json`,
`.adl/reviews/v0.90.2/internal/codebuddy/repo-packet/repo_scope.md`

The CodeBuddy repo packet generated during WP-16 records the repository name as
the issue worktree directory, `adl-wp-2260`, rather than the canonical project
name. The packet evidence otherwise points at the right worktree revision and
repo contents, but the top-level identity is misleading for review consumers.

Impact: this does not invalidate the source evidence, but it makes packet
provenance look local-worktree-specific rather than project-specific. That is a
recurring review polish gap and should be fixed before relying on these packets
as customer-facing or third-party-review inputs.

Recommended action: make `repo-packet-builder` derive canonical repository
identity from the git remote or configured project metadata instead of the
checkout directory basename, while preserving the worktree path/revision as
execution context.

### F3. P3 - Inherited quality-gate script is too broad for bounded WP-16 evidence

Location: `adl/tools/test_demo_v0901_quality_gate.sh`

The inherited v0.90.1 quality-gate script starts a full cargo test run as part
of its proof path. That can be useful as a release-tail confidence check, but it
is too broad and slow to be a clean bounded proof for the v0.90.2 internal
review issue. WP-16 therefore relied on focused v0.90.2 proof commands and
treated the inherited quality-gate run as optional support evidence rather than
release-blocking proof.

Impact: low. The focused v0.90.2 proofs pass, and the release-tail can still
run fuller checks later. The risk is operator confusion about which command is
the bounded v0.90.2 review proof versus a broad inherited quality sweep.

Recommended action: add a bounded v0.90.2 review smoke/check wrapper, or split
the inherited quality gate into fast milestone proof and full release-tail
validation modes.

## Milestone Achievement

The milestone has delivered the main work it claims:

- Runtime v2 inheritance and compression audit from v0.90.1
- CSM run packet contract, invariant map, and violation artifact contract
- `proto-csm-01` boot and citizen admission evidence
- governed resource-pressure scheduling and Freedom Gate mediation
- invalid-action rejection before commit
- snapshot, rehydrate, and duplicate-safe wake continuity
- Observatory packet and operator report
- recovery eligibility and quarantine state evidence
- governed adversarial hook and hardening probes
- integrated first bounded CSM run demo
- D11 feature-proof coverage across every v0.90.2 feature claim
- WP-15 release-truth convergence before this internal review

The tracked docs are materially aligned with the closed implementation tranche.
The demo matrix and feature-proof coverage record describe D1-D11 as landed and
preserve the non-proving boundaries. The release checklist correctly left
WP-16 and later release-tail gates open before this review.

## Validation Performed

Passed during this review:

- `bash adl/tools/closeout_completed_issue_wave.sh --version v0.90.2 --report .adl/reports/closeout/closeout-wave-v0.90.2.md`
  was run before WP-16 execution from the main checkout and reported
  `PASS closeout_completed_issue_wave version=v0.90.2 normalized=32`.
- `bash adl/tools/pr.sh doctor 2260 --version v0.90.2 --json` passed before
  binding execution.
- `python3 <repo-packet-builder-skill>/scripts/build_repo_packet.py . --out .adl/reviews/v0.90.2/internal/codebuddy/repo-packet`
  generated the CodeBuddy-style repo packet used as review input.
- `bash adl/tools/test_skill_documentation_completeness.sh` passed.
- `bash adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh`
  passed.
- `bash adl/tools/test_demo_v0902_multi_agent_repo_review_proof.sh` passed.
- `bash adl/tools/test_demo_v0902_arxiv_writer_field_test.sh` passed.
- `bash adl/tools/test_demo_v0902_paper_sonata_expansion.sh` passed.
- `bash adl/tools/test_milestone_dashboard.sh` passed.
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture`
  passed with feature-proof library and CLI tests.
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture`
  passed with integrated CSM run contract, stability, path-leakage, and
  unsafe-state rejection tests.

Failed or not used as passing evidence:

- `bash adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh`
  failed on the stale shared-reference expectation described in F1.
- The inherited v0.90.1 quality-gate chain was started as support evidence, but
  it is not treated as WP-16 passing evidence because its full cargo-test sweep
  is broader than the bounded v0.90.2 internal review proof requirement.

Not run in this pass:

- full `cargo test --manifest-path adl/Cargo.toml`
- full `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- live provider calls

Rationale: WP-16 owns internal review signal and targeted proof review. Full
release-tail validation remains appropriate for WP-18/WP-20 if the release
owner wants a broader pre-ceremony sweep.

## WP-18 Remediation Queue

Recommended ordering:

1. Fix the multi-agent repo-review suite contract test or metadata reference
   model so the suite-level review proof passes again.
2. Fix `repo-packet-builder` repository naming for issue worktrees before
   third-party/customer-facing review packets rely on the generated manifest.
3. Decide whether v0.90.2 needs a bounded review-quality wrapper distinct from
   the inherited v0.90.1 full quality gate.

## Release-Tail Disposition

WP-16 is complete as an internal review after this record lands. v0.90.2 should
continue to `WP-17` external / third-party review, then use `WP-18` to
remediate or explicitly defer accepted findings before `WP-19` handoff and
`WP-20` release ceremony.

No P0 or P1 findings were identified in this pass. The current assessment is:
ready for external review, not yet release-ready.
