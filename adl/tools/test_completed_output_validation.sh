#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[[ ! -e "$ROOT/adl/tools/validate_structured_prompt.sh" ]] || {
  echo "retired structured-prompt wrapper unexpectedly exists" >&2
  exit 1
}
cargo run --quiet --locked --manifest-path "$ROOT/csdlc-v2/Cargo.toml" --bin csdlc-validate -- --help >/dev/null
echo "PASS: completed-output validation is owned by typed csdlc-validate"
