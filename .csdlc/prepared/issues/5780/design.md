# Issue 5780 Design: Delete Competing Terminal Authority

## Decision

Delete the obsolete `csdlc-closeout` command surface and every current writer for tracked post-merge phases, terminal receipt authority, reconciliation, repair, transport, and coupled pruning. Do not replace the binary with a reduced compatibility wrapper.

The supported terminal flow becomes:

1. `csdlc-publish` records the reviewed exact PR identity.
2. `csdlc-finish` re-observes the exact green head, merges or recognizes terminal GitHub state, and derives the terminal envelope in Git-common state.
3. `csdlc-pr-state` provides live PR status inspection.
4. `csdlc-clean` independently classifies or removes the exact clean issue worktree.

## Compatibility Boundary

Legacy `LifecyclePhase` variants, `TerminalEvidence`, and `TerminalReceipt` remain deserializable. `Store::load_terminal_receipt` remains a read-only compatibility input for `csdlc-clean index-legacy` and the v0.91.8 parity census. No retained method may write, repair, transport, or promote a receipt or tracked terminal projection.

Historical `.csdlc` projections, receipts beneath Git common state, and architecture evidence remain immutable. Current schemas stop advertising mutation requests but may retain read-only legacy data shapes needed to deserialize historical evidence.

## Deletion Slices

### D1: Operator surface

- Remove the `csdlc-closeout` binary target and source.
- Remove its operator skill and installer/coexistence entries.
- Add negative inventory tests preventing reintroduction.

### D2: Writer and reconciliation APIs

- Remove terminal closeout, readiness reconciliation, receipt retention, transport, corruption recovery, recordless recovery, terminal repair, and historical merge reconciliation APIs.
- Remove mutation request types and public schema entries.
- Retain only legacy record/receipt data shapes and read functions required by compatibility indexing.

### D3: Tests and docs

- Delete obsolete behavior tests and replace them with focused proof that legacy records still deserialize, compatibility indexing stays read-only, and no supported terminal writer remains.
- Update active workflow, operator, binary, and agent guidance without rewriting historical evidence.
- Record command, source-line, fixture, and artifact reductions against the pre-change baseline.

## Failure Policy

Fail closed if deletion breaks exact-head finish, derived terminal lineage, cleanup independence, legacy compatibility parity, clean-room installation, schemas, or current lifecycle tests. Do not hide deleted behavior behind feature flags or compatibility wrappers.

## Rollback

Each deletion slice remains independently reviewable in Git. Revert the smallest failing slice; do not restore the complete legacy closeout surface unless the derived terminal or compatibility proof demonstrates a concrete unmet invariant.
