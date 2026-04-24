# Rust Validation Acceleration For v0.90.4

## Purpose

Record the bounded cache and linker posture chosen for the current Rust
validation workflow so operators can understand what was changed, why it was
changed, and what was intentionally left out.

## Baseline

Recent successful CI before this change showed that Rust validation was already
down from the earlier 20-40 minute failure era, but compile-heavy steps still
dominated the remaining wall time.

Observed on successful run `24882019723`:

- `adl-ci`: about `119s`
- `adl-coverage`: about `447s`
- `adl-coverage` step `Coverage run and summary (json)`: about `409s`

Interpretation:

- the remaining long pole is still compile-and-link work inside the full
  coverage lane
- both CI jobs perform Rust compilation on the same PR event
- target-directory caching helps, but the workflow had no compiler-output cache
  and no explicit linker acceleration posture

## Chosen Improvements

This issue implements three bounded changes:

1. Install and enable `sccache` in both `adl-ci` and `adl-coverage`
2. Persist `~/.cache/sccache` through the existing Rust cache action
3. Install and assert `lld` on GitHub-hosted Linux runners

Operational posture:

- `RUSTC_WRAPPER=sccache` is set only in CI
- `SCCACHE_DIR=$HOME/.cache/sccache` is persisted by `Swatinem/rust-cache`
- `lld` is installed in CI before Rust acceleration is configured
- `RUSTFLAGS=-C link-arg=-fuse-ld=lld` is set only after `ld.lld` is verified
- the workflow fails closed if `ld.lld` is still unavailable after install
- each job emits `sccache --show-stats` so operators can verify whether the
  cache is helping in practice

## Why This Shape

This is intentionally conservative.

- It improves repeated CI compiles without forcing a host-specific local tool
  chain on every contributor.
- It does not assume `mold` or another non-default linker is installed.
- It keeps the repo portable locally while making the hosted CI runner posture
  deterministic.
- It produces visible cache stats in the job logs so we can judge whether the
  posture is working.

## Explicit Rejections

Rejected for this issue:

- mandatory `mold`
  Reason: not guaranteed on the GitHub-hosted runners we already use, and it
  would widen infrastructure surface beyond a bounded tuning change.
- tracked repo-global local linker config
  Reason: local host assumptions should not silently become required checkout
  state.
- crate/workspace decomposition
  Reason: that belongs to a different issue class and would be much larger than
  cache/linker tuning.

## Local Workflow Guidance

The repo does not force local developers onto `sccache` or `lld`, but the CI
choice is intentionally compatible with local opt-in use.

If a local operator wants the same posture:

- install `sccache`
- set `RUSTC_WRAPPER=sccache`
- optionally use `lld` when it is available on the local host

That local setup remains optional and out of the tracked repo contract.

## Expected Outcome

The original opportunistic linker posture proved insufficient in practice:

- successful run `24884703780` still logged `RUST_LINK_ACCEL=system-default`
- `adl-coverage` rose from about `447s` baseline to about `600s`
- `Coverage run and summary (json)` rose from about `409s` to about `555s`
- `sccache` was active but only reached about `46.65%` Rust cache hit rate

That evidence means cache help alone is not enough; the linker path must be
truthful and active on the hosted runners.

This issue still does not claim a guaranteed sub-5-minute total validation wall
time by itself. It should make the linker posture truthful, reduce repeated
compile/link cost, and make remaining compile work more observable.

Success criteria for this posture:

- CI stays green on the current workflow
- `sccache` stats appear in job logs
- repeated runs have a chance to reuse compiler outputs instead of paying only
  target-directory cache restores
- linker acceleration is truthfully active in CI rather than silently dormant
