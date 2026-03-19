# Task Bundle Editor

This directory contains the first bounded editor surface for ADL public task bundles.

Open:

- `docs/tooling/editor/index.html`

The editor is intentionally simple:

- no build step
- no framework dependency
- works as a tracked static artifact
- supports the two highest-friction authoring surfaces in the current workflow:
  - `Structured Task Prompt` (STP)
  - `Structured Implementation Prompt` (SIP)

## What This First Slice Does

- guides a human through the core metadata and section fields
- previews the rendered markdown artifact live
- shows schema-aware checks for required fields and section presence
- keeps the canonical tracked destination visible as a task-bundle path under:
  - `docs/records/v0.85/tasks/<task-id>/`

## What It Does Not Do Yet

- it does not write files directly
- it does not replace `pr create`, `pr start`, `pr run`, or `pr finish`
- it does not yet edit `Structured Output Records` (SORs)
- it does not yet call the Ruby validator directly from the browser

## Why This Is Still Useful

This first slice reduces structural editing fragility without pretending the full editor architecture already exists. It gives users a safer tracked surface than raw markdown-only editing while preserving the public task-bundle model and the current deterministic workflow boundaries.
