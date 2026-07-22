#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
shopt -s nullglob
manifests=("$root"/adl-v2/crates/*/Cargo.toml)

if [ "${#manifests[@]}" -eq 0 ]; then
  echo "no ADL v2 crate manifests found" >&2
  exit 2
fi

for manifest in "${manifests[@]}"; do
  relative_manifest="${manifest#"$root/"}"
  echo "Validating $relative_manifest"
  bash "$root/adl/tools/run_cargo_validation.sh" cargo test --locked --manifest-path "$manifest"
  bash "$root/adl/tools/run_cargo_validation.sh" cargo fmt --manifest-path "$manifest" --all -- --check
  bash "$root/adl/tools/run_cargo_validation.sh" cargo clippy --locked --manifest-path "$manifest" --all-targets -- -D warnings
done
