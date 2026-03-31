#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
control_path_root="${1:-artifacts/v086/control_path}"

cd "$repo_root"

cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
  artifact validate-control-path --root "$control_path_root"
