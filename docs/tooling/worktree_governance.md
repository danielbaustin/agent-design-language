# Worktree Governance

This document defines the canonical ADL worktree policy so milestone execution does not drift across unrelated directories, stale temporary registrations, or Codex-internal ephemeral worktrees.

## Canonical namespaces

- Primary checkout: the normal repository root.
- Managed ADL issue worktrees: `$HOME/git/adl-wp-<issue>`
- Optional managed lanes: `$HOME/git/adl-lane-*`
- Codex-ephemeral worktrees: the Codex app worktree namespace

Managed ADL worktrees are the only worktrees that should be used for milestone execution, issue implementation, and PR handoff.

## Status classes

The repository currently needs to distinguish four different classes cleanly:

- `primary_checkout`
  - the main repository checkout
  - fate: keep
- `managed_registered`
  - a registered ADL worktree in the canonical managed namespace
  - fate depends on merge/dirty state
- `stale_registration`
  - git metadata for a worktree that no longer exists on disk
  - fate: prune now
- `codex_ephemeral`
  - a Codex-internal worktree outside the managed ADL namespace
  - fate: ignore for milestone cleanup unless explicitly doing Codex GC
- `foreign_excluded`
  - a lookalike `adl-wp-*` directory that belongs to an unrelated project namespace and should not be swept up by ADL cleanup
- `orphan_dir`
  - a worktree-looking directory on disk that is not registered in `git worktree list`
  - fate: review, and back up before deletion if it contains meaningful state

## Fate policy

- `keep_primary`
  - keep the primary checkout
- `keep_active`
  - keep a registered managed worktree whose branch is still active
- `keep_dirty_active`
  - keep and review a dirty managed worktree that is still active
- `remove_merged_clean`
  - safe candidate for deletion after branch contents are merged and the worktree is clean
- `backup_then_remove`
  - dirty merged worktree or similarly risky local state; capture a patch or tarball before deletion
- `prune_now`
  - stale git worktree registration; safe to clean with `git worktree prune`
- `ignore_ephemeral`
  - do not treat Codex-internal worktrees as milestone blockers
- `ignore_foreign`
  - do not fold unrelated project directories into ADL cleanup
- `review_orphan` / `review_orphan_clean`
  - not registered; inspect before deletion

## Tooling

Use the tracked tooling rather than ad hoc shell commands:

```bash
# inspect current worktree status and recommended fate
./adl/tools/worktree_doctor.sh

# see what safe cleanup would happen
./adl/tools/worktree_prune.sh

# apply only the clearly safe cleanup set
./adl/tools/worktree_prune.sh --apply
```

The pruning flow is intentionally conservative:

- it removes only clean managed worktrees whose branch is already merged into `main`
- it prunes stale git worktree registrations
- it does not silently delete orphan directories, dirty worktrees, or Codex-ephemeral worktrees

## Interaction with `pr.sh`

`pr.sh start` should create managed worktrees in the canonical namespace. In a normal ADL checkout under `$HOME/git`, that means `adl-wp-<issue>` directories are created under `$HOME/git`.

If the repository is being exercised in a temporary test checkout outside the normal namespace, `pr.sh` may fall back to the repository parent or an explicit `ADL_WORKTREE_ROOT` override so tests remain hermetic.
