# Editing Architecture for ADL

## Purpose

This document captures the architectural direction for evolving the current `pr.sh` workflow into a reliable editing system for ADL.

The current shell tooling works, but it is fragile and difficult to extend safely. The goal of this plan is not merely to clean up bash, but to define the editing/control plane that ADL will use for:
- authoring issues and prompts
- validating structured artifacts
- executing workflow commands
- integrating with GitHub
- managing card history and lifecycle
- supporting future editors and UI surfaces
- working cleanly with Gödel and the Adaptive Execution Engine (AEE)
- recovering safely from mistakes

This architecture should be treated as a first-pass plan for building a reasonable editing system during and after v0.85.

For clarity:

- `v0.85` is not only an architecture milestone for editing
- `v0.85` is expected to ship actual working editor surfaces that contributors
  can use
- the concrete first implementation path for those editors is simple tracked
  HTML pages in-repo
- later control-plane hardening and richer integrations should support those
  editors, not displace the requirement to ship them

## Command Status For v0.85

This document follows the canonical command-status model in the editing
five-command execution plan.

| Command | Current state | Repo truth | Notes |
| --- | --- | --- | --- |
| `pr init` | missing | no live command in `adl/tools/pr.sh` | planned work, not current behavior |
| `pr create` | partial / renamed | `pr new` exists, but it is only a rough precursor | do not describe `pr new` as a finished substitute |
| `pr start` | implemented | active backbone command today | real and mature enough to anchor the current control plane |
| `pr run` | missing | no live command in `adl/tools/pr.sh` | execution still happens through narrower paths |
| `pr finish` | implemented | active mature workflow command today | real, but still subject to reliability/polish work |

## Core Insight

`pr.sh` is no longer just a helper script for branches and worktrees. It is already functioning as the nucleus of the editing system.

Today it effectively manages:
- issue creation workflows
- worktree and branch setup
- input/output card setup
- implementation execution handoff
- PR lifecycle completion


The long-term goal is to evolve this into a structured, validated, file-backed control plane. The immediate v0.85 goal is to ship usable editor surfaces while beginning that control-plane evolution without destabilizing the milestone.

A key emerging insight is that `pr start` is likely to become the operational backbone of the editing system. Even if the longer-term lifecycle grows to include additional commands such as `init`, `create`, `run`, and `finish`, `pr start` is the first command that binds an authored task to a concrete execution context. That makes it the natural spine of the near-term control plane.

## Current Problem

The current editing workflow is fragile in several ways:
- artifact structure is only partially enforced
- issue prompts, input cards, and output cards are validated informally
- `pr.sh` is bash-based and difficult to scale safely
- too much logic depends on convention rather than machine-checkable rules
- small authoring errors can create significant workflow damage
- recovery from mistakes is inconsistent
- GitHub integration exists, but is not yet fully closed-loop or schema-driven

In practice, the workflow already wants stronger contracts.

## Architectural Thesis

ADL should have a three-layer editing system:

### Layer 1 - Artifact Contracts

Machine-readable schemas and validation rules for the core editing artifacts:
- tracked issue prompts
- input cards
- output cards

These contracts define:
- required fields
- normalized vocabularies
- allowed enums
- section requirements
- validation expectations
- cross-artifact consistency rules

### Layer 2 - Workflow Control Plane

A command layer that orchestrates the artifact lifecycle and enforces validation at every pass.


This is currently centered on `pr.sh`, but should evolve into a proper programmatic subsystem.

In the near term, the most important command in that subsystem is `pr start`, because it turns a validated authored task into a bound execution context with a worktree, branch, implementation prompt, and output-record expectations. Over time, other lifecycle commands may expand around it, but `pr start` is the clearest candidate for the first strong Rust control-plane command.

The control plane should manage:
- issue creation
- worktree/branch creation
- execution setup
- validation checks
- completion and finish logic
- error handling and safe recovery

### Layer 3 - Authoring and Review Surfaces

User-facing surfaces for creating, editing, and reviewing the artifacts.

This includes:
- issue editor
- implementation editor
- output/review editor
- reviewer tooling and GPT-based helpers
- future UI surfaces

