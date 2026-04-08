# Tooling Documentation

This directory is the main entrypoint for ADL tooling guides, prompt-spec references, reviewer surfaces, editor-related proof surfaces, and maintainability utilities.

The goal of this directory is to make ADL’s tooling surfaces understandable and navigable without forcing the reader to learn the entire internal workflow system first.

## Start Here

- Prompt-spec and structured prompt surfaces: `prompt-spec.md`
- Structured prompt contracts: `structured-prompt-contracts.md`
- Default contributor workflow: `../default_workflow.md`
- Editor and authoring proof surfaces: `editor/README.md`
- Root project overview: `../README.md`

## Core Tooling Areas

### Prompt and Card Surfaces

These docs describe the structured prompt surfaces used to shape issues, input cards, output cards, and deterministic reviewer flows.

- [Prompt Spec](prompt-spec.md)
- [Structured Prompt Contracts](structured-prompt-contracts.md)
- [Prompt/Reviewer Surface Mapping](prompt-review-surface-mapping.md)
- [Prompt Spec Protocol Bindings](prompt-spec.md#protocol-bindings)
- [Issue Prompt Templates](issue-prompts/README.md)

### Reviewer and Validation Surfaces

These docs describe bounded reviewer behavior, deterministic output formats, and provenance/review validation surfaces.

- [Reviewer Surface](reviewer-surface.md)
- [Reviewer Output Provenance](reviewer-provenance.md)
- [Card Reviewer GPT Instructions](card-reviewer-gpt.md)
- [Deterministic Review Output Format](card-review-output-format.md)

Stable reviewer regression fixture:
- `docs/tooling/examples/reviewer-regression/issue-661/input_661.md`
- `docs/tooling/examples/reviewer-regression/issue-661/output_661.md`
- `docs/tooling/examples/reviewer-regression/issue-661/expected_review_output_661.yaml`

### Editor and Authoring Surfaces

These docs describe the bounded editor and authoring surfaces used in the v0.85 authoring/control-plane work.

- [Task Bundle Editor](editor/README.md)
- `editor/five_command_demo.md`
- `editor/five_command_regression_suite.md`

### Worktree and Maintainability Surfaces

These docs describe worktree governance, large-module tracking, and related maintenance guidance.

- [Worktree Governance](worktree_governance.md)
- [Rust Module Watch List](rust_module_watch_list.md)
- [Public Task Records](../records/README.md)

## Tooling Scripts and Utilities

Important repo-local tooling surfaces include:

- `adl/tools/pr.sh doctor` / `adl pr doctor` — canonical PR readiness and drift diagnostics
- `adl/tools/pr.sh run` / `adl pr run` — canonical execution-context binder
- `adl/tools/pr.sh finish` / `adl pr finish` — canonical publication / PR open-update path
- `adl tooling lint-prompt-spec` — Prompt Spec lint and validation
- `adl tooling card-prompt` — deterministic prompt generation from cards
- `adl tooling validate-structured-prompt` — structured prompt contract validation
- `adl tooling verify-review-output-provenance` — provenance verification for review-output artifacts
- `adl tooling review-card-surface` — bounded deterministic review helper
- `adl/tools/*.sh` wrappers remain available as compatibility entrypoints over the Rust-owned commands
- `adl/tools/report_large_rust_modules.sh` — non-blocking Rust implementation-module size report
- `adl/tools/sync_task_bundle_prompts.sh` — refresh canonical local task-bundle prompt layout from compatibility paths

Deprecated compatibility aliases such as `pr ready`, `pr preflight`, and
`pr start` may still appear in older tests or docs, but they are not the
preferred public control-plane surface.

## Current Status

- Current closure milestone: **v0.85**
- Next active milestone: **v0.86**
- Role of this directory: tooling/reference entrypoint for prompt, reviewer, editor, and maintenance surfaces

## Notes

Tooling docs should be read as bounded engineering references. They describe the surfaces that support ADL authoring, review, and maintenance without claiming that every internal helper is equally important to every reader.

## Current Retirement Boundary

The active `adl/tools` surface intentionally excludes a small set of retired legacy
residue that no longer supports the live PR workflow, current demos, or active
regression tests. The `v0.87` bounded cleanup retired:

- `BURST_PLAYBOOK.md`
- `REPORT_SCHEMA.md`
- `default.rules.profiles.example`
- `demo_v0_4.sh`
- `pr_smoke.sh`
