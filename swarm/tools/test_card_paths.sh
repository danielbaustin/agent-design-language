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

assert_symlink() {
  [[ -L "$1" ]] || { echo "assertion failed: expected symlink $1" >&2; exit 1; }
}

assert_eq() {
  [[ "$1" == "$2" ]] || { echo "assertion failed: expected '$1' == '$2'" >&2; exit 1; }
}

mkdir -p .adl/cards/143
echo "canonical" > .adl/cards/143/input_143.md
echo "legacy-a" > .adl/cards/issue-0143__input__v0.3.md
sync_legacy_card_link ".adl/cards/143/input_143.md" ".adl/cards/issue-0143__input__v0.3.md"
assert_file ".adl/cards/issue-0143__input__v0.3.md.bak"
assert_symlink ".adl/cards/issue-0143__input__v0.3.md"
assert_eq "$(readlink .adl/cards/issue-0143__input__v0.3.md)" "143/input_143.md"

rm -f .adl/cards/issue-0143__input__v0.3.md
echo "legacy-b" > .adl/cards/issue-0143__input__v0.3.md
touch .adl/cards/issue-0143__input__v0.3.md.bak
sync_legacy_card_link ".adl/cards/143/input_143.md" ".adl/cards/issue-0143__input__v0.3.md"
assert_file ".adl/cards/issue-0143__input__v0.3.md.bak.1"
assert_symlink ".adl/cards/issue-0143__input__v0.3.md"

rm -rf .adl/cards
mkdir -p .adl/cards
echo "legacy-only" > .adl/cards/issue-0143__input__v0.3.md
resolved_input="$(resolve_input_card_path 143 v0.3)"
assert_eq "$resolved_input" ".adl/cards/143/input_143.md"
assert_file "$resolved_input"
assert_file ".adl/cards/issue-0143__input__v0.3.md.bak"
assert_symlink ".adl/cards/issue-0143__input__v0.3.md"
assert_eq "$(cat "$resolved_input")" "legacy-only"

echo "ok"
