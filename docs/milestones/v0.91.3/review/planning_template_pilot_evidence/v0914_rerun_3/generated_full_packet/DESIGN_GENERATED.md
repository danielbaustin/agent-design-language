<!--
Generated Planning Draft
planning_template_set: 1.0.0
template: design
template_path: docs/templates/planning/1.0.0/design.md
generation_status: generated_draft
claim_boundary: generated draft only; not reviewed or approved
-->

> Generated planning draft. This file proves only template filling;
> it is not reviewed, approved, released, merged, or lifecycle-true.
# v0.91.4 Design

## Metadata
- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-24`
- Owner: `ADL planning-template pilot`
- Related issues: v0.91.4 issue wave

## Purpose
Define what we are building, why, and how we validate it — concisely, with links to issues/PRs.

## Problem Statement
v0.91.4 needs milestone planning that is consistent, reviewable, and executable without hidden context.

## Goals
- C-SDLC lifecycle proof and repeatability
- C-SDLC lifecycle proof and repeatability

## Non-Goals
- replace reviewed milestone truth with generated text
- replace reviewed milestone truth with generated text

## Scope
### In Scope
- C-SDLC milestone planning, routing, and proof surface.
- C-SDLC milestone planning, routing, and proof surface.

### Out Of Scope
- Publication or release approval without review evidence.
- Publication or release approval without review evidence.

## Requirements
### Functional
- Generated planning docs preserve required section structure.
- Generated planning docs preserve required section structure.

### Non-Functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- Generated docs remain deterministic and portable.

## Proposed Design
### Overview
Use canonical planning documents, issue cards, review evidence, and focused validation as the milestone control plane.

### Interfaces And Contracts
- Planning template registry and structural validator contract.
- Planning template registry and structural validator contract.

### Execution Semantics
Each issue moves through the structured card lifecycle and records focused proof before merge or closeout.

## Risks And Mitigations
- Risk: Generated scaffolds may omit milestone-specific semantics.
  - Mitigation: Route generated drafts through planning-doc editor and human review.
- Risk: Generated scaffolds may omit milestone-specific semantics.
  - Mitigation: Route generated drafts through planning-doc editor and human review.

## Alternatives Considered
- Option: Continue hand-authoring milestone docs without template scaffolds.
  - Tradeoff: Templates improve consistency but require editor review for semantics.
- Option: Continue hand-authoring milestone docs without template scaffolds.
  - Tradeoff: Templates improve consistency but require editor review for semantics.

## Validation Plan
- Checks/tests: Generate planning drafts, validate required sections, compare to existing milestone docs, and record findings.
- Success metrics: Reviewers can trace milestone claims to docs, issues, demos, and validation evidence.
- Rollback/fallback: Keep generated drafts non-authoritative until reviewed; preserve existing milestone docs.

## Exit Criteria
- Goals/non-goals and scope boundaries are explicit.
- Validation plan is actionable and referenced by the milestone checklist.
- Major open questions are resolved or tracked in the decision log.