These surfaces should sit on top of the artifact contracts and workflow control plane, not bypass them.

## Proposed Workflow Model

The intended file-backed target-state workflow is:

1. Author structured task prompt
2. Validate the structured task prompt locally
3. Promote the authoritative structured task prompt into tracked public task-bundle records
4. Run `pr create` when issue creation is needed
5. Create or reconcile the GitHub issue from the tracked structured task prompt when GitHub is part of the workflow
6. Run `pr start`
7. Create or reconcile the bound execution context and paired implementation artifacts
8. Author/refine structured implementation prompt
9. Promote authoritative implementation/output artifacts into tracked public records at the lifecycle points where they become official
10. Run the execution pass
11. Review structured output record and evidence
12. Either revise the structured implementation prompt for another pass or run `pr finish`

This means the major artifacts become:
- structured task prompt = design-layer artifact
- GitHub issue = optional rendered/project-management artifact when GitHub issue tracking is part of the workflow
- structured implementation prompt = implementation-layer artifact
- structured output record = execution-record artifact

## Draft Workspace vs Public Record

ADL should distinguish between:

- `.adl/`
  - temporary draft workspace
  - generated intermediate files
  - editor-local scratch state
- tracked task-bundle record directories such as `docs/records/<scope>/tasks/<task-id>/`
  - canonical public record
  - auditable and reviewable history
  - authoritative artifacts for official lifecycle transitions

The important rule is that `.adl/` remains useful, but it is not the durable source of truth. Future editing tools should be able to draft locally while promoting stable artifacts into tracked task bundles before those artifacts become authoritative workflow state.

## Closed-Loop Goal

The workflow should close the loop without requiring direct manual `gh` usage in normal operation.

The intended command path is:
- validate authored task
- `pr create` when issue creation is needed
- `pr start`
- execution pass
- review/repair loop as needed
- `pr finish`

This only becomes a reliable closed loop if validation is enforced at every transition.


Operationally, `pr start` and `pr finish` are the active mature workflow commands today. `pr create` is the desired next control-plane command and should be treated as part of the target-state architecture even where today’s implementation is still incomplete. `pr new` should be understood as the current rough precursor to `pr create`, not as a finished substitute for the target lifecycle command.

The architecture should therefore treat `pr start` as the anchor command for the current system, while `pr create`, execution orchestration, and finish/repair behavior become increasingly formalized around it. This is one reason the longer-term control plane should move into Rust: the backbone command should not remain trapped in brittle bash if it is going to carry more general lifecycle responsibility.

## Why the Editing System Must Be File-Backed

The architecture should prefer file-backed artifacts over ad hoc inline text.

Reasons:
- determinism
- reproducibility
- version control history
- inspectability
- schema validation
- replayability
- future UI/editor compatibility
- portability beyond software-development-specific workflows

Structured task prompts, structured implementation prompts, and structured output records should be treated as real artifacts, not temporary chat text.

Near-term canonical homes should look like:

- `docs/records/<scope>/tasks/<task-id>/stp.md`
- `docs/records/<scope>/tasks/<task-id>/sip.md`
- `docs/records/<scope>/tasks/<task-id>/sor.md`

This keeps milestone narrative docs clean while preserving public, reviewable prompt history in a domain-neutral task-bundle form.

## Immediate v0.85 Architectural Direction

The concrete v0.85 commitment is:

- ship actual working editor surfaces, not only editor-direction documents
- make those first editor surfaces usable through tracked HTML pages in-repo
- keep them grounded in the artifact-contract and control-plane model
- leave richer integrations, deeper automation, and larger rewrites for later
  slices unless they directly help the usable editors land

For v0.85, the primary user-facing editor surfaces should be:

- structured task prompt / issue editor
- structured implementation prompt editor
- structured output record / review editor, or a clearly bounded first slice of
  it if the full surface does not fit safely

Those editors should be real working surfaces that a contributor can open and
use during the milestone, not just mockups, architecture sketches, or future
integration notes.

### 1. Keep `pr.sh`, but narrow its role

For v0.85, `pr.sh` should remain the thin workflow entrypoint.

It should be responsible for calling the validated workflow steps, not containing all the long-term editing logic itself.

