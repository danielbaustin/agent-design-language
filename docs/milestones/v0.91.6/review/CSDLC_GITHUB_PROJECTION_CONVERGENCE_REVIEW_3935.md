# C-SDLC GitHub Projection Convergence Review

## Metadata

- Issue: `#3935`
- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-17`
- Status: draft for review
- Audience: ADL maintainers and workflow-tooling reviewers

## Purpose

Define how ADL should converge GitHub issue/PR surfaces on the canonical
C-SDLC cards so publication and closeout truth remain templated, repairable,
and reviewable instead of drifting through partially-managed duplicate text.

## Source Evidence

Current-state facts in this review are grounded in:

- `adl/src/cli/pr_cmd/finish_support.rs`
- `adl/src/cli/pr_cmd/github.rs`
- `docs/cognitive-sdlc/card-lifecycle.md`
- `docs/architecture/ADL_ARCHITECTURE.md`
- `docs/templates/CARD_LIFECYCLE_TEMPLATE_TARGETS.md`
- `docs/milestones/v0.91.6/features/TOOLING_PROOF_LOOP_RELIABILITY_v0.91.6.md`

Recommendations in this review are design proposals for `#3935`, not claims
that the current tooling already behaves this way.

## Current Observed State

### Facts

- `pr finish` already renders the PR body from selected `SOR` sections rather
  than from the issue prompt or free-form operator text.
- The current PR-body projection includes:
  - closing line when closeout is enabled
  - `Summary`
  - `Artifacts produced`
  - `Validation`
  - optional notes
  - local artifact references and an idempotency key
- The `SOR` remains the card responsible for outcome truth: changed paths,
  validation actually run, review actually performed, PR/merge state, closeout
  state, unresolved follow-ons, and final issue truth.
- The canonical card roles remain distinct:
  - `SIP`: issue/problem truth
  - `STP`: selected task/solution truth
  - `SPP`: execution-plan truth
  - `SRP`: review-result truth
  - `SOR`: outcome/publication/closeout truth

### Failure Mode Observed

- The live GitHub PR body can still drift after publication because GitHub
  remains an editable projection surface.
- When the live body drifts far enough, downstream guards such as PR closing
  linkage can fail even if the intended `SOR`-driven publication shape was
  correct earlier.

## Problem Statement

ADL currently has a partial convergence model:

- the cards are the intended C-SDLC authority
- some GitHub surfaces are rendered from those cards
- GitHub still allows independent edits that can become silently authoritative
  in practice

That split creates a control-plane truth gap. The repo can have:

- a truthful `SOR`
- a mostly-correct finish path
- a drifted live PR body
- a later failing linkage/closeout/janitor path

The underlying issue is broader than PR-body text. ADL needs a policy for which
GitHub surfaces are:

- authoritative card-owned projections
- drift-check-only mirrors
- intentionally manual operator surfaces
- card-local only and never projected outward

## Proposed Convergence Rule

Recommendation:

- C-SDLC cards remain the authority.
- GitHub issue/PR surfaces become deterministic managed projections where ADL
  has claimed ownership.
- Drift must be detected explicitly and repaired from the owning card instead
  of normalized by ad hoc manual edits.

This implies:

1. GitHub should not be the source of truth for lifecycle state.
2. GitHub projections should be templated from card-owned fields.
3. Drift repair should flow from cards to GitHub, not from GitHub back into the
   cards, except for deliberate bootstrap/import paths.

## First Concrete Slice

`#3935` should start with the smallest high-value convergence point:

- make the issue `SOR` the explicit authority for PR publication body truth

That slice should define:

- the templated PR-body section set
- the closing-keyword policy
- the drift detection point
- the repair owner in `finish`, `doctor`, `janitor`, and `closeout`

Recommended PR-body projection contract:

- `Closes #<issue>` or an explicit non-closing declaration
- `## Summary`
- `## Artifacts`
- `## Validation`
- `## Notes` when populated
- durable local artifact references
- publication fingerprint / idempotency key

## Recommended First-Tranche Projection Ownership

To keep the first implementation slice bounded, the initial `v0.91.6`
recommendation should classify these surfaces now:

| Surface | Owning card | First-tranche class | Expected first-tranche behavior |
| --- | --- | --- | --- |
| PR body | `SOR` | `managed_projection` | Render from `SOR`, detect drift, and repair from `SOR` automatically or fail closed when repair is required. |
| PR closing linkage line | `SOR` | `managed_projection` | Treat `Closes #<issue>` or explicit non-closing state as part of the rendered `SOR` projection rather than a side-band edit. |
| Closeout / merge summary comment | `SOR` | `drift_checked_projection` | Define the target shape now, but allow review before making automatic rewrite mandatory. |
| GitHub issue body after bootstrap | `SIP` | `drift_checked_projection` | Classify drift against `SIP` truth, but defer auto-repair policy until bootstrap/import and operator-edit rules are settled. |
| PR title | `STP` | `drift_checked_projection` | Decide the deterministic mapping before promoting it to managed projection. |
| Review findings comments / summary comment | `SRP` | `linked_surface_only` | Prefer linking or summarizing `SRP`/review packets rather than mirroring full review truth into GitHub prose. |
| Labels / milestone / queue metadata | mixed control-plane ownership | `drift_checked_projection` | Classify field ownership first; do not auto-repair until that boundary is explicit. |
| `SPP` execution plan | `SPP` | `card_local_only` | Keep execution-plan truth in the card; do not force a GitHub projection in the first tranche. |

## Generalized Card-To-GitHub Projection Model

Longer-term target recommendation after the first-tranche boundary is approved:

| Card | Canonical responsibility | GitHub-facing projection candidate | Policy |
| --- | --- | --- | --- |
| `SIP` | issue/problem truth, scope, acceptance, dependencies | authored issue body and selected issue metadata | drift-checked first; may later promote selected surfaces to managed projection once bootstrap/import and operator-edit rules are settled |
| `STP` | chosen task/solution, touched surfaces, invariants | PR title and possibly bounded task-summary fragments | drift-checked first; may later promote deterministic fields to managed projection |
| `SPP` | execution sequence, stop conditions, validation/replan plan | usually no durable GitHub text projection; maybe bounded checklists/comments later | card-local by default |
| `SRP` | review instructions, findings, dispositions, residual risk | review summaries, routed findings comments, or linked review packets | linked-surface-first; do not mirror full review truth into GitHub prose |
| `SOR` | outcome truth, validation run, integration/closeout state | PR body, closeout comment, merge/closure status assertions | managed projection for PR body and closing linkage first; drift-checked or linked-only for other GitHub-facing outcome surfaces until explicitly promoted |

## Projection Classes

To keep the model reviewable, each GitHub-facing surface should be assigned one
of four classes:

- `managed_projection`: GitHub content is rendered from a card and may be
  auto-repaired
- `drift_checked_projection`: GitHub content is compared against card truth but
  may require explicit operator confirmation before repair
- `linked_surface_only`: the card links to GitHub state or a packet, but does
  not mirror the content
- `card_local_only`: no GitHub projection is attempted

The first-tranche table above is the operative scope for `#3935`. The
generalized card table is the later target model and must not be read as
promoting additional surfaces into managed projection during Slice 1.

## Why This Should Not Collapse The Cards

This proposal is not to flatten C-SDLC into GitHub text.

The distinct cards still matter because:

- `SIP` expresses problem truth that should outlive PR wording
- `STP` captures the chosen implementation path, not just reviewer-facing prose
- `SPP` is an execution-planning surface and should not be reduced to a PR
  checklist by default
- `SRP` records review truth that should not be swallowed by `SOR`
- `SOR` summarizes outcome/publication truth but should not absorb all planning
  or review details

The goal is projection alignment, not lifecycle collapse.

## Suggested Implementation Slices

### Slice 1

`SOR` owns PR body truth.

- render a canonical PR-body block from `SOR`
- store a publication fingerprint
- detect and repair body drift from `SOR`
- fail closed when a required closing line disappears
- keep first-tranche managed projection scope limited to the PR body and its
  closing-linkage line

### Slice 2

Define the `SIP` to GitHub issue-body ownership boundary.

- distinguish bootstrap/import from ongoing authority
- define which issue metadata fields are card-owned
- specify when GitHub issue edits must be reflected from `SIP`

### Slice 3

Define `SRP` and review-surface projection boundaries.

- keep review findings in `SRP`
- decide whether GitHub comments are summary projections, linked packets, or
  non-authoritative operator aids

### Slice 4

Classify all remaining GitHub-facing surfaces.

- PR title
- labels
- milestone/queue metadata
- closeout comments
- linked review/evidence packets

## Validation Expectations

The first implementation slice should prove at minimum:

- PR body render is fully derived from the intended `SOR` fields
- live-body drift is detected deterministically
- repair rewrites the live PR body back to the card-owned projection
- closing-linkage guard can no longer fail after silent manual drift without
  first classifying or repairing the mismatch
- no issue-template/bootstrap text leaks into the rendered PR body

## Open Questions For Review

- Should PR title remain `STP`-derived, `SOR`-derived, or issue-prompt-derived?
- Which GitHub metadata fields should stay manually editable by operators?
- Should drift repair always be automatic for managed projections, or should
  some surfaces require explicit operator approval?
- How much of `SRP` should ever be projected into GitHub comments versus linked
  as a durable packet?
- Should `SIP` own GitHub issue-body updates after bootstrap, or should the
  issue body become a bounded summary projection once cards exist?

## Recommended Review Outcome

Recommend review approval of the following direction:

- accept `SOR` as the canonical authority for PR publication body truth
- accept a generalized card-to-GitHub projection policy as a `v0.91.6`
  tooling/control-plane requirement
- accept the proposed first-tranche projection classification before broadening
  automatic synchronization to other surfaces
- route implementation in bounded slices so projection ownership becomes
  deterministic without collapsing C-SDLC card roles
