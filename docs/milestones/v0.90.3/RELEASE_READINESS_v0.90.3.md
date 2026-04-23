# Release Readiness - v0.90.3

## Status

WP-19 handoff record, ready for WP-20 ceremony.

This document is the reviewer entry surface for the v0.90.3 citizen-state
substrate milestone after the internal review, third-party review, and accepted
finding disposition pass. It records the current quality posture, review
entrypoints, proof evidence, and the final handoff and ceremony gates.

Important boundary: WP-14A / #2341 has landed the final feature-proof coverage
record. WP-15 aligned docs and quality truth, WP-16 and WP-17 closed the review
passes, and WP-18 closed by zero-finding disposition plus the small internal
stdout-path hygiene cleanup issue #2415. None of those steps approve the
release by themselves; WP-19 and WP-20 still own final handoff and ceremony.

## Review Entry Points

- `README.md`
- `CHANGELOG.md`
- `REVIEW.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.90.3/README.md`
- `docs/milestones/v0.90.3/WBS_v0.90.3.md`
- `docs/milestones/v0.90.3/SPRINT_v0.90.3.md`
- `docs/milestones/v0.90.3/DEMO_MATRIX_v0.90.3.md`
- `docs/milestones/v0.90.3/FEATURE_PROOF_COVERAGE_v0.90.3.md`
- `docs/milestones/v0.90.3/FEATURE_DOCS_v0.90.3.md`
- `docs/milestones/v0.90.3/MILESTONE_CHECKLIST_v0.90.3.md`
- `docs/milestones/v0.90.3/RELEASE_PLAN_v0.90.3.md`
- `docs/milestones/v0.90.3/RELEASE_READINESS_v0.90.3.md`
- `docs/milestones/v0.90.3/RELEASE_NOTES_v0.90.3.md`
- `docs/milestones/v0.90.3/WP_EXECUTION_READINESS_v0.90.3.md`
- `docs/milestones/v0.90.3/WP_ISSUE_WAVE_v0.90.3.yaml`
- `docs/milestones/v0.90.3/CI_RUNTIME_POLICY_v0.90.3.md`
- `docs/adr/0013-runtime-v2-citizen-state-continuity-substrate.md`

## Current Issue State

At WP-19 refresh:

- WP-01 through WP-14A have landed.
- WP-14A / #2341 landed the explicit demo-matrix and feature-proof coverage
  record.
- WP-15 / #2342 records quality, documentation, and reviewer-entry convergence.
- WP-16 / #2343 is closed. Internal review found no P0, P1, or P2 findings and
  left only small P3 polish notes.
- WP-17 / #2344 is closed. Third-party review reported no P0, P1, or P2
  findings and no external remediation bundle was required.
- WP-18 / #2345 is closed by truthful zero-finding disposition plus completion
  of the small internal stdout-path hygiene follow-up #2415.
- WP-19 / #2346 completed the next-milestone planning and handoff pass.
- WP-20 / #2347 remains the final release ceremony.

## Landed Proof Surface

v0.90.3 now has reviewable evidence for:

- v0.90.2 inheritance and unsafe-assumption audit
- canonical private-state format and JSON projection non-authority
- signed private-state envelope and local trust-root fixture
- local sealed quintessence checkpoint boundary
- append-only lineage ledger and materialized-head authority
- continuity witnesses and citizen-facing receipts
- anti-equivocation conflicting-successor negative case
- sanctuary/quarantine behavior for ambiguous wake
- redacted Observatory projections and leakage/authority checks
- citizen, guest, service actor, standing, and communication boundaries
- access-control matrix, access event packet, and denial fixtures
- challenge, freeze, appeal/review, threat-model, and economics-placement
  artifacts
- inhabited CSM Observatory flagship demo runbook and proof command
- feature-proof coverage record and runtime packet for D1-D14
- multimode Observatory UI architecture and design artifacts

## Primary Commands

Run the inhabited v0.90.3 Observatory flagship demo:

```sh
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship
```

Generate the v0.90.3 feature-proof coverage packet:

```sh
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0903/feature-proof-coverage.json
```

Run the focused v0.90.3 citizen-state proof tests:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sealing -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_anti_equivocation -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sanctuary -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture
```

The command set above is the focused proof surface for reviewers. It does not
replace full release-tail validation, full workspace tests, clippy, coverage
gates, or the release ceremony.

## WP-15 Validation Evidence

This convergence pass used focused documentation and release-truth validation:

- version/status scan across README, changelog, review guide, Cargo metadata,
  v0.90.3 milestone docs, and the ADL feature list
- local-path scan across touched tracked docs
- stale-claim scan for v0.90.3 overclaims about birthday, v0.91 moral scope,
  v0.92 migration/birthday scope, full economics, and cloud-enclave dependence
- feature-list currency scan against v0.90.3 and v0.90.5 milestone docs
- quality and coverage tracker scan before the final review tail continues
- git diff whitespace check

## Tracker Review

- Coverage tracker: current tracked release surfaces still carry the active
  coverage truth from the recent quality gate, with workspace line coverage at
  `92.40%`, rounded to the intended `93%` tranche, and no active file-floor
  exclusion documented. WP-15 did not rerun full coverage because this PR is a
  docs/release-truth convergence pass; release evidence must still come from a
  full coverage lane, a runtime PR, a main push, or ceremony validation.
- CI runtime policy: #2392, #2394, #2406, and #2409 have landed the docs-heavy
  PR path policy, skill integration, coverage-impact preflight, and duplicate
  full-Rust-test reduction. Green `adl-ci` and `adl-coverage` checks on
  docs-only PRs can be healthy PR evidence, but they are not full release
  coverage evidence.
- Gap status: the active gap risk before ceremony is no longer review-tail
  findings. The remaining work is handoff truth and ceremony sequencing from
  clean main.
- Rust module watch: no new Rust refactoring scope is introduced by WP-15.
  Runtime/source changes should still use the coverage-impact preflight before
  PR publication.

## Version Truth

- Active milestone: v0.90.3
- Crate version: `0.90.3`
- Most recently completed milestone: v0.90.2
- Current release-tail stage: WP-20 ceremony after clean internal/external
  review closure and completed next-milestone handoff

Reviewers should treat any conflicting older crate-version statement or claim
that v0.90.3 is still pre-issue-wave as stale release-truth drift.

## Explicit Non-Claims

v0.90.3 does not claim:

- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, or wellbeing substrate
- full v0.92 identity/capability rebinding, migration, or birthday record
- full citizen economics, contract markets, payment rails, or inter-polis trade
- mandatory cloud enclave deployment
- unrestricted operator inspection of private citizen state
- production UI readiness for the Observatory

## Remaining Release-Tail Gates

- WP-20 release ceremony must run from clean main after all closeout PRs merge.

## WP-15 Disposition

WP-15 aligns release-truth surfaces and gives reviewers a stable entry point. It
does not approve the release, replace internal or third-party review, or declare
the milestone complete.
