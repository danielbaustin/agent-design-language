# Local `sccache` Evaluation for `#4311`

Status: `measured_local_opt_in_recommendation`
Issue: `#4311`
Sprint umbrella: `#4310`
Date: 2026-06-20

## Scope

This packet evaluates `sccache` as a local opt-in acceleration path for
repeated ADL Rust validation work.

It does not:

- require CI adoption in this issue
- change validation semantics
- require a machine-specific tracked path in the repo
- claim `sccache` should be globally forced for all contributors

## Environment Notes

- Local host did not have `sccache` preinstalled.
- Homebrew is available on this machine, but the proof run intentionally did
  not mutate the host-wide toolchain.
- For proof, `sccache v0.16.0` was installed into a disposable temp root at
  `/private/tmp/adl-sccache-4311`.
- The issue prompt again referenced `.adl/docs/TBD/ADL_BUILD_IMPROVEMENTS.md`,
  but that path was absent in the bound worktree. This packet therefore relies
  on the tracked issue prompt, the current repo tooling, and the earlier CI
  acceleration notes in
  `docs/milestones/v0.90.4/RUST_VALIDATION_ACCELERATION_v0.90.4.md`.

## Portable Setup Guidance

Recommended local setup stays opt-in and environment-driven.

### macOS

```bash
brew install sccache
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR="$HOME/.cache/sccache"
```

### Generic fallback

If package-manager install is unavailable, a local user-scoped install is also
acceptable:

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR="$HOME/.cache/sccache"
```

Notes:

- keep `SCCACHE_DIR` outside the repo checkout
- do not commit machine-local shell config to the repo
- treat this as local operator acceleration, not repo-required state

## Measurement Method

Representative command family:

- single owner binary: `adl-pr-doctor`
- adjacent owner binary: `adl-pr-ready`

Proof shape:

1. measure a clean baseline build without `sccache`
2. clean again, zero stats, and measure first `sccache` build
3. clean `target` again and measure a second build against the warmed cache
4. clean `target` again and measure an adjacent binary build against the same
   warmed cache

Cleaning `target` between runs matters here: it forces a real rebuild so the
second and third `sccache` runs must prove compiler-output reuse rather than
just “Cargo had nothing to do.”

## Commands

```text
CARGO_HOME=/private/tmp/adl-sccache-cargo-home-4311 cargo install sccache --root /private/tmp/adl-sccache-4311

cargo clean --manifest-path adl/Cargo.toml
/usr/bin/time -p cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor

cargo clean --manifest-path adl/Cargo.toml
/private/tmp/adl-sccache-4311/bin/sccache --zero-stats
/usr/bin/time -p env RUSTC_WRAPPER=/private/tmp/adl-sccache-4311/bin/sccache \
SCCACHE_DIR=/private/tmp/adl-sccache-cache-4311 \
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor
/private/tmp/adl-sccache-4311/bin/sccache --show-stats

cargo clean --manifest-path adl/Cargo.toml
/usr/bin/time -p env RUSTC_WRAPPER=/private/tmp/adl-sccache-4311/bin/sccache \
SCCACHE_DIR=/private/tmp/adl-sccache-cache-4311 \
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor
/private/tmp/adl-sccache-4311/bin/sccache --show-stats

cargo clean --manifest-path adl/Cargo.toml
/usr/bin/time -p env RUSTC_WRAPPER=/private/tmp/adl-sccache-4311/bin/sccache \
SCCACHE_DIR=/private/tmp/adl-sccache-cache-4311 \
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-ready
/private/tmp/adl-sccache-4311/bin/sccache --show-stats
```

## Timing Observations

| Surface | Result | Wall time | Interpretation |
|---|---:|---:|---|
| Clean baseline `adl-pr-doctor` build | pass | `68.10s` | Baseline cost without compiler-output cache. |
| Cold `sccache` `adl-pr-doctor` build | pass | `85.66s` | First cache-fill pass is slower than baseline because it pays cache overhead while still compiling everything. |
| Warm `sccache` `adl-pr-doctor` build after `target` clean | pass | `46.04s` | Repeated clean rebuild is materially faster once compiler outputs are cached. |
| Warm `sccache` adjacent `adl-pr-ready` build after `target` clean | pass | `47.78s` | Neighboring owner binary also benefits from cache reuse. |

## Cache Stats

### After first `sccache` build

- Rust cache hits: `0`
- Rust cache misses: `250`
- Overall hit rate: `0.00%`
- Rust hit rate: `0.00%`

### After second `sccache` build

- Rust cache hits: `250`
- Rust cache misses: `250`
- Overall hit rate: `50.00%`
- Rust hit rate: `50.00%`

### After adjacent `adl-pr-ready` build

- Rust cache hits: `500`
- Rust cache misses: `250`
- Overall hit rate: `66.67%`
- Rust hit rate: `66.67%`

## What This Means

1. `sccache` is not a free win for the first build.
   On this machine, the first cached build was about `17.56s` slower than the
   uncached baseline.
2. `sccache` is a real win for repeated clean-ish rebuilds.
   The second clean rebuild dropped from `68.10s` baseline to `46.04s`, about
   a `32%` wall-time reduction.
3. The benefit carries across adjacent owner binaries.
   `adl-pr-ready` built in `47.78s` against the warmed cache, which is close to
   the warmed `adl-pr-doctor` result and much better than paying a fresh cold
   compile every time.

## Recommendation

Recommend local opt-in use of `sccache` for contributors who repeatedly run
Rust owner binaries, focused Rust validation, or clean-ish rebuild loops during
active issue work.

Do not recommend forcing it as mandatory tracked repo state in this issue.

Suggested guidance:

- enable `sccache` when doing repeated Rust validation during an active coding
  session
- keep the cache directory outside the repo
- expect the first build to be slower or neutral
- expect the payoff on the second and later rebuilds, especially across
  adjacent binaries or repeated branch/worktree loops

## Non-Claims

- This packet does not claim universal benefit for every local host.
- This packet does not claim incremental no-op rebuilds need `sccache`.
- This packet does not claim CI should change in `#4311`.
- This packet does not claim linker acceleration and `sccache` are equivalent;
  they address different parts of the build path.
