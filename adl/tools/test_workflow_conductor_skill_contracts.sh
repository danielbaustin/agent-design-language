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
[[ -f "${skills_root}/workflow-conductor/scripts/route_workflow.py" ]]
[[ -f "${skills_root}/workflow-conductor/scripts/select_next_skill.py" ]]
[[ -f "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq "thin orchestrator" "${skills_root}/workflow-conductor/SKILL.md"
grep -Fq "stop after routing and compliance recording" "${skills_root}/workflow-conductor/SKILL.md"
grep -Fq "writes one bounded routing artifact" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq 'continue`, `ask_operator`, or `stop`' "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq 'id: "workflow_conductor.v1"' "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq 'reference_doc: "/Users/daniel/git/agent-design-language/adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "policy.stop_after_routing_must_be_true" "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <validated-json>" "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "route_issue" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "requires \`target.issue_number\`" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "classify known blocker families" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "workflow-conductor" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "resume from partially completed early steps" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "child issue wave already appears to cover the acceptance surface" "${skills_root}/workflow-conductor/SKILL.md"

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
    "pr_state": "open_with_blockers",
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
assert janitor["handoff_state"]["continuation"] == "continue"

required_missing = load("required_subagent_missing.out.json")
assert required_missing["selected_skill"]["skill_name"] == "pr-run"
assert required_missing["workflow_compliance"]["policy_result"] == "FAIL"
assert required_missing["workflow_compliance"]["bypasses"][0]["reason"] == "required_but_not_assigned"
assert required_missing["status"] == "blocked"
assert required_missing["handoff_state"]["next_phase"] == "blocked"
assert required_missing["handoff_state"]["continuation"] == "stop"
assert required_missing["handoff_state"]["escalation_reason"] == "policy_block"
PY

fixture_repo="${tmpdir}/fixture-repo"
mkdir -p "${fixture_repo}/adl/tools" "${fixture_repo}/.adl/v0.88/bodies" "${fixture_repo}/.adl/v0.88/tasks"
git -C "${fixture_repo}" init -q
git -C "${fixture_repo}" config user.email "codex@example.test"
git -C "${fixture_repo}" config user.name "Codex"
git -C "${fixture_repo}" commit --allow-empty -qm "init"

cat >"${fixture_repo}/adl/tools/pr.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
issue="$2"
case "$issue" in
  2001)
    cat <<'JSON'
{"schema":"adl.pr.doctor.v1","issue":2001,"version":"v0.88","slug":"route-run","branch":"codex/2001-route-run","mode":"full","preflight_status":"PASS","open_pr_count":0,"open_prs":[],"lifecycle_state":"pre_run","ready_status":"PASS","worktree":null,"source":".adl/v0.88/bodies/issue-2001-route-run.md","root_stp":".adl/v0.88/tasks/issue-2001__route-run/stp.md","root_input":".adl/v0.88/tasks/issue-2001__route-run/sip.md","root_output":".adl/v0.88/tasks/issue-2001__route-run/sor.md","wt_stp":null,"wt_input":null,"wt_output":null,"doctor_status":"PASS"}
JSON
    ;;
  2004)
    cat <<'JSON'
{"schema":"adl.pr.doctor.v1","issue":2004,"version":"v0.88","slug":"route-finish","branch":"codex/2004-route-finish","mode":"full","preflight_status":"BLOCK","open_pr_count":1,"open_prs":[{"number":9999,"head_ref_name":"codex/other-open-wave","state":"ready","url":"https://example.test/pr/9999"}],"lifecycle_state":"execution_done","ready_status":"PASS","worktree":null,"source":".adl/v0.88/bodies/issue-2004-route-finish.md","root_stp":".adl/v0.88/tasks/issue-2004__route-finish/stp.md","root_input":".adl/v0.88/tasks/issue-2004__route-finish/sip.md","root_output":".adl/v0.88/tasks/issue-2004__route-finish/sor.md","wt_stp":null,"wt_input":null,"wt_output":null,"doctor_status":"BLOCK"}
JSON
    ;;
  2006)
    cat <<'JSON'
{"schema":"adl.pr.doctor.v1","issue":2006,"version":"v0.88","slug":"route-tracker-stop","branch":"codex/2006-route-tracker-stop","mode":"full","preflight_status":"PASS","open_pr_count":0,"open_prs":[],"lifecycle_state":"pre_run","ready_status":"PASS","worktree":null,"source":".adl/v0.88/bodies/issue-2006-route-tracker-stop.md","root_stp":".adl/v0.88/tasks/issue-2006__route-tracker-stop/stp.md","root_input":".adl/v0.88/tasks/issue-2006__route-tracker-stop/sip.md","root_output":".adl/v0.88/tasks/issue-2006__route-tracker-stop/sor.md","wt_stp":null,"wt_input":null,"wt_output":null,"doctor_status":"PASS"}
JSON
    ;;
  *)
    exit 1
    ;;
