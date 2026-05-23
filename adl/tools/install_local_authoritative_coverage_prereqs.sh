#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/install_local_authoritative_coverage_prereqs.sh

Install local operator-side prerequisites for
adl/tools/run_local_authoritative_coverage_gate.sh.

This script exists to keep prerequisite installation separate from validation.
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

command -v cargo >/dev/null 2>&1 || {
  echo "cargo is required to install local authoritative coverage prerequisites" >&2
  exit 1
}

ensure_jq() {
  if command -v jq >/dev/null 2>&1; then
    echo "jq already available."
    return 0
  fi

  if command -v brew >/dev/null 2>&1; then
    echo "Installing jq via Homebrew..."
    brew install jq
  elif command -v apt-get >/dev/null 2>&1; then
    echo "Installing jq via apt-get..."
    sudo apt-get update
    sudo apt-get install -y jq
  else
    echo "jq is required for local authoritative coverage validation." >&2
    echo "Install jq manually because no supported package manager was detected." >&2
    exit 1
  fi

  command -v jq >/dev/null 2>&1 || {
    echo "jq installation did not produce a runnable jq command" >&2
    exit 1
  }
}

ensure_llvm_tools_preview() {
  command -v rustup >/dev/null 2>&1 || {
    echo "rustup is required to install llvm-tools-preview for local authoritative coverage validation" >&2
    exit 1
  }

  if rustup component list --installed | grep -Fxq llvm-tools-preview; then
    echo "llvm-tools-preview already installed."
    return 0
  fi

  echo "Installing llvm-tools-preview via rustup..."
  rustup component add llvm-tools-preview
}

ensure_cargo_tool() {
  local subcommand="$1"
  local package_name="$2"

  if cargo "$subcommand" --version >/dev/null 2>&1; then
    echo "$package_name already available."
    return 0
  fi

  echo "Installing $package_name for local authoritative coverage validation..."
  cargo install "$package_name" --locked
  cargo "$subcommand" --version >/dev/null 2>&1 || {
    echo "$package_name installation did not produce a runnable cargo $subcommand command" >&2
    exit 1
  }
}

ensure_jq
ensure_llvm_tools_preview
ensure_cargo_tool "llvm-cov" "cargo-llvm-cov"
ensure_cargo_tool "nextest" "cargo-nextest"

echo "Local authoritative coverage prerequisites installed successfully."
