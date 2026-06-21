# Local Rust Linker Evaluation for `#4312`

Status: `measured_local_rust_lld_opt_in_recommendation`
Issue: `#4312`
Sprint umbrella: `#4310`
Date: 2026-06-20

## Scope

This packet evaluates faster Rust linker options for local ADL builds on the
current Apple Silicon macOS host.

It does not:

- change CI linker posture in this issue
- add repo-global local linker config
- require Homebrew or another host-wide install
- claim Linux-oriented linker guidance transfers directly to macOS

## Environment Notes

- Host target: `aarch64-apple-darwin`
- System default toolchain on `PATH`: `/usr/bin/clang`, `/usr/bin/ld`
- Not on `PATH`: `lld`, `ld.lld`, `ld64.lld`, `mold`, `zld`, `rust-lld`
- Rust toolchain sysroot does ship bundled linker binaries under
  `$(rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/`
- The issue prompt again referenced `.adl/docs/TBD/ADL_BUILD_IMPROVEMENTS.md`,
  but that path was absent in the bound worktree. This packet therefore relies
  on the tracked issue prompt, the current repo tooling, and
  `docs/milestones/v0.90.4/RUST_VALIDATION_ACCELERATION_v0.90.4.md`.

## Candidate Summary

| Candidate | Observed posture on this host | Outcome |
|---|---|---|
| System default Apple linker path | Available by default | Measured baseline |
| Direct `ld64.lld` linker-driver path from Rust sysroot | Present in sysroot, but not drop-in compatible with rustc's Darwin linker args on this host | Unsupported for this issue |
| Bundled `rust-lld` path from Rust sysroot | Present in sysroot and successfully builds ADL owner binaries | Measured alternative |
| `mold`, `zld`, PATH-level `lld` | Not installed | Recorded as unavailable, not forced |

## Measurement Method

Representative binary:

- `adl-pr-doctor`

Command families:

1. clean full build under the system default linker path
2. relink-focused rebuild by touching `adl/src/bin/adl_pr_doctor.rs` and
   rebuilding the same binary under the system default linker path
3. clean full build under bundled `rust-lld`
4. relink-focused rebuild by touching the same binary source and rebuilding
   under bundled `rust-lld`

The relink-focused rebuild is not a pure linker-only benchmark. It recompiles
the small binary crate and then relinks, which is close enough here to
distinguish full compile cost from the shorter final-stage path without adding
new benchmark tooling.

## Commands

```text
SYSROOT="$(rustc --print sysroot)"
RUST_LLD="$SYSROOT/lib/rustlib/aarch64-apple-darwin/bin/rust-lld"
LD64_LLD="$SYSROOT/lib/rustlib/aarch64-apple-darwin/bin/gcc-ld/ld64.lld"

cargo clean --manifest-path adl/Cargo.toml
/usr/bin/time -p cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor

touch adl/src/bin/adl_pr_doctor.rs
/usr/bin/time -p cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor

CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$LD64_LLD" \
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor

cargo clean --manifest-path adl/Cargo.toml
/usr/bin/time -p env CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$RUST_LLD" \
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor

touch adl/src/bin/adl_pr_doctor.rs
/usr/bin/time -p env CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$RUST_LLD" \
cargo build --quiet --manifest-path adl/Cargo.toml --bin adl-pr-doctor
```

## Timing Observations

| Surface | Result | Wall time | Interpretation |
|---|---:|---:|---|
| Clean baseline `adl-pr-doctor` build | pass | `71.18s` | Default Apple linker path full build. |
| Relink-focused baseline rebuild | pass | `1.91s` | Small compile plus relink under the default linker path. |
| Clean `rust-lld` `adl-pr-doctor` build | pass | `64.07s` | Bundled `rust-lld` improves the full build by about `7.11s` (`10.0%`). |
| Relink-focused `rust-lld` rebuild | pass | `1.65s` | Bundled `rust-lld` improves the shorter relink-focused path by about `0.26s` (`13.6%`). |

## Unsupported Direct `ld64.lld` Path

Using the bundled Mach-O driver path directly as
`CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$LD64_LLD"` failed on this host.

Observed failure shape:

- rejects `-mmacosx-version-min=11.0.0`
- rejects `-Wl,-dead_strip`
- requires explicit `-platform_version`
- reports missing or unsupported `-arch arm64`

Interpretation:

- the bundled direct driver is present, but it is not a drop-in replacement
  for rustc's current Darwin linker invocation on this host
- this issue should not recommend a generic "just point cargo at `ld64.lld`"
  posture

## Recommendation

Recommend optional local use of bundled `rust-lld` on Apple Silicon macOS when
contributors are spending meaningful time in repeated Rust build loops and want
a modest but real local improvement without adding a tracked repo-global
linker policy.

Do not recommend:

- forcing a repo-global local linker config in this issue
- assuming `lld` on `PATH` exists on macOS
- documenting direct `ld64.lld` as a supported drop-in linker driver on this
  host

Suggested local opt-in command shape:

```bash
export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$(rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/rust-lld"
```

For a single-command experiment instead of a shell-wide export:

```bash
env CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$(rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/rust-lld" \
  cargo build --manifest-path adl/Cargo.toml --bin adl-pr-doctor
```

## Non-Claims

- This packet does not claim `rust-lld` is universally faster on every macOS
  host.
- This packet does not claim the direct `ld64.lld` driver path is usable on
  this machine.
- This packet does not claim CI should change in `#4312`.
- This packet does not claim the relink-focused rebuild is a pure linker-only
  benchmark.
