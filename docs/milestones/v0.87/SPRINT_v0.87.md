# Sprint Plan: v0.87

## Metadata
- Sprint: `v0.87-s1`
- Milestone: `v0.87`
- Start date: `2026`
- End date: `TBD`
- Owner: `adl`

## Sprint Goal
Establish the first executable slice of `v0.87` by locking the canonical milestone docs and beginning the substrate work that everything else depends on: trace v1, provider/transport substrate planning, and control-plane/tooling stabilization.

This sprint should leave `v0.87` with:
- canonical docs that match the roadmap
- a clear first work sequence for trace, provider, and operational substrate work
- at least one real implementation slice underway or completed in the foundational substrate band

## Planned Scope
- Canonicalize the milestone doc set for `v0.87` (vision, design, WBS, sprint, checklist, demo matrix, release plan, release notes, decisions, feature-doc index)
- Begin the trace/provider/control-plane substrate sequence with issue-ready planning and the first implementation slice
- Keep scope tightly bounded to foundational substrate work; do not pull `v0.88+` systems into this sprint

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Lock the canonical milestone docs and issue map | `#1292` | `Daniel / Codex.app` | `in progress` |
| 2 | Seed the Sprint 1 substrate tranche: trace schema, provider substrate, shared-memory foundation | `#1293`, `#1295`, `#1297` | `Daniel / Codex.app` | `bootstrapped` |
| 3 | Seed the Sprint 2 tranche: trace linkage, portability, memory coherence, skills, control-plane, review | `#1294`, `#1296`, `#1298`, `#1299`, `#1300`, `#1301`, `#1302` | `Daniel / Codex.app` | `bootstrapped` |
| 4 | Begin the first foundational implementation slice from the seeded tranche | `#1293` or `#1300` | `Daniel / Codex.app` | `ready` |

## Cadence Expectations
- Use issue cards (`input` / `output`) for every execution item.
- Keep each issue mergeable, narrow, and truthfully documented.
- Prefer substrate-first sequencing: trace → provider → shared memory → skills/tooling.
- Use draft PRs until checks pass and proof surfaces are reviewable.
- Run required quality gates (`fmt`, `clippy`, `test`, and any validator/demo command relevant to the changed substrate surface).

## Risks / Dependencies
- Dependency: `v0.86` Sprint 7 closeout must finish cleanly enough that `v0.87` docs are not immediately invalidated.
  - Risk: late `v0.86` fixes may force doc or roadmap churn.
  - Mitigation: keep `v0.87` sprint-1 scope on foundational substrate work and update canonical docs only after closeout truth is stable.

- Dependency: provider/transport redesign touches core architectural surfaces.
  - Risk: vague scope or over-expansion into later capability-routing work.
  - Mitigation: keep sprint scope at provider substrate v1 only: vendor/transport/model separation, `model_ref`, compatibility path, and issue decomposition.

- Dependency: tooling/control-plane work can sprawl.
  - Risk: PR/worktree fixes turn into a broad rewrite.
  - Mitigation: prefer bounded consolidation slices that reduce shell ownership and improve determinism without redesigning the whole workflow layer at once.

## Demo / Review Plan
- Demo artifact: `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md` plus at least one first substrate proof surface (likely trace or tooling-oriented)
- Review date: `TBD`
- Sign-off owners: `Daniel Austin`, `Codex.app`, internal review before wider exposure

## Exit Criteria
- Canonical `v0.87` milestone docs are filled, internally consistent, and aligned with the roadmap.
- The first `v0.87` issue sequence is explicit for trace, provider, shared memory, and operational/control-plane work.
- At least one foundational substrate implementation slice is underway or completed with traceable issue/PR surfaces.
- Scope remains bounded to `v0.87` substrate work; no silent pull-forward of `v0.88+` systems.
- Sprint summary and any deferrals are captured truthfully in milestone docs.
