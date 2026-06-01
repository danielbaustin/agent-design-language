#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/check_pr_closing_linkage.sh"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT
ORIG_PATH="$PATH"

mkdir -p "$TMPDIR/bin"
cat >"$TMPDIR/bin/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == "pr" && "$2" == "view" ]]; then
  if [[ " $* " == *" --json closingIssuesReferences "* ]]; then
    printf '%s\n' "${MOCK_GH_CLOSING_ISSUES:-}"
    exit 0
  fi
  if [[ " $* " == *" --json body "* ]]; then
    printf '%s\n' "${MOCK_GH_PR_BODY:-}"
    exit 0
  fi
fi
if [[ "$1" == "issue" && "$2" == "view" ]]; then
  printf '%s\n' "${MOCK_GH_ISSUE_STATE:-OPEN}"
  exit 0
fi
exit 1
EOF
chmod +x "$TMPDIR/bin/gh"
PATH="$TMPDIR/bin:$PATH"

make_event() {
  local path="$1"
  local body="$2"
  local repo="${3:-example/repo}"
  local pr_number="${4:-77}"
  python3 - "$path" "$body" "$repo" "$pr_number" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
body = sys.argv[2]
repo = sys.argv[3]
pr_number = int(sys.argv[4])
path.write_text(json.dumps({
    "repository": {"full_name": repo},
    "pull_request": {"body": body, "number": pr_number}
}))
PY
}

event_ok="$TMPDIR/ok.json"
make_event "$event_ok" "Closes #1414"
bash "$SCRIPT" --event-name pull_request --event-path "$event_ok" --head-ref "codex/1414-remediation"

event_fix="$TMPDIR/fix.json"
make_event "$event_fix" $'Some notes\n\nFixes #1414'
bash "$SCRIPT" --event-name pull_request --event-path "$event_fix" --head-ref "codex/1414-remediation"

event_bad="$TMPDIR/bad.json"
make_event "$event_bad" "Refs #1414"
if bash "$SCRIPT" --event-name pull_request --event-path "$event_bad" --head-ref "codex/1414-remediation"; then
  echo "expected failure for missing closing linkage" >&2
  exit 1
fi

event_non_closing="$TMPDIR/non-closing.json"
make_event "$event_non_closing" "Non-closing lifecycle PR: issue 1414 remains open"
export MOCK_GH_ISSUE_STATE="OPEN"
bash "$SCRIPT" --event-name pull_request --event-path "$event_non_closing" --head-ref "codex/1414-remediation"

event_non_closing_closed="$TMPDIR/non-closing-closed.json"
make_event "$event_non_closing_closed" "Non-closing lifecycle PR: issue 1414 remains open"
export MOCK_GH_ISSUE_STATE="CLOSED"
if bash "$SCRIPT" --event-name pull_request --event-path "$event_non_closing_closed" --head-ref "codex/1414-remediation"; then
  echo "expected failure for non-closing marker on closed issue" >&2
  exit 1
fi
unset MOCK_GH_ISSUE_STATE

event_stale="$TMPDIR/stale.json"
make_event "$event_stale" "Refs #1414" "example/repo" "88"
export MOCK_GH_CLOSING_ISSUES=""
export MOCK_GH_PR_BODY="Closes #1414"
bash "$SCRIPT" --event-name pull_request --event-path "$event_stale" --head-ref "codex/1414-remediation"

event_other="$TMPDIR/other.json"
make_event "$event_other" "Refs #1414"
bash "$SCRIPT" --event-name push --event-path "$event_other" --head-ref "codex/1414-remediation"
bash "$SCRIPT" --event-name pull_request --event-path "$event_other" --head-ref "feature/no-issue-branch"

echo "test_pr_closing_linkage.sh: PASS"
