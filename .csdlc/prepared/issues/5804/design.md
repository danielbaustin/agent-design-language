# Design: v0.91.8 External-Review Documentation Readiness Repair

## Boundary

Repair only the final v0.91.8 review corpus and handoff truth needed before
WP-19 dispatches the external review. Product source and the external review
itself are out of scope.

## Approach

1. Replace the handoff's placeholder implementation manifest with concrete
   tracked source, test, and evidence entrypoints.
2. Reconcile current GitHub issue state without rewriting dated historical
   snapshots.
3. Replace machine-local command examples with configurable repo-local
   defaults.
4. Run the existing WP-19 corpus, dependency, link, structure, redaction, and
   hygiene checks.
5. Publish a docs-only PR that closes #5804 and explicitly leaves #5357 open.

## Non-Claims

- The external review is not performed by this issue.
- Release approval and v0.92 activation are not claimed.
- Historical evidence is not rewritten merely because live state advanced.