esac
EOF
chmod +x "${fixture_repo}/adl/tools/pr.sh"

mkdir -p "${fixture_repo}/.adl/v0.88/tasks/issue-2001__route-run"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2001__route-run/stp.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2001__route-run/sip.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2001__route-run/sor.md"
touch "${fixture_repo}/.adl/v0.88/bodies/issue-2001-route-run.md"

mkdir -p "${fixture_repo}/.adl/v0.88/tasks/issue-2003__route-editor"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2003__route-editor/stp.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2003__route-editor/sor.md"
touch "${fixture_repo}/.adl/v0.88/bodies/issue-2003-route-editor.md"

mkdir -p "${fixture_repo}/.adl/v0.88/tasks/issue-2004__route-finish"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2004__route-finish/stp.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2004__route-finish/sip.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2004__route-finish/sor.md"
touch "${fixture_repo}/.adl/v0.88/bodies/issue-2004-route-finish.md"

mkdir -p "${fixture_repo}/.adl/v0.88/tasks/issue-2006__route-tracker-stop"
cat >"${fixture_repo}/.adl/v0.88/tasks/issue-2006__route-tracker-stop/stp.md" <<'EOF'
---
wp: WP-20
title: '[v0.88][WP-20] Route tracker stop'
---
EOF
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2006__route-tracker-stop/sip.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-2006__route-tracker-stop/sor.md"
touch "${fixture_repo}/.adl/v0.88/bodies/issue-2006-route-tracker-stop.md"

mkdir -p "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2005__route-worktree-finish"
touch "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2005__route-worktree-finish/stp.md"
touch "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2005__route-worktree-finish/sip.md"
cat >"${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2005__route-worktree-finish/sor.md" <<'EOF'
Task ID: issue-2005
Status: DONE
EOF
touch "${fixture_repo}/.adl/v0.88/bodies/issue-2005-route-worktree-finish.md"
mkdir -p "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2999__extra-worktree-bundle"
touch "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2999__extra-worktree-bundle/stp.md"
touch "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2999__extra-worktree-bundle/sip.md"
touch "${fixture_repo}/.worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2999__extra-worktree-bundle/sor.md"

cat >"${tmpdir}/route_issue.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_issue",
  "repo_root": "${fixture_repo}",
  "target": {
    "issue_number": 2001
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "required",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": true
  }
}
EOF

