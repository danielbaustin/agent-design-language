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

if cargo nextest --version >/dev/null 2>&1; then
  echo "cargo-nextest already available."
  exit 0
fi

echo "Installing cargo-nextest for local authoritative coverage validation..."
cargo install cargo-nextest --locked
cargo nextest --version >/dev/null 2>&1 || {
  echo "cargo-nextest installation did not produce a runnable cargo nextest command" >&2
  exit 1
}

echo "cargo-nextest installed successfully."
