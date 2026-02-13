#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cd "$tmpdir"

mkdir -p swarm/tools swarm/templates/cards
cp "$ROOT_DIR/swarm/tools/pr.sh" swarm/tools/pr.sh
cp "$ROOT_DIR/swarm/tools/card_paths.sh" swarm/tools/card_paths.sh
cp "$ROOT_DIR/swarm/templates/cards/input_card_template.md" swarm/templates/cards/input_card_template.md
cp "$ROOT_DIR/swarm/templates/cards/output_card_template.md" swarm/templates/cards/output_card_template.md

chmod +x swarm/tools/pr.sh

git init -q
git config user.name "Test User"
git config user.email "test@example.com"
echo "test" > README.md
git add README.md
git commit -qm "init"

assert_file() {
  [[ -f "$1" ]] || { echo "assertion failed: expected file $1" >&2; exit 1; }
}

# Keep git available but ensure gh is not available.
NO_GH_PATH="/usr/bin:/bin:/usr/sbin:/sbin"

PATH="$NO_GH_PATH" bash swarm/tools/pr.sh card 87 --no-fetch-issue --slug offline-title >/dev/null
assert_file ".adl/cards/87/input_87.md"

PATH="$NO_GH_PATH" bash swarm/tools/pr.sh output 88 --no-fetch-issue --slug offline-title >/dev/null
assert_file ".adl/cards/88/output_88.md"

PATH="$NO_GH_PATH" bash swarm/tools/pr.sh cards 89 --no-fetch-issue >/dev/null
assert_file ".adl/cards/89/input_89.md"
assert_file ".adl/cards/89/output_89.md"

echo "offline no-fetch pr.sh card generation: ok"