In practice, that means the bash layer should increasingly act as a thin compatibility wrapper while the durable control-plane behavior migrates into Rust. The first command most worth hardening in that way is `pr start`, because it is where authored intent becomes a bound execution context.

### 2. Add schemas first

Before large tooling rewrites, define the contracts for:
- tracked issue prompts
- input cards
- output cards

This is the most important near-term step because all later tooling depends on reliable artifacts, including the working editors that v0.85 is supposed to ship.

### 3. Make `pr.sh` call validation at every pass

Validation should happen at:
- `pr create`
- `pr start`
- `pr finish`

This is what turns the workflow from procedural into reliable and keeps the shipped editors from becoming fragile parallel workflows.

### 4. Begin incremental migration of logic out of bash

Do not attempt a full one-shot rewrite.

Instead:
- keep bash as the outer wrapper initially
- move validation and workflow logic into a more structured implementation layer incrementally
- allow the future Rust core to take over once the artifact contracts are stable

The key sequencing rule is:

- first ship usable editors
- then continue hardening the control plane underneath them
- do not defer the editors until after a large control-plane rewrite

## Long-Term Direction: Rust Core

The long-term goal is to move the core editing/control-plane logic into Rust.

This migration is not primarily about language preference. It is about moving the backbone lifecycle behavior out of fragile shell scripting and into a typed, testable, portable control-plane core that can support editors, validators, recovery logic, later signing/preservation, and non-software-development uses of ADL.

Reasons:
- stronger type safety
- more manageable logic than large bash scripts
- better validation and error handling
- easier integration with future UI/editor layers
- easier state/history management
- better fit for Gödel and AEE integration later

The Rust core should eventually own:
- artifact loading/parsing
- schema validation
- issue/body parsing
- branch/worktree orchestration
- lifecycle commands (`create`, `start`, `finish`)
- validation reporting
- recovery and repair paths
- repo-state portability and validation checks
- command execution wrappers
- generalized task/execution binding beyond software-repository workflows
- workflow portability beyond software-development-specific assumptions

## Relationship to UI and Editors

The concrete first editor implementation for ADL is simple HTML-based authoring
tools tracked in the repo.

Those HTML pages are not placeholders. They are the expected v0.85 shipped
editor surfaces.

The UI must not become the source of truth.

The UI should sit on top of the validated artifact model.

Zed integration is a plausible future authoring surface and may begin appearing as early as the later phases of the control-plane migration. The important architectural rule is that editor integrations should consume the validated artifact/control-plane model rather than inventing parallel state.

The same rule applies to future non-IDE surfaces as well. ADL should not assume that its control plane is only for software development. Editors and integrations may begin in software-focused tools because that is the immediate use case, but the artifact model and lifecycle commands should be defined in a way that can generalize to broader task domains later.

A simple first-pass editor architecture would look like:
- open editor UI
- start screen
- issue editor
- `pr create`
- implementation editor
- `pr start`
- agent runs task
- output record review editor
- `pr finish`

With:
- validation gates between steps
- review loops where needed
- deterministic artifact history preserved across the flow

In practical v0.85 terms, this means shipping real HTML pages for the first
editor surfaces, then letting later Zed or other integrations consume the same
artifact and control-plane model.

## Relationship to Gödel and AEE

This editing system must be compatible with later cognitive/runtime layers.

Why that matters:
- Gödel needs structured state and history to reason over work reliably
- AEE needs machine-readable artifacts and validation results to recover from mistakes
- replayable editing and validation history will matter for adaptive execution later

If the editing/control plane remains informal, later reasoning systems will have poor inputs.

So the editing architecture is not separate from Gödel/AEE; it is enabling infrastructure for them.

That is another reason to de-software-development-ify the architecture early. If ADL is meant to become a general-purpose task/execution system, its editing/control plane should describe authored tasks, implementation prompts, execution records, validation, and recovery in general terms rather than baking in assumptions that only make sense for source-code workflows.

## Core Artifact Types

### Structured Task Prompt

Formal role:
- define roadmap/design intent
- capture issue metadata and acceptance criteria
- create GitHub issues deterministically

