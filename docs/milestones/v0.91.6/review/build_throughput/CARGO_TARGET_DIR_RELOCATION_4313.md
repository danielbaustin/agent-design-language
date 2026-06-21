# Local `CARGO_TARGET_DIR` Relocation Strategy for `#4313`

Status: `measured_relocation_strategy_ready_for_local_opt_in`
Issue: `#4313`
Sprint umbrella: `#4310`
Date: 2026-06-20

## Scope

This packet defines a safe local `CARGO_TARGET_DIR` relocation strategy for
ADL so Rust build artifacts can move off the system volume while preserving
worktree isolation and cleanup safety.

It does not:

- force one machine-specific path for every contributor
- commit repo-global local Cargo config in this issue
- share one target directory across multiple incompatible worktrees
- claim direct host-side proof on `/Volumes/FastWork` from this Codex session
  when the session permission profile blocked writes there

## Host Observations

- System data volume free space observed from this session:
  - `/System/Volumes/Data`: about `274Gi` available
- External/local large-volume candidates observed from this session:
  - `/Volumes/FastWork`: mounted, about `1.8Ti` available
  - `/Volumes/models`: mounted, about `1.5Ti` available
- Several existing ADL worktrees still carry large system-volume `adl/target`
  directories:
  - `26G` at `adl-wp-4246/adl/target`
  - `17G` at `adl-wp-4248/adl/target`
  - `14G` at `adl-wp-4298/adl/target`
  - `12G` at `adl-process-status-fanout/adl/target`
  - other visible worktree targets sum to about `99.6G` total across the
    sampled set

Interpretation:

- target artifacts are a real disk-pressure source on the system volume
- `/Volumes/FastWork` is a plausible operator-approved destination because it
  exists, is nearly empty, and has far more free space than the system volume

## Recommended Layout

Recommended root:

```text
/Volumes/FastWork/adl-cargo-targets/agent-design-language
```

Recommended leaf strategy:

- one target leaf for the main checkout
- one target leaf per issue worktree
- leaf name derived from the checkout/worktree top-level directory basename

Examples:

```text
/Volumes/FastWork/adl-cargo-targets/agent-design-language/main
/Volumes/FastWork/adl-cargo-targets/agent-design-language/adl-wp-4313
```

This keeps artifacts isolated per checkout while still moving them off the
system volume.

## Setup Guidance

### Main checkout

```bash
export ADL_CARGO_TARGET_ROOT=/Volumes/FastWork/adl-cargo-targets/agent-design-language
export CARGO_TARGET_DIR="$ADL_CARGO_TARGET_ROOT/main"
mkdir -p "$CARGO_TARGET_DIR"
```

### Issue worktree

```bash
export ADL_CARGO_TARGET_ROOT=/Volumes/FastWork/adl-cargo-targets/agent-design-language
export CARGO_TARGET_DIR="$ADL_CARGO_TARGET_ROOT/$(basename "$(git rev-parse --show-toplevel)")"
mkdir -p "$CARGO_TARGET_DIR"
```

Notes:

- keep the target root outside the checkout
- keep one leaf per checkout/worktree
- do not set all worktrees to the same shared leaf
- prefer shell-local or session-local exports over committed machine-local repo
  config

## Proof Run

This Codex session could not write directly to `/Volumes/FastWork` because the
session filesystem policy denied non-workspace volume writes. To prove the
relocation mechanics anyway, the same absolute `CARGO_TARGET_DIR` strategy was
executed under a writable absolute probe root:

```text
/private/tmp/adl-build-targets/adl-wp-4313-proof
```

Representative proof commands:

```text
env CARGO_TARGET_DIR=/private/tmp/adl-build-targets/adl-wp-4313-proof \
  cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor

env CARGO_TARGET_DIR=/private/tmp/adl-build-targets/adl-wp-4313-proof \
  bash adl/tools/validate_structured_prompt.sh --type spp --phase execution \
  --input .adl/v0.91.6/tasks/issue-4313__move-cargo-target-artifacts-off-system-volume/spp.md
```

## Proof Results

| Surface | Result | Wall time | Notes |
|---|---:|---:|---|
| Relocated absolute target-dir build for `adl-pr-doctor` | pass | `83.77s` | Full owner-binary build succeeded with artifacts written entirely outside the checkout tree. |
| Relocated absolute target-dir prompt validator run | pass | `0.02s` | Validator path honored the relocated target-dir setup without falling back to the checkout-local `target/`. |

Observed probe artifact footprint:

- `2.1G` at `/private/tmp/adl-build-targets/adl-wp-4313-proof`

Interpretation:

- ADL build and validator flows do work with a relocated absolute
  `CARGO_TARGET_DIR`
- a single worktree can easily emit multiple gigabytes of artifacts
- moving those leaves off the system volume is worth doing for active local
  worktrees

## Cleanup Guidance

Safe cleanup rule:

- delete one target leaf at a time
- never delete the whole external target root unless you truly intend to wipe
  every checkout/worktree cache
- never point cleanup commands at the checkout itself

Examples:

```bash
rm -rf /Volumes/FastWork/adl-cargo-targets/agent-design-language/adl-wp-4313
rm -rf /Volumes/FastWork/adl-cargo-targets/agent-design-language/main
```

Inspection before cleanup:

```bash
find /Volumes/FastWork/adl-cargo-targets/agent-design-language -mindepth 1 -maxdepth 1 -type d | sort
du -sh /Volumes/FastWork/adl-cargo-targets/agent-design-language/*
```

Avoid:

- `rm -rf /Volumes/FastWork/adl-cargo-targets/agent-design-language`
- sharing one leaf such as `/Volumes/FastWork/adl-cargo-targets/agent-design-language/shared`
  across all worktrees
- placing proof artifacts or evidence inside the target leaf

## Recommendation

Recommend local opt-in adoption of a per-checkout/per-worktree
`CARGO_TARGET_DIR` layout rooted at `/Volumes/FastWork` for this machine.

Why this shape:

- it moves large Rust artifacts off the system volume
- it preserves worktree isolation
- it gives cleanup a safe unit of deletion
- it avoids repo-global machine-local config churn

## Non-Claims

- This packet does not claim every operator has `/Volumes/FastWork`.
- This packet does not claim direct `/Volumes/FastWork` write proof was run
  from this session.
- This packet does not claim one shared target leaf is safe across all
  worktrees.
- This packet does not claim CI should adopt the same target-root path.
