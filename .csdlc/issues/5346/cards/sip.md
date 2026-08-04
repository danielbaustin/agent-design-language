# Structured Intent Prompt

Template: 1.0.0

Issue: 5346

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Delete only the reviewed and replaced incumbent ADL language, compiler, engine, CLI, and directly owned compatibility paths after reversible cutover, current acceptance, exact eligibility, and disjoint-manifest gates pass.

## Required Outcome

At least 80 percent of the pinned #5346 eligible incumbent denominator is deleted, with a 90 percent target, while deleted, retained, and newly added LoC remain separate and every retained path has a named owner and reviewed justification.

## Scope

- issue-local typed C-SDLC preparation, validation, review, and evidence records
- read-only eligibility and dependency evaluation before execution
- future exact-path deletion of manifest-approved incumbent ADL language, compiler, engine, CLI, and directly owned compatibility paths
- future deletion eligibility and post-deletion validation manifests under docs/milestones/v0.91.8/evidence/wp13
- future focused, complete, CI, review, serialized merge, and post-merge proof

## Authority

- Preparation protects only #5346 lifecycle and evidence paths; it claims no incumbent source path
- #5346 owns final core ADL language/compiler/engine/CLI deletion; #5347 owns externally owned incumbent bands
- #5346 and #5347 may execute only from reviewed disjoint manifests; their merges and post-merge proof remain serialized
- Existing csdlc-eligibility and Git object identity are the deletion authority; no replacement eligibility engine is in scope
- #5344/#5343 own soak, rollback, and selector cutover; #5358/#5361 own current acceptance truth
- No AWS, raw gh, live credentials, product implementation, publication, Runtime v2 edit, or deletion is part of preparation

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and semantic card operations only
- Keep root main clean and perform tracked work only in /Volumes/FastWork/adl-wp-5346
- Preparation only: no code deletion, product edit, PR, publication, merge, AWS, raw gh, or Runtime v2 edit
- Do not execute until #5344, #5343, #5358, and #5361 are merged, typed closed_out, claim-free, receipt-backed, and ancestral
- Require reviewed #5346/#5347 manifests to be exact-path disjoint before amending any product-path claim
- Use existing COTS and repository tools; do not roll a deletion, hashing, eligibility, or workflow engine
- Run bounded preparation review, fix all actionable findings, commit and push preparation, then wait fail-closed
