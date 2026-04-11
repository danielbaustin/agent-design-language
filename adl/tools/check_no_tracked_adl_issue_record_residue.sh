#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

bad=()
while IFS= read -r path; do
  [[ -n "$path" ]] || continue
  case "$path" in
    .adl/*/bodies/issue-*.md)
      bad+=("$path")
      ;;
    .adl/*/tasks/issue-*/*.md)
      case "$path" in
        */stp.md|*/sip.md|*/sor.md) bad+=("$path") ;;
      esac
      ;;
  esac
done <<EOF
$(git -C "$ROOT" ls-files -- '.adl/*/bodies/issue-*.md' '.adl/*/tasks/issue-*/*.md')
EOF

if [[ "${#bad[@]}" -gt 0 ]]; then
  {
    echo "Tracked legacy .adl issue-record residue is not allowed."
    echo "Move historical/public examples under docs/records/ and keep live issue bundles local-only under ignored .adl/."
    echo
    printf '%s\n' "${bad[@]}"
  } >&2
  exit 1
fi

echo "PASS check_no_tracked_adl_issue_record_residue"
