#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT_DIR/adl/tools/setup_required_coverage_toolchain.sh"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

if "$SCRIPT" >/dev/null 2>&1; then
  echo "expected setup script with no subcommand to fail" >&2
  exit 1
fi

if "$SCRIPT" configure >/dev/null 2>"$tmp_dir/configure.err"; then
  echo "expected configure without GITHUB_ENV path to fail" >&2
  exit 1
fi
grep -F "configure requires a GITHUB_ENV path" "$tmp_dir/configure.err" >/dev/null

if command -v rustc >/dev/null 2>&1 \
  && command -v cargo >/dev/null 2>&1 \
  && command -v sccache >/dev/null 2>&1 \
  && command -v ld.lld >/dev/null 2>&1 \
  && cargo llvm-cov --version >/dev/null 2>&1 \
  && cargo nextest --version >/dev/null 2>&1; then
  env_file="$tmp_dir/github-env"
  "$SCRIPT" verify >/dev/null
  "$SCRIPT" configure "$env_file" >/dev/null
  RUST_LINK_ACCEL=lld "$SCRIPT" stats >/dev/null
  grep -Fx 'RUSTC_WRAPPER=sccache' "$env_file" >/dev/null
  grep -Fx 'RUSTFLAGS=-C link-arg=-fuse-ld=lld' "$env_file" >/dev/null
  grep -Fx 'RUST_LINK_ACCEL=lld' "$env_file" >/dev/null
else
  if "$SCRIPT" verify >/dev/null 2>"$tmp_dir/verify.err"; then
    echo "expected verify to fail when one or more real required tools are unavailable" >&2
    exit 1
  fi
  grep -F "required command is unavailable" "$tmp_dir/verify.err" >/dev/null
fi

echo "PASS test_setup_required_coverage_toolchain"
