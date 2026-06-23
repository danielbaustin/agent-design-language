#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
OBS_SRC="$ROOT_DIR/adl/tools/observability.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
worktree="$repo/.worktrees/issue-4413"
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$repo/adl/target/debug" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"
sleep 1

cat >"$repo/adl/target/debug/adl-pr-doctor" <<'EOF_ADL'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${TMP_ADL_ARGS}"
EOF_ADL
chmod +x "$repo/adl/target/debug/adl-pr-doctor"

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${TMP_CARGO_ARGS}"
exit 0
EOF_CARGO
chmod +x "$mockbin/cargo"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md adl/tools/pr.sh adl/tools/card_paths.sh adl/tools/observability.sh adl/Cargo.toml
  git commit -q -m "init"
  git worktree add -q -b codex/4413 "$worktree" HEAD
)

TMP_ADL_ARGS="$tmpdir/adl_args.txt"
TMP_CARGO_ARGS="$tmpdir/cargo_args.txt"
export TMP_ADL_ARGS
export TMP_CARGO_ARGS
export PATH="$mockbin:$PATH"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == *"4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full"* ]] || {
  echo "assertion failed: expected worktree doctor delegation through the primary checkout direct binary" >&2
  echo "$args" >&2
  exit 1
}
[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: cargo should not run when the primary checkout binary is fresh for the worktree" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

sleep 1
touch "$worktree/adl/Cargo.toml"
: >"$TMP_ADL_ARGS"
: >"$TMP_CARGO_ARGS"

(
  cd "$worktree"
  ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
)

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: stale worktree Cargo.toml should block reuse of the primary checkout direct binary" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: stale worktree Cargo.toml should force cargo fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

echo "pr.sh worktree prefers primary checkout built binary: ok"
