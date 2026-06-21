# Safe Build Artifact Cleanup Policy for `#4314`

Status: `report_first_cleanup_policy_ready`
Issue: `#4314`
Sprint umbrella: `#4310`
Date: 2026-06-20

## Scope

This packet defines a safe local cleanup policy for stale Rust build artifacts
in ADL main and worktree checkouts.

It does not:

- delete worktrees wholesale
- delete dirty source, docs, or evidence changes
- delete `.codex`, user caches, or unrelated host directories
- assume every large `adl/target` directory is safe to remove immediately

## Cleanup Goal

Keep build artifacts bounded while preserving:

- active issue worktrees
- source and documentation changes
- retained review/evidence artifacts
- the per-worktree target isolation model established by `#4313`

## Current Cleanup Pressure

Visible `adl/target` directories in the repo worktree fleet still consume
substantial disk:

- `26G` at `adl-wp-4246/adl/target`
- `17G` at `adl-wp-4248/adl/target`
- `14G` at `adl-wp-4298/adl/target`
- `14G` at `adl-wp-4299/adl/target`
- `12G` at `adl-process-status-fanout/adl/target`
- several additional worktree target trees in the `2G` to `5G` range

Interpretation:

- stale build artifacts are a real system-volume pressure source
- cleanup must be selective because some large targets still belong to active
  worktrees

## Classification Model

### Safe to preserve

Preserve target artifacts for worktrees that are still active or ambiguous:

- active run-bound issues such as `#4298` and `#4299`
- any worktree with dirty tracked source/doc/evidence changes
- detached/manual utility worktrees unless the operator explicitly classifies
  them as disposable

### Candidate for target-only cleanup

Target-only cleanup is safe when all of these are true:

1. the issue is already closed or otherwise no longer active for execution
2. the worktree has no dirty tracked source/doc/evidence changes
3. the cleanup removes only `adl/target`
4. the worktree root itself is preserved

### Not safe by default

Do not automatically clean:

- worktrees still in `run_bound` state
- worktrees with modified tracked files
- shared relocated target roots that may back multiple active leaves
- root directories such as `/Volumes/FastWork/adl-cargo-targets/agent-design-language`

## Recommended Report-First Workflow

### 1. Inventory large targets

```bash
for d in /Users/daniel/git/agent-design-language/.worktrees/*/adl/target; do
  [ -d "$d" ] && du -sh "$d"
done | sort -h
```

### 2. Classify the owning worktree before deletion

Check:

- worktree path and branch
- whether tracked files are dirty
- whether the issue is still open/run-bound

Representative commands:

```bash
git worktree list --porcelain
git -C /path/to/worktree status --short --branch
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token \
  adl-pr-doctor <issue> --json
```

### 3. Delete only the target leaf

```bash
rm -rf /path/to/worktree/adl/target
```

### 4. Re-check the worktree

```bash
git -C /path/to/worktree status --short --branch
```

If the post-cleanup git status changes, stop and investigate.

## Proof: One Safe Cleanup

Selected proof worktree:

- `adl-wp-4305`

Why it qualified:

- `adl-pr-doctor 4305 --json` reported the issue as closed and blocked on stale
  canonical closeout truth rather than active execution
- `git status --short --branch` in the worktree showed no dirty tracked files
- the worktree still had `2.4G` under `adl/target`

Proof commands:

```text
du -sh /path/to/closed-clean-worktree/adl/target
git -C /path/to/closed-clean-worktree status --short --branch
rm -rf /path/to/closed-clean-worktree/adl/target
git -C /path/to/closed-clean-worktree status --short --branch
```

Proof result:

- before: `2.4G`
- after: `adl/target` absent
- post-cleanup git status unchanged

Interpretation:

- target-only cleanup can reclaim meaningful disk for closed issue worktrees
  without touching tracked work

## Active-Worktree Preservation Proof

Representative preserve examples:

- `#4298` remains `run_bound` and still has about `14G` in `adl/target`
- `#4299` remains `run_bound` and still has about `14G` in `adl/target`

Policy consequence:

- do not delete their target trees in routine cleanup
- defer those targets until the issue lifecycle is actually closed or the
  operator explicitly authorizes cleanup

## `cargo sweep` Evaluation

Observed on this host:

- `cargo-sweep` / `cargo sweep` is not installed

Recommendation:

- do not require `cargo sweep` for routine ADL cleanup in this issue
- prefer explicit worktree classification plus target-leaf deletion
- age-based cleanup may be a future follow-on, but it is not needed to
  establish the safe baseline policy

## Cleanup Rules

1. Never delete a worktree root as part of ordinary build-artifact cleanup.
2. Never delete dirty tracked source/doc/evidence changes.
3. Never clean active run-bound issue worktrees unless explicitly requested.
4. Prefer deleting one `adl/target` leaf at a time.
5. For relocated targets from `#4313`, delete one leaf such as
   `$ADL_CARGO_TARGET_ROOT/adl-wp-4313`,
   not the whole root.
6. Re-run `git status --short --branch` after cleanup to confirm only build
   artifacts were removed.

## Recommended Runbook

For ordinary cleanup:

1. inventory large targets
2. classify each owner worktree
3. skip active or dirty worktrees
4. remove only `adl/target` for clean closed worktrees
5. verify post-cleanup git status

## Non-Claims

- This packet does not claim every stale target should be deleted now.
- This packet does not claim detached/manual worktrees are safe to clean
  automatically.
- This packet does not claim `cargo sweep` is required.
- This packet does not claim active issue worktrees should lose cached build
  artifacts during routine hygiene.
