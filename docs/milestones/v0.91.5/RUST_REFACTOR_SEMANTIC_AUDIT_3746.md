# v0.91.5 Rust Refactor Semantic Audit

Issue: #3746
Parent: #3745
Captured: 2026-06-17
Status: ready_for_execution_routing

## Summary

This audit refreshes the Rust hotspot evidence for the semantic refactoring
mini-sprint and classifies the current candidate slices by domain boundary,
change-path cost, and proof posture.

The sprint should proceed, but not in the provisional issue order. The current
low-risk order is:

1. `#3748` prompt-template editor boundary
2. `#3749` run-artifact type/schema boundary
3. `#3747` GitHub control-plane transport boundary
4. `#3750` consolidation and no-op review
5. `#3751` sprint closeout

## Evidence

Primary current evidence:

- `bash adl/tools/report_large_rust_modules.sh`
- `docs/milestones/v0.91.5/REFACTOR_SAFETY_BASELINE_3593.md`
- `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md`
- `docs/milestones/v0.91.5/review/REFACTOR_MINI_SPRINT_CODE_REVIEW_2026-06-04.md`
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md`
- `docs/milestones/v0.91.5/CSDLC_PROMPT_EDITOR_SPLIT_3622.md`
- `docs/milestones/v0.91.5/review/OCTOCRAB_REFACTOR_TEMPLATE_AST_INTEGRATION_CHECKLIST_2026-06-14.md`

Tracker note:

- The issue body cites `.adl/reports/manual/rust_module_watch_list.md`, but that
  operator-local path is ignored by Git and is not present in the bound
  worktree. This packet uses the deterministic current-equivalent scan from
  `adl/tools/report_large_rust_modules.sh` as the authoritative refreshed
  tracker surface for execution and review.

## Current Hotspot Snapshot

Fresh rationale-level hotspots from `bash adl/tools/report_large_rust_modules.sh`:

| File | LoC | Current audit disposition |
| --- | ---: | --- |
| `adl/src/cli/pr_cmd/github.rs` | 4,551 | Execute later in the wave with a narrow transport/validation slice. |
| `adl/src/csdlc_prompt_editor.rs` | 2,468 | Execute next; values split already landed, next split is still justified. |
| `adl/src/cli/pr_cmd/finish_support.rs` | 1,922 | Do not widen this sprint; route remediation follow-on. |
| `adl/src/cli/tests/pr_cmd_inline/basics.rs` | 1,752 | No-op in this wave; large proof surface, but not a semantic production-module refactor target. |
| `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` | 1,650 | No-op in this wave; keep as proof surface unless a later test-navigation issue reopens it. |
| `adl/src/cli/run_artifacts_types.rs` | 1,550 | Execute as a bounded artifact-contract split. |
| `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs` | 1,500 | No-op in this wave; large lifecycle test file, but the current sprint targets production boundaries first. |

Supporting observations:

- `#3622` is already closed, so the prompt-template values document layer was
  successfully extracted from `csdlc_prompt_editor.rs`.
- `#3718` and `#3732` are closed, so the GitHub transport and toolkit
  prerequisites named by the sprint are now satisfied.
- `finish_support.rs` is a newly material rationale-level hotspot but is not one
  of the scoped execution children in `#3745`.
- Several rationale-level hotspots are large CLI test files. They should remain
  explicit in the refreshed tracker view, but they do not outrank the scoped
  production refactor boundaries for this mini-sprint wave.

## Semantic Classification

| Surface | Classification | Decision | Why |
| --- | --- | --- | --- |
| `adl/src/csdlc_prompt_editor.rs` | extraction | execute in `#3748` | The file still mixes editor model loading, template rendering, import/export, structure validation, and schema generation after the `values.rs` split. The next seam is still meaningful and behavior-preserving. |
| `adl/src/cli/run_artifacts_types.rs` | extraction | execute in `#3749` | The file mixes artifact declarations, path sanitization, public-contract checks, and reasoning-graph compatibility validation. A contract-validation split is semantically real and does not require schema changes. |
| `adl/src/cli/pr_cmd/github.rs` | extraction | execute in `#3747`, but after safer slices | The file is very large, but it also carries the most operational coupling: typed octocrab transport, PR validation polling, fallback policy, issue edits, and finish-path helpers. It remains a valid sprint target, but not the first one. |
| `adl/src/cli/pr_cmd/finish_support.rs` | defer | route remediation | The file is now above the rationale threshold, but widening this sprint to include it would break the issue wave contract. |
| `adl/src/cli/tests/pr_cmd_inline/basics.rs` | no-op | leave in place | This is a large proof surface, but not a production semantic-boundary target for this mini-sprint. |
| `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs` | no-op | leave in place | The current issue wave does not justify production-surface delay to reorganize finish-path test layout. |
| `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs` | no-op | leave in place | This remains a large lifecycle proof file, but the safer sprint order is to refactor production boundaries first and revisit tests separately if still needed. |
| `adl/src/cli/run_artifacts/` submodules | no-op | leave in place | This area already has concept-named submodules (`runtime`, `cognitive`, `summary`) and is not suffering from anonymous `parts` sprawl. |
| `adl/src/csdlc_prompt_editor/values.rs` | no-op | leave in place | `#3622` already extracted this boundary cleanly; reopening it would duplicate completed work. |

