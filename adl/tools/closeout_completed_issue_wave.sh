#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION=""
REPO=""
REPORT_PATH=""
ISSUES_CSV=""
DRY_RUN=0

usage() {
  cat <<'EOF'
Usage:
  closeout_completed_issue_wave.sh --version <v0.88> [--repo <owner/name>] [--issues <csv>] [--report <path>] [--dry-run]

Scans the local `.adl/<version>/tasks/` bundle set, finds GitHub issues that are
already CLOSED/COMPLETED for that version, and runs `adl/tools/pr.sh closeout`
for the matching local issue bundles.
EOF
}

die() {
  echo "closeout-wave: $*" >&2
  exit 1
}

trim() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "$value"
}

infer_repo() {
  local remote_url
  remote_url="$(git -C "$ROOT" remote get-url origin 2>/dev/null || true)"
  if [[ "$remote_url" =~ github.com[:/]([^/]+/[^/.]+)(\.git)?$ ]]; then
    printf '%s\n' "${BASH_REMATCH[1]}"
    return 0
  fi
  return 1
}

collect_local_issues() {
  local version="$1"
  local tasks_root="$ROOT/.adl/$version/tasks"
  [[ -d "$tasks_root" ]] || return 0
  find "$tasks_root" -mindepth 1 -maxdepth 1 -type d | \
    while IFS= read -r dir; do
      local base
      base="$(basename "$dir")"
      if [[ "$base" =~ ^issue-([0-9]+)__ ]]; then
        printf '%s\n' "${BASH_REMATCH[1]}"
      fi
    done | sort -n | uniq
}

write_report() {
  local path="$1"
  local mode="$2"
  local eligible_count="$3"
  local normalized_count="$4"
  local failed_count="$5"
  local normalized_list="$6"
  local failed_list="$7"
  mkdir -p "$(dirname "$path")"
  {
    echo "version: $VERSION"
    echo "repo: $REPO"
    echo "mode: $mode"
    echo "eligible_issues: $eligible_count"
    echo "normalized_issues: $normalized_count"
    echo "failed_issues: $failed_count"
    echo "normalized:"
    if [[ -n "$normalized_list" ]]; then
      while IFS= read -r line; do
        [[ -n "$line" ]] && echo "  - $line"
      done <<< "$normalized_list"
    else
      echo "  - none"
    fi
    echo "failures:"
    if [[ -n "$failed_list" ]]; then
      while IFS= read -r line; do
        [[ -n "$line" ]] && echo "  - $line"
      done <<< "$failed_list"
    else
      echo "  - none"
    fi
  } >"$path"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version) VERSION="${2:-}"; shift 2 ;;
    --repo) REPO="${2:-}"; shift 2 ;;
    --issues) ISSUES_CSV="${2:-}"; shift 2 ;;
    --report) REPORT_PATH="${2:-}"; shift 2 ;;
    --dry-run) DRY_RUN=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown arg: $1" ;;
  esac
done

[[ -n "$VERSION" ]] || die "missing --version"
command -v gh >/dev/null 2>&1 || die "gh is required"
command -v python3 >/dev/null 2>&1 || die "python3 is required"

if [[ -z "$REPO" ]]; then
  REPO="$(infer_repo)" || die "could not infer GitHub repo from origin remote; pass --repo"
fi

if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$ROOT/.adl/reports/closeout/closeout-wave-${VERSION}.md"
fi

local_issues="$(collect_local_issues "$VERSION")"
if [[ -z "$local_issues" ]]; then
  write_report "$REPORT_PATH" "$([[ "$DRY_RUN" == "1" ]] && echo dry_run || echo apply)" 0 0 0 "" ""
  echo "PASS closeout_completed_issue_wave version=$VERSION eligible=0"
  exit 0
fi

closed_json="$(gh issue list -R "$REPO" --state closed --label "version:$VERSION" --limit 200 --json number,stateReason)"

eligible_issues="$(
  LOCAL_ISSUES="$local_issues" CLOSED_JSON="$closed_json" ISSUES_CSV="$ISSUES_CSV" python3 - <<'PY'
import json, os
local_issues = {int(v) for v in os.environ.get("LOCAL_ISSUES", "").splitlines() if v.strip()}
closed = json.loads(os.environ.get("CLOSED_JSON", "[]") or "[]")
issue_filter = {
    int(v.strip()) for v in os.environ.get("ISSUES_CSV", "").split(",") if v.strip()
}
eligible = []
for item in closed:
    number = int(item["number"])
    if number not in local_issues:
        continue
    if issue_filter and number not in issue_filter:
        continue
    if (item.get("stateReason") or "").strip() != "COMPLETED":
        continue
    eligible.append(number)
for number in sorted(set(eligible)):
    print(number)
PY
)"

if [[ -z "$eligible_issues" ]]; then
  write_report "$REPORT_PATH" "$([[ "$DRY_RUN" == "1" ]] && echo dry_run || echo apply)" 0 0 0 "" ""
  echo "PASS closeout_completed_issue_wave version=$VERSION eligible=0"
  exit 0
fi

normalized_list=""
failed_list=""
normalized_count=0
failed_count=0
report_log="$(mktemp)"
trap 'rm -f "$report_log"' EXIT

while IFS= read -r issue; do
  [[ -n "$issue" ]] || continue
  if [[ "$DRY_RUN" == "1" ]]; then
    normalized_list+="${issue}"$'\n'
    normalized_count=$((normalized_count + 1))
    continue
  fi
  if bash "$ROOT/adl/tools/pr.sh" closeout "$issue" --version "$VERSION" --no-fetch-issue >>"$report_log" 2>&1; then
    normalized_list+="${issue}"$'\n'
    normalized_count=$((normalized_count + 1))
  else
    failed_count=$((failed_count + 1))
    last_error="$(tail -n 1 "$report_log" | tr '\n' ' ')"
    failed_list+="#${issue}: $(trim "$last_error")"$'\n'
  fi
done <<< "$eligible_issues"

write_report \
  "$REPORT_PATH" \
  "$([[ "$DRY_RUN" == "1" ]] && echo dry_run || echo apply)" \
  "$(printf '%s\n' "$eligible_issues" | sed '/^$/d' | wc -l | tr -d ' ')" \
  "$normalized_count" \
  "$failed_count" \
  "$normalized_list" \
  "$failed_list"

if [[ "$failed_count" != "0" ]]; then
  die "failed to normalize $failed_count closed issue bundle(s); see $REPORT_PATH"
fi

echo "PASS closeout_completed_issue_wave version=$VERSION normalized=$normalized_count"
