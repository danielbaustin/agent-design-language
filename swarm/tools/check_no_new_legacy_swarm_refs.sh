#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
cd "$ROOT_DIR"

base_ref=""
if [[ $# -ge 1 && -n "${1:-}" ]]; then
  base_ref="$1"
elif [[ -n "${GITHUB_BASE_REF:-}" ]]; then
  candidate="origin/${GITHUB_BASE_REF}"
  if git rev-parse --verify "$candidate" >/dev/null 2>&1; then
    base_ref="$candidate"
  else
    git fetch --no-tags --depth=1 origin "$GITHUB_BASE_REF" >/dev/null 2>&1 || true
    if git rev-parse --verify "$candidate" >/dev/null 2>&1; then
      base_ref="$candidate"
    else
      base_ref="FETCH_HEAD"
    fi
  fi
elif git rev-parse --verify origin/main >/dev/null 2>&1; then
  base_ref="origin/main"
else
  base_ref="$(git rev-parse HEAD~1)"
fi

range_spec="${base_ref}...HEAD"

echo "Guardrail base ref: ${base_ref}"
echo "Guardrail diff range: ${range_spec}"

# Allow explicit legacy-name references only in bounded historical or migration-planning surfaces.
allowlist_regex='^(README\.md$|swarm/README\.md$|swarm/examples/README\.md$|\.github/workflows/ci\.yaml$|\.github/workflows/nightly-coverage-ratchet\.yaml$|docs/milestones/v0\.[0-6]/|docs/milestones/v0\.7/SWARM_NAME_CHANGE_PLANNING_v0\.7\.md$|docs/milestones/v0\.85/README\.md$|docs/milestones/v0\.85/SWARM_REMOVAL_PLANNING\.md$|docs/milestones/v0\.85/DECISIONS_v0\.85\.md$|docs/milestones/v0\.85/DESIGN_v0\.85\.md$|docs/milestones/v0\.85/MILESTONE_CHECKLIST_v0\.85\.md$|docs/milestones/v0\.85/RELEASE_PLAN_v0\.85\.md$|docs/milestones/v0\.85/SPRINT_v0\.85\.md$|docs/milestones/v0\.85/WBS_v0\.85\.md$|swarm/src/bin/swarm\.rs$|swarm/src/bin/swarm_remote\.rs$|swarm/src/env_compat\.rs$|swarm/src/cli/mod\.rs$|swarm/src/cli/usage\.rs$|swarm/Cargo\.toml$|swarm/tests/cli_smoke\.rs$|swarm/tools/check_no_new_legacy_swarm_refs\.sh$)'
legacy_regex='(^|[^A-Za-z0-9_])(swarm-remote|SWARM_[A-Z0-9_]+)([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])swarm([^:/A-Za-z0-9_]|$)'

violations=0

while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if [[ "$file" =~ $allowlist_regex ]]; then
    continue
  fi

  added_lines="$(git diff -U0 "$range_spec" -- "$file" | awk '/^\+[^+]/ {print substr($0,2)}')"
  [[ -z "$added_lines" ]] && continue

  if grep -En "$legacy_regex" <<<"$added_lines" >/tmp/adl_legacy_ref_hits.txt; then
    violations=1
    echo "Legacy naming additions are not allowed in: $file"
    sed 's/^/  /' /tmp/adl_legacy_ref_hits.txt
  fi
done < <(git diff --name-only --diff-filter=AMRT "$range_spec")

if [[ "$violations" -ne 0 ]]; then
  echo
  echo "Guardrail failed: found new legacy-name references outside allowlist."
  echo "If intentional, add a scoped allowlist entry in this script with rationale."
  exit 1
fi

echo "Guardrail passed: no new unintended legacy-name references."
