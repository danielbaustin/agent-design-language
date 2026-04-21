#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
docs_root="${skills_root}/docs"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/records-hygiene/SKILL.md" ]]
[[ -f "${skills_root}/records-hygiene/adl-skill.yaml" ]]
[[ -f "${skills_root}/records-hygiene/agents/openai.yaml" ]]
[[ -f "${skills_root}/records-hygiene/references/output-contract.md" ]]
[[ -f "${skills_root}/records-hygiene/references/records-hygiene-playbook.md" ]]
[[ -x "${skills_root}/records-hygiene/scripts/analyze_records_hygiene.py" ]]
[[ -f "${skills_root}/docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: records-hygiene' "${skills_root}/records-hygiene/SKILL.md"
grep -Fq 'id: "records-hygiene"' "${skills_root}/records-hygiene/adl-skill.yaml"
grep -Fq 'id: "records_hygiene.v1"' "${skills_root}/records-hygiene/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md"' "${skills_root}/records-hygiene/adl-skill.yaml"
grep -Fq 'records_hygiene.v1' "${skills_root}/docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md"
grep -Fq "safe_repairs_applied" "${skills_root}/records-hygiene/references/output-contract.md"
grep -Fq "records-hygiene" "${docs_root}/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "placeholder" "${skills_root}/records-hygiene/references/records-hygiene-playbook.md"
grep -Fq "analyze_records_hygiene.py" "${skills_root}/records-hygiene/adl-skill.yaml"

cat >"${tmpdir}/sor.md" <<'EOF'
Task ID: issue-9001
Status: IN_PROGRESS

## Main Repo Integration (REQUIRED)

- Integration state: pr_ready
- Result: TODO
EOF

if python3 "${skills_root}/records-hygiene/scripts/analyze_records_hygiene.py" \
  --repo-root "${repo_root}" \
  "${tmpdir}/sor.md" \
  --out "${tmpdir}/records-hygiene.out.json" >/dev/null 2>&1; then
  echo "expected records hygiene analyzer to report blocking findings" >&2
  exit 1
fi

python3 - "${tmpdir}/records-hygiene.out.json" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
assert payload["schema_version"] == "records_hygiene.analysis.v1"
assert payload["status"] == "blocked"
areas = {finding["area"] for finding in payload["findings"]}
assert "integration_truth" in areas
assert "placeholder_drift" in areas
assert payload["counts"]["blocking"] >= 1
assert payload["handoff_state"]["ready_for_editor"] is True
PY

CODEX_HOME="${tmpdir}/codex-home" \
ADL_OPERATIONAL_SKILLS_SOURCE_ROOT="${skills_root}" \
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/tmp/records-hygiene-install.out
[[ -f "${tmpdir}/codex-home/skills/records-hygiene/SKILL.md" ]]
diff -q \
  "${skills_root}/records-hygiene/SKILL.md" \
  "${tmpdir}/codex-home/skills/records-hygiene/SKILL.md" >/dev/null

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/records-hygiene/SKILL.md"

echo "PASS test_records_hygiene_skill_contracts"
