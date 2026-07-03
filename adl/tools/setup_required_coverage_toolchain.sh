#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'USAGE'
usage: setup_required_coverage_toolchain.sh install-lld|install-nextest|configure <github-env>|verify|stats
USAGE
}

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "::error::required command is unavailable: $cmd" >&2
    exit 1
  fi
}

retry() {
  local label="$1"
  shift
  local attempt
  for attempt in 1 2 3; do
    if "$@"; then
      return 0
    fi
    if [ "$attempt" = "3" ]; then
      echo "::error::$label failed after 3 attempts" >&2
      return 1
    fi
    sleep $((attempt * 10))
  done
}

install_nextest() {
  local version="${ADL_CARGO_NEXTEST_VERSION:-0.9.138}"
  local cargo_home="${CARGO_HOME:-$HOME/.cargo}"
  local bin_dir="$cargo_home/bin"
  mkdir -p "$bin_dir"
  if command -v cargo-nextest >/dev/null 2>&1 || cargo nextest --version >/dev/null 2>&1; then
    cargo nextest --version
    return 0
  fi
  local tmp_dir
  tmp_dir="$(mktemp -d)"
  retry "download cargo-nextest ${version}" \
    curl -fsSL --retry 3 --retry-delay 2 --connect-timeout 10 --max-time 60 \
      "https://get.nexte.st/${version}/linux" -o "$tmp_dir/cargo-nextest.tar.gz"
  tar -xzf "$tmp_dir/cargo-nextest.tar.gz" -C "$tmp_dir"
  install -m 0755 "$tmp_dir/cargo-nextest" "$bin_dir/cargo-nextest"
  rm -rf "$tmp_dir"
  export PATH="$bin_dir:$PATH"
  cargo nextest --version
}

install_lld() {
  if ! command -v ld.lld >/dev/null 2>&1; then
    # GitHub-hosted runners can carry browser apt sources unrelated to Rust.
    # Disable only those transient sources, then retry apt because lld is a
    # required coverage dependency, not a best-effort accelerator.
    sudo find /etc/apt/sources.list.d -type f -name '*google-chrome*' -exec mv {} {}.disabled \; 2>/dev/null || true
    retry "apt-get update" sudo apt-get update
    retry "apt-get install lld" sudo apt-get install -y --no-install-recommends lld
  fi
  require_cmd ld.lld
  ld.lld --version
}

configure() {
  local github_env="${1:-}"
  if [ -z "$github_env" ]; then
    echo "::error::configure requires a GITHUB_ENV path" >&2
    exit 1
  fi
  require_cmd sccache
  require_cmd ld.lld
  mkdir -p "$HOME/.cache/sccache"
  {
    echo "SCCACHE_DIR=$HOME/.cache/sccache"
    echo "SCCACHE_CACHE_SIZE=2G"
    echo "RUSTC_WRAPPER=sccache"
    echo "RUSTFLAGS=-C link-arg=-fuse-ld=lld"
    echo "RUST_LINK_ACCEL=lld"
  } >> "$github_env"
  if ! sccache --start-server 2>/tmp/adl-sccache-start.err; then
    if ! sccache --show-stats >/dev/null 2>&1; then
      cat /tmp/adl-sccache-start.err >&2
      exit 1
    fi
  fi
  rm -f /tmp/adl-sccache-start.err
  sccache --zero-stats
}

verify() {
  require_cmd rustc
  require_cmd cargo
  require_cmd sccache
  require_cmd ld.lld
  rustc -vV
  cargo --version
  cargo llvm-cov --version
  cargo nextest --version
  sccache --version
  ld.lld --version
}

stats() {
  require_cmd sccache
  echo "Linker mode: ${RUST_LINK_ACCEL:-unknown}"
  sccache --show-stats
}

case "${1:-}" in
  install-lld)
    install_lld
    ;;
  install-nextest)
    install_nextest
    ;;
  configure)
    shift
    configure "${1:-}"
    ;;
  verify)
    verify
    ;;
  stats)
    stats
    ;;
  *)
    usage
    exit 2
    ;;
esac
