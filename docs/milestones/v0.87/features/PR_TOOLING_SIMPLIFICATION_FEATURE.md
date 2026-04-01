# PR Tooling Simplification

## Metadata
- Feature Name: `PR Tooling Simplification`
- Milestone Target: `v0.87`
- Status: `planned`
- Owner: `Daniel Austin / Agent Logic`
- Doc Role: `primary`
- Supporting Docs: `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- Feature Types: `architecture`, `policy`, `artifact`
- Proof Modes: `tests`, `review`

## Template Rules

- Every section is completed or explicitly marked `N/A` with justification.
- Demo- and schema-specific requirements are marked `N/A` where this feature is
  control-plane and workflow architecture rather than a user-facing runtime
  feature.

## Purpose

This feature simplifies the ADL PR lifecycle tooling by reducing duplicated
workflow ownership and shrinking the public command surface.

Today the PR flow is split across large shell scripts and a partially migrated
Rust control plane. That split makes the tooling harder to understand, harder to
test, and harder to evolve safely.

This feature exists to make one implementation authoritative, make the command
model easier to learn, and turn `adl/tools/pr.sh` back into a thin
compatibility wrapper instead of a workflow engine.

## Context

- Related milestone: `v0.87`
- Related issues: `N/A yet; derive from future implementation slices`
- Dependencies:
  - `adl/tools/pr.sh`
  - `adl/tools/card_paths.sh`
  - `adl/src/control_plane.rs`
  - `adl/src/cli/pr_cmd.rs`
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

This feature fits into the broader ADL architecture as workflow control-plane
cleanup. It does not change the meaning of STP/SIP/SOR artifacts or the GitHub
review flow. Instead, it changes where the lifecycle rules live and how users
invoke them.

## Milestone Positioning

This feature is part of the `v0.87` operational/control-plane substrate.

It exists to consolidate workflow ownership and reduce execution drift before
later milestones introduce deeper cognitive, memory, and convergence features.

By stabilizing the PR lifecycle under a single Rust-owned control plane, this
feature ensures that subsequent features (AEE, Gödel agent, and higher-order
agency layers) are built on a predictable and deterministic execution surface.

## Coverage / Ownership

This document is the primary feature-level owner for the PR tooling
simplification effort.

- Primary owner doc: `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- Covered surfaces:
  - `adl/tools/pr.sh`
  - `adl/tools/card_paths.sh`
  - `adl/src/control_plane.rs`
  - `adl/src/cli/pr_cmd.rs`
  - PR lifecycle command surface and compatibility policy
- Related / supporting docs:
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

## Overview

This feature makes Rust the single owner of PR lifecycle behavior and reduces
the workflow to a few user-facing intents.

Key capabilities:
- thin-shell delegation from `adl/tools/pr.sh` into Rust `adl pr`
- a reduced command model centered on `create`, `start`, `finish`, and
  `doctor`
- one authoritative implementation of path rules, bootstrap rules, readiness
  checks, and PR lifecycle policy

This feature should be understood as control-plane consolidation rather than
simple tooling cleanup. It establishes a stable execution surface for all
subsequent workflow-driven features and reduces ambiguity in how PR lifecycle
operations are performed.

## Design

### Core Concepts

The main concepts introduced or clarified by this feature are:

- **single ownership**
  - PR lifecycle rules should have one implementation owner
- **intent-oriented commands**
  - users should invoke a few high-level commands rather than many internal
    workflow phases
- **compatibility wrapper**
  - shell remains as a launcher and migration surface, not as the product logic
- **control-plane consolidation**
  - lifecycle behavior is centralized in Rust and treated as core system
    infrastructure rather than auxiliary tooling

### Architecture

The feature restructures the PR tooling around a Rust-owned control plane with a
thin shell entrypoint.

- Inputs (explicit sources / triggers):
  - user invocation of `adl/tools/pr.sh ...` or `adl pr ...`
  - repository state
  - git worktree state
  - GitHub issue and PR state through `gh`
  - canonical local workflow artifacts under `.adl/`
- Outputs (artifacts / side effects):
  - canonical issue prompt, STP, SIP, and SOR bootstrap surfaces
  - branch and worktree creation or reuse
  - staged commits and pushed branches
  - created or updated GitHub PRs
  - readiness and drift diagnostics from `doctor`
- Interfaces (APIs, CLI, files, schemas):
  - `adl/tools/pr.sh`
  - `adl pr create`
  - `adl pr start`
  - `adl pr finish`
  - `adl pr doctor`
  - optional `adl pr open`
  - legacy aliases for `init`, `ready`, and `preflight` during migration
- Invariants (must always hold):
  - Rust is the sole owner of canonical PR lifecycle behavior
  - shell compatibility layers do not reimplement workflow policy
  - canonical path rules have exactly one authoritative implementation
  - readiness inspection is consolidated under `doctor`
  - human review remains preserved through draft-oriented PR flow unless
    explicitly overridden

### Command Semantics

- `create`
  - initializes issue-level artifacts and task-bundle scaffolding
- `start`
  - creates or reuses worktree/branch and transitions into active execution
- `finish`
  - validates outputs, stages changes, and completes PR lifecycle steps
- `doctor`
  - reports readiness, detects workflow drift, surfaces deprecated usage,
    and provides migration guidance

