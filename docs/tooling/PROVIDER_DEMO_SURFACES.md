# Provider Demo Proof Surfaces

This document governs the shared proof surfaces for the `v0.87.1` provider demo wave.

## Purpose

Provider-family issues should not keep redefining the same reviewer-entry and discovery surfaces.
This document sets the shared pattern once so family-specific issues can plug into it predictably.

## Shared Docs

These docs are shared across provider-family demo issues:

- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
  - canonical milestone demo index
  - owns the family issue map and final readiness status
- `docs/tooling/PROVIDER_SETUP.md`
  - canonical setup and credential-entry surface for remote providers
  - owns setup-bundle expectations and provider-family setup guidance
- `docs/tooling/PROVIDER_DEMO_SURFACES.md`
  - this governance guide
  - owns the documentation boundary between shared and family-specific proof surfaces

## Family-Specific Docs

These surfaces belong to individual provider-family issues:

- provider-family demo wrapper scripts under `adl/tools/`
- family-specific example ADL files under `adl/examples/`
- family-specific acceptance tests under `adl/tests/` or `adl/tools/`
- family-specific artifact `README.md` files written by the wrapper itself

## Update Rules

- Update `DEMO_MATRIX_v0.87.1.md` when a provider-family issue changes readiness, command entry points, or reviewer-facing proof mapping.
- Update `PROVIDER_SETUP.md` when setup or credential expectations change for remote-provider families.
- Do not put long family-specific run instructions into the shared docs; keep those in the family wrapper `README.md` output or family issue body.
- Keep one primary proof surface per provider-family demo so reviewers do not have to infer the main artifact.

## Reviewer Entry Pattern

For each provider-family demo, the reviewer should be able to answer:

- which issue owns the family proof surface
- which command is the canonical entry point
- which artifact is the primary proof surface
- whether credentials are required or the demo is local-only

## Current Families

- local Ollama
- bounded HTTP
- mock
- ChatGPT
