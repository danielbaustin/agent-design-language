# PR Tooling Simplification Architecture

## Overview

- Topic: simplify and shrink the PR lifecycle tooling
- Milestone Target: `v0.87`
- Date: `2026-03-31`
- Status: `Planning`
- Scope: `adl/tools/pr.sh`, `adl/tools/card_paths.sh`, and Rust `adl pr`

## Why This Document Exists

`adl/tools/pr.sh` has grown from a convenience wrapper into a large workflow
control plane.

Today the behavior is split across:
- shell orchestration in `adl/tools/pr.sh`
- shell path and compatibility logic in `adl/tools/card_paths.sh`
- partially migrated Rust control-plane logic in `adl/src/cli/pr_cmd.rs`
- shared Rust path/domain logic in `adl/src/control_plane.rs`

This makes the tooling harder to understand, harder to test, and harder to
change safely because the same workflow concepts now exist in more than one
implementation.

The goal of this design is to make the system smaller by making ownership
clearer.

This work is planned for `v0.87` as part of the next milestone's
operational/control-plane consolidation.

## Current State Snapshot

Observed in the repository as of `2026-03-31`:

- `adl/tools/pr.sh` is about `3113` lines.
- `adl/tools/card_paths.sh` is about `320` lines.
- `adl/src/cli/pr_cmd.rs` is about `5071` lines.
- `pr.sh` already delegates several lifecycle commands to Rust when available:
  - `create`
  - `init`
  - legacy alias `start`
  - `ready`
  - `preflight`
  - `finish`
- `create` no longer has a real shell fallback.
- `run`, `card`, `output`, `cards`, `status`, and `open` still live in shell.
- path/domain logic already exists in both shell and Rust.

Interpretation:

- the long-term direction has already been chosen informally: Rust is becoming
  the owner of the PR control plane
- the current architecture still pays the complexity cost of both systems
- the biggest source of accidental complexity is duplicated ownership, not just
  raw line count

## Primary Problems

### 1. Split-Brain Ownership

The same concepts are implemented in both shell and Rust:

- issue normalization
- slug normalization
- task-bundle path rules
- worktree conventions
- card location rules
- workflow bootstrapping behavior

This increases drift risk and makes every change more expensive.

### 2. Shell Owns Too Much Product Logic

The shell script is not just invoking tools. It also owns:

- repository state transitions
- worktree safety policy
- bootstrap repair behavior
- markdown mutation
- prompt and card generation flow
- validation and readiness checks

That is product logic, not glue.

### 3. Command Surface Exposes Internal Workflow Steps

The current interface includes:

- `create`
- `init`
- legacy alias `start`
- `run`
- `card`
- `output`
- `cards`
- `ready`
- `preflight`
- `finish`
- `open`
- `status`

Several of these are internal authoring phases rather than user-level intents.
This makes the tool feel bigger than it needs to be and forces the
implementation to preserve many intermediate states.

### 4. Shell Is Doing Structured Data Work

`pr.sh` uses `awk`, `sed`, temp files, and ad hoc parsing for document
manipulation and validation.

That style is workable for small wrappers, but brittle for a control plane with
many invariants and compatibility rules.

## Design Goals

The rearchitecture should:

- make `pr.sh` tiny enough to reason about in one screen
- make Rust the single owner of PR lifecycle behavior
- reduce the public command surface to a few user-facing intents
- centralize path rules and workflow invariants in typed code
- preserve backward compatibility during migration
- avoid a risky one-shot rewrite

## Non-Goals

This design does not require:

- eliminating shell entrypoints immediately
- changing the underlying git and `gh` tools
- redesigning ADL cards, STP, SIP, or SOR formats
- rewriting demo tooling that is not part of the PR lifecycle

## Proposed Target Architecture

### External Shape

The desired user-facing model is:

- `adl pr init`
- qualitative STP/SIP review before execution
- `adl pr run`
- review / closeout
- `adl pr doctor` as an orthogonal diagnostic and bounded-repair surface

Workflow meanings:

- `init`
  - full mechanical bootstrap for a planned issue
  - create or reconcile the issue record, source prompt, and root STP/SIP/SOR bundle
- qualitative review
  - humans or review skills refine STP and SIP before execution
- `run`
  - create or reuse branch/worktree at the last responsible moment
  - sync the prepared task bundle into the execution context
  - execute the task, write the SOR, and open the PR
- review / closeout
  - review the SOR and PR before merge or issue closure
- `doctor`
  - report readiness and repairable workflow drift
  - remains available throughout the lifecycle rather than being a separate workflow phase

### Compatibility Shape

During migration:

- `adl/tools/pr.sh` remains as the compatibility entrypoint
- it delegates directly to `adl pr ...`
- retired commands remain as aliases temporarily
- deprecated commands should print a short migration hint

Examples:

- `create` remains as a compatibility path while bootstrap semantics settle,
  but the long-term public model should collapse new-issue bootstrap into `init`
