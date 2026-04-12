#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export ADL_TOOLING_MANIFEST_ROOT="$ROOT_DIR"
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

seed_issue_prompt() {
  local issue="$1" slug="$2" path
  path=".adl/v0.86/bodies/issue-${issue}-${slug}.md"
  mkdir -p "$(dirname "$path")"
  cat >"$path" <<EOF
---
issue_card_schema: adl.issue.v1
wp: "test"
slug: "${slug}"
title: "[v0.86][tools] ${slug}"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.86"
issue_number: ${issue}
status: "active"
action: "edit"
depends_on: []
milestone_sprint: "test"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files:
  - "adl/tools/pr.sh"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Test fixture prompt for worktree-safe lifecycle coverage."
pr_start:
  enabled: false
  slug: "${slug}"
---

# [v0.86][tools] ${slug}

## Summary

Exercise the issue worktree lifecycle with an authored prompt surface.

## Goal

Create or reuse the requested issue worktree without switching the primary checkout away from the user's current branch.

## Required Outcome

The lifecycle command should bind the issue worktree from origin/main and leave the primary checkout untouched.

## Deliverables

- issue worktree
- root and worktree task bundles

## Acceptance Criteria

- The command prints the expected worktree and branch.
- The primary checkout remains on its original branch.

## Repo Inputs

- local test fixture

## Dependencies

- none

## Demo Expectations

- No demo is required for this tooling fixture.

## Non-goals

- real GitHub issue mutation

## Issue-Graph Notes

- This is a local test-only issue prompt.

## Notes

- Keep this fixture concrete so lifecycle validation does not reject it as a stub.

## Tooling Notes

- Use origin/main as the branch base for worktree creation.
EOF
}

