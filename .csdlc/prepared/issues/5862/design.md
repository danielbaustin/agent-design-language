# Issue 5862 Design: WP-04-IMP Distributed Guardian Implementation Umbrella

## Outcome And Boundary

Coordinate and reconcile exactly WP-04.01 through WP-04.16 after #5821 is
terminal. This umbrella owns scheduling and final evidence reconciliation
only. It owns no product path and cannot implement, review, merge, or close
a child on the child's behalf.

## Exact Live Denominator

| Child | GitHub issue |
| --- | --- |
| WP-04.01 | #5863 |
| WP-04.02 | #5864 |
| WP-04.03 | #5865 |
| WP-04.04 | #5866 |
| WP-04.05 | #5867 |
| WP-04.06 | #5868 |
| WP-04.07 | #5869 |
| WP-04.08 | #5870 |
| WP-04.09 | #5871 |
| WP-04.10 | #5872 |
| WP-04.11 | #5873 |
| WP-04.12 | #5874 |
| WP-04.13 | #5875 |
| WP-04.14 | #5876 |
| WP-04.15 | #5877 |
| WP-04.16 | #5878 |

## Owned Paths

- `.csdlc/issues/5862`
- `.csdlc/prepared/issues/5862`
- `.csdlc/evidence/5862`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Scheduling And Integration

#5821 and #5820 are hard terminal gates. Children execute only when their
own listed dependencies are terminal and their exclusive paths can be
claimed. WP-04.16 runs only after WP-04.01 through WP-04.15 are terminal,
integrates module registration, and produces real multi-node and native
proof. WP-14 #5832 waits for this umbrella's terminal integrated output.

## Validation And Reconciliation

The issue-local validator compares the exact mapping against the canonical
v0.92 wave, local typed records, approved design digests, null preparation
claims, child dependencies, and exact owned paths. Final reconciliation must
query each live PR through the typed GitHub client, require the GitHub closing
relation, match the PR head and merge commit to the child's terminal record,
recompute the terminal receipt digest, and prove the merge is ancestral to the
candidate head. WP-04.16's integrated execution proof, production validator,
native validator, command logs, artifacts, and digests are a separate mandatory
gate before the WP-14 handoff. Status booleans and umbrella prose are not
authority.

## Rollback

Stop scheduling, preserve child branches and evidence, fence uncertain
distributed owners, and return to the terminal WP-03 single-node Guardian.
The umbrella never rewrites child history or claims product paths.

## Non-Goals

- Direct product implementation or child lifecycle authority.
- Denominator changes after #5821 approval without a new gate review.
- Runtime v2 fallback, custom cryptography, or v0.93 governance.
