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
  mkdir -p "$repo/adl/tools"
  cp "$ROOT/adl/tools/check_no_tracked_adl_issue_record_residue.sh" "$repo/adl/tools/check_no_tracked_adl_issue_record_residue.sh"
  chmod +x "$repo/adl/tools/check_no_tracked_adl_issue_record_residue.sh"
  printf '.adl/\n' >"$repo/.gitignore"
}

PASS_REPO="$TMP/pass"
mkdir -p "$PASS_REPO"
init_repo "$PASS_REPO"
mkdir -p "$PASS_REPO/docs/records/v0.87.1/legacy-issue-records/v0.87/tasks/issue-1414__sample"
printf 'historical\n' >"$PASS_REPO/docs/records/v0.87.1/legacy-issue-records/v0.87/tasks/issue-1414__sample/sor.md"
git -C "$PASS_REPO" add .
git -C "$PASS_REPO" commit -q -m init
(cd "$PASS_REPO" && bash ./adl/tools/check_no_tracked_adl_issue_record_residue.sh >/dev/null)

FAIL_REPO="$TMP/fail"
mkdir -p "$FAIL_REPO"
init_repo "$FAIL_REPO"
mkdir -p "$FAIL_REPO/.adl/v0.87.1/tasks/issue-1596__bad"
printf 'legacy\n' >"$FAIL_REPO/.adl/v0.87.1/tasks/issue-1596__bad/sor.md"
git -C "$FAIL_REPO" add -f .adl/v0.87.1/tasks/issue-1596__bad/sor.md adl/tools/check_no_tracked_adl_issue_record_residue.sh .gitignore
git -C "$FAIL_REPO" commit -q -m init
if (cd "$FAIL_REPO" && bash ./adl/tools/check_no_tracked_adl_issue_record_residue.sh >/dev/null 2>&1); then
  echo "expected tracked residue check to fail" >&2
  exit 1
fi

bash "$ROOT/adl/tools/check_no_tracked_adl_issue_record_residue.sh" >/dev/null
echo "PASS test_check_no_tracked_adl_issue_record_residue"
