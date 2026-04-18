# Early Planning Lane Handoff - v0.90.1

## Status

Issue #2090 opened the v0.90.1 planning lane.

The active planning package lives in local workspace state at:

- `.adl/docs/v0.90.1planning/`

Do not treat this tracked handoff as the full milestone package. The full
package should be promoted later, after v0.90 release-tail planning review.

## Package Shape

The planning lane mirrors the v0.90 package shape:

- root planning docs and WP issue-wave draft in the package root
- implementation-facing feature contracts under `features/`
- context, roadmap, birthday-boundary, and source Runtime v2 drafts under
  `ideas/`

## Planning Surfaces Created

- `README.md`
- `VISION_v0.90.1.md`
- `DESIGN_v0.90.1.md`
- `WBS_v0.90.1.md`
- `SPRINT_v0.90.1.md`
- `DECISIONS_v0.90.1.md`
- `FEATURE_DOCS_v0.90.1.md`
- `DEMO_MATRIX_v0.90.1.md`
- `MILESTONE_CHECKLIST_v0.90.1.md`
- `RELEASE_PLAN_v0.90.1.md`
- `RELEASE_NOTES_v0.90.1.md`
- `WP_ISSUE_WAVE_v0.90.1.yaml`

## Feature Contracts Created

- `features/RUNTIME_V2_FOUNDATION_PROTOTYPE.md`
- `features/MANIFOLD_AND_SNAPSHOT_CONTRACT.md`
- `features/KERNEL_SERVICES_AND_CONTROL_PLANE.md`
- `features/PROVISIONAL_CITIZEN_LIFECYCLE.md`
- `features/INVARIANT_AND_SECURITY_BOUNDARY.md`

## Boundary Captured

v0.90.1 is scoped to the Runtime v2 foundation prototype:

- manifold
- kernel service loop
- provisional citizens
- snapshot and rehydration
- invariant violation artifacts
- operator controls
- one security-boundary proof

It does not claim:

- first true Gödel-agent birthday
- full moral/emotional civilization
- v0.92 identity/capability rebinding
- complete cross-polis migration
- full red/blue/purple security ecology

## Promotion Rule

Promote the local planning package into this tracked milestone directory only
after v0.90 reaches its release-tail planning gate and the package is reviewed
against final v0.90 truth.
