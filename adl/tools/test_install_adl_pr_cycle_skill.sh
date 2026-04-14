#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

export CODEX_HOME="${tmpdir}/codex-home"

bash "${repo_root}/adl/tools/install_adl_pr_cycle_skill.sh" >/dev/null

installed="${CODEX_HOME}/skills/adl_pr_cycle/SKILL.md"
source_path="${repo_root}/docs/tooling/adl_pr_cycle_skill.md"

[[ -f "${installed}" ]]
cmp -s "${source_path}" "${installed}"
bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" "${installed}"
grep -Fq 'preflight -> issue_ready -> init -> start -> codex -> run_if_required -> finish -> report' "${installed}"
grep -Fq '.worktrees/adl-wp-<issue_num>' "${installed}"
grep -Fq 'bash ./adl/tools/pr.sh init <issue_num> --slug <slug> [--version <version>]' "${installed}"
grep -Fq 'bash adl/tools/test_five_command_regression_suite.sh' "${installed}"

malformed_source="${tmpdir}/bad_adl_pr_cycle_skill.md"
cat >"${malformed_source}" <<'EOF'
---
name: adl_pr_cycle
description: broken: yaml
---
EOF

if ADL_PR_CYCLE_SOURCE_PATH="${malformed_source}" bash "${repo_root}/adl/tools/install_adl_pr_cycle_skill.sh" >/dev/null 2>&1; then
  echo "expected malformed adl_pr_cycle source to fail" >&2
  exit 1
fi

echo "PASS test_install_adl_pr_cycle_skill"
