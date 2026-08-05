# WP-01 Design: Canonical v0.92 Activation

## Decision

WP-01 converts the reviewed v0.92 candidate package into current milestone
truth. It first audits the prerequisite evidence and candidate work-package
graph, then updates canonical version and milestone surfaces, opens only the
validated child issue wave, and generates typed six-card bundles for each
opened issue.

## Boundaries

- WP-01 owns planning, canonical version/docs alignment, and issue-wave setup.
- WP-02 owns repository-transfer execution.
- Child WPs own their implementation and proof.
- Deferred v0.92 and later-milestone sources retain explicit dispositions.
- Missing or contradictory prerequisite evidence becomes a named gap; it is
  never converted into a completion claim.

## Execution Shape

1. Inventory live prerequisite, issue, PR, release, and retained source truth.
2. Reconcile the WBS, issue-wave YAML, feature index, ADR plan, sprint, demos,
   README surfaces, and crate version.
3. Requalify issue #5104 loop-runtime evidence against current Runtime v3.
4. Validate dependency acyclicity, links, YAML, versions, and claim boundaries.
5. Open the final issue wave idempotently and generate all six typed cards.
6. Review the exact package once before publication.

## Non-Goals

- No child implementation.
- No repository transfer.
- No v0.93 constitutional-governance implementation.
- No unsupported birthday, personhood, citizenship, or consciousness claim.
