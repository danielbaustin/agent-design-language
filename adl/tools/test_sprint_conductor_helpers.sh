#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

fakebin="${tmpdir}/bin"
mkdir -p "${fakebin}"
log_path="${tmpdir}/gh.log"
touch "${log_path}"
issue_2827_state_file="${tmpdir}/issue-2827-state"
issue_2828_state_file="${tmpdir}/issue-2828-state"
pr_4001_state_file="${tmpdir}/pr-4001-state"
printf 'OPEN\n' > "${issue_2827_state_file}"
printf 'OPEN\n' > "${issue_2828_state_file}"
printf 'OPEN true\n' > "${pr_4001_state_file}"

cat >"${fakebin}/gh" <<'GH_EOF'
#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' "$*" >> "${FAKE_GH_LOG}"

if [[ "$1" == "issue" && "$2" == "view" ]]; then
  issue_number="$3"
  case "${issue_number}" in
    2827)
      state="$(cat "${FAKE_ISSUE_2827_STATE}")"
      printf '{"number":2827,"title":"[v0.91.1][WP-05][runtime] Citizen standing model","state":"%s","url":"https://github.com/danielbaustin/agent-design-language/issues/2827"}\n' "${state}"
      ;;
    2828)
      state="$(cat "${FAKE_ISSUE_2828_STATE}")"
      printf '{"number":2828,"title":"[v0.91.1][WP-06][runtime] Citizen state substrate","state":"%s","url":"https://github.com/danielbaustin/agent-design-language/issues/2828"}\n' "${state}"
      ;;
    3001)
      echo '{"number":3001,"title":"[v0.91.1][sprint-1][management] Trial sprint","state":"OPEN","url":"https://github.com/danielbaustin/agent-design-language/issues/3001"}'
      ;;
    *)
      echo "unexpected GitHub issue view ${issue_number}" >&2
      exit 1
      ;;
  esac
  exit 0
fi

if [[ "$1" == "issue" && "$2" == "create" ]]; then
  echo "https://github.com/danielbaustin/agent-design-language/issues/3001"
  exit 0
fi

if [[ "$1" == "issue" && "$2" == "close" ]]; then
  exit 0
fi

if [[ "$1" == "pr" && "$2" == "view" ]]; then
  read -r pr_state pr_draft < "${FAKE_PR_4001_STATE}"
  printf '{"state":"%s","isDraft":%s,"url":"https://github.com/danielbaustin/agent-design-language/pull/4001"}\n' "${pr_state}" "${pr_draft}"
  exit 0
fi

echo "unexpected gh invocation: $*" >&2
exit 1
GH_EOF
chmod +x "${fakebin}/gh"

cat >"${fakebin}/adl-issue" <<'ADL_ISSUE_EOF'
#!/usr/bin/env bash
set -euo pipefail

subcommand="$1"
shift

case "${subcommand}" in
  view)
    issue_number="$1"
    if [[ "${issue_number}" == "2827" ]]; then
      state="$(cat "${FAKE_ISSUE_2827_STATE}")"
      printf '{"number":2827,"title":"[v0.91.1][WP-05][runtime] Citizen standing model","state":"%s","url":"https://github.com/danielbaustin/agent-design-language/issues/2827"}\n' "${state}"
      exit 0
    fi
    if [[ "${issue_number}" == "2828" ]]; then
      state="$(cat "${FAKE_ISSUE_2828_STATE}")"
      printf '{"number":2828,"title":"[v0.91.1][WP-06][runtime] Citizen state substrate","state":"%s","url":"https://github.com/danielbaustin/agent-design-language/issues/2828"}\n' "${state}"
      exit 0
    fi
    if [[ "${issue_number}" == "3001" ]]; then
      echo '{"number":3001,"title":"[v0.91.1][sprint-1][management] Trial sprint","state":"OPEN","url":"https://github.com/danielbaustin/agent-design-language/issues/3001"}'
      exit 0
    fi
    echo "unexpected adl-issue view ${issue_number}" >&2
    exit 1
    ;;
  create)
    echo '{"number":3001,"url":"https://github.com/danielbaustin/agent-design-language/issues/3001"}'
    exit 0
    ;;
  *)
    echo "unexpected adl-issue invocation: ${subcommand} $*" >&2
    exit 1
    ;;
esac
ADL_ISSUE_EOF
chmod +x "${fakebin}/adl-issue"

fake_tool_repo="${tmpdir}/fake-tool-repo"
mkdir -p "${fake_tool_repo}/adl/target/debug"
cp "${fakebin}/adl-issue" "${fake_tool_repo}/adl/target/debug/adl-issue"
chmod +x "${fake_tool_repo}/adl/target/debug/adl-issue"

export PATH="${fakebin}:${PATH}"
export FAKE_GH_LOG="${log_path}"
export FAKE_ISSUE_2827_STATE="${issue_2827_state_file}"
export FAKE_ISSUE_2828_STATE="${issue_2828_state_file}"
export FAKE_PR_4001_STATE="${pr_4001_state_file}"
export ADL_SPRINT_ISSUE_VIEW_CMD="${fakebin}/adl-issue view"
export ADL_SPRINT_ISSUE_CREATE_CMD="${fakebin}/adl-issue create"

state_path="${tmpdir}/sprint-state.json"
fake_repo="${tmpdir}/fake-repo"
mkdir -p "${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05"
mkdir -p "${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06"
mkdir -p "${fake_repo}/adl/tools"
mkdir -p "${fake_repo}/docs/templates"
cp -R "${repo_root}/docs/templates/prompts" "${fake_repo}/docs/templates/"

cat >"${fake_repo}/adl/tools/pr.sh" <<'PR_EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ "$1" != "init" ]]; then
  echo "unexpected fake pr.sh invocation: $*" >&2
  exit 1
fi

issue_number="$2"
mkdir -p ".adl/v0.91.1/bodies"
mkdir -p ".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint"
cat >".adl/v0.91.1/bodies/issue-${issue_number}-sprint-1-management-trial-sprint.md" <<'EOF2'
generic pr init source stub
EOF2
cat >".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint/stp.md" <<'EOF2'
generic pr init stp stub
EOF2
cat >".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint/sip.md" <<'EOF2'
sip
EOF2
cat >".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint/sor.md" <<'EOF2'
Status: NOT_STARTED
EOF2
cat >".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint/spp.md" <<'EOF2'
issue: 3001
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
EOF2
cat >".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint/vpp.md" <<'EOF2'
issue: 3001
planned_pvf_lane: "tooling"
planned_validation_seconds: "30"
planned_validation_tokens: "800"
EOF2
cat >".adl/v0.91.1/tasks/issue-${issue_number}__sprint-1-management-trial-sprint/srp.md" <<'EOF2'
issue: 3001
EOF2
PR_EOF
chmod +x "${fake_repo}/adl/tools/pr.sh"

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05/stp.md" <<'EOF2'
## Required Outcome

Complete WP-05 sprint trial work.

## Acceptance Criteria

- focused trial proof recorded
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05/sip.md" <<'EOF2'
Branch: not bound yet

## Goal

Execute WP-05 as the first child in the trial sprint.

## Acceptance Criteria

- issue-local proof remains bounded
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05/sor.md" <<'EOF2'
Status: NOT_STARTED
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05/spp.md" <<'EOF2'
---
issue: 2827
status: approved
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-05 task.
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05/vpp.md" <<'EOF2'
---
artifact_type: "structured_validation_planning_prompt"
issue: 2827
status: approved
card_status: ready
planned_pvf_lane: "tooling"
planned_validation_seconds: "30"
planned_validation_tokens: "800"
---

