#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

out_dir="${1:-artifacts/v086/control_path}"

cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- demo demo-g-v086-control-path --run --trace --out "$out_dir"
