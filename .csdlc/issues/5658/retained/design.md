# Issue #5658 Design

Fix typed C-SDLC v2 lifecycle rooting so issue records, cards, locks, prepared
artifacts, and evidence are materialized and consumed from the bound worktree
root used for implementation. The primary checkout on `main` must not receive
tracked issue lifecycle writes after binding unless an operation is explicitly
bootstrap/read-only.

The smallest credible implementation target is the Rust v2 lifecycle/bind path
and its regression tests. The fix must preserve claim, lock, and exact-revision
guards while adding a failing regression for an ignored `.csdlc` issue state in
a newly-created worktree.
