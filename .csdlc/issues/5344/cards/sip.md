# Structured Intent Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Run representative ADL v2 opt-in scenarios at exact revisions, retain a bounded soak packet, and prove explicit restoration of the exact prior selector state after successful and failed selection attempts before handing reviewed cutover authority to child #5343.

## Required Outcome

A deterministic exact-revision soak and rollback packet proves local, CI, Runtime v3, provider-disposition, and demo behavior; exact prior selector bytes and digest are restored through the authoritative selector transaction; #5343 and deletion remain fail-closed until acceptance.

## Scope

- issue-local C-SDLC lifecycle, preparation, review, validation, and evidence records
- adl-v2/tools/run-soak.sh bounded manifest-driven orchestration harness
- adl-v2/tools/prove-rollback.sh authoritative selector rollback and fault harness
- docs/milestones/v0.91.8/evidence/wp12 normalized exact-revision soak and rollback report
- dependency, receipt, ancestry, COTS, budget, PVF, no-deferral, cutover, and rollback gates

## Authority

- #5344 owns only its issue-local records, two bounded soak/rollback harness paths, and its retained report path
- #5345 selector and installer are read-only dependencies invoked only through their authoritative public interface
- #5350 parity implementation and classifications and #5361 Runtime v3 implementation and acceptance are read-only inputs
- #5343 alone owns reviewed default switching; #5344 performs isolated opt-in proof and grants no default-cutover authority
- Runtime v2 and incumbent ADL are untouched rollback evidence and may not be edited, copied, imported, or linked
- No AWS, raw credentials, hidden network, production-provider claim, deletion, or release authority is in scope

## Assumptions

- none

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and current-registry semantic card operations only
- Keep the primary checkout clean on main and all tracked #5344 work in the dedicated FastWork issue worktree
- Stop on any typed protected-path collision and never steal, widen, hand-edit, or bypass another claim
- Preparation only: no soak, selector mutation, cutover, product implementation, publication, PR, AWS, raw gh, or Runtime v2 edit
- Do not execute until #5350 and #5361 are each live merged, typed closed_out, backed by a retained merged receipt, and ancestral to the exact execution revision
- Use only an isolated selector root during later execution and reject production or default selector paths
- Use /Volumes/FastWork for generated validation and build output
- Run bounded preparation review, fix every actionable finding, commit and push the preparation branch, and stop
