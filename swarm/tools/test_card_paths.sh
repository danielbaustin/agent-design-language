#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD_PATHS_LIB="$ROOT_DIR/swarm/tools/card_paths.sh"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
repo="$tmpdir/repo"

mkdir -p "$repo/swarm/tools"
cp "$CARD_PATHS_LIB" "$repo/swarm/tools/card_paths.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  touch .gitkeep
  git add .gitkeep
  git commit -q -m "init"
)

cd "$repo"
# shellcheck disable=SC1091
source "$repo/swarm/tools/card_paths.sh"

assert_file() {
  [[ -f "$1" ]] || { echo "assertion failed: expected file $1" >&2; exit 1; }
}

assert_eq() {
  [[ "$1" == "$2" ]] || { echo "assertion failed: expected '$1' == '$2'" >&2; exit 1; }
}

canon_path() {
  local p="$1"
  mkdir -p "$p"
  (cd "$p" && pwd -P)
}

cards_root="$(cards_root_resolve)"
mkdir -p "$cards_root"
echo "legacy-only" > "$cards_root/issue-0143__input__v0.3.md"
resolved_input="$(resolve_input_card_path 143 v0.3)"
assert_eq "$resolved_input" "$cards_root/143/input_143.md"
assert_file "$resolved_input"
assert_eq "$(cat "$resolved_input")" "legacy-only"
assert_file "$cards_root/_legacy_migrated/issue-0143__input__v0.3.md"
[[ ! -e "$cards_root/issue-0143__input__v0.3.md" ]] || { echo "assertion failed: expected migrated legacy root card" >&2; exit 1; }

echo "legacy-only-2" > "$cards_root/issue-0143__input__v0.3.md"
resolved_input_again="$(resolve_input_card_path 143 v0.3)"
assert_eq "$resolved_input_again" "$cards_root/143/input_143.md"
assert_file "$cards_root/_legacy_migrated/issue-0143__input__v0.3.md.1"

ADL_CARDS_ROOT="custom/cards"
export ADL_CARDS_ROOT
assert_eq "$(canon_path "$(cards_root_resolve)")" "$(canon_path "$repo/custom/cards")"
assert_eq "$(canon_path "$(dirname "$(card_input_path 144)")")" "$(canon_path "$repo/custom/cards/144")"

echo "ok"
