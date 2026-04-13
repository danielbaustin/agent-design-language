# Worktree Governance

This document defines the canonical ADL worktree policy so milestone execution does not drift across unrelated directories, stale temporary registrations, or Codex-internal ephemeral worktrees.

## Canonical namespaces

- Primary checkout: the normal repository root.
- Managed ADL execution clones: `.worktrees/adl-wp-<issue>`
- Optional managed lanes: `.worktrees/adl-lane-*`
- Codex-ephemeral worktrees: the Codex app worktree namespace

Managed ADL worktrees are the only worktrees that should be used for milestone execution, issue implementation, and PR handoff.

Legacy external clones under `$HOME/git/adl-wp-*` are no longer canonical. If a repo-local replacement exists under `.worktrees/`, the external copy should be treated as a cleanup candidate rather than an execution surface.

## Status classes

The repository currently needs to distinguish four different classes cleanly:

- `primary_checkout`
  - the main repository checkout
  - fate: keep
- `managed_registered`
  - a registered ADL worktree in the canonical managed namespace
  - fate depends on merge/dirty state
- `managed_clone`
  - a repo-local execution clone in `.worktrees/adl-wp-*` or `.worktrees/adl-lane-*`
  - fate depends on merge/dirty state
- `managed_scratch`
  - a repo-local scratch directory under `.worktrees/` that is not an issue or lane clone
  - fate: remove if clean
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
- `legacy_external`
  - an old external `$HOME/git/adl-wp-*` or `$HOME/git/adl-lane-*` clone outside the canonical `.worktrees/` namespace
  - fate: remove after replacement if a repo-local clone exists; otherwise review carefully

## Fate policy

- `keep_primary`
  - keep the primary checkout
- `keep_active`
  - keep a registered managed worktree or repo-local managed clone whose branch is still active or intentionally retained for the next queue tranche
- `keep_dirty_active`
  - keep and review a dirty managed worktree or repo-local managed clone that is still active
- `remove_merged_clean`
  - safe candidate for deletion after branch contents are merged and the registered worktree is clean
- `remove_legacy_replaced`
  - safe legacy external clone deletion after a repo-local replacement exists and the external clone is clean
- `remove_scratch_clean`
  - safe deletion for clean repo-local scratch directories under `.worktrees/`
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

# see what safe repo-local cleanup would happen
./adl/tools/worktree_prune.sh

# write a reviewable first-batch report before deletion
./adl/tools/worktree_prune.sh --limit 10 --report docs/tooling/worktree_cleanup/first_batch.md

# apply only the clearly safe repo-local batch after review
./adl/tools/worktree_prune.sh --limit 10 --report docs/tooling/worktree_cleanup/first_batch.md --apply
```

The pruning flow is intentionally conservative:

- it removes only clean managed worktrees whose branch is already merged into `main`
- it prunes stale git worktree registrations
- it does not silently delete orphan directories, dirty worktrees, Codex-ephemeral worktrees, legacy external clones, or repo-local scratch directories unless those broader scopes are explicitly requested

## Interaction with `pr.sh`

`pr.sh start` should create managed execution clones in the canonical namespace under `.worktrees/`.

If the repository is being exercised in a temporary test checkout outside the normal namespace, `pr.sh` may still fall back to an explicit `ADL_WORKTREE_ROOT` override so tests remain hermetic.
