# Tooling Rust Migration Plan (v086planning)

## Metadata
- Milestone: `v086planning`
- Topic: Migrate high-risk workflow tooling out of shell into Rust
- Date: `2026-03-20`
- Status: `Planning`
- Owner: `Daniel Austin / Agent Logic`

## Why This Work Exists

ADL's workflow tooling has accumulated a large amount of control-plane logic in
shell scripts under `swarm/tools/`.

That logic now includes:
- PR lifecycle orchestration
- worktree safety and branch policy
- task-bundle and compatibility-path resolution
- card generation and prompt generation
- structured prompt validation
- editor-adjacent workflow adapters

This is no longer just "glue code." It is real product logic with repository
state transitions, validation rules, and compatibility behavior.

At the moment, that logic is still primarily implemented in Bash.

## Current State Snapshot (2026-03-20)

Key observations from the current tree:
- `swarm/tools/pr.sh` is still the central orchestration surface and is `1677`
  lines long.
- `swarm/tools/card_paths.sh` has grown to `301` lines and now includes
  canonical task-bundle path logic plus compatibility-link behavior.
- `swarm/tools/validate_structured_prompt.sh` is a new shell validator and now
  owns contract-validation behavior that previously lived in Ruby.
- `swarm/tools/editor_action.sh` and `swarm/tools/sync_task_bundle_prompts.sh`
  were added as new shell workflow/control-plane helpers.
- the Rust runtime crate does not currently expose a dedicated tooling CLI for
  `pr`, card/task-bundle path management, prompt validation, or prompt
  generation.

Interpretation:
- there has not yet been a meaningful migration of this tooling into Rust
- the control-plane surface has expanded
- task-bundle architecture has made the shell layer more important, not less
- migration pressure is therefore higher than before

## Executive Summary

Recommendation:
- create a Rust tooling control plane as a first-class surface
- move shared parsing, path, template, and validation logic before moving the
  full PR orchestration flow
- keep shell scripts as compatibility wrappers during migration
- leave demos, mocks, and smoke harnesses in shell unless they become painful

Recommended target shape:
- Rust library modules for:
  - task-bundle and compatibility path resolution
  - card and structured-prompt parsing
  - prompt-spec lint and validation
  - prompt rendering
  - git/gh execution adapters
- Rust CLI subcommands for:
  - `validate`
  - `prompt`
  - `pr card`
  - `pr output`
  - `pr cards`
  - `pr start`
  - `pr finish`
  - later: `codexw` and editor control-plane actions

Recommended migration style:
- do not rewrite all tooling in one branch
- keep behavioral parity visible with existing shell tests
- replace low-risk deterministic pieces first
- move stateful git/gh flows only after the shared core is proven

## Goals

### Primary Goals

- reduce maintenance risk in the workflow control plane
- remove parsing and validation logic from shell where possible
- centralize task-bundle and compatibility-path rules in typed Rust code
- preserve current user-facing workflow behavior during migration
- make future task-bundle/editor workflow work easier to implement safely

### Secondary Goals

- improve cross-platform behavior
- improve testability of validation and path logic
- make `pr.sh` smaller until it becomes a wrapper or is removed entirely

## Non-Goals

- rewriting every demo or mock script into Rust
- changing the workflow model at the same time as the migration
- renaming repo paths or broader identity cleanup as part of this effort
- redesigning all review tooling in one pass

## Scope

### In Scope

- shared task-bundle path logic
- compatibility-link behavior for `.adl/cards/<issue>/...`
- structured prompt validation
- prompt-spec linting
- deterministic prompt generation
- PR/card workflow commands
- codex workflow wrapper logic after the shared core is stable

### Out Of Scope

- demo scripts that just call existing commands
- provider mocks
- one-off coverage/open convenience wrappers
- broad doc cleanup beyond what is needed to explain the new tooling path

## Migration Principles

### 1. Shared Logic Before Orchestration

`pr.sh` should not be the first file ported directly line-for-line. The first
move should be shared Rust library code for:
- path resolution
- slug and issue normalization
- card metadata extraction
- validation helpers
- prompt rendering

This avoids re-porting the same logic multiple times.

### 2. Shell Wrappers During Transition

During migration:
- shell entrypoints remain available
- shell scripts delegate to Rust subcommands where parity exists
- behavior changes should be intentional and documented

This lowers rollout risk and preserves contributor muscle memory.

### 3. Deterministic Surfaces First

The first Rust slices should be deterministic, side-effect-light tools:
- validation
- linting
- prompt generation
- path resolution

These are easier to unit test and less risky than git/gh workflows.

### 4. Keep Harnesses Cheap

Demos, mocks, and shell smoke tests should remain lightweight unless they block
development. They are not the main maintenance burden.

## Proposed Architecture

Recommended internal split:

- `tooling::paths`
  - task id normalization
  - slug validation
  - cards root resolution
  - task-bundle directory and compatibility-path resolution
  - migration helpers

- `tooling::cards`
  - top-level metadata extraction
  - markdown section extraction
  - task-bundle artifact references

- `tooling::validation`
  - STP/SIP/SOR validation
  - prompt-spec linting
  - completed-output checks

- `tooling::prompt`
  - prompt-spec interpretation
  - deterministic prompt rendering

- `tooling::gitops`
  - wrapper for `git` and `gh` command execution
  - worktree inspection
  - branch safety helpers

- `tooling::cli`
  - command wiring and error presentation

Recommended external shape:
- `adl tools validate ...`
- `adl tools prompt ...`
- `adl tools pr ...`
- later:
  - `adl tools codexw ...`
  - `adl tools editor-action ...`

## Phase Plan

### Phase 0: Decision Lock And Command Shape

