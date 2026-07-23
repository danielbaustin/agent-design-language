#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
fast_root="${ADL_WP5340_FAST_ROOT:-/Volumes/FastWork/adl-wp-5340}"
manifest="${repo_root}/.csdlc/prepared/issues/5340/source-authority-validator/Cargo.toml"
mkdir -p "${fast_root}"
fast_root="$(ruby -e 'puts File.realpath(ARGV.fetch(0))' "${fast_root}")"
case "${fast_root}" in /Volumes/FastWork/*) ;; *) echo "validator cache root escaped FastWork" >&2; exit 26 ;; esac
mkdir -p "${fast_root}/cargo-home" "${fast_root}/source-authority-target" "${fast_root}/sccache" "${fast_root}/tmp"

export CARGO_HOME="${fast_root}/cargo-home"
export CARGO_TARGET_DIR="${fast_root}/source-authority-target"
export SCCACHE_DIR="${fast_root}/sccache"
export TMPDIR="${fast_root}/tmp"
for variable in CARGO_HOME CARGO_TARGET_DIR SCCACHE_DIR TMPDIR; do
  canonical="$(ruby -e 'puts File.realpath(ARGV.fetch(0))' "${!variable}")"
  case "${canonical}" in /Volumes/FastWork/*) ;; *) echo "${variable} escaped FastWork" >&2; exit 26 ;; esac
done
export CARGO_INCREMENTAL=0
export CARGO_NET_OFFLINE=false
unset CARGO_BUILD_RUSTC_WRAPPER CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER
cargo fetch --locked --manifest-path "${manifest}"
