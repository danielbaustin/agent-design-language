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
grep -Fq 'preflight -> init -> create -> start -> codex -> run_if_required -> finish -> report' "${installed}"
grep -Fq '.worktrees/adl-wp-<issue_num>' "${installed}"
grep -Fq 'bash ./adl/tools/pr.sh init <issue_num> --slug <slug> [--version <version>]' "${installed}"
grep -Fq 'bash adl/tools/test_five_command_regression_suite.sh' "${installed}"

echo "PASS test_install_adl_pr_cycle_skill"
