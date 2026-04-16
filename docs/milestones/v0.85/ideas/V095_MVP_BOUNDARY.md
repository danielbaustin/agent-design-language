# v0.95 MVP Boundary

## Purpose

This note defines the current MVP boundary for ADL as planned through `v0.95`.

The goal is to prevent accidental scope drift while preserving the current
architectural ambition of the project.

## Core Rule

The MVP is `v0.95`.

The working planning assumption is:

- every currently identified major feature domain on the roadmap is a
  must-have for the `v0.95` MVP
- the main currently unresolved exception is Zed-specific integration

## Must-Have by v0.95

Unless explicitly reclassified later, the MVP includes the full planned feature
set represented in the current roadmap through `v0.95`.

That includes:

- operational maturity and trust/proof surfaces
- authoring lifecycle completion
- real working editor capability
- cognitive control
- instinct and bounded agency
- persistence over time
- AEE convergence
- security and threat modeling
- reasoning graph, signed trace, and trace query
- affect and moral cognition
- identity and capability substrate
- governance and delegation
- MVP convergence, tooling migration, demo-catalog closure, and `1.0` definition freeze

## Editor Requirement

For MVP purposes, the non-negotiable requirement is:

- ADL must have real usable editor capability by `v0.95`

The current acceptable implementation path is:

- HTML-based in-repo editor surfaces

Those editor surfaces should be sufficient to support the MVP authoring,
execution, and review story even if richer host-specific integrations do not
land in time.

## Zed Status

Zed integration is currently classified as:

- preferred
- potentially valuable
- not yet MVP-critical

Current planning rule:

- if HTML-based editor integration is strong enough for the MVP story, Zed may
  be deferred until after `v0.95`
- if later evidence shows that HTML-based editors are insufficient for a
  credible MVP workflow, Zed can be promoted into the must-have set by explicit
  decision

Until that explicit decision is made, Zed should not be treated as silently
required.

## Guardrail

For planning purposes:

- do not quietly cut major roadmap domains from the `v0.95` MVP
- do not quietly promote Zed into the must-have set
- if either happens, document it as an explicit scope decision

## Summary

The current MVP stance is:

- everything on the roadmap is must-have by `v0.95`
- real editor capability is required
- HTML-based editor surfaces are sufficient for now
- Zed integration is the only major currently optional implementation-path
  decision inside the `v0.95` convergence band
