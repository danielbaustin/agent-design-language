# Structured Task Prompt

Template: 1.0.0

Issue: 4763

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Produce a complete preparation packet for #4763 first-birthday docs and external launch surfaces. The packet defines later execution work and validation gates, but does not perform that implementation or publication.

## Deliverables

- Six issue-specific cards for #4763 with explicit preparation-only boundary.
- Design and Mermaid diagram covering first-birthday docs, external launch surfaces, dependencies, paths, budgets, PVF lanes, rollback, and no-deferral gates.
- Bounded gpt-5.5 preparation-review artifact with all actionable preparation findings fixed in this branch.
- Reacquire request and lifecycle-obstruction record showing typed claim reacquisition was attempted and blocked by unrelated #5332 reconciliation.
- Clean committed and pushed preparation branch after origin/main integration.

## Acceptance

1. AC-1: All six #4763 cards are issue-specific, internally rendered, and scoped to preparation only.
2. AC-2: Design and diagram identify first-birthday docs and external launch surfaces without implementing or publishing them.
3. AC-3: #4762 actual retained implementation proof is a later execution dependency, while #4762 claim acquisition, lifecycle receipt, PR publication, merge, and closeout are not preparation blockers and are not proof substitutes.
4. AC-4: Exact issue-local paths and intended later implementation paths are listed.
5. AC-5: COTS posture, LoC budgets, elapsed-time budgets, and validation-time budgets are explicit.
6. AC-6: PVF lanes cover preparation proof, lifecycle/tooling blockers, future docs proof, public-claim redaction, and dependency gates.
7. AC-7: Rollback criteria and no-deferral criteria fail closed for unsupported public claims, missing #4762 implementation proof, path widening, new COTS, or stale lifecycle truth.
8. AC-8: One bounded gpt-5.5 preparation review is recorded with fixes and no PR, publication, merge, implementation, or closeout occurs.

## Dependencies

- origin/main integrated at 51bc5ae51b57c19dbab693af1c5a45142995f4e5 before preparation refresh.
- #4762 actual retained implementation proof for birth witnesses and receipt package is required before later #4763 execution can prove launch readiness.
- #4762 claim acquisition, receipt bookkeeping, PR publication, merge, and closeout are explicitly not blockers for this preparation branch and are not accepted as proof substitutes.
- WP-21 parent #5362 and v0.91.8 activation-test-map constraints remain planning inputs.
- Typed #4763 claim reacquisition currently depends on resolving unrelated #5332 terminal-authority reconciliation in the cross-worktree scanner.

## Inputs

- docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md
- docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md
- docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md
- .csdlc/prepared/issues/4763/reacquire-claim-20260731.json

## Non Goals

- No first-birthday documentation implementation in this preparation branch.
- No external launch copy publication, site update, social/media surface, PR opening, merge, or closeout.
- No #4762 implementation, claim repair, closeout repair, or proof fabrication.
- No unrelated #5332 reconciliation inside the #4763 preparation branch.
- No new runtime, provider, SaaS, or binary dependency.
