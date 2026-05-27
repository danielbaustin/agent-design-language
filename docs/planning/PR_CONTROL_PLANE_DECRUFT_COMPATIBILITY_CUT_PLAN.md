# PR Control-Plane Decruft And Compatibility Cut Plan

## Status

Planning document for issue `#3394`.

This document records the compatibility cut line for a future `v0.92` cleanup
tranche. It is intentionally plan-only for `v0.91.4`: no compatibility code,
workflow behavior, or historical card support is removed by this issue.

The goal is to stop carrying indefinite executable support for obsolete PR
control-plane shapes while preserving historical records as readable audit
artifacts.

## Source Inputs

- `#3394`: plan PR control-plane decruft and compatibility cut line.
- `adl/tools/pr.sh`
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd_cards/`
- `adl/src/cli/pr_cmd/lifecycle/`
- `adl/src/cli/tooling_cmd/structured_prompt.rs`
- `docs/templates/prompts/current.json`
- `docs/templates/prompts/1.0.0/`
- `docs/templates/CARD_LIFECYCLE_TEMPLATE_TARGETS.md`
- `docs/tooling/adl_pr_cycle_skill.md`
- `docs/planning/DESIGN_TIME_CARD_COMPLETION_PLAN.md`
- `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`
- `docs/milestones/v0.91.4/`

## Decision

ADL should support the current C-SDLC PR lifecycle well, not support every
historical card representation forever.

The supported control-plane baseline is:

- commands: `create`, `init`, `doctor`, `run`, `finish`, and `closeout`
- lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`
- template registry: `docs/templates/prompts/current.json`
- card generation: versioned prompt templates under `docs/templates/prompts/`
- card normalization: card editor skills and the human prompt editor
- execution context: bound issue worktrees, never `main`
- review: bounded pre-PR review before publication
- closeout: explicit closeout after merge or intentional closure

Historical card records should remain readable. They do not require every old
workflow alias, template fallback, or shell generation path to remain executable
after a reviewed migration window.

## Cut-Line Principle

The compatibility cut line separates two responsibilities:

- **Audit preservation:** old records remain in Git history or tracked archives
  where appropriate and can be read by humans.
- **Executable workflow support:** only the current C-SDLC lifecycle and one
  reviewed transition window need to remain actively runnable.

This keeps the PR control plane deterministic instead of letting old
compatibility paths become a second process.

## Current Supported Surface

The supported workflow after the cut line should be:

1. `pr create` or `pr init` creates issue-local C-SDLC records from the active
   prompt-template registry.
2. `pr doctor` validates all five cards and blocks execution until `SIP`,
   `STP`, and `SPP` are design-time ready.
3. `workflow-conductor` routes lifecycle and card defects to the appropriate
   skill instead of improvising fixes.
4. `pr run` binds the issue to a dedicated worktree and branch.
5. `pr finish` validates the intended artifact set and opens or updates the PR.
6. `pr-closeout` records issue, card, artifact, and GitHub closure truth after
   merge or intentional closure.

The supported card names are only:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

`SRP` means Structured Review Prompt. `SOR` is the execution and integration
truth record.

## Compatibility Inventory

### Remove Or Hide First

These are low-risk because they are primarily documentation, help, or visible
terminology cleanup:

- `READ` / `WRITE` language in current help, tests, docs, or examples.
- "input card" / "output card" wording where it is presented as current
  canonical language.
- stale references that make `adl/templates/cards/input_card_template.md` or
  `adl/templates/cards/output_card_template.md` sound canonical.
- stale `Structured Review Policy` references where current workflow means
  `Structured Review Prompt`.
- current docs that imply a three-card lifecycle rather than the five-card
  C-SDLC lifecycle.

### Keep Temporarily Behind A Migration Window

These may still be needed by old records, stacked branches, or closeout paths
and should not be deleted until an active-bundle scan proves they are safe:

- legacy SIP/SOR template fallback paths in shell and Rust code
- old `adl/templates/cards/` compatibility references
- old `.adl/templates/` compatibility references
- historical task bundles created before the five-card lifecycle
- older task-bundle editor output shapes
- any compatibility path used only to read old records during closeout

### Replace With One Canonical Implementation

These should become single-owner surfaces:

- Rust owns template registry loading from `docs/templates/prompts/current.json`.
- Rust owns card field models, enum vocabularies, and validation rules.
- Rust owns canonical card rendering and bootstrap behavior.
- `pr.sh` remains a thin orchestration and compatibility wrapper, not a second
  card generator.