# Validation Planning Prompt

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml doctor_full_warns_when_only_open_wave_blocks_ready_issue -- --nocapture
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2827__trial-wp05/srp.md" <<'EOF2'
---
artifact_type: "structured_review_prompt"
issue: 2827
---

# Structured Review Prompt

## Review Results

- Not run yet.
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/stp.md" <<'EOF2'
## Required Outcome

Complete WP-06 sprint trial work.

## Acceptance Criteria

- focused trial proof recorded
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/sip.md" <<'EOF2'
Branch: not bound yet

## Goal

Execute WP-06 after WP-05 closeout.

## Acceptance Criteria

- issue-local proof remains bounded
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/sor.md" <<'EOF2'
Status: NOT_STARTED
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/spp.md" <<'EOF2'
---
issue: 2828
status: approved
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-06 task.
2. [pending] Inspect provider output such as `downloading... done` without treating prose ellipsis as truncation.
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/vpp.md" <<'EOF2'
---
artifact_type: "structured_validation_planning_prompt"
issue: 2828
status: approved
card_status: ready
planned_pvf_lane: "tooling"
planned_validation_seconds: "30"
planned_validation_tokens: "800"
---

# Validation Planning Prompt

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml card_lifecycle_accepts_pre_review_srp_prompt_without_final_results -- --nocapture
EOF2
cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/srp.md" <<'EOF2'
---
artifact_type: "structured_review_prompt"
issue: 2828
---

# Structured Review Prompt

## Review Results

- Not run yet.
EOF2

readiness_packet="${tmpdir}/trial-sep.md"
cat >"${readiness_packet}" <<'EOF2'
# Trial Sprint Execution Packet

## Child Issue Wave

- `#2827`
- `#2828`

## Recommended Execution Order

1. `#2827`
2. `#2828`

## Safe Parallel Lanes

- none for this trial

## Candidate Parallel Lanes

- lane `serial-bootstrap` classified as `serial_gate` for `#2827`
- lane `post-closeout` classified as `blocked_until_dependency` for `#2828`

## Serial Gates

- `#2827` must close out before `#2828`

## Parallelism Outcome Plan

- planned summary: `Trial sprint remains serial because `#2827` blocks `#2828`.`
- actual summary placeholder: `Fill this during closeout once actual concurrency is known.`
- prediction-miss capture rule: `Record any lane that turned out not to be safe and why.`

## Watcher Policy

- every wait state has a watcher
EOF2

readiness_review="${tmpdir}/trial-review.md"
readiness_activity="${tmpdir}/trial-activity.md"
touch "${readiness_review}" "${readiness_activity}"
readiness_tracked_skill_dir="${tmpdir}/tracked-readiness-skill"
readiness_installed_skill_dir="${tmpdir}/installed-readiness-skill"
mkdir -p "${readiness_tracked_skill_dir}" "${readiness_installed_skill_dir}"
printf 'alpha\n' > "${readiness_tracked_skill_dir}/SKILL.md"
printf 'alpha\n' > "${readiness_installed_skill_dir}/SKILL.md"

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/create_missing_sprint_issue.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --execution-mode hybrid \
  --title "[v0.91.1][sprint-1][management] Trial sprint" \
  --goal "Run the narrow sprint-conductor trial" \
  --state "${state_path}" \
  >/dev/null

python3 - "${repo_root}/adl/tools/skills/sprint-conductor/scripts/create_missing_sprint_issue.py" "${fake_tool_repo}" <<'PY'
import importlib.util
import os
import sys
from pathlib import Path

script_path = Path(sys.argv[1])
fake_tool_repo = Path(sys.argv[2])
fake_issue_binary = fake_tool_repo / "adl" / "target" / "debug" / "adl-issue"
sys.path.insert(0, str(script_path.parent))
spec = importlib.util.spec_from_file_location("create_missing_sprint_issue", script_path)
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)
module.SCRIPT_REPO_ROOT = fake_tool_repo
os.environ.pop("ADL_SPRINT_ISSUE_VIEW_CMD", None)
os.environ.pop("ADL_SPRINT_ISSUE_CREATE_CMD", None)
assert module.default_issue_command("view") == [str(fake_issue_binary), "view"]
assert module.default_issue_command("create") == [str(fake_issue_binary), "create"]
assert module.issue_view(2827)["title"] == "[v0.91.1][WP-05][runtime] Citizen standing model"
created = module.issue_create("Trial sprint", Path("/tmp/unused.md"))
assert created["number"] == 3001
assert created["url"].endswith("/issues/3001")
PY

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert state["sprint_issue_number"] == 3001
assert state["sprint_issue_created_by_skill"] is True
assert state["current_issue_number"] == 2827
assert state["execution_mode"] == "hybrid"
assert state["execution_packet_path"].endswith("issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md")
assert len(state["issue_records"]) == 2
assert set(state["structured_prompt_preflight"]["required_card_types"]) == {"stp.md", "sip.md", "sor.md", "spp.md", "vpp.md", "srp.md"}
assert any("SPP, VPP, and SRP" in note for note in state["structured_prompt_preflight"]["notes"])
assert state["readiness_sweep"]["execution_packet"]["status"] == "present"
assert state["readiness_sweep"]["review_paths"]["status"] == "declared"
assert state["readiness_sweep"]["activity_log_paths"]["status"] == "declared"
assert state["review"]["status"] == "not_started"
assert state["closeout"]["status"] == "not_started"
assert state["truth_check"]["status"] == "not_run"
assert state["truth_check"]["gate_passed"] is False
bundle = state["local_bundle"]
assert bundle["bundle_dir"].endswith("issue-3001__sprint-1-management-trial-sprint")
PY

test -f "${fake_repo}/.adl/v0.91.1/bodies/issue-3001-sprint-1-management-trial-sprint.md"
test -f "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"
test -f "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/sip.md"
test -f "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/sor.md"
test -f "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
grep -q "Run the narrow sprint-conductor trial" "${fake_repo}/.adl/v0.91.1/bodies/issue-3001-sprint-1-management-trial-sprint.md"
grep -q "Run the narrow sprint-conductor trial" "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"
grep -q "# Structured Task Prompt" "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"
grep -q "## Watcher Policy" "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
grep -q "## Sprint Activity Log" "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
grep -q "## Sprint-Level Review" "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
grep -q 'Execution mode: `hybrid`' "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
grep -q '^  N1\["#2827"\]$' "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
grep -q '^  N1 --> N2$' "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md"
if grep -q "generic pr init" "${fake_repo}/.adl/v0.91.1/bodies/issue-3001-sprint-1-management-trial-sprint.md"; then
  echo "expected preferred-path bootstrap to replace generic local source prompt" >&2
  exit 1
fi
if grep -q "generic pr init" "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"; then
  echo "expected preferred-path bootstrap to replace generic local STP" >&2
  exit 1
fi

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${state_path}" >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
preflight = state["structured_prompt_preflight"]
assert preflight["status"] == "ready"
assert set(preflight["required_card_types"]) == {"stp.md", "sip.md", "sor.md", "spp.md", "vpp.md", "srp.md"}
assert len(preflight["issue_results"]) == 2
assert all(result["status"] == "ready" for result in preflight["issue_results"])
assert all(result["canonical_slug"] for result in preflight["issue_results"])
PY

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --execution-mode hybrid \
  --execution-packet-path "${fake_repo}/.adl/v0.91.1/sprints/issue-3001__sprint-1-management-trial-sprint/SPRINT_EXECUTION_PACKET.md" \
  --review-path "${readiness_review}" \
  --activity-log-path "${readiness_activity}" \
  --tracked-skill-dir "${readiness_tracked_skill_dir}" \
  --installed-skill-dir "${readiness_installed_skill_dir}" \
  --state "${state_path}" >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
