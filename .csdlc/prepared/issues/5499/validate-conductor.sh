#!/usr/bin/env bash
set -euo pipefail

ruby .csdlc/prepared/issues/5499/check-dependencies.rb

manifest="adl-v2/crates/adl-workcell-conductor/Cargo.toml"
if [[ ! -f "$manifest" ]]; then
  printf 'missing planned conductor manifest: %s\n' "$manifest" >&2
  exit 2
fi

target_dir="${CARGO_TARGET_DIR:-/Volumes/FastWork/adl-5499/conductor-target}"
cargo test --locked --offline --manifest-path "$manifest" --target-dir "$target_dir" --all-targets
cargo clippy --locked --offline --manifest-path "$manifest" --target-dir "$target_dir" --all-targets -- -D warnings
