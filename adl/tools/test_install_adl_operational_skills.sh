#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

assert_skill_bundle() {
  local root="$1"

  for skill in workflow-conductor pr-init pr-ready pr-run pr-finish pr-janitor pr-closeout repo-code-review stp-editor sip-editor sor-editor; do
    [[ -d "${root}/skills/${skill}" ]]
  done

  [[ -f "${root}/skills/workflow-conductor/SKILL.md" ]]
  [[ -f "${root}/skills/pr-init/SKILL.md" ]]
  [[ -f "${root}/skills/pr-ready/SKILL.md" ]]
  [[ -f "${root}/skills/pr-run/SKILL.md" ]]
  [[ -f "${root}/skills/pr-finish/SKILL.md" ]]
  [[ -f "${root}/skills/pr-janitor/SKILL.md" ]]
  [[ -f "${root}/skills/pr-closeout/SKILL.md" ]]
  [[ -f "${root}/skills/repo-code-review/SKILL.md" ]]
  [[ -f "${root}/skills/stp-editor/SKILL.md" ]]
  [[ -f "${root}/skills/sip-editor/SKILL.md" ]]
  [[ -f "${root}/skills/sor-editor/SKILL.md" ]]

  grep -Fq "thin orchestrator" "${root}/skills/workflow-conductor/SKILL.md"
  grep -Fq "qualitative card review" "${root}/skills/pr-init/SKILL.md"
  grep -Fq "execution_readiness" "${root}/skills/pr-ready/references/output-contract.md"
  grep -Fq "perform the bounded implementation work" "${root}/skills/pr-run/SKILL.md"
  grep -Fq "truthful closeout" "${root}/skills/pr-finish/SKILL.md"
  grep -Fq "failed checks or merge conflicts" "${root}/skills/pr-janitor/SKILL.md"
  grep -Fq "post-merge and post-closure cleanup phase" "${root}/skills/pr-closeout/SKILL.md"
  grep -Fq "findings-first" "${root}/skills/repo-code-review/SKILL.md"
  grep -Fq "bounded editing of \`stp.md\`" "${root}/skills/stp-editor/SKILL.md"
  grep -Fq "truthful lifecycle state" "${root}/skills/sip-editor/SKILL.md"
  grep -Fq "truthful execution and integration state" "${root}/skills/sor-editor/SKILL.md"
}

export CODEX_HOME="${tmpdir}/codex-home-copy"
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null
assert_skill_bundle "${CODEX_HOME}"
[[ ! -L "${CODEX_HOME}/skills/pr-init" ]]

export CODEX_HOME="${tmpdir}/codex-home-symlink"
ADL_OPERATIONAL_SKILLS_INSTALL_MODE=symlink bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null
assert_skill_bundle "${CODEX_HOME}"
[[ -L "${CODEX_HOME}/skills/pr-init" ]]
[[ -L "${CODEX_HOME}/skills/pr-ready" ]]
[[ "$(cd "${CODEX_HOME}/skills/pr-init" && pwd -P)" == "${repo_root}/adl/tools/skills/pr-init" ]]

if ADL_OPERATIONAL_SKILLS_INSTALL_MODE=bogus bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null 2>&1; then
  echo "expected invalid install mode to fail" >&2
  exit 1
fi

echo "PASS test_install_adl_operational_skills"