readiness = state["readiness_sweep"]
assert readiness["status"] == "ready"
assert readiness["execution_mode"] == "hybrid"
assert readiness["execution_packet"]["status"] == "present"
assert readiness["review_paths"]["status"] == "declared"
assert readiness["activity_log_paths"]["status"] == "declared"
assert readiness["issue_repairs"] == []
assert state["installed_skill_parity"]["status"] == "matched"
assert state["structured_prompt_preflight"]["status"] == "ready"
PY

blocked_readiness_state_path="${tmpdir}/sprint-state-readiness-blocked.json"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --execution-mode hybrid \
  --review-path "${readiness_review}" \
  --activity-log-path "${readiness_activity}" \
  --tracked-skill-dir "${readiness_tracked_skill_dir}" \
  --installed-skill-dir "${readiness_installed_skill_dir}" \
  --state "${blocked_readiness_state_path}" >/dev/null 2>&1; then
  echo "expected check_sprint_readiness.py to fail when a hybrid sprint omits the execution packet path" >&2
  exit 1
fi

python3 - "${blocked_readiness_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
readiness = state["readiness_sweep"]
assert readiness["status"] == "blocked"
assert readiness["execution_packet"]["status"] == "blocked"
PY

broken_packet="${tmpdir}/trial-sep-missing-candidate.md"
cat >"${broken_packet}" <<'EOF2'
# Trial Sprint Execution Packet

## Child Issue Wave

- `#2827`
- `#2828`

## Recommended Execution Order

1. `#2827`
2. `#2828`

## Safe Parallel Lanes

- none for this trial

## Serial Gates

- `#2827` must close out before `#2828`

## Watcher Policy

- every wait state has a watcher
EOF2

missing_candidate_state_path="${tmpdir}/sprint-state-missing-candidate.json"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --execution-mode hybrid \
  --execution-packet-path "${broken_packet}" \
  --review-path "${readiness_review}" \
  --activity-log-path "${readiness_activity}" \
  --tracked-skill-dir "${readiness_tracked_skill_dir}" \
  --installed-skill-dir "${readiness_installed_skill_dir}" \
  --state "${missing_candidate_state_path}" >/dev/null 2>&1; then
  echo "expected check_sprint_readiness.py to fail when a hybrid sprint omits candidate lane or outcome-plan sections" >&2
  exit 1
fi

python3 - "${missing_candidate_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
readiness = state["readiness_sweep"]
assert readiness["status"] == "needs_repair"
assert readiness["execution_packet"]["status"] == "needs_repair"
assert "## Candidate Parallel Lanes" in readiness["execution_packet"]["missing_sections"]
assert "## Parallelism Outcome Plan" in readiness["execution_packet"]["missing_sections"]
PY

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/srp.md" <<'EOF2'
---
issue: 2828
---

# Structured Review Prompt

## Review Results

- Not run yet.
EOF2

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${state_path}" >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
preflight = state["structured_prompt_preflight"]
assert preflight["status"] == "needs_editor_repair"
wp06 = [result for result in preflight["issue_results"] if result["issue_number"] == 2828][0]
assert "srp-editor" in wp06["required_editor_skills"]
assert any("srp.md" in defect for defect in wp06["design_time_defects"])
PY

repair_readiness_state_path="${tmpdir}/sprint-state-readiness-repair.json"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --execution-mode hybrid \
  --execution-packet-path "${readiness_packet}" \
  --review-path "${readiness_review}" \
  --activity-log-path "${readiness_activity}" \
  --tracked-skill-dir "${readiness_tracked_skill_dir}" \
  --installed-skill-dir "${readiness_installed_skill_dir}" \
  --state "${repair_readiness_state_path}" >/dev/null 2>&1; then
  echo "expected check_sprint_readiness.py to fail with needs_repair when child issue cards need editor work" >&2
  exit 1
fi

python3 - "${repair_readiness_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
readiness = state["readiness_sweep"]
assert readiness["status"] == "needs_repair"
repair = next(item for item in readiness["issue_repairs"] if item["issue_number"] == 2828)
assert "srp-editor" in repair["next_skills"]
assert repair["status"] == "needs_editor_repair"
PY

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/srp.md" <<'EOF2'
---
artifact_type: "structured_review_prompt"
issue: 2828
---

# Structured Review Prompt

## Review Results

- Not run yet.
EOF2

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/spp.md" <<'EOF2'
---
issue: 2828
status: draft
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
---

Design-time generated SPP; review before execution.
EOF2

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${state_path}" >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
preflight = state["structured_prompt_preflight"]
assert preflight["status"] == "needs_editor_repair"
wp06 = [result for result in preflight["issue_results"] if result["issue_number"] == 2828][0]
assert "spp-editor" in wp06["required_editor_skills"]
assert any("spp.md" in defect for defect in wp06["design_time_defects"])
PY

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/spp.md" <<'EOF2'
---
issue: 2828
status: approved
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-06 task.
EOF2

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/spp.md" <<'EOF2'
---
issue: 2828
status: approved
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
---

# Structured Plan Prompt

## Plan Summary

Design-time execution plan for [v0.91.1][WP-06][runtime] Citizen state substrate.

## Codex Plan

1. [pending] Use dependency truth from the linked source issue prompt.
2. [pending] Use repo inputs from the linked source issue prompt.
3. [pending] Use deliverables from the linked source issue prompt.
4. [pending] Satisfy the linked source issue prompt acceptance criteria.
EOF2

generic_v1_state_path="${tmpdir}/sprint-state-generic-v1.json"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${generic_v1_state_path}" >/dev/null

python3 - "${generic_v1_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
preflight = state["structured_prompt_preflight"]
assert preflight["status"] == "needs_editor_repair"
wp06 = [result for result in preflight["issue_results"] if result["issue_number"] == 2828][0]
assert "spp-editor" in wp06["required_editor_skills"]
assert any("generic" in defect for defect in wp06["design_time_defects"])
PY

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/spp.md" <<'EOF2'
---
issue: 2828
status: approved
estimate_elapsed_seconds: "120"
estimate_total_tokens: "4000"
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-06 task.
EOF2

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${state_path}" >/dev/null

brand_new_state_path="${tmpdir}/brand-new-sprint-state.json"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${brand_new_state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2827 \
  --mark-status active >/dev/null 2>&1; then
  echo "expected update_sprint_state.py to refuse creating and mutating a new sprint state in one step" >&2
  exit 1
fi

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2827 \
  --mark-status active >/dev/null 2>&1; then
  echo "expected update_sprint_state.py to fail without a truth gate" >&2
  exit 1
fi

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2827 \
  --mark-status waiting_for_review \
  --pr-url "https://github.com/danielbaustin/agent-design-language/pull/4001" >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2827)
assert record["status"] == "waiting_for_review"
assert record["pr_url"] == "https://github.com/danielbaustin/agent-design-language/pull/4001"
assert state["truth_check"]["status"] == "matched"
assert state["truth_check"]["gate_passed"] is False
assert state["current_issue_number"] == 2827
assert state["continuation"] == "waiting_for_review"
PY

printf 'CLOSED\n' > "${issue_2827_state_file}"
printf 'MERGED false\n' > "${pr_4001_state_file}"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null 2>&1; then
  echo "expected check_sprint_truth.py to fail when a child issue is closed before local closeout truth is recorded" >&2
  exit 1
