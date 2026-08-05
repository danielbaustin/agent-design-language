# Structured Intent Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare a complete, reviewed, fail-closed execution packet for WP-14A without beginning implementation or acceptance work.

## Required Outcome

All six current native typed cards, issue-specific design and diagram, COTS, budget, PVF, protected-path, and dependency gates are durable and reviewed; implementation remains impossible under the preparation claim.

## Scope

- typed C-SDLC v2 issue projection for #5384
- issue-local preparation requests, design, diagram, dependency manifest, and gate checker
- bounded exact preparation review and typed design approval
- preparation-only claim binding with no product paths

## Authority

- Issue #5384 and its routing comments define WP-14A scope
- Checked-in v0.91.8 WBS, issue wave, and platform acceptance feature define dependency topology
- Typed C-SDLC v2 projections and shared-Git receipts define lifecycle truth
- Current origin/main ancestry defines integration truth
- This operator instruction approves preparation and binding only, not implementation, publication, merge, deployment, or predecessor waiver

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in the existing issue worktree; never write on main.
- Prepare only: do not implement, publish, merge, or close #5384 in this step.
- WP-13 deletion #5346/#5347 is deferred and does not block WP-14A.
- Internal review #5356 must wait for the deferred deletion wave and focused post-deletion validation.
- Use one bounded Gemini review before the eventual PR.
- Reuse retained exact proof instead of rerunning expensive soak or lifecycle tests without cause.
