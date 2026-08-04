# Structured Intent Prompt

Template: 1.0.0

Issue: 5502

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare a small pure component that validates bounded task outputs and deterministically emits an integration plan, replan record, or blocked decision without acquiring merge or lifecycle authority.

## Required Outcome

After #5499 and #5498 are terminal, exact assignment-bound outputs can be converged or rejected deterministically while partial success and residual blockers remain visible.

## Scope

- issue-local typed lifecycle, preparation, validation, review, and evidence records
- future isolated adl-v2/crates/adl-workcell-convergence component after dependency admission
- typed output identity, authority, overlap, integration-plan, replan, blocked, and projection contracts
- determinism, security, COTS, budget, PVF, CI, review, and post-merge proof

## Authority

- #5499 owns issue-graph planning and assignment order
- #5498 owns task transport and bounded context/output delivery
- #5502 owns pure output convergence and deterministic replanning decisions only
- #5500 owns read-only dashboard rendering and #5501 owns live workcell proof
- review, publication, merge, and closeout remain independently serialized

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and current-registry semantic card operations only
- Keep root main untouched; all tracked #5502 work stays in /Volumes/FastWork/adl-wp-5502
- Preparation only: no product implementation, PR, publication, merge, Runtime v2, AWS, raw gh, credentials, provider calls, or network execution
- Do not implement until #5499 and #5498 are live GitHub merged and their merge revisions are ancestors of the #5502 execution base; typed closeout, receipts, and claim release are audit-only signals and MUST NOT block execution readiness
- Use /Volumes/FastWork for generated validation output
- Do not add preparation review churn; the required review gate is immediately before PR publication
