# WP-01 — Safe issue-local initialization and design approval

## Problem

`csdlc-init` creates design artifacts before the record bootstrap. When a
design path is inside `.csdlc/issues/<issue>/`, the directory can be mistaken
for an existing canonical record and produce a partial initialization failure.
Separately, `approve_design` refreshes SPP/VPP design digests without
refreshing their diagram digests.

## Design

Keep initialization typed and atomic: validate and stage design/diagram files,
then create the canonical issue record and six cards as one recoverable commit.
An orphan issue directory must be classified as incomplete state, never parsed
as a record; recovery either completes the staged commit or returns a typed
error without claiming success.

Design approval computes both file digests from the same locked snapshot and
updates every design-bearing projection (SPP and VPP) before committing the
new generation. Doctor must see matching design and diagram references.

## Scope and proof

Only `csdlc-v2/src/lifecycle.rs`, `csdlc-v2/src/store.rs`, `csdlc-v2/src/cards.rs`,
and focused v2 tests are in scope. Proof consists of deterministic regressions
for issue-local paths, partial initialization recovery, dual digest refresh,
and a post-approval doctor pass.
