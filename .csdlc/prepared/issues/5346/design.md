# Issue 5346 design: final replaced ADL deletion

Status: preparation only. No deletion or product change is authorized.

## Decision

WP-13 removes only incumbent ADL language, compiler, engine, CLI, and directly
owned compatibility paths that a reviewed eligibility manifest proves have an
accepted replacement. Issue #5346 owns the final core band. Peer #5347 owns
externally owned incumbent bands. Their manifests must be disjoint before
either deletion lane starts, and their merges and post-merge validation remain
serialized.

Deletion uses the existing `csdlc-eligibility` authority and Git path/object
identity. No second eligibility engine, path classifier, deletion calculator,
or workflow runner will be written.

## Dependency Gate

Deletion is prohibited until all of these predicates hold at the exact
execution revision:

1. WP-12 parent #5344 and reviewed selector child #5343 are GitHub merged,
   typed `closed_out`, claim-free, backed by retained terminal receipts, and
   their observed merge SHAs are ancestors of the execution revision.
2. Current C-SDLC v2 acceptance #5358 and Runtime v3 acceptance #5361 satisfy
   the same merge, closeout, receipt, claim-release, and ancestry predicates.
3. WP-14A #5384, WP-15 #5354, WP-16 #5351, and WP-17 #5360 satisfy the same
   GitHub merged, typed `closed_out`, claim-free, retained terminal receipt,
   and ancestry predicates. WP-13 deletion is therefore intentionally prepared
   now but cannot execute while #5351 or #5360 remain open or unprojected.
4. The reviewed #5346 and #5347 eligibility manifests have no equal,
   ancestor, descendant, symlink-target, generated-owner, or Cargo-workspace
   ownership overlap.
5. The #5343 rollback window has expired or an exact reviewed deletion
   approval explicitly authorizes acceleration without weakening rollback.

The current #5358 receipt is an input, not permission to bypass the other
gates. Missing, stale, malformed, contradictory, or non-ancestral evidence
fails closed.

## Eligibility Manifest

The future canonical #5346 manifest is
`docs/milestones/v0.91.8/evidence/wp13/5346-deletion-eligibility.v1.json`.
It must bind to one Git revision and contain, for every incumbent path:

- exact repo-relative path, Git mode, object ID, and baseline physical LoC;
- disposition: `delete`, `retain`, or `owned_by_5347`;
- accepted replacement owner, replacement path, and exact proof reference;
- rollback-window and selector evidence;
- retention owner and justification for every retained path;
- classification evidence for generated, test, fixture, build, and docs paths;
- explicit `protected_path` status and `claim_addition_required` truth so the
  preparation claim never doubles as product deletion authority;
- manifest digest and the exact `csdlc-eligibility` result.

The denominator is the sum of baseline LoC for rows classified as replaced and
eligible inside #5346's band. Deleted, retained, and newly added LoC are
reported separately. The target is 90 percent deletion and 80 percent is the
fail-closed minimum. An 80-to-89-percent result requires an enumerated retained
surface, owner, justification, and exact reviewed cutover approval. Below 80
percent is never complete.

## Disjoint Ownership

#5346 may delete only the final incumbent ADL language/compiler/engine/CLI and
direct compatibility paths named by its reviewed manifest. #5347 owns Runtime,
C-SDLC, cognitive-owner, integration, demo, and proof-tooling bands. Exact-path
and directory-prefix overlap, symlink escape, workspace-member ambiguity, or
an unowned retained file blocks both lanes. Preparation protects only the
issue-local records and future evidence files; product paths are added to the
typed claim only after the terminal dependency and disjointness gates pass.

## Execution Shape

1. Recompute and verify the pinned baseline and both manifests read-only.
2. Run `csdlc-eligibility`; require an exact approved result and all terminal
   dependency predicates, including #5347, WP-14A #5384, WP-15 #5354, WP-16
   #5351, and WP-17 #5360.
3. Amend the typed claim to the exact eligible #5346 paths only.
4. Delete exactly those paths with Git, without broad globbing or filesystem
   traversal.
5. Prove workspace manifests, all consumers, tests, demos, docs links, install,
   selector rollback, and Runtime v3/C-SDLC v2 boundaries on the exact revision.
6. Retain the post-deletion packet, obtain exact review, and serialize merge
   and post-merge validation with #5347.

## COTS And Budget

Use existing Git, `csdlc-eligibility`, Cargo, cargo-nextest, cargo-llvm-cov,
and the repository's validation scripts. Add no crate or package dependency for
manifest comparison, path deletion, hashing, or workflow control.

Preparation and future deletion orchestration must remain at or below 800
nonblank implementation lines and 1,200 test/fixture lines, with modules below
500 lines. Preparation and dependency checks have 120-second budgets, focused
post-deletion validation 300 seconds, complete deterministic validation 1,800
seconds, and post-merge validation 3,600 seconds. Any variance requires exact
measurement and bounded review; it is not implied by the 80-percent floor.

## Failure Policy

Fail closed without deletion, publication, merge, or closeout on any missing
terminal receipt, ancestry failure, active dependency claim, manifest overlap,
unowned retained file, replacement-proof gap, rollback-window violation,
eligibility rejection, denominator drift, deletion below 80 percent, deferred
validation, stale review, red CI, or absent post-merge proof. Runtime v2 is
categorically outside #5346 ownership and may not be edited or deleted by this
issue. Any future Runtime v2 change requires separate explicit issue authority.
