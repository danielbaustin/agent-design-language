# Release Readiness - v0.90.2

## Status

WP-15 docs, quality, and review convergence record.

v0.90.2 implementation and demo/proof coverage have landed through WP-14A. The
milestone is ready for WP-16 internal review after this convergence record is
reviewed and merged.

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

## Version Truth

- Active milestone: v0.90.2
- Crate version: `0.90.2`
- Most recently completed milestone: v0.90.1
- Current release-tail stage: WP-15 complete after this record merges

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

## Remaining Release-Tail Gates

- WP-16 internal review must still run.
- WP-17 external / third-party review must still run.
- WP-18 must fix accepted findings or defer them explicitly.
- WP-19 must complete next-milestone planning and v0.91/v0.92 handoff.
- WP-20 must run the release ceremony from clean main after merge and operator
  fast-forward.

## WP-15 Disposition

WP-15 aligns release-truth surfaces and gives reviewers a stable entry point.
It does not approve the release, replace review, or declare the milestone
complete.
