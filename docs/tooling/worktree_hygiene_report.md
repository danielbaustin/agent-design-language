# Worktree Hygiene Report

This report records the executed cleanup performed after the v0.85 authoring/editor tranche.

## Canonical Keep-Set

Keep repo-local execution clones under `.worktrees/` for the active queue and current follow-on work:

- `adl-wp-881`
- `adl-wp-882`
- `adl-wp-886`
- `adl-wp-901`
- `adl-wp-902`
- `adl-wp-903`
- `adl-wp-982`
- `adl-wp-1009`
- `adl-wp-1012` through `adl-wp-1024`
- `adl-wp-1026`
- `adl-wp-1028`
- `adl-wp-1029`

## Removed During This Pass

The hygiene pass removed these clean legacy external clones because matching repo-local execution clones already exist:

- `adl-wp-881`
- `adl-wp-882`
- `adl-wp-901`
- `adl-wp-902`
- `adl-wp-903`
- `adl-wp-982`
- `adl-wp-1009`
- `adl-wp-1012`
- `adl-wp-1013`
- `adl-wp-1014`
- `adl-wp-1015`
- `adl-wp-1016`
- `adl-wp-1017`
- `adl-wp-1018`
- `adl-wp-1019`
- `adl-wp-1020`
- `adl-wp-1021`
- `adl-wp-1022`
- `adl-wp-1023`
- `adl-wp-1024`
- `adl-wp-1026`
- `adl-wp-1028`

The hygiene pass also removed clean repo-local scratch directories:

- `.worktrees/burst`
- `.worktrees/write-test-clone`

The hygiene pass pruned stale git worktree registrations:

- `adl-wp-937`
- `adl-wp-939`
- `adl-wp-940`
- `adl-wp-948`

## Explicit Exceptions

- Dirty legacy external clones were intentionally retained for later backup/removal, including:
  - `adl-wp-408`, `adl-wp-409`, `adl-wp-474`, `adl-wp-481`, `adl-wp-489`
  - `adl-wp-541`, `adl-wp-573`, `adl-wp-587`, `adl-wp-599`, `adl-wp-601`
  - `adl-wp-616`, `adl-wp-702`, `adl-wp-708`, `adl-wp-741`, `adl-wp-848`
  - `adl-wp-879`, `adl-wp-886`, `adl-wp-887`, `adl-wp-967`
- Unrelated foreign lookalike directories (for example excluded low-numbered `adl-wp-*` namespaces from another project) remain outside ADL cleanup decisions.
- Codex ephemeral worktrees under `~/.codex/worktrees/` remain informational only and are not part of the managed ADL execution namespace.

## Proof Surface

Verification uses:

- `adl/tools/worktree_doctor.sh`
- `adl/tools/worktree_prune.sh`
- this report

to verify the keep-set, executed removals, retained exceptions, and final policy.
