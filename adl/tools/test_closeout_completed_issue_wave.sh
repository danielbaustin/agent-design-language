#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

REPO="$TMP/repo"
mkdir -p "$REPO/adl/tools"
mkdir -p "$REPO/.git"

mkdir -p "$REPO/.adl/v0.88/tasks/issue-100__demo"
cat >"$REPO/.adl/v0.88/tasks/issue-100__demo/sor.md" <<'EOF'
Status: DONE
- Integration state: pr_open
- Verification scope: pr_branch
- Worktree-only paths remaining: worktree/path
EOF

cat >"$REPO/.git/config" <<'EOF'
[remote "origin"]
	url = git@github.com:danielbaustin/agent-design-language.git
EOF

cp "$ROOT/adl/tools/closeout_completed_issue_wave.sh" "$REPO/adl/tools/closeout_completed_issue_wave.sh"
chmod +x "$REPO/adl/tools/closeout_completed_issue_wave.sh"

CLOSEOUT_LOG="$TMP/closeout.log"
cat >"$REPO/adl/tools/pr.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$*" >>"$CLOSEOUT_LOG"
exit 0
EOF
chmod +x "$REPO/adl/tools/pr.sh"

BIN="$TMP/bin"
mkdir -p "$BIN"
cat >"$BIN/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == "issue list" ]]; then
  printf '[{"number":100,"stateReason":"COMPLETED"},{"number":101,"stateReason":"COMPLETED"}]\n'
  exit 0
fi
exit 1
EOF
chmod +x "$BIN/gh"

REPORT="$TMP/report.md"
(cd "$REPO" && PATH="$BIN:$PATH" bash ./adl/tools/closeout_completed_issue_wave.sh --version v0.88 --repo danielbaustin/agent-design-language --report "$REPORT")

grep -Fq 'closeout 100 --version v0.88 --no-fetch-issue' "$CLOSEOUT_LOG"
grep -Fq 'normalized_issues: 1' "$REPORT"
grep -Fq 'failed_issues: 0' "$REPORT"

echo "PASS test_closeout_completed_issue_wave"
