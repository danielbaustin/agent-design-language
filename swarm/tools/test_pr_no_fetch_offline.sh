#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/swarm/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/swarm/tools/card_paths.sh"
INPUT_TPL_SRC="$ROOT_DIR/swarm/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/swarm/templates/cards/output_card_template.md"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/swarm/tools" "$repo/swarm/templates/cards"
cp "$PR_SH_SRC" "$repo/swarm/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/swarm/tools/card_paths.sh"
cp "$INPUT_TPL_SRC" "$repo/swarm/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/swarm/templates/cards/output_card_template.md"
chmod +x "$repo/swarm/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
)

no_gh_bin="$tmpdir/no-gh-bin"
mkdir -p "$no_gh_bin"
for cmd in awk cat cp cut dirname git grep head ln mkdir mktemp mv pwd readlink rm sed touch tr; do
  cmd_path="$(command -v "$cmd")"
  ln -s "$cmd_path" "$no_gh_bin/$cmd"
done

assert_file() {
  [[ -f "$1" ]] || { echo "assertion failed: expected file $1" >&2; exit 1; }
}

assert_has() {
  local pattern="$1" file="$2"
  grep -Fq "$pattern" "$file" || {
    echo "assertion failed: expected '$pattern' in $file" >&2
    exit 1
  }
}

(
  cd "$repo"
  PATH="$no_gh_bin" "$BASH_BIN" swarm/tools/pr.sh card 87 --no-fetch-issue --slug offline-title >/dev/null
  PATH="$no_gh_bin" "$BASH_BIN" swarm/tools/pr.sh output 87 --no-fetch-issue --slug offline-title >/dev/null
  PATH="$no_gh_bin" "$BASH_BIN" swarm/tools/pr.sh cards 88 --no-fetch-issue >/dev/null

  assert_file ".adl/cards/87/input_87.md"
  assert_file ".adl/cards/87/output_87.md"
  assert_file ".adl/cards/88/input_88.md"
  assert_file ".adl/cards/88/output_88.md"

  assert_has "Title: offline-title" ".adl/cards/87/input_87.md"
  assert_has "Title: offline-title" ".adl/cards/87/output_87.md"
  assert_has "Title: issue-88" ".adl/cards/88/input_88.md"
  assert_has "Title: issue-88" ".adl/cards/88/output_88.md"
)

echo "offline no-fetch pr.sh card generation: ok"
