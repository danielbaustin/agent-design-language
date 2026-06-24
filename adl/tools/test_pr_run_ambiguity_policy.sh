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

assert_status_nonzero() {
  local status="$1" label="$2"
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed ($label): expected nonzero exit" >&2
    exit 1
  }
}

repo="$TMP_DIR/repo"
mkdir -p "$repo/adl/tools"
cp "$ROOT_DIR/adl/tools/pr.sh" "$repo/adl/tools/pr.sh"
cp "$ROOT_DIR/adl/tools/pr_delegate.sh" "$repo/adl/tools/pr_delegate.sh"
cp "$ROOT_DIR/adl/tools/pr_usage.sh" "$repo/adl/tools/pr_usage.sh"
cp "$ROOT_DIR/adl/tools/card_paths.sh" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  touch README.md workflow.adl.yaml
  git add README.md workflow.adl.yaml
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
issue_output="$(
  cd "$repo" &&
    ADL_PR_RUST_BIN="$mock_bin" \
    ADL_PR_RUN_CAPTURE="$capture_file" \
    "$BASH_BIN" adl/tools/pr.sh run 1303 --slug example-slug --version v0.91.5
)"

assert_contains "Issue-mode run: binding execution context for issue 1303" "$issue_output" "issue-mode note"
assert_contains "pr start 1303 --slug example-slug --version v0.91.5" "$(cat "$capture_file")" "issue delegation"

set +e
issue_with_runtime_flag="$(
  cd "$repo" &&
    ADL_PR_RUST_BIN="$mock_bin" \
    ADL_PR_RUN_CAPTURE="$capture_file" \
    "$BASH_BIN" adl/tools/pr.sh run 1303 --allow-unsigned 2>&1
)"
issue_with_runtime_flag_status=$?
set -e
assert_status_nonzero "$issue_with_runtime_flag_status" "issue operand with runtime flag"
assert_contains "issue-mode run cannot accept runtime flag '--allow-unsigned'" "$issue_with_runtime_flag" "issue/runtime ambiguity diagnostic"

set +e
runtime_with_issue_flag="$(
  cd "$repo" &&
    "$BASH_BIN" adl/tools/pr.sh run workflow.adl.yaml --slug example-slug 2>&1
)"
runtime_with_issue_flag_status=$?
set -e
assert_status_nonzero "$runtime_with_issue_flag_status" "runtime operand with issue flag"
assert_contains "runtime workflow run cannot accept issue flag '--slug'" "$runtime_with_issue_flag" "runtime/issue ambiguity diagnostic"

set +e
missing_runtime="$(
  cd "$repo" &&
    "$BASH_BIN" adl/tools/pr.sh run missing-workflow.adl.yaml 2>&1
)"
missing_runtime_status=$?
set -e
assert_status_nonzero "$missing_runtime_status" "missing runtime operand"
assert_contains "run: ADL file not found: missing-workflow.adl.yaml" "$missing_runtime" "missing runtime diagnostic"

if grep -R -E 'adl/tools/pr\.sh run [^`[:space:]]+\.adl\.ya?ml|adl pr run [^`[:space:]]+\.adl\.ya?ml' \
  "$ROOT_DIR/docs/templates/prompts" \
  "$ROOT_DIR/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md" >/dev/null 2>&1; then
  echo "assertion failed: generated-card templates must not emit deprecated runtime-through-PR invocations" >&2
  exit 1
fi

echo "PASS test_pr_run_ambiguity_policy"