- `ready` and `preflight` become aliases for `doctor`
- `card`, `output`, and `cards` become either hidden maintenance commands or
  internal Rust helpers rather than prominent user commands

## Ownership Model

### Shell Ownership

Shell should own only:

- locating the repo root
- selecting a cached Rust binary when available
- falling back to `cargo run`
- preserving old command entrypoints during transition

Shell should not own:

- workflow policy
- path construction
- worktree safety rules
- card generation
- prompt generation
- PR body generation
- readiness validation
- repair logic

### Rust Ownership

Rust should own:

- all PR lifecycle state transitions
- all issue/task-bundle/card path rules
- all prompt and card rendering
- all readiness and bootstrap validation
- all git and `gh` orchestration wrappers
- all deprecation and compatibility policy

## Proposed Module Structure

Recommended internal Rust split:

- `control_plane`
  - `IssueRef`
  - slug normalization
  - primary checkout resolution
  - cards root and task-bundle path logic
- `cli/pr/args`
  - argument parsing and command-specific option structs
- `cli/pr/github`
  - `gh issue/pr` wrappers
- `cli/pr/git`
  - fetch, branch, worktree, stage, and ahead-of-main helpers
- `cli/pr/bootstrap`
  - ensure source issue prompt
  - ensure root/worktree STP
  - ensure input/output cards
- `cli/pr/render`
  - generated issue bodies
  - generated prompts
  - generated PR bodies
- `cli/pr/doctor`
  - readiness inspection
  - bounded repair logic
- `cli/pr/commands`
  - `init`
  - `run`
  - `finish` during transition where explicit closeout remains separate
  - `doctor`
  - compatibility shims for `create`, `ready`, and `preflight`

The main architectural rule is:

- domain rules and helpers should be reusable library code
- command handlers should be thin orchestration layers

## Simplified Control Flow

### `create`

1. Parse title, labels, optional body inputs.
2. Normalize slug and version.
3. Create GitHub issue.
4. Write canonical local source issue prompt.
5. Bootstrap the root STP, SIP, and SOR bundle.
6. Print next-step guidance for qualitative review.

### `init`

1. Resolve existing issue title, slug, and version.
2. Ensure the canonical source prompt exists.
3. Ensure the root task-bundle surfaces exist.
4. Stop before branch and worktree creation.
5. Print next-step guidance for qualitative review.

### `run`

1. Resolve issue title, slug, and version.
2. Validate milestone-wave policy.
3. Ensure branch and worktree.
4. Ensure canonical source prompt exists.
5. Sync the prepared root task bundle into the worktree-local execution context.
6. Validate authored readiness for execution.
7. Execute the task and write the SOR.
8. Open or update the PR.

### `doctor`

1. Resolve target issue context.
2. Inspect source prompt, task bundle, cards, branch, and worktree.
3. Report:
   - ready
   - ready with repairs
   - blocked
4. Apply only small bounded repairs when safe.

## Specific Simplifications

### 1. Collapse New-Issue Bootstrap Into `init`

The current `create` and `init` split is mostly mechanical overhead when the
slug, title, labels, and version are already known.

Recommended rule:

- `init` becomes the public bootstrap command
- new-issue creation is one mode of `init`, not a separate long-term workflow step

Optional compatibility:

- keep `create` as a deprecated or expert-level compatibility alias during transition

### 2. Replace `ready`, `preflight`, and `status` With `doctor`

These commands are all variants of readiness inspection.

Recommended rule:

- one inspection command
- optional modes for terse or detailed output

This reduces both implementation branching and mental overhead for users.

#### `preflight` Consolidation Decision

`preflight` should be absorbed into `doctor`.

Recommended transition:

- `doctor` becomes the canonical readiness command
- `preflight` remains temporarily as a deprecated alias to `doctor`
- `ready` also becomes a deprecated alias to `doctor`
- docs and examples stop teaching `preflight` as a first-class command

Rationale:

- `preflight` asks whether execution can safely proceed
- `ready` asks whether the workflow surfaces are execution-ready
- `status` reports the current workflow state

These are different slices of the same underlying concern:

- inspect workflow health
- report readiness
- identify drift
- optionally apply small safe repairs

That combined concern is a better fit for one command than three.

#### Suggested `doctor` Modes

To preserve useful `preflight` behavior without preserving a separate command,
`doctor` can expose explicit modes:

- `adl pr doctor`
  - default human-readable readiness report
- `adl pr doctor --strict`
  - exit non-zero on any blocking readiness issue
- `adl pr doctor --json`
  - machine-readable readiness output
- `adl pr doctor --repair`
  - apply only bounded mechanical repairs that are clearly safe

This preserves the inspection use cases while simplifying the command model.

### 3. Demote `card`, `output`, and `cards`

These are useful maintenance operations, but they should not define the public
shape of the workflow.

Recommended rule:

- keep them as internal helpers or expert-level subcommands
- do not treat them as core lifecycle steps

### 4. Eliminate Duplicate Path Logic