cat >"${tmpdir}/route_task_bundle.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_task_bundle",
  "repo_root": "${fixture_repo}",
  "target": {
    "task_bundle_path": ".adl/v0.88/tasks/issue-2003__route-editor"
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true,
    "required_card_skill_by_type": {
      "sip": "sip-editor"
    }
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_finish.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_issue",
  "repo_root": "${fixture_repo}",
  "target": {
    "issue_number": 2004
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_worktree_finish.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_task_bundle",
  "repo_root": "${fixture_repo}",
  "target": {
    "task_bundle_path": ".worktrees/adl-wp-2005/.adl/v0.88/tasks/issue-2005__route-worktree-finish"
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_worktree_disambiguated.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_worktree",
  "repo_root": "${fixture_repo}",
  "target": {
    "issue_number": 2005,
    "worktree_path": "${fixture_repo}/.worktrees/adl-wp-2005"
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_tracker_satisfied.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_issue",
  "repo_root": "${fixture_repo}",
  "target": {
    "issue_number": 2006
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

mock_bin="${tmpdir}/mock-bin"
mkdir -p "${mock_bin}"
cat >"${mock_bin}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == "pr" && "$2" == "view" && "$3" == "3001" ]]; then
  cat <<'JSON'
{"state":"OPEN","isDraft":false,"reviewDecision":"CHANGES_REQUESTED","mergeStateStatus":"BLOCKED","headRefName":"codex/3001-pr-blocked","statusCheckRollup":[{"status":"COMPLETED","conclusion":"FAILURE"}]}
JSON
  exit 0
fi
if [[ "$1" == "issue" && "$2" == "list" ]]; then
  cat <<'JSON'
[
  {"number":2011,"state":"CLOSED","title":"child-a","body":"## Issue-Graph Notes\n- child of #2006"},
  {"number":2012,"state":"CLOSED","title":"child-b","body":"## Issue-Graph Notes\n- child of #2006"}
]
JSON
  exit 0
fi
if [[ "$1" == "pr" && "$2" == "view" && "$3" == "3004" ]]; then
  cat <<'JSON'
{"state":"OPEN","isDraft":false,"reviewDecision":null,"mergeStateStatus":"BLOCKED","headRefName":"codex/3004-pr-linkage-only","statusCheckRollup":[{"status":"COMPLETED","conclusion":"SUCCESS"}]}
JSON
  exit 0
fi
if [[ "$1" == "pr" && "$2" == "view" && "$3" == "3002" ]]; then
  cat <<'JSON'
{"state":"MERGED","isDraft":false,"reviewDecision":null,"mergeStateStatus":"UNKNOWN","headRefName":"codex/3002-pr-merged","statusCheckRollup":[]}
JSON
  exit 0
fi
if [[ "$1" == "pr" && "$2" == "view" && "$3" == "3003" ]]; then
  cat <<'JSON'
{"state":"OPEN","isDraft":false,"reviewDecision":"APPROVED","mergeStateStatus":"CLEAN","headRefName":"codex/3003-pr-clean","statusCheckRollup":[{"status":"COMPLETED","conclusion":"SUCCESS"}]}
JSON
  exit 0
fi
exit 1
EOF
chmod +x "${mock_bin}/gh"

mkdir -p "${fixture_repo}/.adl/v0.88/tasks/issue-3001__pr-blocked" "${fixture_repo}/.adl/v0.88/tasks/issue-3002__pr-merged" "${fixture_repo}/.adl/v0.88/tasks/issue-3003__pr-clean"
mkdir -p "${fixture_repo}/.adl/v0.88/tasks/issue-3004__pr-linkage-only"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-3001__pr-blocked/stp.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3001__pr-blocked/sip.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3001__pr-blocked/sor.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-3002__pr-merged/stp.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3002__pr-merged/sip.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3002__pr-merged/sor.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-3003__pr-clean/stp.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3003__pr-clean/sip.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3003__pr-clean/sor.md"
touch "${fixture_repo}/.adl/v0.88/tasks/issue-3004__pr-linkage-only/stp.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3004__pr-linkage-only/sip.md" "${fixture_repo}/.adl/v0.88/tasks/issue-3004__pr-linkage-only/sor.md"
touch "${fixture_repo}/.adl/v0.88/bodies/issue-3001-pr-blocked.md" "${fixture_repo}/.adl/v0.88/bodies/issue-3002-pr-merged.md" "${fixture_repo}/.adl/v0.88/bodies/issue-3003-pr-clean.md" "${fixture_repo}/.adl/v0.88/bodies/issue-3004-pr-linkage-only.md"

cat >"${tmpdir}/route_pr_blocked.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_pr",
  "repo_root": "${fixture_repo}",
  "target": {
    "pr_number": 3001
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_pr_merged.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_pr",
  "repo_root": "${fixture_repo}",
  "target": {
    "pr_number": 3002
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_pr_clean.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_pr",
  "repo_root": "${fixture_repo}",
  "target": {
    "pr_number": 3003
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

cat >"${tmpdir}/route_pr_linkage_only.json" <<EOF
{
  "skill_input_schema": "workflow_conductor.v1",
  "mode": "route_pr",
  "repo_root": "${fixture_repo}",
  "target": {
    "pr_number": 3004
  },
  "policy": {
    "skills_required": true,
    "card_editor_skills_required": true,
    "subagent_requirement": "optional",
    "bypass_without_explicit_blocker": false,
    "allow_phase_inference": true,
    "stop_after_routing": true
  },
  "observed_state": {
    "subagent_assigned": false
  }
}
EOF

python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_issue.json" --artifact-path ".adl/reviews/route-issue.md" >"${tmpdir}/route_issue.out.json"
python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_task_bundle.json" --artifact-path ".adl/reviews/route-task-bundle.md" >"${tmpdir}/route_task_bundle.out.json"
python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_finish.json" --artifact-path ".adl/reviews/route-finish.md" >"${tmpdir}/route_finish.out.json"
python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_worktree_finish.json" --artifact-path ".adl/reviews/route-worktree-finish.md" >"${tmpdir}/route_worktree_finish.out.json"
python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_worktree_disambiguated.json" --artifact-path ".adl/reviews/route-worktree-disambiguated.md" >"${tmpdir}/route_worktree_disambiguated.out.json"
PATH="${mock_bin}:$PATH" python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_tracker_satisfied.json" --artifact-path ".adl/reviews/route-tracker-satisfied.md" >"${tmpdir}/route_tracker_satisfied.out.json"
PATH="${mock_bin}:$PATH" python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_pr_blocked.json" --artifact-path ".adl/reviews/route-pr-blocked.md" >"${tmpdir}/route_pr_blocked.out.json"
PATH="${mock_bin}:$PATH" python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_pr_merged.json" --artifact-path ".adl/reviews/route-pr-merged.md" >"${tmpdir}/route_pr_merged.out.json"
PATH="${mock_bin}:$PATH" python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_pr_clean.json" --artifact-path ".adl/reviews/route-pr-clean.md" >"${tmpdir}/route_pr_clean.out.json"
PATH="${mock_bin}:$PATH" python3 "${skills_root}/workflow-conductor/scripts/route_workflow.py" --input "${tmpdir}/route_pr_linkage_only.json" --artifact-path ".adl/reviews/route-pr-linkage-only.md" >"${tmpdir}/route_pr_linkage_only.out.json"

python3 - "$tmpdir" "$fixture_repo" <<'PY'
import json
import pathlib
import sys

tmp = pathlib.Path(sys.argv[1])
repo = pathlib.Path(sys.argv[2])

def load(name):
    return json.loads((tmp / name).read_text())

route_issue = load("route_issue.out.json")
assert route_issue["selected_skill"]["skill_name"] == "pr-run"
assert route_issue["artifact"]["path"].endswith(".adl/reviews/route-issue.md")
assert "wrote routing artifact to" in route_issue["actions_taken"][-1]
assert (repo / ".adl/reviews/route-issue.md").exists()

route_editor = load("route_task_bundle.out.json")
assert route_editor["selected_skill"]["skill_name"] == "sip-editor"
assert route_editor["status"] == "done"
assert (repo / ".adl/reviews/route-task-bundle.md").exists()

route_finish = load("route_finish.out.json")
assert route_finish["selected_skill"]["skill_name"] == "pr-finish"
assert route_finish["workflow_state"]["blocker_class"] == "open_pr_wave_only"
assert route_finish["handoff_state"]["continuation"] == "ask_operator"
assert route_finish["handoff_state"]["escalation_reason"] == "operator_override_required"

route_tracker_satisfied = load("route_tracker_satisfied.out.json")
assert route_tracker_satisfied["selected_skill"]["skill_name"] == "none"
assert route_tracker_satisfied["workflow_state"]["blocker_class"] == "satisfied_by_child_issue_wave"
assert route_tracker_satisfied["handoff_state"]["next_phase"] == "human_review"
assert route_tracker_satisfied["handoff_state"]["continuation"] == "ask_operator"
assert route_tracker_satisfied["handoff_state"]["escalation_reason"] == "child_issue_wave_satisfied"

route_worktree_finish = load("route_worktree_finish.out.json")
assert route_worktree_finish["selected_skill"]["skill_name"] == "pr-finish"

route_worktree_disambiguated = load("route_worktree_disambiguated.out.json")
assert route_worktree_disambiguated["selected_skill"]["skill_name"] == "pr-finish"
assert route_worktree_disambiguated["target"]["issue_number"] == 2005

route_pr_blocked = load("route_pr_blocked.out.json")
assert route_pr_blocked["selected_skill"]["skill_name"] == "pr-janitor"
assert route_pr_blocked["workflow_state"]["blocker_class"] == "review_changes_requested"

route_pr_merged = load("route_pr_merged.out.json")
assert route_pr_merged["selected_skill"]["skill_name"] == "pr-closeout"

route_pr_clean = load("route_pr_clean.out.json")
assert route_pr_clean["selected_skill"]["skill_name"] == "none"
assert route_pr_clean["handoff_state"]["next_phase"] == "human_review"
assert route_pr_clean["workflow_state"]["blocker_class"] == "none"
assert route_pr_clean["handoff_state"]["continuation"] == "ask_operator"
assert route_pr_clean["handoff_state"]["escalation_reason"] == "healthy_pr_waiting"

route_pr_linkage_only = load("route_pr_linkage_only.out.json")
assert route_pr_linkage_only["selected_skill"]["skill_name"] == "pr-janitor"
assert route_pr_linkage_only["workflow_state"]["blocker_class"] == "merge_blocked"
PY

echo "PASS test_workflow_conductor_skill_contracts"
