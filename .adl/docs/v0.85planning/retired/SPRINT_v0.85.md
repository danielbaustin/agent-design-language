# Sprint Plan — v0.85

## Metadata
- Sprint plan: `v0.85`
- Milestone: `v0.85`
- Start date: `2026-03-10`
- End date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## How To Use
- Keep scope small enough to finish each sprint with green CI and merged PRs.
- Execute work in dependency order from the WBS.
- Use issue cards (`input` / `output`) for each item.
- Treat docs as first-class deliverables; planning docs should converge early.
- Include an explicit **internal review and external review** step before the release ceremony.

## Sprint Goal
Deliver v0.85 as the milestone where ADL transitions from an experimental substrate into a **more operationally mature agent engineering platform** by completing:

- dependable execution improvements
- verifiable inference surfaces
- stronger authoring and tooling surfaces
- bounded Adaptive Execution Engine progress
- early cognitive substrate work (affect model + reasoning graph foundations)

For the authoring track, v0.85 is expected to ship real usable HTML-based
editor surfaces, not only editor-direction docs.

## Sprint Structure
v0.85 is organized into **three implementation sprints plus a release tail**.

### Sprint 1 — Planning Alignment + Runtime Foundations
Goal:
Finalize milestone planning artifacts and begin the core runtime infrastructure improvements required for dependable execution.

Scope:
- WP-01 Design pass (planning alignment)
- WP-02 Runtime reliability improvements
- WP-03 Deterministic queue / checkpoint surfaces
- WP-04 Retry / backoff / bounded adaptation surfaces

### Sprint 2 — Authoring + Trust Surfaces
Goal:
Strengthen the development workflow and improve the trust and verification infrastructure around ADL artifacts.

Scope:
- WP-05 Prompt Spec completeness improvements
- WP-06 Authoring and tooling improvements, including working HTA/task-bundle editor flow
- WP-07 Artifact validation and verification improvements
- WP-08 Review and CI surfaces

### Sprint 3 — Cognitive Substrate (Gödel + Affect)
Goal:
Advance the cognitive architecture that supports Gödel-style experimentation and adaptive reasoning.

Scope:
- WP-09 Hypothesis engine foundations
- WP-10 Reasoning graph schema
- WP-11 ObsMem integration for reasoning artifacts
- WP-12 Affective reasoning / evaluation signals

### Release Tail — Demos, Quality Gate, Docs Alignment, Review, Ceremony
Goal:
Stabilize the milestone, validate demonstrations, enforce quality gates, converge documentation, execute internal and external reviews, and perform the release ceremony.

Scope:
- WP-13 Demo matrix + integration demos
- WP-14 Coverage / quality gate
- WP-15 Docs alignment + review sequence
  - Step 1: docs consistent
  - Step 2: internal review
  - Step 3: external review
- Review findings / fixes
- WP-16 Release ceremony

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | WP-01 Design pass (planning alignment) | #674 | Daniel / Codex | Planned |
| 2 | WP-02 Runtime reliability improvements | TBD | Daniel / Codex | Planned |
| 3 | WP-03 Deterministic queue / checkpoint surfaces | TBD | Daniel / Codex | Planned |
| 4 | WP-04 Retry / adaptation surfaces | TBD | Daniel / Codex | Planned |
| 5 | WP-05 Prompt Spec completeness | #716 | Daniel / Codex | Planned |
| 6 | WP-06 Authoring / tooling improvements | TBD | Daniel / Codex | Planned |
| 7 | WP-07 Artifact validation / verification | #729 | Daniel / Codex | Planned |
| 8 | WP-08 Review and CI surfaces | TBD | Daniel / Codex | Planned |
| 9 | WP-09 Hypothesis engine foundations | #748 | Daniel / Codex | Planned |
| 10 | WP-10 Reasoning graph schema | #750 | Daniel / Codex | Planned |
| 11 | WP-11 ObsMem reasoning integration | #751 | Daniel / Codex | Planned |
| 12 | WP-12 Affective reasoning model | #752 | Daniel / Codex | Planned |
| 13 | WP-13 Demo matrix + integration demos | TBD | Daniel / Codex | Planned |
| 14 | WP-14 Coverage / quality gate | TBD | Daniel / Codex | Planned |
| 15 | WP-15 Docs alignment + review sequence | TBD | Daniel / Codex | Planned |
| 16 | Internal review pass | TBD | Daniel / reviewers | Planned |
| 17 | External review pass | TBD | Daniel / external reviewer | Planned |
| 18 | WP-16 Release ceremony | TBD | Daniel / Codex | Planned |

## Cadence Expectations
- Each WP should have its own issue, branch, worktree, and input/output cards.
- Code changes must pass `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` unless the WP is documentation-only.
- Documentation should converge before formal review begins.
- Once review begins, milestone docs should remain stable except for explicit review findings.
- Demos should be runnable with deterministic, CI-friendly commands when possible.

## Risks / Dependencies
- Dependency: ongoing runtime refactors
  - Risk: implementation surfaces move faster than milestone docs
  - Mitigation: treat `DESIGN_v0.85.md` and `WBS_v0.85.md` as alignment anchors

- Dependency: cognitive subsystem design still evolving
  - Risk: Gödel / reasoning graph design churn
  - Mitigation: keep early implementations bounded and schema-focused

- Dependency: CI and coverage work still in progress
  - Risk: quality gate delays release
  - Mitigation: allow bounded exceptions with documented rationale

## Demo / Review Plan
- Demo artifacts:
  - reasoning graph example
  - hypothesis evaluation loop example
- HTA/editor/review workflow example

- Review sequence:
  1. Internal planning and design convergence
  2. Documentation alignment
  3. Internal review
  4. External review
  5. Fix findings / explicit deferrals
  6. Release ceremony

- Sign-off owners:
  - Daniel Austin
  - External reviewer (for external review step)

## Exit Criteria
- All planned scope items are completed or explicitly deferred with rationale.
- Linked issues/PRs are updated and traceable.
- CI is green for merged work.
- Major v0.85 planning docs are internally consistent.
- Internal and external reviews are completed before release ceremony.
- Sprint outcomes are captured in milestone documentation.
