# Release Readiness - v0.90.2

## Status

WP-20 release-closeout record.

v0.90.2 implementation and demo/proof coverage have landed through WP-14A.
WP-15 docs convergence, WP-16 internal review, WP-17 external review, and WP-18
accepted-finding remediation are complete. WP-19 next-milestone handoff and
WP-20 release ceremony preflight are complete. Final tag/release publication
runs from clean main after the closeout PR merges.

## Review Entry Points

- `README.md`
- `CHANGELOG.md`
- `REVIEW.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.90.2/README.md`
- `docs/milestones/v0.90.2/WBS_v0.90.2.md`
- `docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md`
- `docs/milestones/v0.90.2/FEATURE_DOCS_v0.90.2.md`
- `docs/milestones/v0.90.2/FEATURE_PROOF_COVERAGE_v0.90.2.md`
- `docs/milestones/v0.90.2/MILESTONE_CHECKLIST_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_PLAN_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_EVIDENCE_v0.90.2.md`
- `docs/milestones/v0.90.2/RELEASE_NOTES_v0.90.2.md`
- `docs/milestones/v0.90.2/WP_ISSUE_WAVE_v0.90.2.yaml`

## Landed Proof Surface

v0.90.2 now has reviewable proof coverage for:

- v0.90.1 inheritance and compression readiness
- CSM run packet contract
- invariant map and violation artifact contract
- manifold boot and citizen admission
- governed resource-pressure scheduling
- Freedom Gate mediation
- invalid-action rejection before commit
- snapshot, rehydrate, and wake continuity
- CSM Observatory packet and operator report
- recovery eligibility and quarantine state evidence
- governed adversarial hook and bounded hardening probes
- integrated first bounded CSM run demo
- feature-by-feature proof coverage across D1-D11

## Primary Commands

Run the bounded v0.90.2 review-quality gate for internal/external review proof:

```sh
bash adl/tools/demo_v0902_review_quality_gate.sh
```

Generate the feature-proof coverage packet:

```sh
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0902/feature-proof-coverage.json
```

Run the integrated first CSM run demo:

```sh
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run
```

Run the focused feature-proof test:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture
```

Run the integrated first-run test:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture
```

The v0.90.2 review-quality gate is intentionally bounded. It runs the focused
review/demo proof scripts, milestone dashboard smoke proof, and the Runtime v2
feature-proof and integrated-run tests used as internal/external review
evidence. It does not replace full release-tail validation, full workspace
`cargo test`, full clippy, coverage gates, or the release ceremony.

Use `bash adl/tools/test_demo_v0901_quality_gate.sh` only as broader
release-tail support evidence inherited from v0.90.1 when operators want a
larger confidence sweep.

## WP-15 Validation Evidence

The WP-15 convergence pass ran these focused checks:

- `cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture`
  passed with five focused feature-proof tests across library and CLI surfaces.
- `cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_integrated_run -- --nocapture`
  passed with four focused integrated-run tests.
- `cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0902/feature-proof-coverage.json`
  emitted the D11 feature-proof coverage packet.
- `cargo run --manifest-path adl/Cargo.toml -- runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run`
  emitted the D10 integrated first-run demo packet and printed the Observatory
  operator report.
- Local path scan of touched tracked docs found no absolute host-path leakage.
- Stale status scan found no remaining claims that v0.90.2 is pre-issue-wave or
  merely planning-only.
- `git diff --check` passed.

## Tracker Review

WP-15 also reviewed the active local quality and maintenance trackers before
handoff to WP-16:

- Coverage tracker: current workspace line coverage remains `92.40%`, which is
  approximately `93.00%` when rounded. The workspace coverage gate and per-file
  coverage gate both pass with no active file-floor exclusion.
- Gap status: the active v0.90.2 gap-analysis note lives in the local TBD
  surface and is intentionally not treated as a canonical tracked release
  document. WP-15 captures the current gap state directly from current issue
  and validation evidence. The v0.90.2 implementation tranche is substantial
  through WP-14A, and release-tail verification has now passed through WP-20
  preflight. Remaining publication work is the final tag/release operation from
  clean main after closeout merge.
- Rust module watch tracker: the current largest Rust hotspots are Runtime v2
  tests, `runtime_v2/governed_episode.rs`, and `long_lived_agent.rs`. The test
  split and long-lived-agent continuation landed in the refactor lane; issue
  #2309 remains open for the governed-episode split and should be reflected in
  the Rust tracker after it merges.

## Version Truth

- Active milestone: v0.90.2
- Crate version: `0.90.2`
- Most recently completed milestone: v0.90.1
- Current release-tail stage: WP-20 preflight complete; final publication after closeout merge

Reviewers should treat any conflicting older v0.90.1 crate-version statement
or claim that v0.90.2 is still pre-issue-wave as a stale release-truth defect.

## Explicit Non-Claims

v0.90.2 does not claim:

- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, or wellbeing substrate
- full v0.92 identity/capability rebinding
- cross-polis migration
- complete red/blue/purple security ecology
- live unbounded Runtime v2 autonomy
- Observatory mutation authority outside governed command packets

## Review-Tail Disposition

- WP-16 internal review completed and its accepted findings were fixed by
  #2317, #2318, #2319, and #2320.
- WP-17 external / third-party review completed with zero P0/P1/P2/P3 findings.
- WP-18 accepted-finding remediation completed; optional non-blocking ideas
  were routed to backlog candidates.

## Remaining Release-Tail Gate

- Final tag/release publication must run from clean main after this closeout PR
  merges and Daniel fast-forwards main.

## WP-15 Disposition

WP-15 aligns release-truth surfaces and gives reviewers a stable entry point.
It does not approve the release, replace review, or declare the milestone
complete.