### Structured Implementation Prompt

Formal role:
- define the implementation brief
- narrow scope to concrete surfaces, validation, and demo/proof requirements
- guide execution precisely

### Structured Output Record

Formal role:
- record what changed
- record validation evidence
- capture remaining issues and next steps
- support review and finish logic

These artifacts should be schema-backed and versioned.

## Source of Truth Model

The editing system must make ownership explicit so artifacts do not drift.

Canonical ownership should be:
- structured task prompt body = canonical issue content
- structured task prompt front matter = canonical machine-readable issue metadata
- structured implementation prompt = canonical implementation brief
- structured output record = canonical execution record
- reconciliation manifest = canonical issue-graph mapping state
- GitHub issue = optional rendered/project-management projection of the tracked structured task prompt, not the authoring source of truth
- `.adl/` = draft workspace and generated intermediate state, not canonical public record

UI surfaces may refer to these artifacts informally as “cards” where that helps usability, but tracked/tooling terminology should remain stable.

## Source of Truth and Precedence

ADL distinguishes **intent** from **state** for issue-graph management.

- **Structured Task Prompt (front matter)** = canonical source of **intent** (proposed mutations)
  - Fields include: `action`, `depends_on`, `supersedes`, `duplicates`, and related graph-mutation intent.

- **Reconciliation Manifest** = canonical source of **state** (accepted graph)
  - Contains resolved issue numbers, applied relationships, and the committed issue-graph mapping.

**Precedence and transitions**
- During `pr create` / `pr start`: prompt intent is validated against manifest state.
- On conflict: the operation must fail or require explicit reconciliation (no silent overrides).
- On success: the manifest is updated to reflect the new canonical state.

This yields a transaction model:
- prompt = proposed transaction
- manifest = committed state

## Proposed Validation Points

### `pr create`
Validate:
- validates that the structured task prompt is locally complete and can be rendered into a GitHub issue (schema, required sections, normalized enums, labels, title/body)
- validates that the authoritative structured task prompt has been promoted into the tracked public record location before issue creation/reconciliation
- validates local/repo references (tracked vs local-only, valid dependencies by number or placeholder)
- template/version mismatch where applicable

### `pr start`
Validate:
- input card structure
- required execution metadata
- required sections
- normalized execution vocabularies
- source issue prompt URL
- source issue prompt link correctness (GitHub issue ↔ structured task prompt mapping)
- consistency between prompt intent and manifest state (no drift)
- authoritative SIP/SOR publication rules where the implementation brief is expected to become public at start-time
- validation-plan completeness
- tracked vs local-only references
- stale worktree/card mismatch
- missing worktree/card pairs where the execution context is incomplete
- template/version mismatch where applicable

### `pr finish`
Validate:
- output card structure
- changed-files summary
- validation evidence presence
- completion state consistency
- no forbidden leakage or malformed artifacts
- stale worktree/card mismatch
- template/version mismatch where applicable
- graph/artifact mismatch before completion

## Validation Scope Model

Schema validation should explicitly separate machine-readable structure from high-value prose.

In the near term, validation should strongly enforce:
- machine-readable headers and metadata
- required section presence
- normalized enums and vocabulary where defined
- required links and references
- version/template compatibility where applicable

Validation should avoid overconstraining high-value prose too early. The goal is to make the workflow reliable without freezing the writing model before the vocabulary and artifact shapes have fully stabilized.

## Immediate New Issues Recommended

### Issue 1

Create validation schemas for:
- tracked issue prompts
- input cards
- output cards

This issue should define:
- required fields
- optional fields
- normalized vocabularies
- validation semantics
- representative examples/fixtures

### Issue 2

Modify `pr.sh` so every pass runs validation:
- `pr create`
- `pr start`
- `pr finish`

This issue should make validation a required part of the workflow, not an optional human habit.

## Additional Near-Term Work

A third likely follow-on issue is artifact normalization or migration.

Once schemas exist, existing issue prompts/cards may need:
- enum normalization
- terminology cleanup
- metadata repair
- template reconciliation

This can follow the first two issues.

