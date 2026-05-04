# Release Readiness - v0.90.5

## Status

WP-21 reviewer entry surface, ready for WP-22 internal review.

This document is the reviewer entry surface for the v0.90.5 Governed Tools
v1.0 milestone after the feature-proof coverage and quality-gate passes. It
records the current docs, quality, proof, review-entry, and non-claim posture
before the formal internal and third-party review steps.

Important boundary: WP-19 / #2584 landed the explicit feature-proof coverage
record and demo matrix, and WP-20 / #2585 landed the canonical quality gate.
WP-21 aligns the release-truth and reviewer-entry docs. None of those steps
approve the release by themselves; WP-22 through WP-26 still own review,
remediation, handoff, and ceremony.

## Review Entry Points

- `README.md`
- `CHANGELOG.md`
- `REVIEW.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/WBS_v0.90.5.md`
- `docs/milestones/v0.90.5/SPRINT_v0.90.5.md`
- `docs/milestones/v0.90.5/DEMO_MATRIX_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_DOCS_v0.90.5.md`
- `docs/milestones/v0.90.5/FEATURE_PROOF_COVERAGE_v0.90.5.md`
- `docs/milestones/v0.90.5/QUALITY_GATE_v0.90.5.md`
- `docs/milestones/v0.90.5/MILESTONE_CHECKLIST_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_PLAN_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/RELEASE_NOTES_v0.90.5.md`
- `docs/milestones/v0.90.5/WP_EXECUTION_READINESS_v0.90.5.md`
- `docs/milestones/v0.90.5/WP_ISSUE_WAVE_v0.90.5.yaml`
- `docs/milestones/v0.90.5/GET_WELL_PLAN_v0.90.5.md`
- `docs/adr/0014-contract-market-architecture.md`

## Current Issue State

At WP-21 refresh:

- WP-01 through WP-20 are closed.
- WP-19 / #2584 landed the explicit demo-matrix and feature-proof coverage
  record.
- WP-20 / #2585 records the canonical quality and coverage gate.
- WP-21 / #2586 is the active docs + review pass.
- WP-22 / #2587 through WP-26 / #2591 remain the internal review,
  third-party review, remediation, next-milestone planning, and release
  ceremony tail.
- The only additional open v0.90.5 follow-on outside the canonical WP state
  machine at this moment is #2700, a bounded tooling fix for orphaned
  post-merge closeout watchers in PR-finish tests.

## Landed Proof Surface

v0.90.5 now has reviewable evidence for:

- tool-call threat model and governed capability non-goals
- Universal Tool Schema v1.0 public-compatible schema and conformance
- ADL Capability Contract v1.0 authority, privacy, visibility, delegation,
  trace, and replay semantics
- deterministic tool-registry, compiler, normalization, policy, and governed
  executor behavior
- trace, replay, redaction, and evidence constraints for governed tool actions
- dangerous negative safety tests that fail closed
- bounded model-proposal benchmarking and local/Gemma-focused evaluation
- the Governed Tools v1.0 flagship demo
- explicit feature-proof coverage and demo-matrix classification before review
  convergence
- the landed first-level ACIP / Comms tranche: general protocol architecture,
  canonical message envelope and identity shape, invocation and Freedom Gate
  linkage, conformance fixtures, review/coding specializations, and ACIP proof
  coverage

## Primary Commands

Generate the v0.90.5 feature-proof coverage packet:

```sh
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0905/feature-proof-coverage.json
```

Run the v0.90.5 flagship governed-tools demo:

```sh
cargo run --manifest-path adl/Cargo.toml -- demo demo-v0905-governed-tools-flagship --run --trace --out artifacts/v0905/flagship-demo --no-open
```

Run the bounded local-model PR reviewer fixture lane:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling code-review --out artifacts/v0905/local-model-pr-reviewer-fixture --backend fixture --visibility read-only-repo --issue 2603 --writer-session codex-writer --reviewer-session fixture-reviewer
```

The command set above is the focused proof surface for reviewers. It does not
replace full release-tail validation, full workspace tests, coverage
remeasurement, or the final ceremony.

## WP-21 Validation Evidence

This convergence pass should remain docs/truth focused:

- version/status scan across README, changelog, review guide, Cargo metadata,
  active milestone docs, and the reviewer-entry surfaces
- local-path scan across touched tracked docs
- stale-claim scan for overclaims about UTS public-standard status, execution
  authority, unrestricted tool execution, or later-milestone cognitive/identity
  work
- release-tail docs/readiness consistency scan against the current open issue
  set
- git diff whitespace check

## Tracker Review

- Quality gate: `QUALITY_GATE_v0.90.5.md` is the canonical gate. It records the
  green PR evidence that landed the milestone proof surfaces and the explicit
  red main-branch coverage posture that still remains a release-tail exception.
- Get-well status: the runtime-reduction wave is documented and remains
  explicit rather than being silently treated as solved.
- Review posture: the package is now ready for internal review entry, but WP-22
  has not yet issued a findings-first review result.
- Closeout posture: the remaining canonical milestone tail is review,
  remediation, planning handoff, and ceremony. There is no claim here that the
  release is already complete.

## Version Truth

- Active milestone: v0.90.5
- Crate version: `0.90.5`
- Most recently completed milestone: v0.90.4
- Current release-tail stage: WP-21 docs/review convergence before WP-22
  internal review

Reviewers should treat any conflicting older crate-version statement or claim
that v0.90.4 is still the active line as stale release-truth drift.

## Explicit Non-Claims

v0.90.5 does not claim:

- that UTS alone is enough for ADL safety
- that valid JSON or schema compatibility grants execution authority
- arbitrary shell, network, or secret-bearing execution by model output
- payment rails, billing, legal contracting, or inter-polis economics
- full v0.91 moral/cognitive-being substrate
- full v0.91.1 identity/capability, memory, ToM, ANRM/Gemma, or wider learning
  follow-on work
- release completion before WP-22 through WP-26 finish

## Remaining Release-Tail Gates

- WP-22 internal review
- WP-23 external / 3rd-party review
- WP-24 accepted-finding remediation or explicit deferral
- WP-25 next-milestone planning handoff
- WP-26 release ceremony

## WP-21 Disposition

WP-21 aligns the release-truth and reviewer-entry surfaces. It does not
approve the release, replace internal or third-party review, or declare the
milestone complete.
