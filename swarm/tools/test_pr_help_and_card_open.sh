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

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

(
  cd "$repo"

  help_out="$("$BASH_BIN" swarm/tools/pr.sh help)"
  assert_contains "Commands:" "$help_out" "help alias"

  card_help_out="$("$BASH_BIN" swarm/tools/pr.sh card --help)"
  assert_contains "Usage:" "$card_help_out" "card --help"

  in_path="$("$BASH_BIN" swarm/tools/pr.sh card 271 input --no-fetch-issue --slug demo-title)"
  [[ "$in_path" == *"/.adl/cards/271/input_271.md" ]] || {
    echo "assertion failed: unexpected input card path: $in_path" >&2
    exit 1
  }
  [[ -f ".adl/cards/271/input_271.md" ]] || {
    echo "assertion failed: expected input card file to exist" >&2
    exit 1
  }

  out_path="$("$BASH_BIN" swarm/tools/pr.sh card 271 output --no-fetch-issue --slug demo-title)"
  [[ "$out_path" == *"/.adl/cards/271/output_271.md" ]] || {
    echo "assertion failed: unexpected output card path via card command: $out_path" >&2
    exit 1
  }
  [[ -f ".adl/cards/271/output_271.md" ]] || {
    echo "assertion failed: expected output card file to exist" >&2
    exit 1
  }

  out_path2="$("$BASH_BIN" swarm/tools/pr.sh output 271 output --no-fetch-issue --slug demo-title)"
  [[ "$out_path2" == *"/.adl/cards/271/output_271.md" ]] || {
    echo "assertion failed: unexpected output card path via output command: $out_path2" >&2
    exit 1
  }

  in_path2="$("$BASH_BIN" swarm/tools/pr.sh output 271 input --no-fetch-issue --slug demo-title)"
  [[ "$in_path2" == *"/.adl/cards/271/input_271.md" ]] || {
    echo "assertion failed: unexpected input card path via output command: $in_path2" >&2
    exit 1
  }

  set +e
  bad_out="$("$BASH_BIN" swarm/tools/pr.sh card 271 nope 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected non-zero status for unknown card arg" >&2
    exit 1
  }
  assert_contains "Usage:" "$bad_out" "card unknown arg usage"
)

echo "pr.sh help + card positional open flows: ok"
