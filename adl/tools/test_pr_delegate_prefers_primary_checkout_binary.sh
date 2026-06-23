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

cat >"$repo/adl/target/debug/adl" <<'EOF_BROAD'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${TMP_BROAD_ARGS}"
EOF_BROAD
chmod +x "$repo/adl/target/debug/adl"

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
TMP_BROAD_ARGS="$tmpdir/broad_args.txt"
export TMP_ADL_ARGS
export TMP_CARGO_ARGS
export TMP_BROAD_ARGS
export PATH="$mockbin:$PATH"

run_doctor_from_worktree() {
  (
    cd "$worktree"
    ADL_PRIMARY_CHECKOUT_ROOT="$repo" \
      "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full >/dev/null
  )
}

reset_delegate_logs() {
  : >"$TMP_ADL_ARGS"
  : >"$TMP_CARGO_ARGS"
  : >"$TMP_BROAD_ARGS"
}

run_doctor_from_worktree

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
reset_delegate_logs
run_doctor_from_worktree

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

git -C "$worktree" checkout -- adl/Cargo.toml
sleep 1
touch "$worktree/adl/Cargo.lock"
reset_delegate_logs
run_doctor_from_worktree

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: stale worktree Cargo.lock should block reuse of the primary checkout direct binary" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: stale worktree Cargo.lock should force cargo fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

rm -f "$worktree/adl/Cargo.lock"
sleep 1
touch "$worktree/adl/build.rs"
reset_delegate_logs
run_doctor_from_worktree

[[ ! -s "$TMP_ADL_ARGS" ]] || {
  echo "assertion failed: stale worktree build.rs should block reuse of the primary checkout direct binary" >&2
  cat "$TMP_ADL_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: stale worktree build.rs should force cargo fallback" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

rm -f "$repo/adl/target/debug/adl-pr-doctor"
rm -f "$worktree/adl/build.rs"
sleep 1
touch "$repo/adl/target/debug/adl"
reset_delegate_logs
run_doctor_from_worktree

[[ ! -s "$TMP_CARGO_ARGS" ]] || {
  echo "assertion failed: broad adl binary should be reused when no worktree Rust inputs are newer" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}
grep -F -- "pr doctor 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_BROAD_ARGS" >/dev/null || {
  echo "assertion failed: expected worktree doctor delegation through the primary checkout broad adl binary" >&2
  cat "$TMP_BROAD_ARGS" >&2
  exit 1
}

sleep 1
touch "$worktree/adl/build.rs"
reset_delegate_logs
run_doctor_from_worktree

[[ ! -s "$TMP_BROAD_ARGS" ]] || {
  echo "assertion failed: stale worktree build.rs should block reuse of the broad primary checkout adl binary" >&2
  cat "$TMP_BROAD_ARGS" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug rust-start --no-fetch-issue --version v0.91.6 --mode full" "$TMP_CARGO_ARGS" >/dev/null || {
  echo "assertion failed: stale worktree build.rs should force cargo fallback instead of the broad adl binary" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

echo "pr.sh worktree prefers primary checkout built binary: ok"
