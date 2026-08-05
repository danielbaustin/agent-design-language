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

- `.csdlc/issues/5862/`
- `.csdlc/prepared/issues/5862/`
- `.csdlc/evidence/5862/`

No `adl-runtime/`, `adl-runtime-kernel/`, `adl/tools/`, or API schema path is
owned by the umbrella. WP-04.16 owns final module registration and product
integration paths.

## Scheduling And Integration

#5821 and #5820 are hard terminal gates. Children execute only when their
own listed dependencies are terminal and their exclusive paths can be
claimed. WP-04.16 runs only after WP-04.01 through WP-04.15 are terminal,
integrates module registration, and produces real multi-node and native
proof. WP-14 #5832 waits for this umbrella's terminal integrated output.

## Validation And Reconciliation

The issue-local validator compares the exact mapping against the canonical
v0.92 wave, local typed records, approved design digests, null preparation
claims, child dependencies, and exclusive paths. Final reconciliation must
derive live issue/PR/merge/terminal receipts and exact-head evidence; status
booleans and umbrella prose are not authority.

## Rollback

Stop scheduling, preserve child branches and evidence, fence uncertain
distributed owners, and return to the terminal WP-03 single-node Guardian.
The umbrella never rewrites child history or claims product paths.

## Non-Goals

- Direct product implementation or child lifecycle authority.
- Denominator changes after #5821 approval without a new gate review.
- Runtime v2 fallback, custom cryptography, or v0.93 governance.
