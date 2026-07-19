# Terminal SOR artifact-reference repair

## Decision

Add one narrow typed terminal repair operation that updates stale SOR artifact references when terminal reconciliation has moved the same authored design or diagram bytes into the canonical retained directory.

The active authority issue must hold a live claim. The target must be `closed_out`, claim-free, backed by a valid retained terminal receipt, and match exact generation, record digest, and receipt digest. Each requested replacement must identify one existing SOR artifact value, one canonical retained replacement, and the expected byte digest. The operation verifies that the replacement is one of the target record's retained authored artifacts and that its bytes match the receipt before changing values.

The repair updates SOR values, re-renders all projections and digests, refreshes the target record and terminal receipt under the shared terminal transaction lock, and preserves all other lifecycle evidence. Failure at any stage restores record and receipt parity.

## Application

Use the operation to replace #5390's deleted `.csdlc/issues/5390/diagram.mmd` SOR artifact with `.csdlc/issues/5390/retained/diagram.mmd`. No other #5390 evidence changes.

## Non-goals

- General terminal SOR editing
- Adding or removing arbitrary artifacts
- Changing execution, validation, review, readiness, or disposition truth
- Runtime changes
- AWS execution
