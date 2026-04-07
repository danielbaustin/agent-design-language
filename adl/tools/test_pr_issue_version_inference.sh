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
REAL_ADL_BIN="$ROOT_DIR/adl/target/debug/adl"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -x "$REAL_ADL_BIN" ]]; then
  cargo build --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl >/dev/null
fi

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
if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue="${3:-}"
  shift 3
  if [[ "$issue" != "975" && "$issue" != "976" ]]; then
    exit 1
  fi
  if [[ "$issue" == "975" && "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    echo "version:v0.85"
    exit 0
  fi
  if [[ "$issue" == "976" && "$*" == *"--json labels"* && "$*" == *"-q .labels[].name"* ]]; then
    exit 0
  fi
  if [[ "$issue" == "975" && "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    echo "[v0.85][process] Infer current milestone card version from issue title when labels are missing"
    exit 0
  fi
  if [[ "$issue" == "976" && "$*" == *"--json title"* && "$*" == *"-q .title"* ]]; then
    echo "[v0.87.1][process] Infer dot suffixed milestone card version from issue title when labels are missing"
    exit 0
  fi
fi
exit 1
EOF
chmod +x "$bindir/gh"

canon_path() {
  local p="$1"
  mkdir -p "$p"
  (cd "$p" && pwd -P)
}

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
  export ADL_PR_RUST_BIN="$REAL_ADL_BIN"

  out_init="$("$BASH_BIN" adl/tools/pr.sh init 975 --slug v085-process-infer-card-version-from-issue-title)"
  assert_contains "STATE    ISSUE_AND_BUNDLE_READY" "$out_init" "init ready state"
  [[ -f ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" ]] || {
    echo "assertion failed: expected canonical input card under the root .adl/v0.85/tasks" >&2
    exit 1
  }
  [[ -f ".adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sor.md" ]] || {
    echo "assertion failed: expected canonical output card under the root .adl/v0.85/tasks" >&2
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
  if grep -Fq "v0.3" "$gh_log"; then
    echo "assertion failed: unexpected v0.3 inference in gh issue view path" >&2
    exit 1
  fi

  out_start="$("$BASH_BIN" adl/tools/pr.sh start 975 --slug v085-process-infer-card-version-from-issue-title)"
  assert_contains "WORKTREE $(canon_path "$repo/.worktrees/adl-wp-975")" "$out_start" "start prints worktree path"
  [[ -f "$repo/.worktrees/adl-wp-975/.adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sip.md" ]] || {
    echo "assertion failed: expected start to create input card inside worktree-local v0.85 task bundle" >&2
    exit 1
  }
  [[ -f "$repo/.worktrees/adl-wp-975/.adl/v0.85/tasks/issue-0975__v085-process-infer-card-version-from-issue-title/sor.md" ]] || {
    echo "assertion failed: expected start to create output card inside worktree-local v0.85 task bundle" >&2
    exit 1
  }
  [[ ! -e "$repo/.adl/v0.3/tasks/issue-0975__v085-process-infer-card-version-from-issue-title" ]] || {
    echo "assertion failed: unexpected v0.3 fallback task bundle after start" >&2
    exit 1
  }

  out_init_dot="$("$BASH_BIN" adl/tools/pr.sh init 976 --slug v0871-process-infer-dot-suffixed-version-from-title)"
  assert_contains "STATE    ISSUE_AND_BUNDLE_READY" "$out_init_dot" "dot-suffixed init ready state"
  [[ -f ".adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" ]] || {
    echo "assertion failed: expected canonical input card under the root .adl/v0.87.1/tasks" >&2
    exit 1
  }
  grep -Fq "Version: v0.87.1" ".adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" || {
    echo "assertion failed: expected input card version v0.87.1" >&2
    exit 1
  }

  out_start_dot="$("$BASH_BIN" adl/tools/pr.sh start 976 --slug v0871-process-infer-dot-suffixed-version-from-title)"
  assert_contains "WORKTREE $(canon_path "$repo/.worktrees/adl-wp-976")" "$out_start_dot" "dot-suffixed start prints worktree path"
  [[ -f "$repo/.worktrees/adl-wp-976/.adl/v0.87.1/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title/sip.md" ]] || {
    echo "assertion failed: expected start to create input card inside worktree-local v0.87.1 task bundle" >&2
    exit 1
  }
  [[ ! -e "$repo/.adl/v0.3/tasks/issue-0976__v0871-process-infer-dot-suffixed-version-from-title" ]] || {
    echo "assertion failed: unexpected v0.3 fallback task bundle for dot-suffixed version after start" >&2
    exit 1
  }

)

echo "pr.sh init/start title+version inference: ok"