Accept these decisions before implementation:
- Rust is the long-term owner of workflow control-plane logic
- shell remains as compatibility entrypoints during migration
- demos and mocks are not first-wave migration targets
- task-bundle path logic becomes a shared library, not embedded per-command

Deliverables:
- agreed command namespace
- agreed compatibility policy
- agreed first PR slice

### Phase 1: Path And Card Core

Port first:
- `swarm/tools/card_paths.sh`

Include:
- cards root resolution
- task-bundle root resolution
- issue normalization and zero-padding
- task-id construction
- task-bundle path construction
- compatibility-link behavior
- legacy-card migration helpers

Success criteria:
- Rust library reproduces current path outputs
- existing shell tests can be adapted to validate Rust-backed behavior
- no git/gh side effects required yet

### Phase 2: Validation Core

Port next:
- `swarm/tools/validate_structured_prompt.sh`
- `swarm/tools/lint_prompt_spec.sh`

Include:
- STP/SIP/SOR validation
- prompt-spec key and section checks
- absolute-host-path guardrails
- completed-phase validation rules

Success criteria:
- deterministic fixture parity with current validation tests
- shell validator can become a thin wrapper around the Rust validator

### Phase 3: Prompt Generation

Port next:
- `swarm/tools/card_prompt.sh`

Include:
- prompt-spec parsing
- section ordering
- section inclusion/exclusion flags
- deterministic output formatting

Success criteria:
- prompt generation is fully Rust-owned
- current prompt tests continue to pass with wrapper-based invocation

### Phase 4: Card Command Flow

Port next:
- `pr card`
- `pr output`
- `pr cards`

This should reuse the path/card/template/validation modules from earlier phases.

Success criteria:
- shell `pr.sh` can delegate these subcommands to Rust
- task-bundle and compatibility-link behavior is unchanged

### Phase 5: `pr start`

Port next:
- worktree path checks
- branch creation/reuse logic
- template seeding
- bootstrap validation
- primary-checkout safety behavior

Success criteria:
- temp-repo integration tests cover the current idempotent and collision paths
- shell `pr.sh start` becomes a wrapper or compatibility fallback

### Phase 6: `pr finish`

Port last among core `pr` commands:
- output-record validation
- staging rules
- idempotency fingerprinting
- commit/push/PR create-or-update behavior
- ready/merge modes

This is the highest-risk slice and should only happen after earlier phases are
stable.

Success criteria:
- fixture and temp-repo integration coverage for:
  - relative card-path handling
  - no-change behavior
  - idempotent merged-PR skip
  - draft/update flows

### Phase 7: Codex And Editor Adapters

Reassess and migrate:
- `swarm/tools/codexw.sh`
- `swarm/tools/codex_pr.sh`
- `swarm/tools/editor_action.sh`
- possibly remove `swarm/tools/sync_task_bundle_prompts.sh` once canonical
  writes happen directly

Goal:
- reduce the number of shell-only workflow adapters
- keep only thin compatibility wrappers where they still add value

## First PR Recommendation

The best first implementation slice is:

1. add a Rust `tooling::paths` module
2. add a Rust `tooling::validation` module for STP/SIP/SOR validation
3. add a new Rust CLI entrypoint for `validate`
4. change `swarm/tools/validate_structured_prompt.sh` to delegate to the Rust
   binary
5. keep current shell tests and fixture tests, updating them only as needed for
   the new entrypoint

Why this first:
- low side-effect risk
- high leverage
- immediate reduction in shell parsing complexity
- creates the shared foundation for later `pr` migration

## Recommended Review Checkpoints

These should be treated as explicit review gates:

- path parity:
  - canonical task-bundle locations
  - compatibility `.adl/cards` links
  - legacy-card migration behavior

- validation parity:
  - bootstrap SIP/SOR
  - completed SOR
  - prompt-spec lint rules

- prompt parity:
  - section ordering
  - omitted sections
  - stable output references

- PR workflow parity:
  - worktree reuse
  - branch collision failure
  - dirty primary-checkout guardrails
  - relative card path handling in finish flow

## Risks

### 1. Silent Behavioral Drift

The shell tooling has many small behavioral contracts that are not all obvious
from help text alone.

Mitigation:
- keep existing shell tests
- add Rust unit tests and temp-repo integration tests
- migrate one command family at a time

### 2. Over-Bundled Refactor

Trying to migrate `pr.sh`, validators, and Codex wrappers all at once will slow
review and increase rollback risk.

Mitigation:
- phase the work
- keep first PR small and deterministic

### 3. Command Churn For Contributors

Changing contributor commands too early would create confusion.

Mitigation:
- preserve shell entrypoints during the transition
- document the Rust-backed path after parity is proven

### 4. Transitional Compatibility Surfaces Becoming Permanent

The task-bundle sync and compatibility-link layers should not become indefinite
architecture by accident.

Mitigation:
- explicitly mark transitional scripts as transitional
- remove them once the Rust path writes canonical outputs directly

## Success Criteria

This migration should be considered successful when:
- validation and prompt generation are Rust-owned
- path and task-bundle rules are Rust-owned
- `pr card/output/cards/start/finish` are Rust-owned or Rust-first
- shell tooling is reduced to wrappers, demos, mocks, and lightweight harnesses
- contributor workflow remains stable during the cutover

## Bottom Line

This work should be approached as a control-plane extraction, not a generic
"rewrite the scripts" exercise.

The right order is:
- paths
- validation
- prompt generation
- low-risk `pr` subcommands
- `pr start`
- `pr finish`
- Codex/editor adapters

If done in that order, the migration reduces risk quickly and sets up the
task-bundle/editor architecture on a more durable foundation.