fi

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert state["truth_check"]["status"] == "drift_detected"
assert state["truth_check"]["gate_passed"] is False
assert any("record_child_issue_closeout.py must run before sprint state can advance" in note for note in state["truth_check"]["notes"])
PY

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2828)
record["status"] = "closed_out"
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

printf 'OPEN\n' > "${issue_2828_state_file}"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null 2>&1; then
  echo "expected check_sprint_truth.py to fail when unrelated truth drift is present" >&2
  exit 1
fi

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_child_issue_closeout.py" \
  --state "${state_path}" \
  --issue-number 2827 \
  --issue-closed true \
  --pr-state merged \
  --root-sor-status done \
  --worktree-status pruned \
  --pr-url "https://github.com/danielbaustin/agent-design-language/pull/4001" >/dev/null 2>&1; then
  echo "expected record_child_issue_closeout.py to refuse closeout when unrelated truth drift is present" >&2
  exit 1
fi

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2828)
record["status"] = "pending"
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

printf 'CLOSED\n' > "${issue_2828_state_file}"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null 2>&1; then
  echo "expected check_sprint_truth.py to fail when multiple child issues are drifting" >&2
  exit 1
fi

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_child_issue_closeout.py" \
  --state "${state_path}" \
  --issue-number 2827 \
  --issue-closed true \
  --pr-state merged \
  --root-sor-status done \
  --worktree-status pruned \
  --pr-url "https://github.com/danielbaustin/agent-design-language/pull/4001" >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2827)
assert record["status"] == "closed_out"
assert record["closeout_gate"]["pr_state"] == "merged"
assert state["current_issue_number"] == 2828
assert state["continuation"] == "continue"
assert state["truth_check"]["gate_passed"] is False
PY

printf 'OPEN\n' > "${issue_2828_state_file}"

deferred_state_path="${tmpdir}/sprint-state-deferred-next.json"
cp "${state_path}" "${deferred_state_path}"
python3 - "${deferred_state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
state["completed_issue_numbers"] = []
state["current_issue_number"] = 2827
state["continuation"] = "continue"
state["truth_check"] = {
    "status": "matched",
    "source": "github_live",
    "gate_passed": True,
    "checked_issue_numbers": [2827, 2828],
    "checked_pr_urls": [],
    "notes": [],
}
record_2827 = next(record for record in state["issue_records"] if record["issue_number"] == 2827)
record_2827["status"] = "pending"
record_2828 = next(record for record in state["issue_records"] if record["issue_number"] == 2828)
record_2828["status"] = "deferred"
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_child_issue_closeout.py" \
  --state "${deferred_state_path}" \
  --issue-number 2827 \
  --issue-closed true \
  --pr-state merged \
  --root-sor-status done \
  --worktree-status pruned >/dev/null

python3 - "${deferred_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert state["current_issue_number"] == 2828
assert state["continuation"] == "ask_operator"
PY

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2828 \
  --mark-status blocked >/dev/null

printf 'CLOSED\n' > "${issue_2828_state_file}"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null 2>&1; then
  echo "expected check_sprint_truth.py to fail when a blocked child issue is closed before local closeout truth is recorded" >&2
  exit 1
fi

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_child_issue_closeout.py" \
  --state "${state_path}" \
  --issue-number 2828 \
  --issue-closed true \
  --pr-state not_applicable \
  --root-sor-status done \
  --worktree-status retained_with_reason \
  --worktree-note "Retained for post-sprint audio inspection." >/dev/null 2>&1; then
  echo "expected record_child_issue_closeout.py to refuse deterministic closeout for blocked child state" >&2
  exit 1
fi

printf 'OPEN\n' > "${issue_2828_state_file}"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2828 \
  --mark-status pending >/dev/null

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2827 \
  --artifact-path ".adl/reviews/example.md" >/dev/null 2>&1; then
  echo "expected second update_sprint_state.py call to fail after gate consumption" >&2
  exit 1
fi

