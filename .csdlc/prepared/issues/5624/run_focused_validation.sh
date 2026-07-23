#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
target_root="/Volumes/FastWork/adl-5624-cargo-target"
export CARGO_TARGET_DIR="$target_root"

cd "$repo_root"
cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check
bash adl/tools/test_run_cargo_validation.sh
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --test gate10a
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --test gate7 prune_guard
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --test gate7_lifecycle prune
cargo test --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --all-targets
cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --target-dir "$target_root" --all-targets -- -D warnings
