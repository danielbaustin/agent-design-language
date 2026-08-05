#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

[[ ! -e "$ROOT/adl/tools/check_issue_metadata_parity.sh" ]] || {
  echo "retired metadata-parity helper unexpectedly exists" >&2
  exit 1
}

grep -Fq 'csdlc-github-issue run --request' "$ROOT/adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq 'csdlc-doctor --repo <repo> --issue <issue>' "$ROOT/adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md"

echo "PASS test_check_issue_metadata_parity"