reopened_state_path="${tmpdir}/sprint-state-reopened.json"
cp "${state_path}" "${reopened_state_path}"
python3 - "${reopened_state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2827)
record["status"] = "closed_out"
state["completed_issue_numbers"] = [2827]
state["current_issue_number"] = 2828
state["continuation"] = "continue"
state["truth_check"] = {
    "status": "matched",
    "source": "github_live",
    "gate_passed": True,
    "checked_issue_numbers": [2827, 2828],
    "checked_pr_urls": [],
    "notes": [],
}
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${reopened_state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2827 \
  --mark-status pending >/dev/null

python3 - "${reopened_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2827)
assert record["status"] == "pending"
assert 2827 not in state["completed_issue_numbers"]
assert state["current_issue_number"] == 2827
assert state["continuation"] == "continue"
PY

printf 'CLOSED\n' > "${issue_2828_state_file}"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${fake_repo}" \
  --state "${state_path}" \
  --require-match >/dev/null 2>&1; then
  echo "expected check_sprint_truth.py to fail when the final child issue is closed before local closeout truth is recorded" >&2
  exit 1
fi

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_child_issue_closeout.py" \
  --state "${state_path}" \
  --issue-number 2828 \
  --issue-closed true \
  --pr-state not_applicable \
  --root-sor-status done \
  --worktree-status retained_with_reason \
  --worktree-note "Retained for post-sprint audio inspection." \
  --follow-up-issue 5001 \
  --follow-up-summary "Document one post-sprint conductor follow-up." >/dev/null

incomplete_close_state_path="${tmpdir}/sprint-state-incomplete-close.json"
cp "${state_path}" "${incomplete_close_state_path}"
python3 - "${incomplete_close_state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
record = next(record for record in state["issue_records"] if record["issue_number"] == 2828)
record["status"] = "pending"
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

incomplete_closeout_artifact="${tmpdir}/sprint-closeout-incomplete.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/write_sprint_closeout_artifact.py" \
  --state "${incomplete_close_state_path}" \
  --out "${incomplete_closeout_artifact}" >/dev/null

grep -Fq 'closure cleanliness: `residual_debt`' "${incomplete_closeout_artifact}"

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py" \
  --state "${incomplete_close_state_path}" \
  --summary "Should fail because one child is still stale." >/dev/null 2>&1; then
  echo "expected close_sprint_issue.py to refuse sprint close when any child lacks closeout truth" >&2
  exit 1
fi

must_land_state_path="${tmpdir}/sprint-state-must-land.json"
cp "${state_path}" "${must_land_state_path}"
python3 - "${must_land_state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
state["follow_up_issues"] = [
    {
        "issue_number": 6001,
        "disposition": "must_land_before_sprint_close",
        "summary": "Blocking post-sprint repair.",
    }
]
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py" \
  --state "${must_land_state_path}" \
  --summary "Should fail because must-land follow-ups remain." >/dev/null 2>&1; then
  echo "expected close_sprint_issue.py to refuse sprint close when must-land follow-up issues remain" >&2
  exit 1
fi

missing_artifact_state_path="${tmpdir}/sprint-state-missing-artifact.json"
cp "${state_path}" "${missing_artifact_state_path}"
python3 - "${missing_artifact_state_path}" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
state = json.loads(path.read_text())
state.pop("closeout", None)
path.write_text(json.dumps(state, indent=2, sort_keys=True) + "\n")
PY

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py" \
  --state "${missing_artifact_state_path}" \
  --summary "Should fail because no closeout artifact is recorded." >/dev/null 2>&1; then
  echo "expected close_sprint_issue.py to refuse sprint close when no retained closeout artifact is recorded" >&2
  exit 1
fi

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert state["continuation"] == "stop"
assert state["current_issue_number"] is None
assert state["follow_up_issue_policy"] == "post_sprint_follow_on"
assert state["follow_up_issues"][0]["issue_number"] == 5001
assert state["follow_up_issues"][0]["disposition"] == "post_sprint_follow_on"
PY

closeout_artifact="${tmpdir}/sprint-closeout.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/write_sprint_closeout_artifact.py" \
  --state "${state_path}" \
  --out "${closeout_artifact}" >/dev/null

grep -Fq 'closure cleanliness: `clean_with_post_sprint_followups`' "${closeout_artifact}"
grep -Fq '#5001' "${closeout_artifact}"

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/close_sprint_issue.py" \
  --state "${state_path}" \
  --summary "Sprint completed cleanly." >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert state["sprint_issue_closed"] is True
assert state["sprint_issue_close_summary"] == "Sprint completed cleanly."
assert state["closeout"]["closure_cleanliness"] == "clean_with_post_sprint_followups"
PY

if grep -Fq "issue create" "${log_path}"; then
  echo "expected sprint helper bootstrap to avoid raw gh issue create in the default fixture path" >&2
  exit 1
fi
grep -Fq "issue close 3001 --comment Sprint completed cleanly." "${log_path}"
grep -Fq "pr view https://github.com/danielbaustin/agent-design-language/pull/4001 --json state,isDraft,url" "${log_path}"

tracked_skill_dir="${tmpdir}/tracked-skill"
installed_skill_dir="${tmpdir}/installed-skill"
mkdir -p "${tracked_skill_dir}" "${installed_skill_dir}"
printf 'alpha\n' > "${tracked_skill_dir}/SKILL.md"
printf 'alpha\n' > "${installed_skill_dir}/SKILL.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_installed_skill_parity.py" \
  --repo-root "${fake_repo}" \
  --tracked-skill-dir "${tracked_skill_dir}" \
  --installed-skill-dir "${installed_skill_dir}" \
  --state "${state_path}" >/dev/null

printf 'beta\n' > "${installed_skill_dir}/SKILL.md"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_installed_skill_parity.py" \
  --repo-root "${fake_repo}" \
  --tracked-skill-dir "${tracked_skill_dir}" \
  --installed-skill-dir "${installed_skill_dir}" >/dev/null 2>&1; then
  echo "expected installed skill parity drift to fail" >&2
  exit 1
fi

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/sor.md" <<'EOF2'
Status: IN_PROGRESS
No implementation has started yet
EOF2
broken_state_path="${tmpdir}/sprint-state-broken.json"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${broken_state_path}" >/dev/null

python3 - "${broken_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
preflight = state["structured_prompt_preflight"]
assert preflight["status"] == "needs_editor_repair"
problem = next(result for result in preflight["issue_results"] if result["issue_number"] == 2828)
assert problem["status"] == "needs_editor_repair"
assert "sor.md" in problem["contradictory_cards"]
assert "sor-editor" in problem["required_editor_skills"]
PY

rm -f "${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/srp.md"
missing_srp_state_path="${tmpdir}/sprint-state-missing-srp.json"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --state "${missing_srp_state_path}" >/dev/null

python3 - "${missing_srp_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
preflight = state["structured_prompt_preflight"]
assert preflight["status"] == "needs_editor_repair"
problem = next(result for result in preflight["issue_results"] if result["issue_number"] == 2828)
assert "srp.md" in problem["missing_cards"]
assert "srp-editor" in problem["required_editor_skills"]
PY

goal_metrics_state_path="${tmpdir}/goal-metrics-state.json"
cat >"${goal_metrics_state_path}" <<'JSON'
{
  "sprint_issue_number": 7001,
  "ordered_issue_numbers": [7002, 7003],
  "issue_records": [
    {"issue_number": 7002, "status": "closed_out", "pr_url": null, "artifact_paths": []},
    {"issue_number": 7003, "status": "closed_out", "pr_url": null, "artifact_paths": []}
  ],
  "closeout": {}
}
JSON
goal_metrics_log="${tmpdir}/issue-goal-metrics.jsonl"

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py" \
  --state "${goal_metrics_state_path}" \
  --issue-number 7002 \
  --sink "${goal_metrics_log}" \
  --capture-stage merge_closeout \
  --data-source codex_goal_tool \
  --recorded-at "2026-06-20T03:00:00Z" \
  --issue-goal-ref "goal:v0.91.6:sprint:7001:issue:7002" \
  --sprint-goal-ref "goal:v0.91.6:sprint:7001" \
  --goal-metrics-rollup-ref ".adl/v0.91.6/sprints/issue-7001__sample/goal-metrics.jsonl" \
  --goal-id "goal-7002" \
  --started-at "2026-06-20T02:30:00Z" \
  --completed-at "2026-06-20T02:56:02Z" \
  --elapsed-seconds 1562 \
  --active-work-seconds 1220 \
  --validation-seconds 200 \
  --pr-wait-seconds 142 \
  --ci-wait-seconds not_applicable \
  --total-tokens 325020 \
  --metrics-confidence high \
  --completion-state completed \
  --model-ref "gpt-5-codex" \
  --session-ref "codex-session-7002" \
  --thread-id "thread-7002" >/dev/null

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py" \
  --state "${goal_metrics_state_path}" \
  --issue-number 7003 \
  --sink "${goal_metrics_log}" \
  --capture-stage review_handoff \
  --data-source manual_entry \
  --recorded-at "2026-06-20T03:10:00Z" \
  --goal-id-state not_available \
  --elapsed-seconds not_collected \
  --total-tokens unknown \
  --model-ref "gpt-5-codex" >/dev/null

python3 - "${goal_metrics_state_path}" "${goal_metrics_log}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
lines = [json.loads(line) for line in Path(sys.argv[2]).read_text().splitlines() if line.strip()]
assert len(lines) == 2
record_7002 = next(record for record in state["issue_records"] if record["issue_number"] == 7002)
record_7003 = next(record for record in state["issue_records"] if record["issue_number"] == 7003)
assert record_7002["goal_metrics"]["status"] == "recorded"
assert record_7002["goal_metrics"]["issue_goal_ref"] == "goal:v0.91.6:sprint:7001:issue:7002"
assert record_7002["goal_metrics"]["sprint_goal_ref"] == "goal:v0.91.6:sprint:7001"
assert record_7002["goal_metrics"]["goal_metrics_rollup_ref"] == ".adl/v0.91.6/sprints/issue-7001__sample/goal-metrics.jsonl"
assert record_7002["goal_metrics"]["elapsed_seconds"] == 1562
assert record_7002["goal_metrics"]["elapsed_availability"] == "known"
assert record_7002["goal_metrics"]["active_work_seconds"] == 1220
assert record_7002["goal_metrics"]["active_work_availability"] == "known"
assert record_7002["goal_metrics"]["validation_seconds"] == 200
assert record_7002["goal_metrics"]["validation_availability"] == "known"
assert record_7002["goal_metrics"]["pr_wait_seconds"] == 142
assert record_7002["goal_metrics"]["pr_wait_availability"] == "known"
assert record_7002["goal_metrics"]["ci_wait_availability"] == "not_applicable"
assert record_7002["goal_metrics"]["completion_state"] == "completed"
assert record_7002["goal_metrics"]["metrics_confidence"] == "high"
assert record_7002["goal_metrics"]["token_usage"]["total_tokens"] == 325020
assert record_7002["goal_metrics"]["token_usage"]["total_availability"] == "known"
assert record_7003["goal_metrics"]["goal_id_availability"] == "not_available"
assert record_7003["goal_metrics"]["elapsed_availability"] == "not_collected"
assert record_7003["goal_metrics"]["token_usage"]["total_availability"] == "unknown"
rollup = state["closeout"]["goal_metrics_rollup"]
assert rollup["issue_count"] == 2
assert rollup["issues_with_recorded_metrics"] == 2
assert rollup["issues_with_known_elapsed"] == 1
assert rollup["issues_with_unknown_elapsed"] == 0
assert rollup["issues_with_known_active_work"] == 1
assert rollup["issues_with_unknown_active_work"] == 1
assert rollup["issues_with_known_validation_seconds"] == 1
assert rollup["issues_with_unknown_validation_seconds"] == 1
assert rollup["issues_with_known_pr_wait"] == 1
assert rollup["issues_with_unknown_pr_wait"] == 1
assert rollup["issues_with_known_ci_wait"] == 0
assert rollup["issues_with_unknown_ci_wait"] == 1
assert rollup["issues_with_known_total_tokens"] == 1
assert rollup["issues_with_unknown_total_tokens"] == 1
assert rollup["total_elapsed_seconds_known_sum"] == 1562
assert rollup["total_active_work_seconds_known_sum"] == 1220
assert rollup["total_validation_seconds_known_sum"] == 200
assert rollup["total_pr_wait_seconds_known_sum"] == 142
assert rollup["total_ci_wait_seconds_known_sum"] == 0
assert rollup["total_tokens_known_sum"] == 325020
assert rollup["data_source_counts"]["codex_goal_tool"] == 1
assert rollup["data_source_counts"]["manual_entry"] == 1
assert rollup["elapsed_availability_counts"]["not_collected"] == 1
assert rollup["ci_wait_availability_counts"]["not_applicable"] == 1
assert rollup["completion_state_counts"]["completed"] == 1
assert rollup["completion_state_counts"]["unknown"] == 1
PY

goal_metrics_default_state_path="${tmpdir}/goal-metrics-default-state.json"
cat >"${goal_metrics_default_state_path}" <<'JSON'
{
  "sprint_issue_number": 7001,
  "ordered_issue_numbers": [7002],
  "issue_records": [
    {
      "issue_number": 7002,
      "status": "pending",
      "pr_url": null,
      "artifact_paths": [],
      "goal_metrics": {
        "status": "not_recorded",
        "raw_log_path": null,
        "record_count": 0,
        "phases_recorded": [],
        "selected_stage": null,
        "recorded_at": null,
        "data_source": "unknown",
        "goal_id": null,
        "goal_id_availability": "unknown",
        "started_at": null,
        "completed_at": null,
        "elapsed_seconds": null,
        "elapsed_availability": "unknown",
        "token_usage": {
          "total_tokens": null,
          "prompt_tokens": null,
          "completion_tokens": null,
          "availability": "unknown",
          "total_availability": "unknown",
          "prompt_availability": "unknown",
          "completion_availability": "unknown"
        },
        "model_ref": null,
        "session_ref": null,
        "thread_id": null
      }
    }
  ],
  "closeout": {}
}
JSON

python3 - "${goal_metrics_default_state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
token_usage = state["issue_records"][0]["goal_metrics"]["token_usage"]
assert token_usage["availability"] == "unknown"
assert token_usage["total_availability"] == "unknown"
assert token_usage["prompt_availability"] == "unknown"
assert token_usage["completion_availability"] == "unknown"
PY

goal_metrics_invalid_log="${tmpdir}/issue-goal-metrics-invalid.jsonl"
if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py" \
  --state "${goal_metrics_state_path}" \
  --issue-number 7999 \
  --sink "${goal_metrics_invalid_log}" \
  --capture-stage merge_closeout \
  --data-source codex_goal_tool >/dev/null 2>"${tmpdir}/goal-metrics-invalid.stderr"; then
  echo "expected non-member issue metrics recording to fail" >&2
  exit 1
fi
grep -Fq 'issue #7999 is not present in ordered_issue_numbers' "${tmpdir}/goal-metrics-invalid.stderr"
python3 - "${goal_metrics_state_path}" "${goal_metrics_invalid_log}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert [record["issue_number"] for record in state["issue_records"]] == [7002, 7003]
assert not Path(sys.argv[2]).exists()
PY

goal_metrics_artifact="${tmpdir}/goal-metrics-closeout.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/write_sprint_closeout_artifact.py" \
  --state "${goal_metrics_state_path}" \
  --out "${goal_metrics_artifact}" >/dev/null

grep -Fq '## Goal Metrics Rollup' "${goal_metrics_artifact}"
grep -Fq 'issues with recorded metrics: `2/2`' "${goal_metrics_artifact}"
grep -Fq "goal refs: \`issue=goal:v0.91.6:sprint:7001:issue:7002, sprint=goal:v0.91.6:sprint:7001, rollup=.adl/v0.91.6/sprints/issue-7001__sample/goal-metrics.jsonl\`" "${goal_metrics_artifact}"
grep -Fq 'goal timing buckets: `active_work=1220, validation=200, pr_wait=142, ci_wait=not_applicable`' "${goal_metrics_artifact}"
grep -Fq "data sources: \`{'codex_goal_tool': 1, 'derived_sprint_state': 0, 'manual_entry': 1, 'unknown': 0}\`" "${goal_metrics_artifact}"
grep -Fq "goal-id availability: \`{'known': 1, 'not_applicable': 0, 'not_available': 1, 'not_collected': 0, 'unknown': 0}\`" "${goal_metrics_artifact}"
grep -Fq "completion states: \`{'blocked': 0, 'cancelled': 0, 'completed': 1, 'completed_with_follow_on': 0, 'deferred': 0, 'failed': 0, 'unknown': 1}\`" "${goal_metrics_artifact}"
grep -Fq "elapsed seconds: \`known_sum=1562, known_issue_count=1, unknown_issue_count=0, availability_counts={'known': 1, 'not_applicable': 0, 'not_available': 0, 'not_collected': 1, 'unknown': 0}\`" "${goal_metrics_artifact}"
grep -Fq "active work seconds: \`known_sum=1220, known_issue_count=1, unknown_issue_count=1, availability_counts={'known': 1, 'not_applicable': 0, 'not_available': 0, 'not_collected': 0, 'unknown': 1}\`" "${goal_metrics_artifact}"
grep -Fq "validation seconds: \`known_sum=200, known_issue_count=1, unknown_issue_count=1, availability_counts={'known': 1, 'not_applicable': 0, 'not_available': 0, 'not_collected': 0, 'unknown': 1}\`" "${goal_metrics_artifact}"
grep -Fq "pr wait seconds: \`known_sum=142, known_issue_count=1, unknown_issue_count=1, availability_counts={'known': 1, 'not_applicable': 0, 'not_available': 0, 'not_collected': 0, 'unknown': 1}\`" "${goal_metrics_artifact}"
grep -Fq "ci wait seconds: \`known_sum=0, known_issue_count=0, unknown_issue_count=1, availability_counts={'known': 0, 'not_applicable': 1, 'not_available': 0, 'not_collected': 0, 'unknown': 1}\`" "${goal_metrics_artifact}"
grep -Fq "total tokens: \`known_sum=325020, known_issue_count=1, unknown_issue_count=1, availability_counts={'known': 1, 'not_applicable': 0, 'not_available': 0, 'not_collected': 0, 'unknown': 1}\`" "${goal_metrics_artifact}"

echo "PASS test_sprint_conductor_helpers"

closeout_readiness_blocked_state="${tmpdir}/closeout-readiness-blocked-state.json"
cat >"${closeout_readiness_blocked_state}" <<'JSON'
{
  "sprint_issue_number": 5001,
  "ordered_issue_numbers": [5002, 5003],
  "issue_records": [
    {"issue_number": 5002, "status": "closed_out", "pr_url": "https://example.test/pr/5002", "artifact_paths": ["docs/review-5002.md"], "closeout_gate": {"issue_closed": true, "pr_state": "merged", "root_sor_status": "done", "worktree_status": "pruned", "worktree_note": null}},
    {"issue_number": 5003, "status": "waiting_for_review", "pr_url": "https://example.test/pr/5003", "artifact_paths": []}
  ],
  "blocked_issue_number": null,
  "deferred_issue_numbers": [],
  "follow_up_issues": [],
  "review": {"status": "done", "packet_path": "docs/review/packet.md", "code_review_path": "docs/review/code.md", "test_review_path": "docs/review/tests.md", "synthesis_path": "docs/review/synthesis.md"},
  "validation": {"status": "PASS"},
  "coverage": {"source": "existing_quality_gate", "summary": "Existing quality gate reused for sprint closeout."},
  "rust_tracker": {"source": "existing_quality_gate", "watch_count": 3, "review_count": 2, "rationale_count": 1},
  "closeout": {}
}
JSON
blocked_artifact="${tmpdir}/closeout-readiness-blocked.md"
blocked_summary="${tmpdir}/closeout-readiness-blocked-summary.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_closeout_readiness.py" \
  --state "${closeout_readiness_blocked_state}" \
  --out "${blocked_artifact}" \
  --summary-out "${blocked_summary}" \
  --print-json > "${tmpdir}/closeout-readiness-blocked.json"
python3 - "${tmpdir}/closeout-readiness-blocked.json" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
assert payload["classification"] == "blocked"
assert payload["closeout_status"] == "blocked"
assert any("child closeout truth is incomplete" in item for item in payload["blockers"])
PY
grep -Fq 'ready_to_close' "${blocked_summary}" && {
  echo "blocked closeout summary should not claim ready_to_close" >&2
  exit 1
} || true
grep -Fq 'Blocking conditions:' "${blocked_summary}"

actionable_closeout_state="${tmpdir}/closeout-readiness-remediation-state.json"
cat >"${actionable_closeout_state}" <<'JSON'
{
  "sprint_issue_number": 5001,
  "ordered_issue_numbers": [5002, 5003],
  "issue_records": [
    {"issue_number": 5002, "status": "closed_out", "pr_url": "https://example.test/pr/5002", "artifact_paths": ["docs/review-5002.md"], "closeout_gate": {"issue_closed": true, "pr_state": "merged", "root_sor_status": "done", "worktree_status": "pruned", "worktree_note": null}},
    {"issue_number": 5003, "status": "closed_out", "pr_url": "https://example.test/pr/5003", "artifact_paths": ["docs/review-5003.md"], "closeout_gate": {"issue_closed": true, "pr_state": "merged", "root_sor_status": "done", "worktree_status": "pruned", "worktree_note": null}}
  ],
  "blocked_issue_number": null,
  "deferred_issue_numbers": [],
  "follow_up_issues": [],
  "review": {"status": "not_started"},
  "closeout": {}
}
JSON
remediation_artifact="${tmpdir}/closeout-readiness-remediation.md"
remediation_summary="${tmpdir}/closeout-readiness-remediation-summary.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_closeout_readiness.py" \
  --state "${actionable_closeout_state}" \
  --out "${remediation_artifact}" \
  --summary-out "${remediation_summary}" \
  --print-json > "${tmpdir}/closeout-readiness-remediation.json"
python3 - "${tmpdir}/closeout-readiness-remediation.json" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
assert payload["classification"] == "needs_remediation"
assert payload["closeout_status"] == "in_progress"
assert any("Sprint review status is not done." == item for item in payload["remediation"])
assert payload["closeout_artifact_path"]
PY
grep -Fq 'Remediation required before sprint close:' "${remediation_summary}"

actionable_ready_state="${tmpdir}/closeout-readiness-ready-state.json"
cat >"${actionable_ready_state}" <<'JSON'
{
  "sprint_issue_number": 5001,
  "ordered_issue_numbers": [5002, 5003],
  "issue_records": [
    {"issue_number": 5002, "status": "closed_out", "pr_url": "https://example.test/pr/5002", "artifact_paths": ["docs/review-5002.md"], "closeout_gate": {"issue_closed": true, "pr_state": "merged", "root_sor_status": "done", "worktree_status": "pruned", "worktree_note": null}},
    {"issue_number": 5003, "status": "closed_out", "pr_url": "https://example.test/pr/5003", "artifact_paths": ["docs/review-5003.md"], "closeout_gate": {"issue_closed": true, "pr_state": "merged", "root_sor_status": "done", "worktree_status": "pruned", "worktree_note": null}}
  ],
  "blocked_issue_number": null,
  "deferred_issue_numbers": [],
  "follow_up_issues": [
    {"issue_number": 6001, "disposition": "post_sprint_follow_on", "summary": "Document residual SEP learnings."}
  ],
  "review": {"status": "done", "packet_path": "docs/review/packet.md", "code_review_path": "docs/review/code.md", "test_review_path": "docs/review/tests.md", "synthesis_path": "docs/review/synthesis.md"},
  "validation": {"status": "PASS"},
  "coverage": {"source": "existing_quality_gate", "summary": "Existing quality gate reused for sprint closeout."},
  "rust_tracker": {"source": "existing_quality_gate", "watch_count": 3, "review_count": 2, "rationale_count": 1},
  "closeout": {
    "planned_vs_actual_parallelism": {
      "planned_summary": "Expected one safe docs lane plus one serial gate.",
      "actual_summary": "The docs lane stayed serial because both issues touched the same tracker file.",
      "prediction_misses": [
        {
          "lane_id": "docs-lane",
          "issue_numbers": [5002, 5003],
          "why_wrong": "Both child issues mutated the same tracker path during review preparation.",
          "corrective_action": "Keep the lane serial until tracker writes are split."
        }
      ]
    }
  }
}
JSON
ready_artifact="${tmpdir}/closeout-readiness-ready.md"
ready_summary="${tmpdir}/closeout-readiness-ready-summary.md"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_closeout_readiness.py" \
  --state "${actionable_ready_state}" \
  --out "${ready_artifact}" \
  --summary-out "${ready_summary}" \
  --print-json > "${tmpdir}/closeout-readiness-ready.json"
python3 - "${tmpdir}/closeout-readiness-ready.json" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text())
assert payload["classification"] == "ready_to_close"
assert payload["closeout_status"] == "done"
assert payload["closure_cleanliness"] == "clean_with_post_sprint_followups"
assert payload["closeout_artifact_path"]
assert "#6001" in payload["summary"]
state = json.loads(Path(payload["state_path"]).read_text())
assert state["closeout"]["sprint_issue_close_summary"] == payload["summary"]
PY
grep -Fq 'Follow-up routing:' "${ready_summary}"
grep -Fq '#6001' "${ready_summary}"
grep -Fq '## Planned Vs Actual Parallelism' "${ready_artifact}"
grep -Fq 'planned summary: `Expected one safe docs lane plus one serial gate.`' "${ready_artifact}"
grep -Fq 'actual summary: `The docs lane stayed serial because both issues touched the same tracker file.`' "${ready_artifact}"
grep -Fq 'lane=`docs-lane` issues=`5002, 5003` why_wrong=Both child issues mutated the same tracker path during review preparation. corrective_action=Keep the lane serial until tracker writes are split.' "${ready_artifact}"

