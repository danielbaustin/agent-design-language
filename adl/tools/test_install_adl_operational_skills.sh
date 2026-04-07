#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

export CODEX_HOME="${tmpdir}/codex-home"

bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null

for skill in pr-init pr-ready pr-run pr-finish pr-janitor repo-code-review; do
  [[ -d "${CODEX_HOME}/skills/${skill}" ]]
done

[[ -f "${CODEX_HOME}/skills/pr-init/SKILL.md" ]]
[[ -f "${CODEX_HOME}/skills/pr-ready/SKILL.md" ]]
[[ -f "${CODEX_HOME}/skills/pr-run/SKILL.md" ]]
[[ -f "${CODEX_HOME}/skills/pr-finish/SKILL.md" ]]
[[ -f "${CODEX_HOME}/skills/pr-janitor/SKILL.md" ]]
[[ -f "${CODEX_HOME}/skills/repo-code-review/SKILL.md" ]]

grep -Fq "qualitative card review" "${CODEX_HOME}/skills/pr-init/SKILL.md"
grep -Fq "execution_readiness" "${CODEX_HOME}/skills/pr-ready/references/output-contract.md"
grep -Fq "perform the bounded implementation work" "${CODEX_HOME}/skills/pr-run/SKILL.md"
grep -Fq "truthful closeout" "${CODEX_HOME}/skills/pr-finish/SKILL.md"
grep -Fq "failed checks or merge conflicts" "${CODEX_HOME}/skills/pr-janitor/SKILL.md"
grep -Fq "findings-first" "${CODEX_HOME}/skills/repo-code-review/SKILL.md"

echo "PASS test_install_adl_operational_skills"
