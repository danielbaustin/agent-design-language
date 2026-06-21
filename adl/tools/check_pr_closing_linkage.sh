#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

maybe_delegate_to_rust() {
  [[ "${ADL_PR_CLOSING_LINKAGE_DISABLE_RUST:-0}" == "1" ]] && return 0
  if [[ -n "${ADL_PR_CLOSING_LINKAGE_BIN:-}" && -x "${ADL_PR_CLOSING_LINKAGE_BIN}" ]]; then
    exec "${ADL_PR_CLOSING_LINKAGE_BIN}" "$@"
  fi
}

maybe_delegate_to_rust "$@"

event_name="${GITHUB_EVENT_NAME:-}"
event_path="${GITHUB_EVENT_PATH:-}"
head_ref="${GITHUB_HEAD_REF:-${GITHUB_REF_NAME:-}}"
repo="${GITHUB_REPOSITORY:-}"

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
    --repo|-R)
      repo="${2:-}"
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
live_body_path=""
live_status="unavailable"

if [[ -n "$repo" ]]; then
  payload_repo="$repo"
else
  payload_repo="$(python3 - "$event_path" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
print(((payload.get("repository") or {}).get("full_name") or "").strip())
PY
)"
  repo="$payload_repo"
fi

pr_number="$(python3 - "$event_path" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
number = (payload.get("pull_request") or {}).get("number") or ""
print(number)
PY
)"

gh_bin="${CHECK_PR_CLOSING_LINKAGE_GH_BIN:-gh}"
if [[ -n "$repo" && -n "$pr_number" && ( -n "${GH_TOKEN:-}" || -n "${GITHUB_TOKEN:-}" ) ]]; then
  if command -v "$gh_bin" >/dev/null 2>&1; then
    live_body_path="$(mktemp)"
    if "$gh_bin" pr view "$pr_number" --repo "$repo" --json body --jq '.body // ""' >"$live_body_path" 2>"$live_body_path.err"; then
      live_status="available"
    else
      live_status="failed"
    fi
  else
    live_status="missing_gh"
  fi
fi

cleanup() {
  [[ -n "$live_body_path" ]] && rm -f "$live_body_path" "$live_body_path.err"
  true
}
trap cleanup EXIT

python3 - "$event_path" "$issue" "$live_status" "$live_body_path" <<'PY'
import json
import re
import sys
from pathlib import Path

event_path = Path(sys.argv[1])
issue = sys.argv[2]
live_status = sys.argv[3]
live_body_path = sys.argv[4]
payload = json.loads(event_path.read_text())
pr_number = (payload.get("pull_request") or {}).get("number") or "unknown"
event_body = (payload.get("pull_request") or {}).get("body") or ""

def has_non_closing(body: str) -> bool:
    return bool(re.search(r"\bNon-closing lifecycle PR\b", body, flags=re.IGNORECASE))

closing = re.compile(
    rf"\b(close[sd]?|fix(e[sd])?|resolve[sd]?)\s+#?{re.escape(issue)}\b",
    flags=re.IGNORECASE,
)

def has_closing(body: str) -> bool:
    return bool(closing.search(body))

def accept_body(body: str, source: str) -> bool:
    if has_non_closing(body):
        print(
            f"check_pr_closing_linkage: PR #{pr_number} declares non-closing lifecycle work for issue #{issue} ({source})"
        )
        return True
    if has_closing(body):
        print(f"check_pr_closing_linkage: PR #{pr_number} closes issue #{issue} ({source})")
        return True
    return False

if live_status == "available":
    live_body = Path(live_body_path).read_text() if live_body_path else ""
    if accept_body(live_body, "live PR metadata"):
        raise SystemExit(0)
    print(
        f"check_pr_closing_linkage: live PR body for PR #{pr_number} is missing closing linkage to issue #{issue}; "
        f"update the PR body with a closing keyword such as 'Closes #{issue}' or declare a non-closing lifecycle PR",
        file=sys.stderr,
    )
    raise SystemExit(1)

if accept_body(event_body, "event payload"):
    raise SystemExit(0)

live_note = {
    "unavailable": "live PR metadata was unavailable because repo, PR number, or token context was missing",
    "missing_gh": "live PR metadata was unavailable because the gh CLI was not found",
    "failed": "live PR metadata fetch failed",
}.get(live_status, "live PR metadata was unavailable")
print(
    f"check_pr_closing_linkage: PR #{pr_number} is missing closing linkage to issue #{issue}; "
    f"include a closing keyword such as 'Closes #{issue}' or declare a non-closing lifecycle PR. "
    f"{live_note}. If this is a rerun after a PR-body-only repair, GitHub may be reusing a stale pull_request event payload; "
    f"rerun with token/repo context for live metadata validation or push a fresh commit to refresh the event payload.",
    file=sys.stderr,
)
raise SystemExit(1)
PY