`adl/tools/card_paths.sh` and `adl/src/control_plane.rs` should not both be
authoritative.

Recommended rule:

- Rust owns all canonical path logic
- shell wrappers call Rust instead of reimplementing path behavior

### 5. Remove Structured Markdown Mutation From Shell

All document rewriting should happen in Rust where:

- invariants can be typed
- tests can be more precise
- parsing and rewriting can be centralized

## Minimal End-State For `pr.sh`

The desired shell entrypoint should look conceptually like:

```bash
#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="$ROOT/adl/target/debug/adl"

if [[ -x "$BIN" ]]; then
  exec "$BIN" pr "$@"
fi

exec cargo run --quiet --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- pr "$@"
```

Small additions are acceptable for compatibility aliases and friendly error
messages, but `pr.sh` should no longer implement lifecycle behavior itself.

## Migration Plan

### Phase 1: Freeze New Shell Logic

Policy:

- do not add new business logic to `adl/tools/pr.sh`
- all new PR lifecycle behavior goes into Rust

Success criteria:

- shell line count stops growing
- new behavior lands only in Rust

### Phase 2: Finish Rust Ownership Of Existing Lifecycle Commands

Rust becomes the sole implementation for:

- `create`
- `init`
- `run` issue-mode binding
- `finish`
- `ready`
- `preflight`

Then remove shell fallbacks for:

- `init`
- `run` issue-mode binding
- `finish`
- `ready`
- `preflight`

Success criteria:

- lifecycle behavior works with Rust-only implementations
- shell only dispatches

### Phase 3: Port Remaining Shell Commands Or Retire Them

Decide for each remaining shell-owned command:

- `run`
  - keep issue-mode binding as the public lifecycle surface
  - keep the ADL runtime convenience path only if it remains genuinely distinct
- `card`
- `output`
- `cards`
- `status`
- `open`

Recommended outcomes:

- `open` may remain trivial shell or move to Rust
- `status`, `ready`, and `preflight` converge into `doctor`
- `card`, `output`, and `cards` move to Rust and become secondary commands
- `run` keeps one clear issue-mode lifecycle meaning for branch/worktree binding

### Phase 4: Collapse Public Command Surface

Public docs and examples should converge on:

- `init`
- qualitative review
- `run`
- review / closeout
- `doctor`

Compatibility aliases remain temporarily.

Success criteria:

- new users can learn the workflow from four clear steps plus one diagnostic command
- implementation no longer revolves around historical subcommand sprawl

### Phase 5: Shrink Or Remove Shell Compatibility Layer

After callers and docs have moved:

- reduce `adl/tools/pr.sh` to a tiny shim
- optionally replace it with a stable thin wrapper permanently

Success criteria:

- `pr.sh` is small and boring
- Rust is the only owner of lifecycle behavior

## Testing Strategy

Prefer tests at three levels:

- unit tests
  - slug normalization
  - path resolution
  - branch/worktree naming
  - prompt rendering
  - PR body rendering
- integration tests
  - fake `git` and `gh` process boundaries
  - full `init`, `run`, compatibility-shim, and `doctor` flows
- compatibility tests
  - `adl/tools/pr.sh ...` still maps correctly to Rust commands

Important rule:

- parity tests should validate behavior, not shell implementation details

## Rollout Policy

Behavior changes should be intentional and visible.

Recommended policy:

- keep old commands as aliases before removing them
- print migration hints for deprecated commands
- keep direct skill-authoring detail in a separate planning doc so this document stays architectural
- update docs and examples in parallel with code changes
- prefer one command-shape change per PR wave rather than a giant cutover

Suggested readiness-command alias policy:

- `preflight` invokes `doctor` and prints a deprecation note
- `ready` invokes `doctor` and prints a deprecation note

## Risks

### Risk: Rust Becomes A Bigger Monolith

Moving logic to Rust is not enough by itself.

Mitigation:

- split by domain modules
- keep command handlers thin
- move reusable rules into library code, not giant command files

### Risk: Shell And Rust Drift During Migration

Mitigation:

- freeze new shell business logic
- remove shell fallbacks as soon as Rust parity is proven

### Risk: User Workflow Breakage

Mitigation:

- compatibility aliases
- migration hints
- phased documentation updates

## Decision Summary

Recommended decisions:

- Rust is the long-term owner of PR tooling behavior.
- `adl/tools/pr.sh` becomes a compatibility shim, not a workflow engine.
- `start` should be retired from the taught public lifecycle and retained only as a narrow legacy alias while callers migrate.
- `ready`, `preflight`, and `status` should converge into `doctor`.
- `card`, `output`, and `cards` should be demoted from core lifecycle commands.
- canonical path and workflow rules must have exactly one implementation owner.

## Definition Of Success

This effort is successful when:

- `pr.sh` is tiny and contains almost no business logic
- users mostly learn and use four main commands
- Rust owns all lifecycle rules and state transitions
- path logic exists in one authoritative implementation
- the command surface is easier to remember and the code is easier to change
