# Task Bundle Editor

This directory contains the bounded editor surface for ADL task bundles and current workflow-skill handoff.

Open:

- `docs/tooling/editor/index.html`

The editor is intentionally simple:

- no build step
- no framework dependency
- works as a tracked static artifact
- supports a linked task-bundle workspace with:
  - `Structured Task Prompt` (STP)
  - `Structured Implementation Prompt` (SIP)
  - bounded review-first `Structured Output Record` (SOR) surface
- exposes one bounded workflow action surface for:
  - copy-only lifecycle command preparation via `adl/tools/editor_action.sh prepare`
- defines one explicit near-term adapter contract in:
  - `docs/tooling/editor/command_adapter.md`
- exposes one bounded review-flow surface for:
  - reviewer checklist
  - derived review recommendation
  - copyable review note tied to SOR proof and follow-ups

## What This First Slice Does

- presents one task bundle as a linked three-card workspace
- guides a human through the core metadata and section fields
- previews the rendered markdown artifact live
- shows contract-aware checks for:
  - required fields and sections
  - normalized task IDs, run IDs, versions, enums, and booleans
  - placeholder content that still needs real authoring
  - structured section formats for bounded STP/SIP surfaces
- provides a bounded SOR review surface for:
  - evidence/proof notes
  - integration state
  - artifact verification and deferred follow-ups
- turns the SOR surface into a bounded review loop by:
  - summarizing whether the SOR is ready for handoff or needs iteration
  - checking proof, artifact verification, and follow-up coverage
  - generating a reviewer-facing note without inventing a second review system
- keeps the current local issue-bundle destination visible as:
  - `.adl/<version>/tasks/<task-id>__<slug>/`

## What It Does Not Do Yet

- it does not write files directly
- it does not replace `pr init`, `pr ready`/`pr doctor`, `pr run`, `pr finish`, `pr janitor`, or `pr closeout`
- it does not imply direct browser invocation for the authoring lifecycle
- it does not yet provide the full SOR decision loop or acceptance workflow
- it does not try to replace human review judgment with browser-only automation
- it does not yet execute the control plane directly from browser JS
- it does not yet call the structured-prompt validator directly from the browser
- it does not attempt full contract completeness for every machine-readable field

## Why This Is Still Useful

This slice reduces structural editing fragility without pretending the full editor architecture already exists. It makes the three-card bundle visible as one workspace, aligns the preview with the current `.adl` issue-bundle convention, and exposes a copy-only validated path back into the control plane without claiming browser direct execution.

## Canonical Demo / Proof Surface

The proof surface for this editor slice is:

- `docs/tooling/editor/command_adapter.md`
- `docs/tooling/editor/demo.md`
- `docs/tooling/editor/current_skill_wiring_demo.md`
- `docs/tooling/editor/five_command_demo.md`
- `docs/tooling/editor/five_command_regression_suite.md`

Review those together to see the actual supported loop, the legacy compatibility guardrails, and the remaining manual gaps.
