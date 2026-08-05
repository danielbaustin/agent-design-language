# Structured Task Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

C-SDLC v2 terminal truth recovery for the issue #5748 inventory only; no product-scope widening or unrelated lifecycle cleanup.

## Deliverables

- receipt-backed closed_out projections for the #5748 inventory
- typed repair outcomes for special disposition and retained-artifact cases
- doctor, receipt-equality, retained-artifact, and diff-hygiene proof
- explicit blocker and prune report

## Acceptance

1. AC-1: Every inventoried issue has remote state, disposition, PR or no-PR reason, terminal receipt, tracked projection, and claim release in agreement.
2. AC-2: Every resulting projection passes csdlc-doctor and exact receipt equality.
3. AC-3: Missing-record or false-terminal recovery uses only a current typed deterministic route.
4. AC-4: No dirty or foreign-owned worktree content is discarded.
5. AC-5: No stale review, missing proof, missing closing linkage, or mismatched terminal PR is inferred away.
6. AC-6: Final reporting includes every inventoried issue with no silent omissions.
7. AC-7: No AWS, force-prune, GitHub mutation outside typed C-SDLC v2, v1 wrapper, or import route is used; read-only GitHub observation is allowed for live parity evidence.

## Dependencies

- closed remote issue disposition for every inventory item
- valid issue-local terminal receipts or an explicit typed repair route
- current origin/main as the reconciliation base

## Inputs

- GitHub issue #5748 body
- Git-common csdlc-v2 closeout receipts
- .csdlc/issues/<issue> typed projections
- csdlc-v2 closeout and doctor binaries

## Non Goals

- product implementation
- manual receipt or generated-card edits
- sunset v1 or csdlc-import recovery
- AWS or Spot execution
- force-pruning dirty or foreign-owned worktrees
