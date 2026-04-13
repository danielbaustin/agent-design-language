# Worktree Cleanup Wave 1

This report records the first bounded cleanup wave against the repo-local
`.worktrees/*` backlog. The scope is intentionally narrow: only repo-local
managed worktrees that the cleanup tooling classified as `remove_merged_clean`
were eligible.

## Selection Summary

- mode: apply
- repo: repository root
- managed_root: `.worktrees`
- include_legacy_external: no
- include_scratch: no
- limit: 5
- registered_removals_selected: 5
- directory_removals_selected: 0
- stale_registrations_present: no

## Selected Registered Removals

- `.worktrees/adl-wp-1348`
- `.worktrees/adl-wp-1409`
- `.worktrees/adl-wp-1411`
- `.worktrees/adl-wp-1544`
- `.worktrees/adl-wp-1550`

## Commands Used

Preview:

```bash
bash adl/tools/worktree_prune.sh --repo /Users/daniel/git/agent-design-language --limit 5 --report docs/tooling/worktree_cleanup/WORKTREE_CLEANUP_WAVE_1.md
```

Apply:

```bash
bash adl/tools/worktree_prune.sh --repo /Users/daniel/git/agent-design-language --limit 5 --apply --report docs/tooling/worktree_cleanup/WORKTREE_CLEANUP_WAVE_1.md
```

Post-cleanup verification:

```bash
bash adl/tools/worktree_doctor.sh --repo /Users/daniel/git/agent-design-language --format tsv > /tmp/adl-worktree-doctor-post.tsv
```

## Executed Actions

- `git worktree remove .worktrees/adl-wp-1348`
- `git worktree remove .worktrees/adl-wp-1409`
- `git worktree remove .worktrees/adl-wp-1411`
- `git worktree remove .worktrees/adl-wp-1544`
- `git worktree remove .worktrees/adl-wp-1550`

## Verification

Removed paths confirmed absent:

- `.worktrees/adl-wp-1348`
- `.worktrees/adl-wp-1409`
- `.worktrees/adl-wp-1411`
- `.worktrees/adl-wp-1544`
- `.worktrees/adl-wp-1550`

Remaining repo-local managed `remove_merged_clean` backlog after this wave: `9`

Remaining candidates:

- `.worktrees/adl-wp-1559`
- `.worktrees/adl-wp-1658`
- `.worktrees/adl-wp-1724`
- `.worktrees/adl-wp-1731`
- `.worktrees/adl-wp-1732`
- `.worktrees/adl-wp-1733`
- `.worktrees/adl-wp-1735`
- `.worktrees/adl-wp-1739`
- `.worktrees/adl-wp-1740`

## Excluded By Default

- legacy external worktrees outside `.worktrees/*`
- Codex ephemeral worktrees
- dirty or otherwise review-first repo-local worktrees
