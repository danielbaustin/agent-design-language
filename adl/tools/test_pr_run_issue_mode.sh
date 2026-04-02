#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASH_BIN="$(command -v bash)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

repo="$TMP_DIR/repo"
mkdir -p "$repo/adl/tools"
cp "$ROOT_DIR/adl/tools/pr.sh" "$repo/adl/tools/pr.sh"
cp "$ROOT_DIR/adl/tools/card_paths.sh" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  touch README.md
  git add README.md
  git commit -q -m "init"
)

mock_bin="$TMP_DIR/mock-adl"
cat >"$mock_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"$ADL_PR_RUN_CAPTURE"
EOF
chmod +x "$mock_bin"

capture_file="$TMP_DIR/capture.txt"
run_out="$(
  cd "$repo" &&
    ADL_PR_RUST_BIN="$mock_bin" \
    ADL_PR_RUN_CAPTURE="$capture_file" \
    "$BASH_BIN" adl/tools/pr.sh run 1303 --slug example-slug --version v0.86
)"

captured="$(cat "$capture_file")"
assert_contains 'Issue-mode run: binding execution context for issue 1303' "$run_out" "issue-mode note"
assert_contains 'pr start 1303 --slug example-slug --version v0.86' "$captured" "delegates to start binder"

echo "pr.sh run issue-mode delegation: ok"
