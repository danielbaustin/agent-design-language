#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/setup_required_coverage_toolchain.sh"

require_or_skip() {
  local command_name="$1"
  local install_note="$2"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "SKIP test_setup_required_coverage_toolchain: $command_name is unavailable; $install_note"
    exit 0
  fi
}

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
env_file="$tmp_dir/github-env"

require_or_skip rustc "install the Rust toolchain"
require_or_skip cargo "install the Rust toolchain"
require_or_skip cargo-llvm-cov "install cargo-llvm-cov before running this contract"
require_or_skip cargo-nextest "install cargo-nextest before running this contract"
require_or_skip sccache "install sccache before running this contract"
require_or_skip ld.lld "install lld before running this contract"

"$SCRIPT" configure "$env_file" >/dev/null
"$SCRIPT" verify >/dev/null
RUST_LINK_ACCEL=lld "$SCRIPT" stats >/dev/null

grep -Fx 'RUSTC_WRAPPER=sccache' "$env_file" >/dev/null
grep -Fx 'RUSTFLAGS=-C link-arg=-fuse-ld=lld' "$env_file" >/dev/null
grep -Fx 'RUST_LINK_ACCEL=lld' "$env_file" >/dev/null

echo "PASS test_setup_required_coverage_toolchain"
