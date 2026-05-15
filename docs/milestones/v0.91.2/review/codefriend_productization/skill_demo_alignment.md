# CodeFriend Skill And Demo Alignment

## Purpose

Map the current CodeFriend product lane to already-landed ADL skill surfaces so
`WP-06` packages the workflow truthfully instead of implying a separate hidden
system.

## Current Product Lane

CodeFriend currently aligns to these bounded surfaces:

### Review packet construction

- `repo-packet-builder`

What it contributes:

- repo scope
- inventory
- evidence index
- specialist assignment map

### Specialist review lanes

- `repo-review-code`
- `repo-review-security`
- `repo-review-tests`
- `repo-review-docs`
- `repo-architecture-review`
- `repo-review-synthesis`

What they contribute:

- bounded findings
- evidence-backed review output
- residual-risk synthesis

### Product report packaging

- `product-report-writer`

What it contributes:

- customer-grade report packaging
- publication-boundary language
- residual-risk preservation

### Quality and publication boundary

- `review-quality-evaluator`
- `redaction-and-evidence-auditor`

What they contribute:

- report-quality gate
- publication-safety gate

## Demo Alignment

For `WP-06`, the truthful demo surface is:

- a repeatable packet-to-report workflow package

That means the `WP-06` proving route is:

1. packet shape exists
2. report shape exists
3. evidence rules exist
4. product-language non-claims exist

It does not mean:

- heuristic review quality is fully proven
- live collaborative-doc publication is proven
- mass modernization workflows are proven

Those belong to:

- `WP-07` review heuristics
- `WP-08` and `WP-09` Workspace bridge lanes
- `WP-10` modernization lane

## Roadmap Alignment

`WP-06` should leave Sprint 2 with:

- one named CodeFriend packet workflow package
- one aligned product-report template
- one explicit evidence policy
- one truthful handoff to `WP-07`

`WP-07` should then build on this by proving:

- heuristic review repeatability
- bounded demo outputs
- acceptance checklist quality

## Non-Claims

- This alignment note does not claim the existing skills are final product UX.
- This alignment note does not claim external review publication is ready.
- This alignment note does not claim CodeFriend is a standalone shipped product
  at `WP-06`.
