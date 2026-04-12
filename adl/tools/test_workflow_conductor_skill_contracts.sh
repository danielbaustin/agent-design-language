#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/workflow-conductor/SKILL.md" ]]
[[ -f "${skills_root}/workflow-conductor/adl-skill.yaml" ]]
[[ -f "${skills_root}/workflow-conductor/agents/openai.yaml" ]]
[[ -f "${skills_root}/workflow-conductor/references/conductor-playbook.md" ]]
[[ -f "${skills_root}/workflow-conductor/references/output-contract.md" ]]
[[ -f "${skills_root}/workflow-conductor/scripts/select_next_skill.py" ]]
[[ -f "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq "thin orchestrator" "${skills_root}/workflow-conductor/SKILL.md"
grep -Fq "stop after routing and compliance recording" "${skills_root}/workflow-conductor/SKILL.md"
grep -Fq 'id: "workflow_conductor.v1"' "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq 'reference_doc: "/Users/daniel/git/agent-design-language/adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "policy.stop_after_routing_must_be_true" "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "python3 adl/tools/skills/workflow-conductor/scripts/select_next_skill.py" "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "route_issue" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "requires \`target.issue_number\`" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "workflow-conductor" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "resume from partially completed early steps" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

cat >"${tmpdir}/bootstrap_missing.json" <<'EOF'
{
  "target": {"issue_number": 1647},
  "workflow_state": {
    "bootstrap_present": false,
    "lifecycle_state": "unknown",
    "ready_state": "unknown",
    "pr_state": "none",
    "subagent_assigned": false,
    "evidence_used": ["missing_root_bundle"]
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional"
  }
}
EOF

cat >"${tmpdir}/stp_blocker.json" <<'EOF'
{
  "target": {"issue_number": 1647},
  "workflow_state": {
    "bootstrap_present": true,
    "card_blocker": "stp",
    "lifecycle_state": "pre_run",
    "ready_state": "unknown",
    "pr_state": "none",
    "subagent_assigned": false,
    "evidence_used": ["stp_path"]
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional"
  }
}
EOF

cat >"${tmpdir}/resume_to_run.json" <<'EOF'
{
  "target": {"issue_number": 1647},
  "workflow_state": {
    "bootstrap_present": true,
    "lifecycle_state": "pre_run",
    "ready_state": "pass",
    "pr_state": "none",
    "subagent_assigned": true,
    "evidence_used": ["doctor_json"]
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "required"
  }
}
EOF

cat >"${tmpdir}/pr_in_flight.json" <<'EOF'
{
  "target": {"issue_number": 1647, "pr_number": 1664},
  "workflow_state": {
    "bootstrap_present": true,
    "lifecycle_state": "execution_done",
    "ready_state": "pass",
    "pr_state": "open",
    "subagent_assigned": false,
    "evidence_used": ["pr_state"]
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "recommended"
  }
}
EOF

cat >"${tmpdir}/required_subagent_missing.json" <<'EOF'
{
  "target": {"issue_number": 1647},
  "workflow_state": {
    "bootstrap_present": true,
    "lifecycle_state": "run_bound",
    "ready_state": "pass",
    "pr_state": "none",
    "subagent_assigned": false,
    "evidence_used": ["worktree_path"]
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "required"
  }
}
EOF

python3 "${skills_root}/workflow-conductor/scripts/select_next_skill.py" --input "${tmpdir}/bootstrap_missing.json" >"${tmpdir}/bootstrap_missing.out.json"
python3 "${skills_root}/workflow-conductor/scripts/select_next_skill.py" --input "${tmpdir}/stp_blocker.json" >"${tmpdir}/stp_blocker.out.json"
python3 "${skills_root}/workflow-conductor/scripts/select_next_skill.py" --input "${tmpdir}/resume_to_run.json" >"${tmpdir}/resume_to_run.out.json"
python3 "${skills_root}/workflow-conductor/scripts/select_next_skill.py" --input "${tmpdir}/pr_in_flight.json" >"${tmpdir}/pr_in_flight.out.json"
python3 "${skills_root}/workflow-conductor/scripts/select_next_skill.py" --input "${tmpdir}/required_subagent_missing.json" >"${tmpdir}/required_subagent_missing.out.json"

python3 - "$tmpdir" <<'PY'
import json
import pathlib
import sys

tmp = pathlib.Path(sys.argv[1])

def load(name):
    return json.loads((tmp / name).read_text())

bootstrap = load("bootstrap_missing.out.json")
assert bootstrap["selected_skill"]["skill_name"] == "pr-init"

stp = load("stp_blocker.out.json")
assert stp["selected_skill"]["skill_name"] == "stp-editor"
assert stp["selected_skill"]["editor_skill"] == "stp-editor"

resume = load("resume_to_run.out.json")
assert resume["selected_skill"]["skill_name"] == "pr-run"
assert resume["workflow_compliance"]["policy_result"] == "PASS"

janitor = load("pr_in_flight.out.json")
assert janitor["selected_skill"]["skill_name"] == "pr-janitor"
assert janitor["workflow_compliance"]["policy_result"] == "PARTIAL"

required_missing = load("required_subagent_missing.out.json")
assert required_missing["selected_skill"]["skill_name"] == "pr-run"
assert required_missing["workflow_compliance"]["policy_result"] == "FAIL"
assert required_missing["workflow_compliance"]["bypasses"][0]["reason"] == "required_but_not_assigned"
PY

echo "PASS test_workflow_conductor_skill_contracts"
