#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

skill_root="${skills_root}/issue-folding"
python_script="${skill_root}/scripts/classify_issue_folding.py"

[[ -f "${skill_root}/SKILL.md" ]]
[[ -f "${skill_root}/adl-skill.yaml" ]]
[[ -f "${skill_root}/agents/openai.yaml" ]]
[[ -f "${skill_root}/references/issue-folding-playbook.md" ]]
[[ -f "${skill_root}/references/output-contract.md" ]]
[[ -x "${python_script}" ]]
[[ -f "${skills_root}/docs/ISSUE_FOLDING_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "issue-folding"' "${skill_root}/adl-skill.yaml"
grep -Fq 'id: "issue_folding.v1"' "${skill_root}/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ISSUE_FOLDING_SKILL_INPUT_SCHEMA.md"' "${skill_root}/adl-skill.yaml"
grep -Fq "linked_dispositions_require_reference_evidence" "${skill_root}/adl-skill.yaml"
grep -Fq "issue disposition classifier and closeout-routing helper" "${skill_root}/SKILL.md"
grep -Fq "issue_closed: false" "${skill_root}/references/output-contract.md"
grep -Fq "Schema id: \`issue_folding.v1\`" "${skills_root}/docs/ISSUE_FOLDING_SKILL_INPUT_SCHEMA.md"

duplicate_root="${tmpdir}/duplicate"
duplicate_report="${tmpdir}/duplicate-report"
mkdir -p "${duplicate_root}/task"
cat >"${duplicate_root}/source.md" <<'MD'
## Summary

This is a duplicate of #2400 and the work is already tracked by PR: pull/2401.
MD
cat >"${duplicate_root}/task/stp.md" <<'MD'
Issue is duplicate work already tracked by #2400 and should not execute separately.
MD
cat >"${duplicate_root}/task/sip.md" <<'MD'
Execution should stop because the issue is duplicate work.
MD
cat >"${duplicate_root}/task/sor.md" <<'MD'
No execution happened here.
MD

python3 "${python_script}" \
  --task-bundle "${duplicate_root}/task" \
  --source-prompt "${duplicate_root}/source.md" \
  --out "${duplicate_report}" \
  --run-id issue-folding-duplicate-test >/tmp/issue-folding-duplicate.out

[[ -f "${duplicate_report}/issue_folding_report.json" ]]
[[ -f "${duplicate_report}/issue_folding_report.md" ]]

actionable_root="${tmpdir}/actionable"
actionable_report="${tmpdir}/actionable-report"
mkdir -p "${actionable_root}/task"
cat >"${actionable_root}/source.md" <<'MD'
## Summary

Implement the new skill with no prior linked closure markers.
MD
cat >"${actionable_root}/task/stp.md" <<'MD'
The issue still needs direct execution.
MD

python3 "${python_script}" \
  --task-bundle "${actionable_root}/task" \
  --source-prompt "${actionable_root}/source.md" \
  --out "${actionable_report}" \
  --run-id issue-folding-actionable-test >/tmp/issue-folding-actionable.out

blocked_root="${tmpdir}/blocked"
blocked_report="${tmpdir}/blocked-report"
mkdir -p "${blocked_root}/task"
cat >"${blocked_root}/source.md" <<'MD'
## Summary

This issue is duplicate of #2600 but also obsolete after the policy change.
MD

python3 "${python_script}" \
  --task-bundle "${blocked_root}/task" \
  --source-prompt "${blocked_root}/source.md" \
  --out "${blocked_report}" \
  --run-id issue-folding-blocked-test >/tmp/issue-folding-blocked.out

python3 - "${tmpdir}" "${duplicate_report}" "${actionable_report}" "${blocked_report}" <<'PY'
import json
import sys
from pathlib import Path

tmpdir = sys.argv[1]
duplicate_report = Path(sys.argv[2])
actionable_report = Path(sys.argv[3])
blocked_report = Path(sys.argv[4])

duplicate_json = json.loads((duplicate_report / "issue_folding_report.json").read_text())
assert duplicate_json["schema"] == "adl.issue_folding_report.v1"
assert duplicate_json["run_id"] == "issue-folding-duplicate-test"
assert duplicate_json["status"] == "foldable"
assert duplicate_json["classification"] == "duplicate"
assert duplicate_json["closure_outcome"] == "duplicate"
assert duplicate_json["recommended_handoff"] == "pr-closeout"
assert duplicate_json["worktree_action"] == "retire_bound_worktree_if_present"
assert "#2400" in duplicate_json["closure_references"]
assert "PR:2401" in duplicate_json["closure_references"]
duplicate_md = (duplicate_report / "issue_folding_report.md").read_text()
assert "## Closure Outcome" in duplicate_md
assert "issue_closed: false" in duplicate_md

actionable_json = json.loads((actionable_report / "issue_folding_report.json").read_text())
assert actionable_json["status"] == "actionable"
assert actionable_json["classification"] == "actionable"
assert actionable_json["closure_outcome"] is None
assert actionable_json["recommended_handoff"] == "workflow-conductor"

blocked_json = json.loads((blocked_report / "issue_folding_report.json").read_text())
assert blocked_json["status"] == "blocked"
assert blocked_json["classification"] == "blocked"
assert blocked_json["recommended_handoff"] == "operator-review"

for report_root in (duplicate_report, actionable_report, blocked_report):
    for path in report_root.iterdir():
        if tmpdir in path.read_text():
            raise AssertionError("issue folding artifacts should not leak absolute temp paths")
PY

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skill_root}/SKILL.md"

echo "PASS test_issue_folding_skill_contracts"
