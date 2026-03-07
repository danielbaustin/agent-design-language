
# Sprint Plan — v0.8

## Metadata
- Sprint plan: `v0.8`
- Milestone: `v0.8`
- Start date: `2026-03-07`
- End date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## How To Use
- Keep scope small enough to finish each sprint with green CI and merged PRs.
- Execute work in dependency order from the WBS.
- Use issue cards (`input` / `output`) for each item.
- Treat docs as first-class deliverables; freeze docs before formal review.
- Include an explicit **3rd party review** step before release ceremony.

## Sprint Goal
Deliver v0.8 as a focused milestone for **controlled experimentation and authoring** by completing:

- the core Gödel experiment artifact spine
- the first deterministic card/prompt automation surfaces
- the flagship Rust transpiler / migration demo
- a disciplined release tail with docs freeze, 3rd party review, and ceremony

## Sprint Structure
v0.8 is organized into **three implementation sprints plus a release tail**.

### Sprint 1 — Design + Schema Spine
Goal:
Finalize milestone planning and define the core machine-readable experiment contracts needed for Gödel workflows.

Scope:
- WP-01 Design pass
- WP-02 ExperimentRecord schema v1
- WP-03 Canonical Evidence View
- WP-04 Mutation format v1
- WP-05 EvaluationPlan v1

### Sprint 2 — Experiment Runtime + Memory Integration
Goal:
Turn the schema spine into executable deterministic experiment workflows and usable memory surfaces.

Scope:
- WP-06 Gödel experiment workflow template
- WP-07 ObsMem indexing for run summaries + experiment records
- WP-08 ToolResult contract hardening

### Sprint 3 — Authoring Surfaces + Flagship Demo
Goal:
Make the structured authoring pipeline actionable and prove the system with the Rust transpiler / migration workflow.

Scope:
- WP-09 Authoring surfaces v1
- WP-10 Prompt automation + reviewer-ready execution flow
- WP-11 Rust transpiler fixture + workflow scaffold
- WP-12 Rust transpiler verification + adaptive execution evidence

### Release Tail — Demos, Quality Gate, Docs Freeze, Review, Ceremony
Goal:
Stabilize the milestone, validate flagship demos, enforce quality gates, freeze docs, execute 3rd party review, resolve findings, and release.

Scope:
- WP-13 Demo matrix + integration demos
- WP-14 Coverage / quality gate
- WP-15 Documentation pass + review convergence (docs freeze before review)
- Explicit 3rd party review step
- Review findings / fixes
- WP-16 Release ceremony

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | WP-01 Design pass | TBD | Daniel / Codex | Planned |
| 2 | WP-02 ExperimentRecord schema v1 | #609 | Daniel / Codex | Planned |
| 3 | WP-03 Canonical Evidence View | #610 | Daniel / Codex | Planned |
| 4 | WP-04 Mutation format v1 | #611 | Daniel / Codex | Planned |
| 5 | WP-05 EvaluationPlan v1 | #612 | Daniel / Codex | Planned |
| 6 | WP-06 Gödel experiment workflow template | #613 | Daniel / Codex | Planned |
| 7 | WP-07 ObsMem indexing for run summaries + experiment records | #614 | Daniel / Codex | Planned |
| 8 | WP-08 ToolResult contract hardening | #618 | Daniel / Codex | Planned |
| 9 | WP-09 Authoring surfaces v1 | #517 | Daniel / Codex | Planned |
| 10 | WP-10 Prompt automation + reviewer-ready execution flow | TBD | Daniel / Codex | Planned |
| 11 | WP-11 Rust transpiler fixture + workflow scaffold | TBD | Daniel / Codex | Planned |
| 12 | WP-12 Rust transpiler verification + adaptive execution evidence | TBD | Daniel / Codex | Planned |
| 13 | WP-13 Demo matrix + integration demos | TBD | Daniel / Codex | Planned |
| 14 | WP-14 Coverage / quality gate | TBD | Daniel / Codex | Planned |
| 15 | WP-15 Documentation pass + review convergence | TBD | Daniel / Codex | Planned |
| 16 | 3rd party review pass | TBD | Daniel / external reviewer / independent agent | Planned |
| 17 | Review findings / fixes | TBD | Daniel / Codex | Planned |
| 18 | WP-16 Release ceremony | TBD | Daniel / Codex | Planned |

## Cadence Expectations
- Each WP should have its own issue, branch, worktree, and input/output cards.
- Code changes must pass `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` unless the WP is documentation-only.
- Documentation changes must be finalized before formal review begins.
- Once the review pass begins, milestone docs are frozen and further doc edits require explicit review findings/issues.
- Demos should be validated with deterministic, local, CI-friendly commands whenever possible.

## Risks / Dependencies
- Dependency: v0.75 release completion
  - Risk: unfinished v0.75 stabilization could delay v0.8 start
  - Mitigation: keep v0.8 planning moving in parallel but do not begin implementation sprints until v0.75 is cleanly released
- Dependency: design-stage Gödel schemas may not yet be runtime-enforced
  - Risk: documentation/code drift
  - Mitigation: pair schemas with examples, tests, or consuming surfaces as early as possible
- Dependency: Rust transpiler demo may expose environment/toolchain nondeterminism
  - Risk: flaky demo and weak evidence story
  - Mitigation: use a small fixture crate, pinned assumptions, explicit evidence artifacts, and bounded retry logic
- Dependency: card automation may rely on brittle markdown parsing
  - Risk: automation instability
  - Mitigation: anchor generation on Prompt Spec and Verification Summary machine-readable blocks

## Demo / Review Plan
- Demo artifacts:
  - Gödel experiment workflow example
  - Rust transpiler / migration demo evidence bundle
  - card → prompt → execution → review pipeline example
- Review sequence:
  1. Internal docs/design convergence
  2. Docs freeze
  3. 3rd party review
  4. Fix findings / explicit deferrals
  5. Release ceremony
- Sign-off owners:
  - Daniel Austin
  - Independent reviewer / 3rd party review step

## Exit Criteria
- All planned scope items are completed or explicitly deferred with rationale.
- Linked issues/PRs are updated and traceable.
- CI is green for merged work.
- v0.8 docs are frozen before formal review.
- The 3rd party review step is completed before release ceremony.
- Sprint summary and release-tail outcomes are captured in milestone docs.
