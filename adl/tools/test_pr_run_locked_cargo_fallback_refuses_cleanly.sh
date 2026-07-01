#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
PR_DELEGATE_SRC="$ROOT_DIR/adl/tools/pr_delegate.sh"
PR_USAGE_SRC="$ROOT_DIR/adl/tools/pr_usage.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"

cat >"$repo/adl/Cargo.toml" <<'EOF_CARGO_TOML'
[package]
name = "adl"
version = "0.1.0"
edition = "2021"
EOF_CARGO_TOML

cat >"$repo/adl/Cargo.lock" <<'EOF_LOCK'
# synthetic lockfile fixture
EOF_LOCK

cat >"$mockbin/cargo" <<'EOF_MOCK_CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TMP_CARGO_ARGS"
if [[ " $* " != *" --locked "* ]]; then
  printf '\n# mutated by fallback\n' >> "$TMP_LOCKFILE"
  echo "cargo fallback unexpectedly ran without --locked" >&2
  exit 97
fi
echo "error: the lock file $TMP_LOCKFILE needs to be updated but --locked was passed to prevent this" >&2
exit 42
EOF_MOCK_CARGO
chmod +x "$mockbin/cargo"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md adl/Cargo.toml adl/Cargo.lock adl/tools/pr.sh adl/tools/pr_delegate.sh adl/tools/pr_usage.sh adl/tools/card_paths.sh
  git commit -q -m "init"
)

TMP_CARGO_ARGS="$tmpdir/cargo_args.txt"
TMP_LOCKFILE="$repo/adl/Cargo.lock"
export TMP_CARGO_ARGS TMP_LOCKFILE
export PATH="$mockbin:/usr/bin:/bin:/usr/sbin:/sbin"

set +e
run_output="$(
  cd "$repo" &&
  ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 "$BASH_BIN" adl/tools/pr.sh run 4306 --slug locked-fallback --no-fetch-issue --version v0.86 2>&1
)"
run_status=$?
set -e

[[ $run_status -eq 42 ]] || {
  echo "assertion failed: expected locked cargo refusal status 42, got $run_status" >&2
  echo "$run_output" >&2
  exit 1
}

grep -Fq -- "--locked" "$TMP_CARGO_ARGS" || {
  echo "assertion failed: expected cargo fallback to include --locked" >&2
  cat "$TMP_CARGO_ARGS" >&2
  exit 1
}

grep -Fq "needs to be updated but --locked was passed" <<<"$run_output" || {
  echo "assertion failed: expected actionable stale-lockfile refusal output" >&2
  echo "$run_output" >&2
  exit 1
}

git_status="$(cd "$repo" && git status --short)"
[[ -z "$git_status" ]] || {
  echo "assertion failed: expected stale-lock refusal to leave checkout clean" >&2
  echo "$git_status" >&2
  exit 1
}

echo "pr.sh run locked cargo fallback refuses stale lockfile without dirtying checkout: ok"
