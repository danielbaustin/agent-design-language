# Release Readiness - v0.91

## Status

Ceremony-ready release-readiness summary for `v0.91`.

This document is the compact reviewer-entry and release-summary surface for the
`v0.91` moral-governance and cognitive-being milestone. The release is not
complete until the final ceremony script, tag, and release disposition are
verified.

## Review Entry Points

- `CHANGELOG.md`
- `README.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.91/README.md`
- `docs/milestones/v0.91/WBS_v0.91.md`
- `docs/milestones/v0.91/SPRINT_v0.91.md`
- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `docs/milestones/v0.91/QUALITY_GATE_v0.91.md`
- `docs/milestones/v0.91/MILESTONE_CHECKLIST_v0.91.md`
- `docs/milestones/v0.91/RELEASE_PLAN_v0.91.md`
- `docs/milestones/v0.91/RELEASE_NOTES_v0.91.md`
- `docs/milestones/v0.91/RELEASE_EVIDENCE_v0.91.md`
- `docs/milestones/v0.91/END_OF_MILESTONE_REPORT_v0.91.md`
- `docs/milestones/v0.91/NEXT_MILESTONE_HANDOFF_v0.91.md`
- `docs/milestones/v0.91/WP_EXECUTION_READINESS_v0.91.md`
- `docs/milestones/v0.91/WP_ISSUE_WAVE_v0.91.yaml`

## Current Issue State

- WP-01 through WP-24 are closed.
- WP-17 / #2751 landed the cognitive-being flagship demo.
- WP-18 / #2752 landed the demo matrix and feature-proof coverage record.
- WP-19 / #2753 recorded the canonical quality and coverage gate.
- WP-20 / #2754 aligned docs, README, changelog, feature list, Cargo metadata,
  milestone docs, and ADRs for review.
- WP-21 / #2755 recorded internal review.
- WP-22 / #2756 recorded the zero-finding external review result.
- WP-23 / #2757 completed accepted-finding remediation and closeout.
- WP-24 / #2758 completed the v0.91.1 and v0.91.2 next-milestone handoff.
- WP-25 / #2759 remains active for final release ceremony.

## Landed Proof Surface

`v0.91` now has reviewable evidence for:

- Freedom Gate moral event records and moral-event validation
- moral trace examples for ordinary, refusal, delegation, and deferred paths
- outcome linkage and attribution with uncertainty and delegation lineage
- moral metrics as trace evidence rather than verdicts
- trajectory review and anti-harm constraints
- private wellbeing diagnostics with redacted operator/reviewer/public views
- kindness, humor/absurdity, affect reasoning-control, moral resources, and
  cultivated-intelligence surfaces
- durable `SPP` and `SRP` workflow artifacts
- secure local Agent Comms and explicit A2A/external boundary planning
- cognitive-being flagship demo and multi-agent conversation proof surfaces
- demo matrix and feature-proof coverage across the milestone feature set

## Primary Commands

Generate the v0.91 cognitive-being flagship proof packet:

```sh
cargo run --manifest-path adl/Cargo.toml -- runtime-v2 cognitive-being-flagship-demo --out artifacts/v091/cognitive-being-flagship
```

Inspect the v0.91 multi-agent triad workflow plan:

```sh
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-91-chatgpt-gemini-claude-triad-conversation.adl.yaml --print-plan
```

Run the safe ceremony preflight:

```sh
bash adl/tools/release_ceremony.sh --version v0.91
```

## Quality Evidence

- Main CI run `25514295183` after `WP-18`: `adl-ci` success and
  `adl-coverage` success.
- Coverage run evidence: 1813 tests run, 1813 passed, 2 skipped, 90.37%
  workspace line coverage, and per-file coverage gate passing at the 80%
  threshold.
- Closed-issue SOR truth validator: PASS for the closed `v0.91` issue set.
- Internal review: complete.
- External review: complete with `A+` / `100/100` and zero findings.
- Accepted-finding remediation: complete or explicitly dispositioned.

## Version Truth

- Active milestone during this release line: v0.91
- Crate version: `0.91.0`
- Most recently completed milestone before ceremony: v0.90.5
- Next prepared milestone packages: v0.91.1 and v0.91.2
- Current release-tail stage: WP-25 release ceremony pending

## Explicit Non-Claims

v0.91 does not claim:

- production moral agency
- legal personhood
- consciousness or subjective feeling
- complete constitutional authority
- the first true Gödel-agent birthday
- durable identity architecture
- scalar karma, scalar happiness, or final moral judgment
- public wellbeing diagnostics or public reputation derived from private
  wellbeing state
- external or cross-polis agent communication without TLS/mTLS-equivalent
  protection
- full v0.91.1 inhabited-runtime readiness
- full v0.91.2 tooling, evaluation, productization, publication, or workflow
  hardening

## Ceremony Disposition

The release tail is ready for final ceremony:

- WP-20 aligned release-truth and reviewer-entry surfaces
- WP-21 recorded internal review
- WP-22 recorded the zero-finding external review result
- WP-23 recorded accepted-finding remediation/disposition
- WP-24 recorded the next-milestone handoff
- WP-25 must still run the final ceremony and record tag/release disposition
