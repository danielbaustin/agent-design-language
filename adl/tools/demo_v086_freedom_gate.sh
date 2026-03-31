#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

out_dir="${1:-artifacts/v086/freedom_gate}"

cargo run --quiet --manifest-path adl/Cargo.toml --bin demo_v086_freedom_gate -- "$out_dir"
