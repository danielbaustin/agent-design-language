#!/usr/bin/env bash
set -euo pipefail

if ! git diff --quiet -- adl/Cargo.lock; then
  echo "Cargo.lock is dirty before Memory Palace validation; refusing to overwrite it." >&2
  exit 2
fi

restore_lock() {
  if ! git diff --quiet -- adl/Cargo.lock; then
    git restore adl/Cargo.lock
    echo "Restored transient Cargo.lock refresh after offline focused validation." >&2
  fi
}
trap restore_lock EXIT

cargo test --offline --manifest-path adl/Cargo.toml memory_palace -- --nocapture
