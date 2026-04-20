# v0.90.1 Planning Package

## Status

Active milestone package. The issue wave is open, WP-01 through WP-19 are
complete or represented by tracked release-tail docs, and WP-20 release ceremony
remains.

WP-01 opened the v0.90.1 issue wave after the v0.90 release ceremony. WP-01 is
#2141; WP-02 through WP-20 are #2142 through #2160. WP-15A third-party review is
#2215 and sits between internal review and remediation.

The Runtime v2 implementation slice has now landed through WP-12, WP-13 aligned
the docs package, WP-14 defined the quality/coverage gate, WP-15 completed
internal review, WP-15A completed third-party review, the accepted WP-16
remediation bundles are closed, and WP-17 through WP-19 release-tail docs are
assembled. Remaining work is WP-20 release ceremony.

## Thesis

v0.90.1 should turn the v0.90 long-lived-agent runtime work into the first
bounded Runtime v2 foundation prototype.

The milestone is not the birth of the first true Gödel agent. It is the
substrate proof that makes that later birth credible: kernel services,
provisional citizen records, snapshots, manifold links, invariant violation
artifacts, and operator controls that reviewers can inspect.

## Directory Shape

This package mirrors the v0.90 planning package:

- root planning docs and WP YAML live in this directory
- feature contracts live under `features/`
- context and later-band backgrounders live under `ideas/`

## Scope Boundary

In scope:

- compression-enablement work that makes v0.90.1 and v0.90.2 faster and safer:
  issue-wave template alignment, worktree-first hardening, and current execution
  policy
- bounded Runtime v2 kernel/service loop
- provisional citizen record and lifecycle state machine
- manifold and snapshot contract
- invariant violation artifacts
- operator inspect, pause, resume, terminate, and wake controls
- one security-boundary proof that protects the polis without making red/blue
  ecology the core runtime thesis
- migration-ready shape without claiming full migration semantics

Out of scope:

- first true Gödel-agent birthday
- full moral/emotional civilization
- v0.92 identity/capability rebinding
- complete cross-polis migration
- full red/blue/purple security ecology
- business-product execution for CodeBuddy or capability testing

## Canonical Planning Docs

- Vision: `VISION_v0.90.1.md`
- Design: `DESIGN_v0.90.1.md`
- Work Breakdown Structure: `WBS_v0.90.1.md`
- Sprint plan: `SPRINT_v0.90.1.md`
- Decisions log: `DECISIONS_v0.90.1.md`
- Demo matrix: `DEMO_MATRIX_v0.90.1.md`
- Feature index: `FEATURE_DOCS_v0.90.1.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.90.1.md`
- Quality gate: `QUALITY_GATE_v0.90.1.md`
- Internal review: `INTERNAL_REVIEW_v0.90.1.md`
- Third-party review disposition: `THIRD_PARTY_REVIEW_v0.90.1.md`
- Release readiness: `RELEASE_READINESS_v0.90.1.md`
- v0.91/v0.92 handoff: `V091_V092_HANDOFF_v0.90.1.md`
- Release evidence packet: `RELEASE_EVIDENCE_v0.90.1.md`
- Release plan: `RELEASE_PLAN_v0.90.1.md`
- Release notes draft: `RELEASE_NOTES_v0.90.1.md`
- Issue wave plan: `WP_ISSUE_WAVE_v0.90.1.yaml`
- Issue-wave generator proof: `ISSUE_WAVE_GENERATOR_PROOF_v0.90.1.md`
- Compression-era execution policy:
  `features/COMPRESSION_ERA_EXECUTION_POLICY.md`

## Feature And Idea Lanes

- `features/` contains implementation-facing contracts for the Runtime v2
  foundation prototype.
- `ideas/` contains context that should inform the milestone without being
  treated as v0.90.1 implementation scope.

## Issue-Wave Rule

WP-01 opened the actual GitHub issue wave and authored the local task cards
from this tracked package.

The wave is now open. Treat WP_ISSUE_WAVE_v0.90.1.yaml as the issue-number
source of truth for WP routing.

WP-02 adds `ISSUE_WAVE_GENERATOR_PROOF_v0.90.1.md` as the mechanical proof that
the reusable generator can read this package shape without hand repair.

WP-04 adds `features/COMPRESSION_ERA_EXECUTION_POLICY.md` as the policy note for
skill routing, worktree-first execution, subagent assignment, validation
profiles, and SOR evidence expectations.

## Compression Rule

WP-02 through WP-04 should run before Runtime v2 coding starts. They are the
enablement layer that should reduce issue-wave drift, unsafe checkout behavior,
and validation-policy ambiguity during the runtime work.
