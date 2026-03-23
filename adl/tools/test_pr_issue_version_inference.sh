#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
PROMPT_LINT_SRC="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"
PROMPT_VALIDATOR_SRC="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
STP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_task_prompt.contract.yaml"
SIP_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_implementation_prompt.contract.yaml"
SOR_CONTRACT_SRC="$ROOT_DIR/adl/schemas/structured_output_record.contract.yaml"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
bindir="$tmpdir/bin"
gh_log="$tmpdir/gh.log"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas" "$bindir"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$PROMPT_LINT_SRC" "$repo/adl/tools/lint_prompt_spec.sh"
cp "$PROMPT_VALIDATOR_SRC" "$repo/adl/tools/validate_structured_prompt.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
cp "$STP_CONTRACT_SRC" "$repo/adl/schemas/structured_task_prompt.contract.yaml"
cp "$SIP_CONTRACT_SRC" "$repo/adl/schemas/structured_implementation_prompt.contract.yaml"
cp "$SOR_CONTRACT_SRC" "$repo/adl/schemas/structured_output_record.contract.yaml"
chmod +x "$repo/adl/tools/pr.sh"
chmod +x "$repo/adl/tools/lint_prompt_spec.sh" "$repo/adl/tools/validate_structured_prompt.sh"

cat >"$bindir/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
LOG_FILE="${GH_LOG_FILE:?}"
printf '%s\n' "$*" >>"$LOG_FILE"
if [[ "${1:-}" == "issue" && "${2:-}" == "create" ]]; then
  echo "https://github.com/example/repo/issues/975"
  exit 0
fi
if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" != "975" ]]; then
    exit 1
  fi
  if [[ "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    exit 0
  fi
  if [[ "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    echo "[v0.85][process] Infer current milestone card version from issue title when labels are missing"
    exit 0
  fi
fi
exit 1
EOF
chmod +x "$bindir/gh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
  git branch -M main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
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
  export PATH="$bindir:$PATH"
  export GH_LOG_FILE="$gh_log"

  out="$("$BASH_BIN" adl/tools/pr.sh new \
    --title "[v0.85][process] Infer current milestone card version from issue title when labels are missing" \
    --slug v085-process-infer-card-version-from-issue-title \
    --version v0.85 \
    --body "test body")"

  assert_contains "ISSUE_NUM=975" "$out" "new prints issue number"
  [[ -f ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" ]] || {
    echo "assertion failed: expected canonical input card under .adl/v0.85/tasks" >&2
    exit 1
  }
  [[ -f ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sor.md" ]] || {
    echo "assertion failed: expected canonical output card under .adl/v0.85/tasks" >&2
    exit 1
  }
  grep -Fq "Version: v0.85" ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected input card version v0.85" >&2
    exit 1
  }
  grep -Fq "Title: [v0.85][process] Infer current milestone card version from issue title when labels are missing" \
    ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" || {
    echo "assertion failed: expected preserved issue title in input card" >&2
    exit 1
  }
  grep -Fq -- "--label version:v0.85" "$gh_log" || {
    echo "assertion failed: expected issue create to use version:v0.85" >&2
    exit 1
  }
  if grep -Fq -- "--label version:v0.3" "$gh_log"; then
    echo "assertion failed: unexpected version:v0.3 label in issue create" >&2
    exit 1
  fi
)

echo "pr.sh new/start title+version inference: ok"
