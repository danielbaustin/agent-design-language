# Structured Task Prompt

Template: 1.0.0

Issue: 5686

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Reapply and verify the two retained #5662 terminal-projection commits on current origin/main, then publish through a dedicated repair PR.

## Deliverables

- Receipt-matching #5662 tracked terminal projection
- Focused validation evidence
- Exact-head bounded review
- PR targeting main

## Acceptance

1. AC-1: the #5662 projected record digest matches the canonical closeout receipt
2. AC-2: the repair changes no implementation source, Unity assets, runtime code, owner binaries, or canonical receipts
3. AC-3: focused typed lifecycle validation passes
4. AC-4: exact-head review has no unresolved actionable findings
5. AC-5: the published PR targets main and contains only #5662 projection plus #5686 lifecycle truth

## Dependencies

- Closed issue #5662 and merged PR #5677
- Canonical closeout receipt .git/csdlc-v2/closeout/5662.json
- Retained commits 6487f1ef8d97549c5ccf092946d93a7aa67c60de and d95b4b0c5ebcc4c4fa95d8dccf19558296c53c6c

## Inputs

- GitHub issue #5686
- .csdlc/issues/5662
- .csdlc/publication/5662.intent.json
- .git/csdlc-v2/closeout/5662.json

## Non Goals

- No #5662 implementation changes
- No Unity work
- No canonical receipt mutation
- No direct commit to main
- No broad lifecycle tooling refactor