### Data / Artifacts

This feature does not introduce a brand-new artifact family. It governs how
existing workflow artifacts are created, validated, and reconciled.

- canonical issue prompts under `.adl/.../bodies/`
- task-bundle artifacts under `.adl/.../tasks/...`
- compatibility card locations under `.adl/cards/...`
- deprecation and compatibility behavior for CLI entrypoints

## Execution Flow

This is an artifact-bearing workflow feature, so execution flow applies.

1. A user invokes `adl/tools/pr.sh` or `adl pr`.
2. The thin shell wrapper delegates to Rust `adl pr`.
3. The Rust control plane resolves the requested lifecycle intent:
   - `create`
   - `start`
   - `finish`
   - `doctor`
4. Rust applies canonical path rules, bootstrap rules, and readiness checks.
5. Rust performs the required side effects through `git` and `gh`.
6. Rust reports workflow state, readiness, drift diagnostics, and migration hints back to the user.

## Determinism and Constraints

- Determinism guarantees (what must be repeatable and how):
  - canonical path resolution must be deterministic for the same repo state and
    issue inputs
  - slug normalization and issue-derived branch naming must be deterministic
  - readiness inspection must produce stable results for the same repository and
    workflow state
  - deprecated commands must map predictably to the same Rust-owned behavior as
    their canonical replacements
- Constraints (performance, ordering, limits):
  - shell should remain minimal and should not accumulate new business logic
  - migration should be incremental rather than a one-shot rewrite
  - compatibility aliases should preserve user workflows during transition
  - git and GitHub side effects must remain explicit and reviewable

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| `git` worktree and branch state | read/write | Used for branch creation, worktree management, staging, commit flow, and readiness validation. |
| GitHub CLI `gh` | read/write | Used for issue fetch/create, PR create/edit/view, and readiness-related repository metadata. |
| `.adl/` workflow artifacts | read/write | Source prompts, task bundles, and compatibility cards remain the canonical local workflow surfaces. |
| shell compatibility entrypoint | trigger | `adl/tools/pr.sh` remains as the user-facing launcher during migration but delegates to Rust. |
| Rust control-plane library | read/write/trigger | Shared domain logic and command handlers provide the authoritative lifecycle behavior. |

## Validation

This feature is primarily validated through tests, command compatibility, and
manual review of the simplified command surface.

### Demo (if applicable)
- Demo script(s): `N/A`
- Expected behavior: `N/A; this is workflow/control-plane architecture rather than a milestone demo feature`

### Deterministic / Replay
- Replay requirements:
  - command-to-command mapping must remain stable during migration
  - path and readiness computations should be reproducible from repository state
- Determinism guarantees:
  - deterministic slug, path, and branch derivation
  - deterministic `doctor` readiness classification for identical state

### Schema / Artifact Validation
- Schemas involved: `N/A; no new artifact schema is introduced by this feature`
- Artifact checks:
  - existing issue prompt, task-bundle, and compatibility card surfaces still
    resolve correctly
  - shell compatibility entrypoint invokes the Rust-owned command path without
    behavioral drift

### Tests
- Test surfaces:
  - unit tests for slug normalization, path resolution, and command alias rules
  - integration tests for `create`, `start`, `finish`, and `doctor`
  - compatibility tests verifying `pr.sh` delegates correctly

### Review / Proof Surface
- Review method (manual/automated): `both`
- Evidence location:
  - Rust unit and integration tests
  - command-level compatibility tests
  - `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

## Acceptance Criteria

- Functional correctness (what must work):
  - `create`, `start`, `finish`, and `doctor` are sufficient to represent the
    primary PR lifecycle
  - `preflight` and `ready` are absorbed into `doctor` via compatibility aliases
    during migration
  - `init` is merged into `start` conceptually and operationally
- Determinism / replay correctness:
  - canonical path, slug, and branch derivations remain stable
  - readiness reporting remains stable for identical repository state
- Validation completeness (tests/schema/demo/review as applicable):
  - Rust-owned lifecycle behavior is covered by tests
  - compatibility behavior is explicitly tested
  - docs point users to the simplified command model

## Risks

- Primary risks (failure modes):
  - Rust command code could become a new monolith if the migration only moves
    code without improving module boundaries
  - shell and Rust behavior could drift during transition
  - command consolidation could break existing user habits or scripts
- Mitigations:
  - split Rust code by domain rather than one giant command file
  - freeze new shell business logic immediately
  - keep deprecated aliases temporarily and emit migration hints
  - validate compatibility paths with tests

## Future Work

- Follow-ups / extensions:
  - decide whether `run` remains a separate wrapper or moves into the same Rust
    command surface
  - decide whether `card`, `output`, and `cards` remain expert-level commands or
    become internal helpers only
- Known gaps / deferrals:
  - milestone placement is `v0.87` (operational/control-plane substrate milestone)
  - implementation sequencing and issue decomposition remain to be planned in
    follow-on work

## Notes

This feature doc is the concise product-facing summary for the effort.

The more detailed design and migration reasoning lives in:

- `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`

The key simplification decisions are:

- Rust owns the PR lifecycle behavior
- `adl/tools/pr.sh` becomes a thin compatibility shim
- `preflight`, `ready`, and most of `status` collapse into `doctor`
- `start` absorbs the current `init` behavior
