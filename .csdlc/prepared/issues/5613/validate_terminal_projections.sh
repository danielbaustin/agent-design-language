#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
common="$(git -C "$root" rev-parse --git-common-dir)"

check_target() {
  local issue="$1"
  local pr="$2"
  local observed_sha="$3"
  local index="$root/.csdlc/issues/$issue/index.json"
  local receipt="$common/csdlc-v2/closeout/$issue.json"

  jq -e --argjson issue "$issue" --argjson pr "$pr" --arg sha "$observed_sha" '
    .issue == $issue and
    .phase == "closed_out" and
    .claim == null and
    .publication.pull_request == $pr and
    .publication.observed_state == "merged" and
    .terminal.pull_request == $pr and
    .terminal.disposition == "merged" and
    .terminal.observed_sha == $sha
  ' "$index" >/dev/null

  jq -e --argjson issue "$issue" --argjson pr "$pr" --arg sha "$observed_sha" --slurpfile index "$index" '
    .issue == $issue and
    .record.phase == "closed_out" and
    .record.claim == null and
    .record.publication.pull_request == $pr and
    .record.terminal.pull_request == $pr and
    .record.terminal.disposition == "merged" and
    .record.terminal.observed_sha == $sha and
    .record.digest == $index[0].digest
  ' "$receipt" >/dev/null
}

check_target 5337 5607 73665c0571d417dedf689f3afeb009df7cbeea1f
check_target 5339 5612 ba604e5f0ee16af901a4d8d7cb801c323500828d
check_target 5358 5606 e048230245b1ad101c8056678123a2747faa4b60
check_target 5591 5608 59b61985c9964e56d83272efa7035f12be462fd7
check_target 5602 5604 6c6b512286cca4833fd19e35b9377b3994d76b5b

if rg -n '/Volumes/FastWork|/private/tmp|/var/folders|/Users/[^/]+/' \
  "$root/.csdlc/issues/5591/cards/sor.values.json" \
  "$root/.csdlc/issues/5591/cards/sor.md" || \
  jq -e '
    .cards.sor.content.values.actual_validation
    | tostring
    | test("/Volumes/FastWork|/private/tmp|/var/folders|/Users/[^/]+/")
  ' "$common/csdlc-v2/closeout/5591.json" >/dev/null; then
  echo "issue 5591 SOR retains a machine-local path" >&2
  exit 1
fi

test ! -e "$root/.csdlc/evidence/5591/exact-revision/guardian-soak.json"

printf 'issue 5613 terminal projection proof: pass\n'
