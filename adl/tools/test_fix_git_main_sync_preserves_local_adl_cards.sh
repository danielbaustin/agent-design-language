#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

ORIGIN="$TMP/origin.git"
SEED="$TMP/seed"
LOCAL="$TMP/local"
CARD_PATH=".adl/v0.88/tasks/issue-1650__v0-88-wp-05-temporal-query-and-retrieval/sor.md"

git init --bare -q "$ORIGIN"

git clone -q "$ORIGIN" "$SEED"
git -C "$SEED" checkout -q -b main
git -C "$SEED" config user.name "Test User"
git -C "$SEED" config user.email "test@example.com"
mkdir -p "$SEED/adl/tools"
cp "$ROOT/adl/tools/fix_git_main_sync.sh" "$SEED/adl/tools/fix_git_main_sync.sh"
cp "$ROOT/adl/tools/closeout_completed_issue_wave.sh" "$SEED/adl/tools/closeout_completed_issue_wave.sh"
chmod +x "$SEED/adl/tools/fix_git_main_sync.sh"
chmod +x "$SEED/adl/tools/closeout_completed_issue_wave.sh"
printf '.adl/\n' >"$SEED/.gitignore"
cat >"$SEED/adl/tools/pr.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"$TEST_CLOSEOUT_LOG"
exit 0
EOF
chmod +x "$SEED/adl/tools/pr.sh"
mkdir -p "$SEED/$(dirname "$CARD_PATH")"
printf 'tracked residue\n' >"$SEED/$CARD_PATH"
git -C "$SEED" add -f .gitignore adl/tools/fix_git_main_sync.sh adl/tools/closeout_completed_issue_wave.sh adl/tools/pr.sh "$CARD_PATH"
git -C "$SEED" commit -q -m "seed tracked residue"
git -C "$SEED" push -q -u origin main

git clone -q "$ORIGIN" "$LOCAL"
git -C "$LOCAL" checkout -q main
git -C "$LOCAL" config user.name "Test User"
git -C "$LOCAL" config user.email "test@example.com"

git -C "$SEED" rm -q "$CARD_PATH"
git -C "$SEED" commit -q -m "remove tracked residue"
git -C "$SEED" push -q

BIN="$TMP/bin"
mkdir -p "$BIN"
cat >"$BIN/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == "issue list" ]]; then
  printf '[{"number":1650,"stateReason":"COMPLETED"}]\n'
  exit 0
fi
exit 1
EOF
chmod +x "$BIN/gh"

export TEST_CLOSEOUT_LOG="$TMP/closeout.log"

(cd "$LOCAL" && PATH="$BIN:$PATH" ADL_MAIN_SYNC_CLOSEOUT_VERSIONS=v0.88 ADL_MAIN_SYNC_CLOSEOUT_REPO=danielbaustin/agent-design-language bash ./adl/tools/fix_git_main_sync.sh >/dev/null)

if [[ ! -f "$LOCAL/$CARD_PATH" ]]; then
  echo "expected local card to be restored after fast-forward sync" >&2
  exit 1
fi

if [[ -n "$(git -C "$LOCAL" status --porcelain)" ]]; then
  echo "expected local checkout to remain clean after sync" >&2
  git -C "$LOCAL" status --short >&2
  exit 1
fi

grep -Fq 'closeout 1650 --version v0.88 --no-fetch-issue' "$TEST_CLOSEOUT_LOG"

echo "PASS test_fix_git_main_sync_preserves_local_adl_cards"
