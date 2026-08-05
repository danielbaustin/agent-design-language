# Hardlinked Rust Dependency Cache Warmup

ADL issue worktrees often pay avoidable Rust cold-build cost. In one EC2 warm-build comparison, warming build state cut build time by roughly 60%. This helper turns that lesson into a small, explicit local tool without introducing a shared global `CARGO_TARGET_DIR`.

The design follows the dependency-artifact hardlinking pattern described in Howard John, [Sharing Rust Build Cache](https://blog.howardjohn.info/posts/shared-rust-build/): do not share one target directory, and do not copy mutable workspace outputs. Instead, reuse dependency artifacts by hardlinking them into a fresh target directory. ADL's first helper uses `Cargo.lock` rather than live registry metadata so dry-run and review paths stay local and predictable, and it starts with the safer `deps/` artifact surface only. Cargo dep-info files (`*.d`) are excluded because they can contain donor-target build/source paths; they should not be warmed unless dep-info rewriting is implemented and proven.

## Tool

```sh
python3 adl/tools/warm_rust_dependency_cache.py \
  --source-target /path/to/warm/target \
  --dest-target /path/to/issue/worktree/adl/target \
  --manifest-path /path/to/issue/worktree/adl/Cargo.toml \
  --dry-run \
  --json
```

Remove `--dry-run` only after inspecting the JSON summary.

## Safety Contract

- The tool uses `Cargo.lock` package names as a local, deterministic dependency classifier.
- The current manifest package and explicit workspace-member packages are excluded from the eligible prefix set.
- Files under `target/<profile>/deps` and dependency `.fingerprint` entries are eligible for warmup; `build` metadata is intentionally not hardlinked.
- Cargo dep-info files (`*.d`) are intentionally skipped because they can encode paths from the source target.
- Dependency package artifacts are eligible for warmup when their filenames match lockfile-derived prefixes.
- Workspace package outputs are not selected intentionally.
- Existing destination files are skipped unless `--replace` is supplied.
- Missing source target/profile directories fail closed.
- The tool does not set or require a shared `CARGO_TARGET_DIR`.
- The warmup is an optimization only; correctness still belongs to Cargo and the normal validation lane.

## Recommended ADL Usage

Use the shared shell wrapper before Rust-heavy validation in a fresh or cold
issue worktree when a trusted warm source target already exists on the same
host and filesystem:

```sh
bash adl/tools/rust_validation_warm_cache.sh
```

The wrapper computes deterministic defaults from the current checkout. In an
ADL issue worktree under `.worktrees/adl-wp-*`, it automatically prefers the
primary checkout's `adl/target` as the warm source when that target exists. It
respects `CARGO_TARGET_DIR` for the destination target, emits one JSON status
payload, and skips safely when no trusted source target is available. Typical
places to use it:

- after `pr run <issue>` binds a fresh issue worktree and before the first
  focused Rust validation command
- before owner-lane validation such as
  `bash adl/tools/run_owner_validation_lane.sh csdlc|runtime|review|all`
- on EC2 or remote builders after checkout/worktree setup and before the first
  Rust validation pass

For local issue worktrees, use a trusted warm source target from the same
checkout family and toolchain:

```sh
ADL_RUST_WARM_CACHE_SOURCE_TARGET=/Users/daniel/git/agent-design-language/adl/target \
ADL_RUST_WARM_CACHE_DEST_TARGET=/Users/daniel/git/agent-design-language/.worktrees/adl-wp-XXXX/adl/target \
ADL_RUST_WARM_CACHE_MANIFEST_PATH=/Users/daniel/git/agent-design-language/.worktrees/adl-wp-XXXX/adl/Cargo.toml \
  bash adl/tools/rust_validation_warm_cache.sh
```

For EC2 or remote builders, warm from the local persistent target cache on that host before running focused validation. Keep the source and destination on the same filesystem so hardlinks work.

The lower-level Python helper remains available for direct dry-run inspection:

```sh
python3 adl/tools/warm_rust_dependency_cache.py \
  --source-target /path/to/warm/target \
  --dest-target /path/to/issue/worktree/adl/target \
  --manifest-path /path/to/issue/worktree/adl/Cargo.toml \
  --dry-run \
  --json
```

## Agent Workflow Integration

Agents should not rely on memory for this optimization. The repo surfaces this
helper through:

- root `AGENTS.md`, under the normal issue execution and validation flow
- `adl/tools/skills/pr-run/SKILL.md`, before bounded validation
- typed v2 doctor/bind guidance, when routing to execution
- `adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md`, under build acceleration
- `adl/config/validation_lane_selector.v0.91.6.json`, which assigns the helper
  and its contract test to `rust_dependency_cache_warmup_contracts`

The helper has its own focused behavior test:

```sh
python3 adl/tools/test_warm_rust_dependency_cache.py
bash adl/tools/test_rust_validation_warm_cache.sh
```

That test proves dependency artifacts and dependency fingerprint files are
linked, Cargo dep-info files are not linked, workspace outputs are not linked,
`build` outputs are not linked, and `--replace` is explicit.

## Non-Claims

This does not replace sccache, remote builders, PVF scheduling,
validation-manager routing, or any required validation lane. It is a standalone
worktree warmup primitive that agents and future tooling can call before Rust
validation to reduce cold-build cost.
