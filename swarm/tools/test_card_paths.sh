#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD_PATHS_LIB="$ROOT_DIR/swarm/tools/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
cd "$tmpdir"

assert_file() {
  [[ -f "$1" ]] || { echo "assertion failed: expected file $1" >&2; exit 1; }
}

assert_eq() {
  [[ "$1" == "$2" ]] || { echo "assertion failed: expected '$1' == '$2'" >&2; exit 1; }
}

rm -rf .adl/cards
mkdir -p .adl/cards
echo "legacy-only" > .adl/cards/issue-0143__input__v0.3.md
resolved_input="$(resolve_input_card_path 143 v0.3)"
assert_eq "$resolved_input" ".adl/cards/143/input_143.md"
assert_file "$resolved_input"
assert_eq "$(cat "$resolved_input")" "legacy-only"
assert_file ".adl/cards/_legacy_migrated/issue-0143__input__v0.3.md"
[[ ! -e ".adl/cards/issue-0143__input__v0.3.md" ]] || { echo "assertion failed: expected migrated legacy root card" >&2; exit 1; }

echo "legacy-only-2" > .adl/cards/issue-0143__input__v0.3.md
resolved_input_again="$(resolve_input_card_path 143 v0.3)"
assert_eq "$resolved_input_again" ".adl/cards/143/input_143.md"
assert_file ".adl/cards/_legacy_migrated/issue-0143__input__v0.3.md.1"

echo "ok"