(
  cd "$repo"
  export ADL_PR_RUST_BIN="$REAL_ADL_BIN"

  seed_issue_prompt 999 test-smoke
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
  [[ -f "$repo/.adl/v0.86/tasks/issue-0999__test-smoke/stp.md" ]] || {
    echo "assertion failed: expected root canonical stp to exist after start" >&2
    exit 1
  }
  [[ -f "$repo/.adl/cards/999/stp_999.md" ]] || {
    echo "assertion failed: expected root stp card to exist after start" >&2
    exit 1
  }
  [[ -f "$repo/.adl/cards/999/input_999.md" ]] || {
    echo "assertion failed: expected root input card to exist after start" >&2
    exit 1
  }
  [[ -f "$repo/.adl/cards/999/output_999.md" ]] || {
    echo "assertion failed: expected root output card to exist after start" >&2
    exit 1
  }
  wt_path="$(cd "$wt_path" && pwd -P)"
  [[ "$(git -C "$wt_path" rev-parse --abbrev-ref HEAD)" == "codex/999-test-smoke" ]] || {
    echo "assertion failed: expected branch in worktree" >&2
    exit 1
  }
  [[ -f "$wt_path/.adl/v0.86/tasks/issue-0999__test-smoke/stp.md" ]] || {
    echo "assertion failed: expected canonical stp inside the worktree-local task bundle" >&2
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
  [[ -L "$wt_path/.adl/cards/999/stp_999.md" ]] || {
    echo "assertion failed: expected stp compatibility link" >&2
    exit 1
  }
  [[ -L "$wt_path/.adl/cards/999/output_999.md" ]] || {
    echo "assertion failed: expected output compatibility link" >&2
    exit 1
  }

  ready_out="$("$BASH_BIN" adl/tools/pr.sh ready 999 --slug test-smoke --no-fetch-issue --version v0.86)"
  assert_contains "READY=PASS" "$ready_out" "ready reports pass"
  assert_contains "ROOT_INPUT=.adl/v0.86/tasks/issue-0999__test-smoke/sip.md" "$ready_out" "ready prints root input"
  assert_contains "WT_INPUT=.worktrees/adl-wp-999/.adl/v0.86/tasks/issue-0999__test-smoke/sip.md" "$ready_out" "ready prints worktree input"

  out2="$("$BASH_BIN" adl/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "WORKTREE $wt_path" "$out2" "start idempotent worktree reuse"
  assert_contains "STATE  FULLY_STARTED" "$out2" "start idempotent state"
  [[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || {
    echo "assertion failed: primary checkout should remain on main after rerun" >&2
    exit 1
  }

  git branch --unset-upstream codex/999-test-smoke
  out3="$("$BASH_BIN" adl/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "WORKTREE $wt_path" "$out3" "upstream-less rerun still reuses worktree"
  assert_contains "STATE  FULLY_STARTED" "$out3" "upstream-less rerun state"

  custom_root="$tmpdir/custom-managed"
  mkdir -p "$custom_root"
  seed_issue_prompt 995 root-override
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
  seed_issue_prompt 994 fetch-fallback
  out_fetch_fallback="$(PATH="$fakebin:$PATH" "$BASH_BIN" adl/tools/pr.sh start 994 --slug fetch-fallback --no-fetch-issue)"
  fetch_wt="$repo/.worktrees/adl-wp-994"
  fetch_wt="$(cd "$fetch_wt" && pwd -P)"
  assert_contains "WORKTREE $fetch_wt" "$out_fetch_fallback" "fetch fallback still creates worktree"
  [[ -d "$fetch_wt" ]] || {
    echo "assertion failed: expected fetch-fallback worktree" >&2
    exit 1
  }

  fakecargo="$tmpdir/fakecargo"
  mkdir -p "$fakecargo"
  cat >"$fakecargo/cargo" <<'EOF'
#!/usr/bin/env bash
exit 1
EOF
  chmod +x "$fakecargo/cargo"
  seed_issue_prompt 989 rust-delegate-fallback
  out_rust_fallback="$(PATH="$fakecargo:$PATH" "$BASH_BIN" adl/tools/pr.sh start 989 --slug rust-delegate-fallback --no-fetch-issue)"
  rust_fallback_wt="$(cd "$repo/.worktrees/adl-wp-989" && pwd -P)"
  assert_contains "WORKTREE $rust_fallback_wt" "$out_rust_fallback" "rust fallback still creates worktree"
  [[ -d "$rust_fallback_wt" ]] || {
    echo "assertion failed: expected rust-fallback worktree" >&2
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
  seed_issue_prompt 997 guardrail
  out_dirty="$("$BASH_BIN" adl/tools/pr.sh start 997 --slug guardrail --no-fetch-issue)"
  dirty_wt="$(cd "$repo/.worktrees/adl-wp-997" && pwd -P)"
  assert_contains "WORKTREE $dirty_wt" "$out_dirty" "dirty primary checkout still starts issue worktree"
  [[ "$(git rev-parse --abbrev-ref HEAD)" == "codex/996-dirty" ]] || {
    echo "assertion failed: primary checkout should stay on the user's dirty branch" >&2
    exit 1
  }
  [[ -f untracked.txt ]] || {
    echo "assertion failed: dirty primary checkout file should remain untouched" >&2
    exit 1
  }

  git switch -q main
  rm -f untracked.txt
  mkdir -p "$repo/.adl/locks/pr-bootstrap.lock"
  echo "999999" > "$repo/.adl/locks/pr-bootstrap.lock/pid"
  seed_issue_prompt 993 stale-lock-recovery
  stale_lock_out="$("$BASH_BIN" adl/tools/pr.sh start 993 --slug stale-lock-recovery --no-fetch-issue)"
  stale_wt="$(cd "$repo/.worktrees/adl-wp-993" && pwd -P)"
  assert_contains "WORKTREE $stale_wt" "$stale_lock_out" "stale lock recovery still starts"

  "$BASH_BIN" adl/tools/pr.sh cards 991 --no-fetch-issue --version v0.86 >"$tmpdir/cards991.out" &
  cards_pid1=$!
  "$BASH_BIN" adl/tools/pr.sh cards 992 --no-fetch-issue --version v0.86 >"$tmpdir/cards992.out" &
  cards_pid2=$!
  wait "$cards_pid1"
  wait "$cards_pid2"

  [[ -f "$repo/.adl/cards/991/input_991.md" ]] || {
    echo "assertion failed: expected cards for issue 991 to exist after concurrent cards" >&2
    exit 1
  }
  [[ -f "$repo/.adl/cards/992/input_992.md" ]] || {
    echo "assertion failed: expected cards for issue 992 to exist after concurrent cards" >&2
    exit 1
  }

  fakegh="$tmpdir/fakegh"
  mkdir -p "$fakegh"
  cat >"$fakegh/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == "pr" && "$2" == "list" ]]; then
  cat <<'JSON'
[{"number":1169,"title":"[v0.86][tools] Keep tools queue busy","url":"https://example.test/pr/1169","headRefName":"codex/1161-v0-86-tools-keep-tools-queue-busy","baseRefName":"main","isDraft":true}]
JSON
  exit 0
fi
exit 1
EOF
  chmod +x "$fakegh/gh"

  seed_issue_prompt 990 blocked-wave
  preflight_out="$(PATH="$fakegh:$PATH" "$BASH_BIN" adl/tools/pr.sh preflight 990 --slug blocked-wave --version v0.86 --no-fetch-issue)"
  assert_contains "OPEN_PR_COUNT=1" "$preflight_out" "preflight detects open wave"
  assert_contains "PREFLIGHT=BLOCK" "$preflight_out" "preflight blocks"

  set +e
  blocked_start="$(PATH="$fakegh:$PATH" "$BASH_BIN" adl/tools/pr.sh start 990 --slug blocked-wave --version v0.86 --no-fetch-issue 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected start to block on unresolved open PR wave" >&2
    exit 1
  }
  assert_contains "unresolved open PR queue detected for v0.86 [tools:inferred]" "$blocked_start" "start guard message"
  assert_contains "#1169 [draft]" "$blocked_start" "start guard lists open pr"
  assert_contains "[queue=tools]" "$blocked_start" "start guard shows queue"

  allowed_start="$(PATH="$fakegh:$PATH" "$BASH_BIN" adl/tools/pr.sh start 990 --slug blocked-wave --version v0.86 --no-fetch-issue --allow-open-pr-wave)"
  assert_contains "STATE  FULLY_STARTED" "$allowed_start" "override bypasses start guard"

  cat >"$fakegh/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == "pr" && "$2" == "list" ]]; then
  cat <<'JSON'
[{"number":1169,"title":"[v0.86][WP-06] Runtime lane","url":"https://example.test/pr/1169","headRefName":"codex/1161-v0-86-wp-06-runtime-lane","baseRefName":"main","isDraft":true}]
JSON
  exit 0
fi
exit 1
EOF
  chmod +x "$fakegh/gh"
  cross_preflight="$(PATH="$fakegh:$PATH" "$BASH_BIN" adl/tools/pr.sh preflight 990 --slug blocked-wave --version v0.86 --no-fetch-issue)"
  assert_contains "TARGET_QUEUE=tools" "$cross_preflight" "preflight reports target queue"
  assert_contains "OPEN_PR_COUNT=0" "$cross_preflight" "cross-queue preflight does not block"
  assert_contains "PREFLIGHT=PASS" "$cross_preflight" "cross-queue preflight passes"

  chmod 555 "$repo/.git" "$repo/.git/refs" "$repo/.git/refs/heads"
  set +e
  metadata_blocked="$("$BASH_BIN" adl/tools/pr.sh start 988 --slug metadata-blocked --no-fetch-issue 2>&1)"
  status=$?
  set -e
  chmod 755 "$repo/.git" "$repo/.git/refs" "$repo/.git/refs/heads"
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected start to fail when git metadata is unwritable" >&2
    exit 1
  }
  assert_contains "git metadata directory" "$metadata_blocked" "metadata preflight message"
  assert_contains "restore write access to git metadata before rerunning" "$metadata_blocked" "metadata remediation"
)

echo "pr.sh start worktree-safe/idempotent flows: ok"
