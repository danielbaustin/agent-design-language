#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[[ ! -e "$ROOT/adl/tools/review_card_surface.sh" ]] || {
  echo "retired review-card wrapper unexpectedly exists" >&2
  exit 1
}
cargo run --quiet --locked --manifest-path "$ROOT/csdlc-v2/Cargo.toml" --bin csdlc-review -- --help >/dev/null
echo "PASS: review-card wrapper remains retired"
