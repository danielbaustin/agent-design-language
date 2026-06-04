#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

surfaces=(
  "AGENTS.md"
  "adl/tools/skills"
  "docs/templates"
  "docs/tooling/editor"
)

forbidden_patterns=(
  "adl pr create"
  "adl pr init"
  "adl pr doctor"
  "adl pr ready"
  "adl pr preflight"
  "adl pr run"
  "adl pr finish"
  "adl tooling prompt-template"
  "adl tooling code-review"
  "adl tooling review-card-surface"
  "adl tooling review-runtime-surface"
  "adl tooling verify-review-output-provenance"
  "adl tooling verify-repo-review-contract"
)

for pattern in "${forbidden_patterns[@]}"; do
  if rg -n --fixed-strings "$pattern" "${surfaces[@]}"; then
    echo "FAIL: live command guidance still teaches deprecated command: $pattern" >&2
    exit 1
  fi
done

required_patterns=(
  "adl/tools/pr.sh run <issue>"
  "adl-csdlc tooling prompt-template"
  "adl-review verify-repo-contract --review <review.md>"
)

for pattern in "${required_patterns[@]}"; do
  if ! rg -n --fixed-strings "$pattern" "${surfaces[@]}" >/dev/null; then
    echo "FAIL: live command guidance is missing required command: $pattern" >&2
    exit 1
  fi
done

echo "PASS: live CLI owner command guidance is aligned with wrapper migration policy"
