#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

init_repo() {
  local repo="$1"
  git init -q "$repo"
  git -C "$repo" config user.name "Test User"
  git -C "$repo" config user.email "test@example.com"
  git -C "$repo" remote add origin https://github.com/example/repo.git
  mkdir -p "$repo/adl/tools" "$repo/.adl/v0.87.1/tasks"
  cp "$ROOT/adl/tools/check_milestone_closed_issue_sor_truth.sh" "$repo/adl/tools/check_milestone_closed_issue_sor_truth.sh"
  chmod +x "$repo/adl/tools/check_milestone_closed_issue_sor_truth.sh"
}

write_fake_gh() {
  local bin_dir="$1"
  local payload="$2"
  mkdir -p "$bin_dir"
  cat >"$bin_dir/gh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
if [[ "\$1 \$2" == "issue list" ]]; then
  cat <<'JSON'
$payload
JSON
  exit 0
fi
exit 1
EOF
  chmod +x "$bin_dir/gh"
}

PASS_REPO="$TMP/pass"
mkdir -p "$PASS_REPO"
init_repo "$PASS_REPO"
mkdir -p "$PASS_REPO/.adl/v0.87.1/tasks/issue-1546__sample"
cat >"$PASS_REPO/.adl/v0.87.1/tasks/issue-1546__sample/stp.md" <<'EOF'
title: sample
EOF
cat >"$PASS_REPO/.adl/v0.87.1/tasks/issue-1546__sample/sip.md" <<'EOF'
Title: sample
EOF
cat >"$PASS_REPO/.adl/v0.87.1/tasks/issue-1546__sample/sor.md" <<'EOF'
Status: DONE
- Integration state: merged
- Verification scope: main_repo
- Worktree-only paths remaining: none
EOF
write_fake_gh "$PASS_REPO/bin" '[{"number":1546,"stateReason":"COMPLETED","title":"sample"}]'
(
  cd "$PASS_REPO"
  PATH="$PASS_REPO/bin:$PATH" bash ./adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.87.1 --repo example/repo >/dev/null
)

mkdir -p "$PASS_REPO/.worktrees/adl-wp-1546/adl/tools"
cp "$ROOT/adl/tools/check_milestone_closed_issue_sor_truth.sh" "$PASS_REPO/.worktrees/adl-wp-1546/adl/tools/check_milestone_closed_issue_sor_truth.sh"
chmod +x "$PASS_REPO/.worktrees/adl-wp-1546/adl/tools/check_milestone_closed_issue_sor_truth.sh"
(
  cd "$PASS_REPO/.worktrees/adl-wp-1546"
  PATH="$PASS_REPO/bin:$PATH" bash ./adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.87.1 --repo example/repo >/dev/null
)

FAIL_REPO="$TMP/fail"
mkdir -p "$FAIL_REPO"
init_repo "$FAIL_REPO"
mkdir -p "$FAIL_REPO/.adl/v0.87.1/tasks/issue-1547__sample"
cat >"$FAIL_REPO/.adl/v0.87.1/tasks/issue-1547__sample/stp.md" <<'EOF'
title: sample
EOF
cat >"$FAIL_REPO/.adl/v0.87.1/tasks/issue-1547__sample/sor.md" <<'EOF'
Status: IN_PROGRESS
- Integration state: pr_open
- Verification scope: worktree
- Worktree-only paths remaining: adl/src/foo.rs
EOF
write_fake_gh "$FAIL_REPO/bin" '[{"number":1547,"stateReason":"COMPLETED","title":"sample"}]'
if (
  cd "$FAIL_REPO"
  PATH="$FAIL_REPO/bin:$PATH" bash ./adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.87.1 --repo example/repo >/dev/null 2>&1
); then
  echo "expected closed-issue stale-record check to fail" >&2
  exit 1
fi

echo "PASS test_check_milestone_closed_issue_sor_truth"
