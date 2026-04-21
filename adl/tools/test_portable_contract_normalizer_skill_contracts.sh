#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

skill_root="${skills_root}/portable-contract-normalizer"
python_script="${skill_root}/scripts/normalize_portable_contracts.py"

[[ -f "${skill_root}/SKILL.md" ]]
[[ -f "${skill_root}/adl-skill.yaml" ]]
[[ -f "${skill_root}/agents/openai.yaml" ]]
[[ -f "${skill_root}/references/portable-contract-normalizer-playbook.md" ]]
[[ -f "${skill_root}/references/output-contract.md" ]]
[[ -x "${python_script}" ]]
[[ -f "${skills_root}/docs/PORTABLE_CONTRACT_NORMALIZER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "portable-contract-normalizer"' "${skill_root}/adl-skill.yaml"
grep -Fq 'id: "portable_contract_normalizer.v1"' "${skill_root}/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/PORTABLE_CONTRACT_NORMALIZER_SKILL_INPUT_SCHEMA.md"' "${skill_root}/adl-skill.yaml"
grep -Fq "scan_and_apply_safe_fixes_requires_operator_approval" "${skill_root}/adl-skill.yaml"
grep -Fq "portability guard and optional narrow normalizer" "${skill_root}/SKILL.md"
grep -Fq "legitimate_evidence_redacted: false" "${skill_root}/references/output-contract.md"
grep -Fq "Schema id: \`portable_contract_normalizer.v1\`" "${skills_root}/docs/PORTABLE_CONTRACT_NORMALIZER_SKILL_INPUT_SCHEMA.md"

fixture_root="${tmpdir}/fixture"
report_root="${tmpdir}/report"
mkdir -p "${fixture_root}"
cat >"${fixture_root}/contract.md" <<'MD'
# Contract

Path: /Users/daniel/git/agent-design-language/.worktrees/adl-wp-2362
Temp: /private/var/folders/example/tmp/output.json
Env: USER=daniel
This has a hard-coded skill inventory that should be reviewed.
MD

python3 "${python_script}" \
  --root "${fixture_root}" \
  --out "${report_root}" \
  --run-id portable-contract-normalizer-contract-test >/tmp/portable-contract-normalizer.out

[[ -f "${report_root}/portable_contract_normalizer_report.json" ]]
[[ -f "${report_root}/portable_contract_normalizer_report.md" ]]
grep -Fq '"schema": "adl.portable_contract_normalizer_report.v1"' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"run_id": "portable-contract-normalizer-contract-test"' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"status": "findings"' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"absolute_host_path": 1' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"brittle_worktree_name": 1' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"machine_local_temp_path": 1' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"environment_specific_assertion": 1' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"stale_contract_reference": 1' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq '"mutated_repository": false' "${report_root}/portable_contract_normalizer_report.json"
grep -Fq "## Safe Mechanical Normalization" "${report_root}/portable_contract_normalizer_report.md"
grep -Fq "## Design Decisions Required" "${report_root}/portable_contract_normalizer_report.md"
grep -Fq "mutated_repository: false" "${report_root}/portable_contract_normalizer_report.md"

apply_root="${tmpdir}/apply-fixture"
apply_report="${tmpdir}/apply-report"
mkdir -p "${apply_root}"
cp "${fixture_root}/contract.md" "${apply_root}/contract.md"
python3 "${python_script}" \
  --root "${apply_root}" \
  --out "${apply_report}" \
  --run-id portable-contract-normalizer-apply-test \
  --apply >/tmp/portable-contract-normalizer-apply.out

grep -Fq '"status": "blocked"' "${apply_report}/portable_contract_normalizer_report.json"
grep -Fq '"mutated_repository": true' "${apply_report}/portable_contract_normalizer_report.json"
grep -Fq '<host-path>' "${apply_root}/contract.md"
grep -Fq '<temp-path>' "${apply_root}/contract.md"
grep -Fq '.worktrees/adl-wp-<issue>' "${apply_root}/contract.md"
grep -Fq 'USER=daniel' "${apply_root}/contract.md"

missing_report="${tmpdir}/missing-report"
python3 "${python_script}" \
  --root "${tmpdir}/does-not-exist" \
  --out "${missing_report}" \
  --run-id portable-contract-normalizer-missing-test >/tmp/portable-contract-normalizer-missing.out
grep -Fq '"status": "not_run"' "${missing_report}/portable_contract_normalizer_report.json"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skill_root}/SKILL.md"

echo "PASS test_portable_contract_normalizer_skill_contracts"

