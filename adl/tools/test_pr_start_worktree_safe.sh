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
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards" "$repo/adl/schemas"
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

  out1="$("$BASH_BIN" adl/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "WORKTREE" "$out1" "start prints worktree"
  assert_contains "BRANCH codex/999-test-smoke" "$out1" "start prints branch"
  [[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || {
    echo "assertion failed: primary checkout should remain on main" >&2
    exit 1
  }

  wt_path="$repo/.worktrees/adl-wp-999"
  [[ -d "$wt_path" ]] || {
    echo "assertion failed: expected worktree to exist at $wt_path" >&2
    exit 1
  }
  wt_path="$(cd "$wt_path" && pwd -P)"
  [[ "$(git -C "$wt_path" rev-parse --abbrev-ref HEAD)" == "codex/999-test-smoke" ]] || {
    echo "assertion failed: expected branch in worktree" >&2
    exit 1
  }
  [[ -f "$wt_path/.adl/v0.86/tasks/issue-0999__test-smoke/sip.md" ]] || {
    echo "assertion failed: expected canonical input card inside the worktree-local task bundle" >&2
    exit 1
  }
  [[ -f "$wt_path/.adl/v0.86/tasks/issue-0999__test-smoke/sor.md" ]] || {
    echo "assertion failed: expected canonical output card inside the worktree-local task bundle" >&2
    exit 1
  }
  [[ -L "$wt_path/.adl/cards/999/input_999.md" ]] || {
    echo "assertion failed: expected input compatibility link" >&2
    exit 1
  }
  [[ -L "$wt_path/.adl/cards/999/output_999.md" ]] || {
    echo "assertion failed: expected output compatibility link" >&2
    exit 1
  }

  out2="$("$BASH_BIN" adl/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "Reusing existing worktree for branch: $wt_path" "$out2" "start idempotent worktree reuse"
  [[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || {
    echo "assertion failed: primary checkout should remain on main after rerun" >&2
    exit 1
  }

  git branch --unset-upstream codex/999-test-smoke
  out3="$("$BASH_BIN" adl/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "Warning: branch 'codex/999-test-smoke' upstream is '<none>'" "$out3" "upstream warning"

  custom_root="$tmpdir/custom-managed"
  mkdir -p "$custom_root"
  out4="$(ADL_WORKTREE_ROOT="$custom_root" "$BASH_BIN" adl/tools/pr.sh start 995 --slug root-override --no-fetch-issue)"
  assert_contains "WORKTREE $custom_root/adl-wp-995" "$out4" "custom managed root"
  [[ -d "$custom_root/adl-wp-995" ]] || {
    echo "assertion failed: expected custom-root worktree" >&2
    exit 1
  }

  fakebin="$tmpdir/fakebin"
  mkdir -p "$fakebin"
  cat >"$fakebin/git" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == "fetch" && "\$2" == "origin" && "\$3" == "main" ]]; then
  echo "error: cannot open '.git/FETCH_HEAD': Operation not permitted" >&2
  exit 1
fi
exec "$(command -v git)" "\$@"
EOF
  chmod +x "$fakebin/git"
  out_fetch_fallback="$(PATH="$fakebin:$PATH" "$BASH_BIN" adl/tools/pr.sh start 994 --slug fetch-fallback --no-fetch-issue)"
  fetch_wt="$repo/.worktrees/adl-wp-994"
  fetch_wt="$(cd "$fetch_wt" && pwd -P)"
  assert_contains "start: fetch origin main failed; reusing existing local origin/main" "$out_fetch_fallback" "fetch fallback warning"
  assert_contains "WORKTREE $fetch_wt" "$out_fetch_fallback" "fetch fallback still creates worktree"
  [[ -d "$fetch_wt" ]] || {
    echo "assertion failed: expected fetch-fallback worktree" >&2
    exit 1
  }

  git branch codex/998-collision origin/main
  git worktree add -q "$tmpdir/other-path" codex/998-collision
  set +e
  bad="$("$BASH_BIN" adl/tools/pr.sh start 998 --slug collision --no-fetch-issue 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected branch/worktree collision to fail" >&2
    exit 1
  }
  assert_contains "already checked out in worktree" "$bad" "collision message"
  assert_contains "git worktree remove" "$bad" "collision remediation"

  git branch codex/997-guardrail origin/main
  git switch -q -c codex/996-dirty origin/main
  echo "keep-me" > untracked.txt
  set +e
  bad2="$("$BASH_BIN" adl/tools/pr.sh start 997 --slug guardrail --no-fetch-issue 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected dirty primary checkout guard to fail" >&2
    exit 1
  }
  assert_contains "with local changes" "$bad2" "dirty guard message"
  assert_contains "commit/stash there, switch to main, then rerun" "$bad2" "dirty guard remediation"

  git switch -q main
  rm -f untracked.txt
  mkdir -p "$(git rev-parse --git-common-dir)/pr-bootstrap.lock"
  set +e
  bad3="$("$BASH_BIN" adl/tools/pr.sh start 996 --slug bootstrap-lock --no-fetch-issue 2>&1)"
  status=$?
  set -e
  rm -rf "$(git rev-parse --git-common-dir)/pr-bootstrap.lock"
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected bootstrap lock contention to fail" >&2
    exit 1
  }
  assert_contains "another pr.sh bootstrap operation appears to be running" "$bad3" "bootstrap lock message"
)

echo "pr.sh start worktree-safe/idempotent flows: ok"
