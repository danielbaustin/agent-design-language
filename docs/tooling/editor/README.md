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
  - visible `Structured Output Record` (SOR) shell

## What This First Slice Does

- presents one task bundle as a linked three-card workspace
- guides a human through the core metadata and section fields
- previews the rendered markdown artifact live
- shows contract-aware checks for:
  - required fields and sections
  - normalized task IDs, run IDs, versions, enums, and booleans
  - placeholder content that still needs real authoring
  - structured section formats for bounded STP/SIP surfaces
- keeps the canonical tracked destination visible as a task-bundle path under:
  - `docs/records/v0.85/tasks/<task-id>/`

## What It Does Not Do Yet

- it does not write files directly
- it does not replace `pr create`, `pr start`, `pr run`, or `pr finish`
- it does not yet provide the full SOR review flow
- it does not yet invoke the control plane directly
- it does not yet call the structured-prompt validator directly from the browser
- it does not attempt full contract completeness for every machine-readable field

## Why This Is Still Useful

This first slice reduces structural editing fragility without pretending the full editor architecture already exists. It gives users a safer tracked surface than raw markdown-only editing while preserving the public task-bundle model and making the three-card bundle visible as one workspace.