codex_goal_snapshot="${tmpdir}/codex-goal-state.json"
cat >"${codex_goal_snapshot}" <<'JSON'
{
  "goal": {
    "threadId": "thread-4431",
    "objective": "Issue #4431: restore authoritative workflow time and token accounting",
    "status": "active",
    "tokensUsed": 39238,
    "timeUsedSeconds": 55,
    "createdAt": 1782153534,
    "updatedAt": 1782153590
  },
  "remainingTokens": null,
  "completionBudgetReport": null
}
JSON
codex_goal_sink="${tmpdir}/issue-4431-goal-metrics.jsonl"
codex_goal_summary="${tmpdir}/issue-4431-goal-metrics-summary.json"
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_codex_goal_tool_snapshot.py" \
  --goal-state "${codex_goal_snapshot}" \
  --issue-number 4431 \
  --sink "${codex_goal_sink}" \
  --summary-out "${codex_goal_summary}" \
  --capture-stage issue_start \
  --issue-goal-ref "goal:v0.91.6:issue:4431" \
  --metrics-confidence high \
  --model-ref "gpt-5-codex" >/dev/null
python3 - "${codex_goal_sink}" "${codex_goal_summary}" <<'PY'
import json
import sys
from pathlib import Path

rows = [json.loads(line) for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
assert len(rows) == 1
record = rows[0]
assert record["data_source"] == "codex_goal_tool"
assert record["issue_goal_ref"] == "goal:v0.91.6:issue:4431"
assert record["thread_id"] == "thread-4431"
assert record["active_work_seconds"] == 55
assert record["active_work_availability"] == "known"
assert record["elapsed_seconds"] == 56
assert record["elapsed_availability"] == "known"
assert record["token_usage"]["total_tokens"] == 39238
assert record["token_usage"]["total_availability"] == "known"
assert record["goal_id_availability"] == "not_available"
summary = json.loads(Path(sys.argv[2]).read_text())
assert summary["status"] == "recorded"
assert summary["selected_stage"] == "issue_start"
assert summary["thread_id"] == "thread-4431"
assert summary["active_work_seconds"] == 55
assert summary["token_usage"]["total_tokens"] == 39238
assert summary["elapsed_seconds"] == 56
assert summary["elapsed_availability"] == "known"
assert summary["metrics_confidence"] == "high"
PY

codex_budgetlimited_snapshot="${tmpdir}/codex-goal-budgetlimited-state.json"
cat >"${codex_budgetlimited_snapshot}" <<'JSON'
{
  "goal": {
    "threadId": "thread-4431-budgetlimited",
    "objective": "Issue #4431: budget limited session",
    "status": "budgetLimited",
    "tokensUsed": 41000,
    "timeUsedSeconds": 91,
    "createdAt": 1782153534,
    "updatedAt": 1782153625
  }
}
JSON
python3 - "${repo_root}" "${codex_budgetlimited_snapshot}" <<'PY'
import sys

sys.path.insert(0, sys.argv[1] + "/adl/tools/skills/sprint-conductor/scripts")
from issue_goal_metrics import parse_codex_goal_tool_snapshot

snapshot = parse_codex_goal_tool_snapshot(sys.argv[2])
assert snapshot["status"] == "budgetlimited"
assert snapshot["completion_state"] == "deferred"
assert snapshot["completed_at"] == "2026-06-22T18:40:25Z"
assert snapshot["elapsed_seconds_raw"] == "91"
PY

goal_stage_artifacts_dir="${tmpdir}/issue-4431-artifacts/goal_metrics"
goal_stage_issue_start_a="${tmpdir}/issue-4431-stage-issue-start-a.json"
cat >"${goal_stage_issue_start_a}" <<'JSON'
{
  "goal": {
    "threadId": "thread-4431-stage",
    "objective": "Issue #4431 stage helper",
    "status": "active",
    "tokensUsed": 1000,
    "timeUsedSeconds": 10,
    "createdAt": 1782153534,
    "updatedAt": 1782153544
  }
}
JSON
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_artifacts.py" \
  --goal-state "${goal_stage_issue_start_a}" \
  --issue-number 4431 \
  --artifacts-dir "${goal_stage_artifacts_dir}" \
  --capture-stage issue_start \
  --issue-goal-ref "goal:v0.91.6:issue:4431" \
  --metrics-confidence high \
  --model-ref "gpt-5-codex" >/dev/null

goal_stage_issue_start_b="${tmpdir}/issue-4431-stage-issue-start-b.json"
cat >"${goal_stage_issue_start_b}" <<'JSON'
{
  "goal": {
    "threadId": "thread-4431-stage",
    "objective": "Issue #4431 stage helper",
    "status": "active",
    "tokensUsed": 2000,
    "timeUsedSeconds": 20,
    "createdAt": 1782153534,
    "updatedAt": 1782153554
  }
}
JSON
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_artifacts.py" \
  --goal-state "${goal_stage_issue_start_b}" \
  --issue-number 4431 \
  --artifacts-dir "${goal_stage_artifacts_dir}" \
  --capture-stage issue_start \
  --issue-goal-ref "goal:v0.91.6:issue:4431" \
  --metrics-confidence high \
  --model-ref "gpt-5-codex" >/dev/null

goal_stage_pr_publication="${tmpdir}/issue-4431-stage-pr-publication.json"
cat >"${goal_stage_pr_publication}" <<'JSON'
{
  "goal": {
    "threadId": "thread-4431-stage",
    "objective": "Issue #4431 stage helper",
    "status": "complete",
    "tokensUsed": 3000,
    "timeUsedSeconds": 30,
    "createdAt": 1782153534,
    "updatedAt": 1782153564
  }
}
JSON
python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_artifacts.py" \
  --goal-state "${goal_stage_pr_publication}" \
  --issue-number 4431 \
  --artifacts-dir "${goal_stage_artifacts_dir}" \
  --capture-stage pr_publication \
  --issue-goal-ref "goal:v0.91.6:issue:4431" \
  --metrics-confidence high \
  --model-ref "gpt-5-codex" >/dev/null

python3 - "${goal_stage_artifacts_dir}" <<'PY'
import json
import sys
from pathlib import Path

artifacts_dir = Path(sys.argv[1])
issue_start_snapshot = artifacts_dir / "issue-4431-goal-state.json"
pr_publication_snapshot = artifacts_dir / "issue-4431-goal-state-pr-publication.json"
assert issue_start_snapshot.exists()
assert pr_publication_snapshot.exists()

rows = [
    json.loads(line)
    for line in (artifacts_dir / "issue-4431-goal-metrics.jsonl").read_text().splitlines()
    if line.strip()
]
assert len(rows) == 2
issue_start_rows = [row for row in rows if row["capture_stage"] == "issue_start"]
assert len(issue_start_rows) == 1
assert issue_start_rows[0]["token_usage"]["total_tokens"] == 2000
assert issue_start_rows[0]["elapsed_seconds"] == 20
pr_rows = [row for row in rows if row["capture_stage"] == "pr_publication"]
assert len(pr_rows) == 1
assert pr_rows[0]["token_usage"]["total_tokens"] == 3000
summary = json.loads((artifacts_dir / "issue-4431-goal-metrics-summary.json").read_text())
assert summary["record_count"] == 2
assert summary["phases_recorded"] == ["issue_start", "pr_publication"]
assert summary["selected_stage"] == "pr_publication"
assert summary["token_usage"]["total_tokens"] == 3000
assert summary["elapsed_seconds"] == 30
assert summary["completion_state"] == "completed"
PY