- Python helper scripts may call the canonical path, but should not define card
  semantics independently.

### Remove After Migration Checks

These are deletion candidates after the migration window and fixtures pass:

- duplicate shell card-generation paths
- stale command aliases such as `start`, `ready`, or `preflight` if they remain
  as executable aliases rather than documented historical names
- old `READ` / `WRITE` contract tests
- compatibility template fallbacks that no active or closeout-supported record
  still needs
- legacy editor output branches that can emit invalid modern SOR/SRP shapes

## Proposed v0.92 Cleanup Tranche

### Slice 1: Visible Terminology Cleanup

Remove or qualify stale current-facing docs and help text.

Acceptance:

- current docs name only the five-card C-SDLC lifecycle
- `SRP` is consistently Structured Review Prompt
- old input/output and `READ`/`WRITE` terms appear only as historical notes

### Slice 2: Active-Bundle Compatibility Scan

Add or run a bounded scan over active issue bundles and closeout-sensitive
records to identify any remaining executable dependency on legacy paths.

Acceptance:

- scan output separates active, historical, and unknown references
- no deletion proceeds while active references remain unknown
- historical-only references are routed to readability, not executable support

### Slice 3: Rust-Owned Template Registry Enforcement

Move card bootstrap and rendering paths behind the active prompt-template
registry.

Acceptance:

- Rust loads `docs/templates/prompts/current.json`
- Rust uses one template loader/replacer for `SIP`, `STP`, `SPP`, `SRP`, and
  `SOR`
- generated cards validate without relying on shell-specific card text

### Slice 4: Thin `pr.sh`

Reduce `adl/tools/pr.sh` to orchestration, environment checks, and calls into
the canonical Rust/control-plane path.

Acceptance:

- shell no longer independently defines card structure
- shell help points to the current C-SDLC lifecycle
- shell remains useful for operator ergonomics and compatibility diagnostics

### Slice 5: Delete Legacy Fallbacks

Remove compatibility paths that the scan and fixtures prove are no longer
needed.

Acceptance:

- deletion PR names each removed fallback
- tests prove current issue creation, doctor, run, finish, and closeout still
  work
- old records remain readable even when old execution paths are gone

### Slice 6: Regression Fixtures

Add focused fixtures for the supported workflow instead of relying on broad
full-suite runs for every cleanup slice.

Acceptance:

- fixture proves modern `create` / `init` creates five cards from the active
  template registry
- fixture proves `doctor` blocks incomplete design-time cards
- fixture proves `run` binds a clean issue worktree
- fixture proves `finish` refuses local-only or stale integration truth
- fixture proves closeout can record terminal truth without legacy card names

## Validation Policy

The cleanup tranche should use focused validation, not reflexive full test
cycles for every docs/tooling slice.

Recommended validation:

- `git diff --check`
- structured prompt validation for affected cards or fixtures
- targeted PR-control-plane tests for card bootstrap and doctor readiness
- targeted finish/closeout tests when integration truth changes
- sprint-conductor readiness helper tests when sprint behavior changes
- Markdown or link checks for docs-only terminology slices

Full Rust or release-scale validation should be reserved for slices that change
runtime behavior broadly enough to justify it.

## Non-Regression Rules

The cleanup must preserve:

- issue work never happens on `main`
- conductor routing remains mandatory
- card edits route through editor skills or the human prompt editor
- `SIP`, `STP`, and `SPP` must be ready before execution binding
- `SRP` must not claim review results before review runs
- `SOR` must not claim execution, PR, merge, or closeout truth before it exists
- sprint-conductor preflight must still block on child-card readiness
- historical records remain inspectable even when old execution paths are gone

## Risks

- Removing fallbacks too early can strand old closeout or review records.
- Leaving fallbacks indefinitely preserves ambiguity and makes the current
  process harder to trust.
- Letting shell and Rust both define card structure can reintroduce drift.
- Treating historical readability as executable compatibility can make the
  control plane harder to stabilize.

## Decision Gates

Before deletion begins in `v0.92`:

1. This plan is reviewed and accepted.
2. The active-bundle compatibility scan is complete.
3. The current lifecycle fixtures pass.
4. The cleanup slice has a bounded issue with its own cards and focused proof.
5. Review confirms the slice does not widen into unrelated tooling cleanup.

## Deferred Implementation

Implementation is deferred to `v0.92`.

This issue only establishes the plan and compatibility cut line. It does not
remove code, change command behavior, delete template fallbacks, or rewrite
historical records.
