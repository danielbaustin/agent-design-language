# v0.90.2 Planning Package

## Status

Early planning lane. Do not promote into tracked milestone docs or open the
issue wave until v0.90.1 has produced the Runtime v2 foundation prototype and
the v0.90.2 promotion gate has reviewed this package.

## Thesis

v0.90.2 is the first bounded CSM run milestone.

v0.90.1 should build the Runtime v2 substrate and the first visibility window.
v0.90.2 should make a small governed world actually run: boot a manifold, admit
citizens, schedule governed episodes, reject one invalid action, snapshot,
rehydrate, wake, and emit Observatory-visible evidence.

The hardening work still matters, but it should wrap the first CSM run rather
than replace it. The milestone should prove both sunny-day execution and
legible failure behavior.

## Directory Shape

- root planning docs and WP YAML live in this directory
- feature contracts live under `features/`
- context and later-band backgrounders live under `ideas/`

## Scope Boundary

In scope:

- first bounded CSM run for `proto-csm-01`
- manifold boot, citizen admission, governed episode execution, resource
  scheduling, local snapshot, local rehydrate, and wake continuity
- Observatory-visible packet, operator report, and proof surfaces
- invariant expansion across normal, failure, recovery, and quarantine paths
- stable violation artifacts suitable for review, demos, and release evidence
- recovery and quarantine semantics
- stronger operator review surfaces for failure and recovery paths
- security-boundary evidence that defends the polis without redefining Runtime v2
- release evidence that makes the hardening proof easy to audit

Out of scope:

- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, wellbeing, cultivation, or
  harm-prevention substrate
- v0.92 identity/capability rebinding, migration semantics, or birth record
- complete red/blue/purple adversarial ecology
- business-product execution for CodeBuddy or capability testing

## Canonical Planning Docs

- Vision: `VISION_v0.90.2.md`
- Design: `DESIGN_v0.90.2.md`
- Work Breakdown Structure: `WBS_v0.90.2.md`
- Sprint plan: `SPRINT_v0.90.2.md`
- Decisions log: `DECISIONS_v0.90.2.md`
- Demo matrix: `DEMO_MATRIX_v0.90.2.md`
- Feature index: `FEATURE_DOCS_v0.90.2.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.90.2.md`
- Release plan: `RELEASE_PLAN_v0.90.2.md`
- Release notes draft: `RELEASE_NOTES_v0.90.2.md`
- Issue wave draft: `WP_ISSUE_WAVE_v0.90.2.yaml`

## Promotion Rule

This package is a managed local planning lane. A later promotion issue should
review it against the actual v0.90.1 outcome before copying anything into
tracked milestone docs or creating v0.90.2 GitHub issues.

## Compression Rule

v0.90.2 should inherit the v0.90.1 compression model:

- issue wave and task cards should be generated from the reviewed planning
  package
- work starts in issue worktrees only
- docs-only and fixture-only issues use focused local validation plus CI gating
- runtime, schema, security, and release issues use fuller validation
- every SOR records the validation profile used and the exact proof surfaces

Compression is allowed only when it makes evidence easier to produce and review.
It is not permission to skip demos, tests, or release truth.
