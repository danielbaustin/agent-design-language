#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION=""
REPO=""

usage() {
  cat <<'EOF'
Usage:
  check_milestone_closed_issue_sor_truth.sh --version <v0.87.1> [--repo <owner/name>]

Scans closed GitHub issues for the target milestone label and verifies the
canonical local `.adl/<version>/tasks/issue-*__*/sor.md` records are not stale.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --repo)
      REPO="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

[[ -n "$VERSION" ]] || {
  echo "ERROR: --version is required" >&2
  exit 2
}

if [[ -z "$REPO" ]]; then
  remote_url="$(git -C "$ROOT" remote get-url origin 2>/dev/null || true)"
  if [[ "$remote_url" =~ github.com[:/]([^/]+/[^/.]+)(\.git)?$ ]]; then
    REPO="${BASH_REMATCH[1]}"
  else
    echo "ERROR: could not infer GitHub repo from origin remote; pass --repo <owner/name>" >&2
    exit 1
  fi
fi

command -v gh >/dev/null 2>&1 || {
  echo "ERROR: gh is required" >&2
  exit 1
}
command -v python3 >/dev/null 2>&1 || {
  echo "ERROR: python3 is required" >&2
  exit 1
}

issues_json="$(gh issue list -R "$REPO" --state closed --label "version:$VERSION" --limit 200 --json number,stateReason,title)"

ISSUES_JSON="$issues_json" python3 - "$ROOT" "$VERSION" <<'PY'
import json
import os
import sys
from pathlib import Path

root = Path(sys.argv[1])
version = sys.argv[2]
issues = json.loads(os.environ.get("ISSUES_JSON", "[]") or "[]")
issues.sort(key=lambda item: item["number"])

errors = []
checked = 0

def extract(text: str, prefix: str):
    for line in text.splitlines():
        if line.startswith(prefix):
            return line[len(prefix):].strip()
    return None

for issue in issues:
    number = issue["number"]
    state_reason = (issue.get("stateReason") or "").strip()
    matches = sorted((root / ".adl" / version / "tasks").glob(f"issue-{number}__*/sor.md"))
    if not matches:
        errors.append(f"issue #{number}: missing canonical sor.md under .adl/{version}/tasks/issue-{number}__*/sor.md")
        continue
    if len(matches) > 1:
        for path in matches:
            errors.append(f"{path.relative_to(root)}: duplicate task bundle for closed issue #{number}")
        continue

    checked += 1
    path = matches[0]
    text = path.read_text()
    status = extract(text, "Status:")
    integration_state = extract(text, "- Integration state:")
    verification_scope = extract(text, "- Verification scope:")
    worktree_paths = extract(text, "- Worktree-only paths remaining:")

    if status != "DONE":
        errors.append(f"{path.relative_to(root)}: Status expected 'DONE' for closed issue #{number} but found {status!r}")
    if integration_state in (None, "worktree_only", "pr_open"):
        errors.append(
            f"{path.relative_to(root)}: Integration state for closed issue #{number} must not be worktree_only/pr_open; found {integration_state!r}"
        )
    if worktree_paths != "none":
        errors.append(
            f"{path.relative_to(root)}: Worktree-only paths remaining for closed issue #{number} expected 'none' but found {worktree_paths!r}"
        )

    if state_reason == "COMPLETED":
        if integration_state != "merged":
            errors.append(
                f"{path.relative_to(root)}: Integration state for CLOSED/COMPLETED issue #{number} expected 'merged' but found {integration_state!r}"
            )
        if verification_scope != "main_repo":
            errors.append(
                f"{path.relative_to(root)}: Verification scope for CLOSED/COMPLETED issue #{number} expected 'main_repo' but found {verification_scope!r}"
            )

if errors:
    print(f"ERROR: stale closed-issue SOR truth detected for version {version}", file=sys.stderr)
    for error in errors:
        print(error, file=sys.stderr)
    raise SystemExit(1)

print(f"PASS check_milestone_closed_issue_sor_truth version={version} checked={checked}")
PY
