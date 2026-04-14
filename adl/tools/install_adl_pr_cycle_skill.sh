#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "${repo_root}" ]]; then
  echo "install_adl_pr_cycle_skill.sh: must be run from inside the repo" >&2
  exit 1
fi

codex_home="${CODEX_HOME:-$HOME/.codex}"
source_path="${ADL_PR_CYCLE_SOURCE_PATH:-${repo_root}/docs/tooling/adl_pr_cycle_skill.md}"
dest_dir="${codex_home}/skills/adl_pr_cycle"
dest_path="${dest_dir}/SKILL.md"

if [[ ! -f "${source_path}" ]]; then
  echo "install_adl_pr_cycle_skill.sh: source skill contract missing: ${source_path}" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" "${source_path}"

mkdir -p "${dest_dir}"
cp "${source_path}" "${dest_path}"

if ! cmp -s "${source_path}" "${dest_path}"; then
  echo "install_adl_pr_cycle_skill.sh: install verification failed for ${dest_path}" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" "${dest_path}"

echo "INSTALLED ${dest_path}"
