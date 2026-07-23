#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
WRAPPER="$ROOT_DIR/adl/tools/run_cargo_validation.sh"

if [[ ! -x "$WRAPPER" ]]; then
  echo "portable Cargo validation wrapper is not implemented: $WRAPPER" >&2
  exit 1
fi

cd "$ROOT_DIR"
bash "$WRAPPER" cargo test --locked --manifest-path csdlc-v2/Cargo.toml
bash "$WRAPPER" cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check
exec bash "$WRAPPER" cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings
