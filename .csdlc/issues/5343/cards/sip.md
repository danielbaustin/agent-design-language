# Structured Intent Prompt

Template: 1.0.0

Issue: 5343

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Execute one reviewed exact-revision default-generation transaction through the #5345 selector only after #5344 proves soak and exact rollback, while retaining explicit v1 override throughout a documented compatibility window.

## Required Outcome

A deterministic cutover receipt binds the selected generation to a verified fresh-install executable and receipt, preserves exact prior selector identity, proves explicit v1 override and rollback, opens a bounded rollback window, and grants no deletion authority.

## Scope

- issue-local typed lifecycle, preparation, review, validation, and evidence records
- normalized exact-revision cutover evidence under docs/milestones/v0.91.8/evidence/wp12/cutover-5343
- read-only use of the authoritative #5345 selector, installer, CLI, transaction, and rollback interfaces
- exact #5344 merge, typed closeout, retained receipt, ancestry, soak, rollback, fresh-install, and handoff acceptance gates
- transaction, failure-preservation, fresh-install, explicit-v1, rollback-window, budget, PVF, no-deferral, CI, and post-merge proof

## Authority

- #5343 owns only issue-local records and its normalized cutover evidence path
- #5345 owns selector, installer, CLI, locking, compare-and-swap, receipt, and rollback implementation and is a read-only dependency
- #5344 owns soak, rollback, compatibility-window, and accepted cutover-handoff evidence and must be terminal before execution
- Runtime v2 and incumbent ADL remain untouched rollback targets and may not be edited, copied, imported, linked, or deleted
- WP-13 deletion, release acceptance, cloud work, and production-provider claims are outside #5343

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and current-registry semantic card operations only
- Keep root main untouched and clean; all tracked #5343 work stays in /Volumes/FastWork/adl-wp-5343
- Preparation only: no selector transaction, installation, product implementation, publication, PR, merge, cutover, deletion, AWS, raw gh, or Runtime v2 edit
- Do not execute until #5344 is GitHub merged, typed closed_out, claim-free, backed by a retained merged receipt, ancestral, and accompanied by an accepted exact-revision soak/rollback handoff
- Do not execute until #5345 selector and installer authority is merged, typed closed_out, receipt-backed, ancestral, and exact-install verifiable
- Use /Volumes/FastWork for generated validation output
- Run bounded preparation review, fix every actionable finding, commit and push preparation only, and stop
