# v0.90.1 Planning Package

## Status

Tracked next-milestone planning package promoted during the v0.90 WP-19
release-tail planning gate.

The package is ready for v0.90.1 issue-wave authoring after v0.90 release
ceremony approval. It is not evidence that v0.90.1 implementation has started.

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
- Release plan: `RELEASE_PLAN_v0.90.1.md`
- Release notes draft: `RELEASE_NOTES_v0.90.1.md`
- Issue wave plan: `WP_ISSUE_WAVE_v0.90.1.yaml`

## Feature And Idea Lanes

- `features/` contains implementation-facing contracts for the Runtime v2
  foundation prototype.
- `ideas/` contains context that should inform the milestone without being
  treated as v0.90.1 implementation scope.

## Issue-Wave Rule

After v0.90 release ceremony approval, v0.90.1 WP-01 should open the actual
GitHub issue wave and author the task cards from this tracked package.

Do not treat the tracked package itself as an opened issue wave. The wave is
not open until the WP-01 issue-creation step records real issue numbers and
updates the YAML.

## Compression Rule

WP-02 through WP-04 should run before Runtime v2 coding starts. They are the
enablement layer that should reduce issue-wave drift, unsafe checkout behavior,
and validation-policy ambiguity during the runtime work.
