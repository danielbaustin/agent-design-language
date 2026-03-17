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
- first practical Gödel runtime progress
- a minimal working affect engine and reasoning-graph integration
- a stronger demo program that proves the milestone with runnable bounded examples

## Sprint Structure
v0.85 is organized into **four explicit sprints**.

### Sprint 1 — Milestone Reorganization + Execution Substrate
Goal:
Finalize milestone planning artifacts and begin the core execution-substrate improvements required for dependable execution.

Scope:
- WP-01 Milestone reorganization and docs alignment
- WP-02 Deterministic queue / checkpoint / steering substrate
- WP-03 Cluster / distributed execution groundwork
- WP-04 Prompt Spec completeness for editors

### Sprint 2 — Authoring Surfaces + Review Tooling
Goal:
Make authoring and review materially easier with real tools.

Scope:
- WP-05 First authoring/editor surfaces
- WP-06 Editing and review GPT/tooling surfaces
- WP-07 Dependable execution runtime surfaces
- WP-08 Verifiable inference runtime surfaces

### Sprint 3 — Gödel, Affect, Reasoning Graphs, and AEE Progress
Goal:
Deliver the milestone's major cognitive/runtime leap.

Scope:
- WP-09 Adaptive Execution Engine bounded progress
- WP-10 Deterministic hypothesis generation engine
- WP-11 Policy-learning and adaptive Gödel loop
- WP-12 Experiment prioritization and strategy confidence
- WP-13 Cross-workflow learning and recursive improvement
- WP-14 Promotion and eval-report artifact loop
- WP-15 Affect engine core
- WP-16 Reasoning graph and affect integration
- WP-17 Affect-plus-Gödel vertical slice

### Sprint 4 — Demos, Quality Gate, Review, Release, and Next-Milestone Planning
Goal:
Prove the milestone, complete review and release work, and plan the next milestone cleanly.

Scope:
- WP-18 Demo program for v0.85 features
- WP-19 Coverage / quality gate
- WP-20 Documentation consistency pass
- WP-21 Internal review
- WP-22 External review
- WP-23 Review findings remediation
- WP-24 Release ceremony
- WP-25 Next milestone planning
- Final `swarm` -> `adl` cutover runs only after the rest of the milestone is stable and should follow `SWARM_REMOVAL_PLANNING.md`

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | WP-01 Milestone reorganization and docs alignment | #886 | Daniel / Codex | In progress |
| 2 | WP-02 Deterministic queue / checkpoint / steering substrate | #674 | Daniel / Codex | Planned |
| 3 | WP-03 Cluster / distributed execution groundwork | #868 provisional remap | Daniel / Codex | Planned |
| 4 | WP-04 Prompt Spec completeness for editors | #716 / #869 provisional remap | Daniel / Codex | Planned |
| 5 | WP-05 First authoring/editor surfaces | #870 provisional remap | Daniel / Codex | Planned |
| 6 | WP-06 Editing and review GPT/tooling surfaces | #871 provisional remap | Daniel / Codex | Planned |
| 7 | WP-07 Dependable execution runtime surfaces | #872 provisional remap | Daniel / Codex | Planned |
| 8 | WP-08 Verifiable inference runtime surfaces | #873 provisional remap | Daniel / Codex | Planned |
| 9 | WP-09 Adaptive Execution Engine bounded progress | #874 provisional remap | Daniel / Codex | Planned |
| 10 | WP-10 Deterministic hypothesis generation engine | #748 | Daniel / Codex | Planned |
| 11 | WP-11 Policy-learning and adaptive Gödel loop | #749 | Daniel / Codex | Planned |
| 12 | WP-12 Experiment prioritization and strategy confidence | #750 | Daniel / Codex | Planned |
| 13 | WP-13 Cross-workflow learning and recursive improvement | #751 | Daniel / Codex | Planned |
| 14 | WP-14 Promotion and eval-report artifact loop | #752 | Daniel / Codex | Planned |
| 15 | WP-15 Affect engine core | #875 provisional remap | Daniel / Codex | Planned |
| 16 | WP-16 Reasoning graph and affect integration | #876 provisional remap | Daniel / Codex | Planned |
| 17 | WP-17 Affect-plus-Gödel vertical slice | #877 provisional remap | Daniel / Codex | Planned |
| 18 | WP-18 Demo program for v0.85 features | #743 / #878 provisional remap | Daniel / Codex | Planned |
| 19 | WP-19 Coverage / quality gate | #879 provisional remap | Daniel / Codex | Planned |
| 20 | WP-20 Documentation consistency pass | #880 provisional remap | Daniel / Codex | Planned |
| 21 | WP-21 Internal review | New follow-on issue under #886 | Daniel / reviewers | Planned |
| 22 | WP-22 External review | New follow-on issue under #886 | Daniel / external reviewer | Planned |
| 23 | WP-23 Review findings remediation | New follow-on issue under #886 | Daniel / Codex | Planned |
| 24 | WP-24 Release ceremony | New follow-on issue under #886 | Daniel / Codex | Planned |
| 25 | WP-25 Next milestone planning | New follow-on issue under #886 | Daniel / Codex | Planned |

## Cadence Expectations
- Each WP should have its own issue, branch, worktree, and input/output cards.
- Code changes must pass `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` unless the WP is documentation-only.
- Documentation should converge before formal review begins.
- Once review begins, milestone docs should remain stable except for explicit review findings.
- Demos should be runnable with deterministic, CI-friendly commands when possible.
- The milestone should explicitly prove steering/queueing, HITL/editor/review flow, and affect-plus-Gödel behavior through bounded demos.
- The final `swarm` -> `adl` cutover should happen after the milestone’s other code changes have settled, not in parallel with high-churn feature work.

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
  - steering / queueing / checkpoint example
  - authoring + verification workflow example
  - Gödel hypothesis-engine example
  - affect-engine example
  - affect-plus-Gödel reasoning example

- Review sequence:
  1. Documentation consistency pass
  2. Internal review
  3. External review
  4. Fix findings / explicit deferrals
  5. Release ceremony
  6. Next milestone planning

- Sign-off owners:
  - Daniel Austin
  - External reviewer (for external review step)

## Exit Criteria
- All planned scope items are completed or explicitly deferred with rationale.
- Linked issues/PRs are updated and traceable.
- CI is green for merged work.
- Major v0.85 planning docs are internally consistent.
- Internal and external reviews are completed before release ceremony.
- Required demo proof surfaces are present and reviewable.
- Sprint outcomes are captured in milestone documentation.
