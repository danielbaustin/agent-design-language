# Task Bundle Editor

This directory contains the first bounded editor surface for ADL public task bundles.

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
  - `pr start` via `adl/tools/editor_action.sh`
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
- keeps the canonical tracked destination visible as a task-bundle path under:
  - `docs/records/v0.85/tasks/<task-id>/`

## What It Does Not Do Yet

- it does not write files directly
- it does not replace `pr create`, `pr start`, `pr run`, or `pr finish`
- it does not imply direct browser invocation for the full five-command lifecycle
- it does not yet provide the full SOR decision loop or acceptance workflow
- it does not try to replace human review judgment with browser-only automation
- it does not yet execute the control plane directly from browser JS
- it does not yet call the structured-prompt validator directly from the browser
- it does not attempt full contract completeness for every machine-readable field

## Why This Is Still Useful

This first slice reduces structural editing fragility without pretending the full editor architecture already exists. It gives users a safer tracked surface than raw markdown-only editing while preserving the public task-bundle model, making the three-card bundle visible as one workspace, and exposing a thin validated path back into the existing control plane.

## Canonical Demo / Proof Surface

The canonical tracked proof surface for this editor slice is:

- `docs/tooling/editor/command_adapter.md`
- `docs/tooling/editor/demo.md`
- `docs/records/v0.85/tasks/task-v085-wp05-demo/`

Review those together to see the actual supported loop and the remaining manual gaps.
