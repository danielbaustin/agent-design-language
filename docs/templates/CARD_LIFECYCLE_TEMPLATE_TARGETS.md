# Card Lifecycle Template Targets

## Purpose

This document records the target template shape for all five ADL issue cards:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

It is a template and schema planning surface only. It does not enforce the new
lifecycle, migrate active bundles, or change conductor/editor routing.

## Compatibility Policy

Current tooling still depends on some historical filenames and artifact
identifiers:

- `adl/templates/cards/input_card_template.md`
- `adl/templates/cards/output_card_template.md`
- `adl/schemas/structured_implementation_prompt.contract.yaml`
- `adl/schemas/structured_output_record.contract.yaml`
- `docs/templates/STRUCTURED_REVIEW_POLICY_TEMPLATE.md`

Those compatibility surfaces remain valid during this mini-sprint. Their
semantic roles are now clarified as `SIP`, `SOR`, and `SRP` target-state
surfaces. Follow-on validator and conductor issues decide when compatibility
aliases become warnings or migration blockers.

## Shared Lifecycle Fields

New or revised templates should be able to express:

- `lifecycle_stage`: one of `SIP`, `STP`, `SPP`, `SRP`, `SOR`.
- `activation_state`: the card's current lifecycle readiness.
- `source_refs`: links to earlier cards that bound the current card.
- `legacy_compatibility`: whether old naming or structure is intentionally
  retained.

Recommended activation states are:

- `scaffold`: file exists for path stability but is not authoritative.
- `draft`: issue-specific content is being authored or reviewed.
- `active`: card is the current authoritative lifecycle surface.
- `reviewed`: review has been applied and results are recorded.
- `pr_open`: outcome is represented by an open PR.
- `merged`: outcome has landed on `main`.
- `closed_no_pr`: issue closed intentionally without a merged PR.
- `superseded`: card was replaced by a later revision or issue.
- `legacy_compatible`: historical shape retained and detectable.

## SIP Target

`SIP` means Structured Issue Prompt.

Target responsibility:

- problem statement
- context and evidence
- scope and non-scope boundaries
- acceptance criteria
- dependencies and issue-graph truth
- source issue prompt linkage

Compatibility surfaces:

- current file: `sip.md`
- current generator template: `adl/templates/cards/input_card_template.md`
- current schema filename:
  `adl/schemas/structured_implementation_prompt.contract.yaml`

## STP Target

`STP` means Structured Task Prompt.

Target responsibility:

- selected task or solution
- touched surfaces
- invariants and proof shape
- issue-specific deliverables
- rationale for the chosen implementation path

Compatibility surfaces:

- current file: `stp.md`
- current schema filename: `adl/schemas/structured_task_prompt.contract.yaml`
- source issue body/front matter remains the practical STP authoring shape
  until a dedicated tracked STP template is introduced.

## SPP Target

`SPP` means Structured Plan Prompt.

Target responsibility:

- execution sequence
- dependencies and stop conditions
- validation plan
- review handoff plan
- branch/worktree constraints
- risks and fallback path

Compatibility surfaces:

- current file: `spp.md`
- current template: `docs/templates/STRUCTURED_PLAN_PROMPT_TEMPLATE.md`
- current validator surface: Rust validator-backed contract, not a dedicated
  schema file yet

## SRP Target

`SRP` means Structured Review Prompt.

Target responsibility:

- review instructions and policy
- evidence rules
- findings
- dispositions
- residual risks
- recommended outcome

Compatibility surfaces:

- current file: `srp.md`
- current template filename:
  `docs/templates/STRUCTURED_REVIEW_POLICY_TEMPLATE.md`
- current artifact type: `structured_review_prompt`

The template filename remains legacy-compatible, but new content should use the
Structured Review Prompt artifact type, sections, and review-result fields.

## SOR Target

`SOR` means Structured Outcome Record.

Target responsibility:

- actual changed paths
- validation actually run
- review actually performed
- PR and merge state
- closeout state
- unresolved follow-ups
- final issue truth

Compatibility surfaces:

- current file: `sor.md`
- current generator template: `adl/templates/cards/output_card_template.md`
- current schema filename: `adl/schemas/structured_output_record.contract.yaml`

`SOR` should summarize and link to `SIP`, `STP`, `SPP`, and `SRP`; it should
not absorb their full planning or review content.

## Non-Goals

- Do not enforce the lifecycle from this document.
- Do not invalidate active v0.91.2 bundles from this document.
- Do not rename compatibility files in this issue.
- Do not make sprint-scoped `SPP` mandatory from this document.