## Approved Child Execution Plan

### `#3748` Refactor prompt-template editor domain boundaries

Approved boundary:

- Extract rendered-card structure/schema behavior from
  `adl/src/csdlc_prompt_editor.rs` into a dedicated concept-named module.

Why this slice:

- The file already has one successful internal split (`values.rs`).
- Structure/schema behavior is internally dense and locally coherent:
  structure-schema loading and writing, Markdown AST structure extraction,
  locked-line checks, and rendered-card structure validation.
- This is a behavior-preserving internal seam that does not require prompt-card
  lifecycle changes.

Callers:

- `adl tooling prompt-template ...`
- `adl-csdlc tooling prompt-template ...`
- prompt-template validator and import/render flows in CLI tooling tests

Proof posture:

- Focused prompt-template Rust tests
- `bash adl/tools/test_csdlc_prompt_editor.sh`
- `python3 adl/tools/test_prompt_template_structure_schemas.py`
- prompt-template schema validation

Rollback:

- Collapse the extracted module back into `csdlc_prompt_editor.rs` without
  changing template semantics or generated card output.

### `#3749` Refactor run-artifact type and schema boundaries

Approved boundary:

- Extract public-contract and artifact-validation helpers from
  `adl/src/cli/run_artifacts_types.rs` while leaving type definitions and schema
  versions behaviorally stable.

Why this slice:

- The file contains a real separation between artifact type declarations and
  contract-enforcement helpers such as public-ref validation, path sanitization,
  and reasoning-graph/upstream-delegation checks.
- The boundary is narrower and lower-risk than broad type regrouping.

Callers:

- run-artifact builders under `adl/src/cli/run_artifacts/`
- runtime artifact emission and validation paths
- reasoning-graph contract checks

Proof posture:

- Focused Rust tests for `run_artifacts_types`
- fixture-stability checks for serialized artifacts
- patch hygiene

Rollback:

- Move the validation helpers back into `run_artifacts_types.rs` without
  changing serialized contract fields or references.

### `#3747` Refactor GitHub control-plane transport boundaries

Approved boundary:

- Extract PR validation status/wait/report behavior from
  `adl/src/cli/pr_cmd/github.rs` into a dedicated concept-named module while
  keeping issue/PR transport behavior stable.

Why this slice:

- The file contains a concentrated PR validation subsystem:
  snapshot models, polling, classification, report generation, and observability
  helpers.
- That seam is narrower and safer than attempting to split all octocrab
  transport or all finish-path behavior in one issue.
- The file remains the highest-risk refactor surface, so it should land after
  the lower-risk `#3748` and `#3749` slices.

Callers:

- `pr.sh finish`
- `pr.sh doctor`
- PR validation, wait, and publication paths in the C-SDLC control plane

Proof posture:

- Focused `github.rs` Rust tests for validation classification and wait/report
  behavior
- C-SDLC owner-lane validation
- patch hygiene

Rollback:

- Inline the validation module back into `github.rs` without changing transport
  behavior or fail-closed policy.

## Revised Execution Order

The provisional order under `#3745` should change from numeric surface order to
risk-ordered execution:

1. `#3746` current audit and tracker refresh
2. `#3748` prompt-template editor structure/schema split
3. `#3749` run-artifact contract-validation split
4. `#3747` GitHub PR validation/wait/report split
5. `#3750` consolidation and no-op review
6. `#3751` closeout

Why revise:

- `#3748` and `#3749` each have narrower, already-demonstrated semantic seams.
- `#3747` is still justified, but its blast radius is larger and it depends on
  maintaining current control-plane GitHub truth.
- `#3750` should review the landed slices and classify the untouched hotspots,
  especially `finish_support.rs`.

## Anti-Pattern Guidance

This sprint should continue rejecting generic `parts` module splitting.

Unsafe patterns for this wave:

- splitting `github.rs` into anonymous transport parts without separating a real
  PR validation concept
- splitting `csdlc_prompt_editor.rs` by file size instead of editor/render/
  structure/schema concepts
- moving artifact structs into generic helper buckets without preserving a clear
  contract boundary

Acceptable patterns:

- concept-named modules such as validation, structure, schema, contracts, or
  public_refs
- one semantic boundary per issue
- rollback paths that only reverse file movement, not behavior

## Problems Captured For Remediation

These are real problems discovered during the audit, but they are not safe to
absorb into this sprint beyond routing and closeout truth:

1. The manual tracker path in the issue body is operator-local and ignored by
   Git, so bound worktrees cannot rely on it as tracked execution truth.
2. `adl/src/cli/pr_cmd/finish_support.rs` is now a rationale-level hotspot and
   is not covered by the scoped child wave.
3. The original prompt-editor follow-on from `#3622` is partially consumed, so
   `#3748` must explicitly target the next boundary instead of restating the old
   values split.

## Non-Claims

- This audit does not claim any of the implementation children are complete.
- This audit does not authorize widening the sprint to include
  `finish_support.rs`.
- This audit does not claim the full Rust suite was run.
- This audit does not change prompt-template lifecycle semantics or artifact
  schema behavior.

## Validation

Commands used for this audit:

- `bash adl/tools/report_large_rust_modules.sh`
- `git diff --check`
