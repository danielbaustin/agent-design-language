#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

assert_skill_bundle() {
  local root="$1"

  for skill in workflow-conductor pr-init pr-ready pr-run pr-finish pr-janitor pr-closeout repo-code-review test-generator demo-operator medium-article-writer arxiv-paper-writer diagram-author stp-editor sip-editor sor-editor; do
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
  [[ -f "${root}/skills/test-generator/SKILL.md" ]]
  [[ -f "${root}/skills/demo-operator/SKILL.md" ]]
  [[ -f "${root}/skills/medium-article-writer/SKILL.md" ]]
  [[ -f "${root}/skills/arxiv-paper-writer/SKILL.md" ]]
  [[ -f "${root}/skills/diagram-author/SKILL.md" ]]
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
  grep -Fq "smallest truthful test surface" "${root}/skills/test-generator/SKILL.md"
  grep -Fq "run one named demo" "${root}/skills/demo-operator/SKILL.md"
  grep -Fq "stopping before publication" "${root}/skills/medium-article-writer/SKILL.md"
  grep -Fq "without submitting, publishing, inventing citations" "${root}/skills/arxiv-paper-writer/SKILL.md"
  grep -Fq "diagram-as-code and model-as-code router" "${root}/skills/diagram-author/SKILL.md"
  grep -Fq "bounded editing of \`stp.md\`" "${root}/skills/stp-editor/SKILL.md"
  grep -Fq "truthful lifecycle state" "${root}/skills/sip-editor/SKILL.md"
  grep -Fq "truthful execution and integration state" "${root}/skills/sor-editor/SKILL.md"

  bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
    "${root}/skills/workflow-conductor/SKILL.md" \
    "${root}/skills/pr-init/SKILL.md" \
    "${root}/skills/pr-ready/SKILL.md" \
    "${root}/skills/pr-run/SKILL.md" \
    "${root}/skills/pr-finish/SKILL.md" \
    "${root}/skills/pr-janitor/SKILL.md" \
    "${root}/skills/pr-closeout/SKILL.md" \
    "${root}/skills/repo-code-review/SKILL.md" \
    "${root}/skills/test-generator/SKILL.md" \
    "${root}/skills/demo-operator/SKILL.md" \
    "${root}/skills/medium-article-writer/SKILL.md" \
    "${root}/skills/arxiv-paper-writer/SKILL.md" \
    "${root}/skills/diagram-author/SKILL.md" \
    "${root}/skills/stp-editor/SKILL.md" \
    "${root}/skills/sip-editor/SKILL.md" \
    "${root}/skills/sor-editor/SKILL.md"
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
[[ -L "${CODEX_HOME}/skills/arxiv-paper-writer" ]]
[[ -L "${CODEX_HOME}/skills/diagram-author" ]]
[[ "$(cd "${CODEX_HOME}/skills/pr-init" && pwd -P)" == "${repo_root}/adl/tools/skills/pr-init" ]]

malformed_root="${tmpdir}/malformed-skills"
cp -R "${repo_root}/adl/tools/skills" "${malformed_root}"
cat >"${malformed_root}/workflow-conductor/SKILL.md" <<'EOF'
---
name: broken
description: first
description: second
---
EOF
if ADL_OPERATIONAL_SKILLS_SOURCE_ROOT="${malformed_root}" \
  bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null 2>&1; then
  echo "expected malformed operational skill source to fail" >&2
  exit 1
fi

if ADL_OPERATIONAL_SKILLS_INSTALL_MODE=bogus bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null 2>&1; then
  echo "expected invalid install mode to fail" >&2
  exit 1
fi

echo "PASS test_install_adl_operational_skills"
