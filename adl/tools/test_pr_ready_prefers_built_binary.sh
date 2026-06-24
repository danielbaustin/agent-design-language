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
mkdir -p "$repo/adl/tools" "$repo/adl/target/debug" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"
sleep 1

cat >"$repo/adl/target/debug/adl-pr-ready" <<'EOF_ADL'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TMP_ADL_ARGS"
EOF_ADL
chmod +x "$repo/adl/target/debug/adl-pr-ready"

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
echo "cargo should not be called when built adl binary is fresh" >&2
exit 99
EOF_CARGO
chmod +x "$mockbin/cargo"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

TMP_ADL_ARGS="$tmpdir/adl_args.txt"
export TMP_ADL_ARGS
export PATH="$mockbin:$PATH"

(
  cd "$repo"
  "$BASH_BIN" adl/tools/pr.sh ready 1152 --slug rust-start --no-fetch-issue --version v0.86 >/dev/null
)

args="$(cat "$TMP_ADL_ARGS")"
[[ "$args" == *"1152 --slug rust-start --no-fetch-issue --version v0.86"* ]] || {
  echo "assertion failed: expected built direct ready binary delegation" >&2
  echo "$args" >&2
  exit 1
}

echo "pr.sh ready built direct-binary delegation: ok"
