#!/usr/bin/env bash
set -euo pipefail

event_name="${GITHUB_EVENT_NAME:-}"
event_path="${GITHUB_EVENT_PATH:-}"
head_ref="${GITHUB_HEAD_REF:-${GITHUB_REF_NAME:-}}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --event-name)
      event_name="${2:-}"
      shift 2
      ;;
    --event-path)
      event_path="${2:-}"
      shift 2
      ;;
    --head-ref)
      head_ref="${2:-}"
      shift 2
      ;;
    *)
      echo "check_pr_closing_linkage: unknown argument '$1'" >&2
      exit 2
      ;;
  esac
done

if [[ "$event_name" != "pull_request" ]]; then
  echo "check_pr_closing_linkage: skipped for event '$event_name'"
  exit 0
fi

if [[ -z "$event_path" || ! -f "$event_path" ]]; then
  echo "check_pr_closing_linkage: missing GitHub pull_request event payload" >&2
  exit 2
fi

if [[ ! "$head_ref" =~ (^|/)codex/([0-9]+)- ]]; then
  echo "check_pr_closing_linkage: skipped for non-issue branch '$head_ref'"
  exit 0
fi

issue="${BASH_REMATCH[2]}"

python3 - "$event_path" "$issue" <<'PY'
import json
import re
import sys
from pathlib import Path

event_path = Path(sys.argv[1])
issue = sys.argv[2]
payload = json.loads(event_path.read_text())
body = (payload.get("pull_request") or {}).get("body") or ""
pr_number = (payload.get("pull_request") or {}).get("number") or "unknown"

if re.search(r"\bNon-closing lifecycle PR\b", body, flags=re.IGNORECASE):
    print(
        f"check_pr_closing_linkage: PR #{pr_number} declares non-closing lifecycle work for issue #{issue}"
    )
    raise SystemExit(0)

closing = re.compile(
    rf"\b(close[sd]?|fix(e[sd])?|resolve[sd]?)\s+#?{re.escape(issue)}\b",
    flags=re.IGNORECASE,
)
if closing.search(body):
    print(f"check_pr_closing_linkage: PR #{pr_number} closes issue #{issue}")
    raise SystemExit(0)

print(
    f"check_pr_closing_linkage: PR #{pr_number} is missing closing linkage to issue #{issue}; "
    f"include a closing keyword such as 'Closes #{issue}' or declare a non-closing lifecycle PR",
    file=sys.stderr,
)
raise SystemExit(1)
PY
