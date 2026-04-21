#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
docs_root="${skills_root}/docs"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/pr-stack-manager/SKILL.md" ]]
[[ -f "${skills_root}/pr-stack-manager/adl-skill.yaml" ]]
[[ -f "${skills_root}/pr-stack-manager/agents/openai.yaml" ]]
[[ -f "${skills_root}/pr-stack-manager/references/output-contract.md" ]]
[[ -f "${skills_root}/pr-stack-manager/references/pr-stack-manager-playbook.md" ]]
[[ -x "${skills_root}/pr-stack-manager/scripts/analyze_pr_stack.py" ]]
[[ -f "${docs_root}/PR_STACK_MANAGER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: pr-stack-manager' "${skills_root}/pr-stack-manager/SKILL.md"
grep -Fq 'id: "pr-stack-manager"' "${skills_root}/pr-stack-manager/adl-skill.yaml"
grep -Fq 'id: "pr_stack_manager.v1"' "${skills_root}/pr-stack-manager/adl-skill.yaml"
grep -Fq 'pr_stack_manager.v1' "${docs_root}/PR_STACK_MANAGER_SKILL_INPUT_SCHEMA.md"
grep -Fq "dependency_graph" "${skills_root}/pr-stack-manager/references/output-contract.md"
grep -Fq "Repair Policy" "${skills_root}/pr-stack-manager/references/pr-stack-manager-playbook.md"
grep -Fq "pr-stack-manager" "${docs_root}/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "analyze_pr_stack.py" "${skills_root}/pr-stack-manager/adl-skill.yaml"

cat >"${tmpdir}/stack.json" <<'EOF'
{
  "root_issue": 2285,
  "target": {
    "issue_number": 2285
  },
  "nodes": [
    {
      "issue_number": 2283,
      "branch": "codex/2283-records-hygiene",
      "pr_number": 2301,
      "state": "open",
      "base_ref": "main",
      "expected_base_ref": "main",
      "depends_on": []
    },
    {
      "issue_number": 2285,
      "branch": "codex/2285-pr-stack-manager",
      "pr_number": 2302,
      "state": "open",
      "base_ref": "main",
      "expected_base_ref": "codex/2283-records-hygiene",
      "depends_on": [2283]
    }
  ]
}
EOF

if python3 "${skills_root}/pr-stack-manager/scripts/analyze_pr_stack.py" \
  "${tmpdir}/stack.json" \
  --out "${tmpdir}/pr-stack.out.json" >/dev/null 2>&1; then
  echo "expected stack analyzer to report blocking base drift" >&2
  exit 1
fi

python3 - "${tmpdir}/pr-stack.out.json" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert payload["schema_version"] == "pr_stack_manager.analysis.v1"
assert payload["status"] == "blocked"
areas = {finding["area"] for finding in payload["findings"]}
assert "base_alignment" in areas
assert payload["dependency_graph"]["cycle_detected"] is False
assert payload["planned_actions"][0]["action"] == "retarget_or_rebase_base"
PY

CODEX_HOME="${tmpdir}/codex-home" \
ADL_OPERATIONAL_SKILLS_SOURCE_ROOT="${skills_root}" \
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/tmp/pr-stack-manager-install.out
[[ -f "${tmpdir}/codex-home/skills/pr-stack-manager/SKILL.md" ]]
diff -q \
  "${skills_root}/pr-stack-manager/SKILL.md" \
  "${tmpdir}/codex-home/skills/pr-stack-manager/SKILL.md" >/dev/null

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/pr-stack-manager/SKILL.md"

echo "PASS test_pr_stack_manager_skill_contracts"
