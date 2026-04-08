#!/usr/bin/env bash
set -euo pipefail

EVENT_NAME="${GITHUB_EVENT_NAME:-}"
EVENT_PATH="${GITHUB_EVENT_PATH:-}"
HEAD_REF="${GITHUB_HEAD_REF:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --event-name)
      EVENT_NAME="${2:-}"
      shift 2
      ;;
    --event-path)
      EVENT_PATH="${2:-}"
      shift 2
      ;;
    --head-ref)
      HEAD_REF="${2:-}"
      shift 2
      ;;
    -h|--help)
      cat <<'EOF'
Usage:
  check_pr_closing_linkage.sh [--event-name <name>] [--event-path <path>] [--head-ref <ref>]

Enforces that pull requests from codex/<issue>-... branches contain a real GitHub
closing keyword for that issue in the PR body so the source issue auto-closes on merge.
EOF
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

if [[ "${EVENT_NAME}" != "pull_request" ]]; then
  echo "skip: event '${EVENT_NAME:-unknown}' is not pull_request"
  exit 0
fi

if [[ -z "$HEAD_REF" ]]; then
  echo "ERROR: missing pull request head ref" >&2
  exit 1
fi

if [[ ! "$HEAD_REF" =~ ^codex/([0-9]+)- ]]; then
  echo "skip: head ref '$HEAD_REF' is not an issue-linked codex branch"
  exit 0
fi

ISSUE_NUMBER="${BASH_REMATCH[1]}"

if [[ -z "$EVENT_PATH" || ! -f "$EVENT_PATH" ]]; then
  echo "ERROR: missing GitHub event payload at '$EVENT_PATH'" >&2
  exit 1
fi

PR_BODY="$(
python3 - "$EVENT_PATH" <<'PY'
import json
import sys
from pathlib import Path

event_path = Path(sys.argv[1])
data = json.loads(event_path.read_text())
body = data.get("pull_request", {}).get("body") or ""
print(body)
PY
)"

if python3 -c '
import re
import sys

issue = sys.argv[1]
body = sys.stdin.read()
pattern = re.compile(rf"\b(?:close[sd]?|fix(?:e[sd])?|resolve[sd]?)\s+#?{re.escape(issue)}\b", re.IGNORECASE)
if pattern.search(body):
    raise SystemExit(0)
raise SystemExit(1)
' "$ISSUE_NUMBER" <<<"$PR_BODY"; then
  echo "closing linkage OK for issue #$ISSUE_NUMBER"
  exit 0
fi

echo "ERROR: PR body for branch '$HEAD_REF' is missing closing linkage for issue #$ISSUE_NUMBER" >&2
echo "Add a real closing keyword such as 'Closes #$ISSUE_NUMBER' to the PR body." >&2
exit 1
