# #5363 WP-20 Remediation And Release Preflight Design

## Status

Preparation-only packet for v0.91.8 WP-20. It grants no authority to remediate
findings, edit product code, publish, merge, close, or treat receipts as
execution gates.

## Objective

Prepare the issue-local lifecycle surface for future remediation and release
preflight after WP-19 has produced accepted internal/external review findings.
Future execution must bind exact revisions, fix only accepted findings, and
rerun focused and integrated checks before releasing WP-21.

## Authority Boundary

Preparation owns only `.csdlc/issues/5363`, `.csdlc/locks/5363.lock`,
`.csdlc/prepared/issues/5363`, and `.csdlc/evidence/5363`.

## Dependency Gate

Release-tail order is WP-15 #5354, WP-16 #5351, WP-17 #5360, WP-18 #5356,
WP-19 #5357, WP-20 #5363, WP-21 #5362, WP-21A #5355, WP-22 #5359, and WP-23
#5348. Execution is blocked until WP-19 #5357 is live-merged into the exact
#5363 execution base and the observed merge SHA is an ancestor of that base.
Retained receipts may be audited, but they are not admission authority.

The C-SDLC acceptance-defect children that feed WP-20 are evidence dependencies,
not open implementation scope for this preparation branch:

- #5548 is CLOSED/COMPLETED. Future execution must verify the already-merged
  causal evidence from PR #5598, commit `aac8eaa7dffaa904ed1dfb0ec17fbf667c1ef9f0`,
  remains ancestral to the current execution base.
- #5558 is CLOSED/COMPLETED. Future execution must verify PR #5749, commit
  `c34f0c9412495039a6374f7ce88fa39e34bb5042`, and PR #5769, commit
  `a5df18f19a4c651eb6594e5690e294c7b7929261`, remain ancestral to the current
  execution base.

## Future Work Shape

The future remediation packet should inventory accepted WP-18/WP-19 review
findings, separate accepted fixes from non-goals, preserve completed child
evidence as current-main ancestry checks, run the smallest proving checks for
each fix class, and preserve unresolved or unsupported claims as blockers.

## Validation

The preparation proof is typed packet validation, `csdlc-doctor`, current-main
ancestry checks for #5548/#5558 merged evidence, and focused hygiene over only
the #5363 issue-local paths. It proves the typed C-SDLC v2 packet is
structurally usable for later execution without claiming any WP-20 review
finding is fixed.
