---
name: csdlc-v2-clean
description: Safely classify or remove one exact issue worktree, and inspect legacy terminal compatibility without making delivery truth depend on cleanup.
---

# C-SDLC v2 Clean

Use `csdlc-install resolve` to locate the active `csdlc-clean` binary.

For worktree cleanup, pass the versioned typed cleanup request to `cleanup`.
The request names the issue, exact branch, absolute registered worktree path,
and either `classify` or `remove`. The binary locks cleanup per issue, validates
the live Git worktree registration and issue projection, rejects the primary
checkout, and reports dirty, missing, or drifted state without destructive
fallbacks. Removal is always non-forced and idempotent.

Use `compatibility-index` for a sorted read-only view of tracked terminal
projections, optional retained receipts, and optional derived terminal caches.
Use `validate-census` with the retained v0.91.8 terminal audit to prove that the
tracked terminal set remains compatible. Neither command rewrites lifecycle
state, receipts, or derived terminal evidence.

Do not make merge, finish, issue closure, or derived terminal truth depend on
successful worktree removal.