Schema adoption will require migration and normalization tooling. Older artifacts may remain valid under legacy versions for some period, and normalization should be incremental rather than blocking. The system should prefer explicit versioning and bounded migration paths over forcing a single disruptive conversion.

## Design Principles

The editing architecture should follow these principles:

### 1. File-backed first
Artifacts should exist as files before they become runtime actions.

### 1a. Draft locally, promote before authority
Drafting in `.adl/` is acceptable and expected. Official lifecycle transitions should promote authoritative artifacts into tracked public records before they become canonical state.

### 2. Validation before transition
No major workflow transition should happen without validation.

### 3. Deterministic artifact lifecycle
The same input artifacts should drive the same workflow transitions.

### 4. Stable repo-local proof surfaces
Validation must rely on repo-local commands, tests, fixtures, or artifacts wherever possible.

### 5. Narrow interfaces
Editors, validators, command execution, and GitHub integration should not bleed into one another unnecessarily.

### 6. Incremental migration
Do not attempt to solve the entire editing system in one milestone leap.

### 7. Domain portability
The control plane should be framed in general task/execution terms and should avoid unnecessary coupling to software-development-specific assumptions, even when the first implementations are repo- and PR-oriented.

## Near-Term Implementation Slice

A strong first landing zone for this architecture is:
- define minimal schemas for structured task prompt, structured implementation prompt, and structured output record headers
- add lint checks for required sections and enum normalization
- wire those checks into `pr start` and `pr finish`
- prepare `pr create` to use the same validation path once that command is promoted into the active workflow

This slice is intentionally narrow. It is enough to reduce real workflow failures without requiring the full long-term editing system to exist immediately.

## Proposed Build Plan

### Phase 1 - v0.85 foundation

Build now:
- tracked issue prompt template and machinery
- stronger input/output card structure
- tracked public record homes for STP, SIP, and SOR artifacts
- schemas for tracked issue prompts, input cards, and output cards
- `pr.sh` validation hooks for every pass
- first real authoring/review surfaces
- explicit normalization of terminology and contracts so the control plane is not defined as software-development-only

### Phase 2 - post-v0.85 workflow hardening

Build next:
- stronger validator implementation
- better normalization/migration tooling
- cleaner issue/card lifecycle utilities
- stronger GitHub integration
- better error messaging and recovery paths

### Phase 3 - Rust control-plane core

Build later:
- Rust implementation of the editing/control-plane core
- `pr start` hardened first as the backbone lifecycle command
- artifact parsing/loading
- workflow orchestration
- validation engine
- lifecycle management
- generalized task/execution binding that can outgrow software-repository assumptions

### Phase 4 - richer UI/editor system

Build on top:
- issue editor
- implementation editor
- output review editor
- GPT-based helpers
- visual state/history surfaces

These editors should treat `.adl/` as working state and tracked task-bundle directories as the publication/canonical layer.

## What Success Looks Like

A successful ADL editing system should allow a user to:
- author an issue prompt in a structured editor
- create the GitHub issue from a file-backed artifact
- start implementation in a validated worktree
- run an implementation pass with a precise input card
- review output artifacts deterministically
- finish the workflow safely
- recover from mistakes without chaos
- preserve canonical task-bundle history for later signing and Gödel/AEE consumption
- detect stale local state before transitions
- detect missing worktree/artifact pairs
- refuse invalid lifecycle transitions on inconsistent graph or artifact state
  - provide a recommended repair path when possible
  - use the same artifact/control-plane model in software-development contexts and in broader task/execution contexts later

And it should do this using artifacts that are:
- schema-backed
- versioned
- replayable
- reviewable
- compatible with later cognitive/runtime systems

## Conclusion

The current `pr.sh` workflow is not a dead end; it is the seed of the editing system.

The correct path is:
- define schemas
- enforce validation in every pass
- keep the workflow file-backed
- treat `pr start` as the near-term backbone of the control plane
- incrementally move lifecycle logic out of brittle bash and into Rust
- avoid locking the architecture to software-development-only assumptions
- put simple editors and GPT tooling on top of that core

This will give ADL a reasonable editing system that supports current milestone work while laying the foundation for a much more reliable authoring, review, and execution architecture that can grow beyond software development.
