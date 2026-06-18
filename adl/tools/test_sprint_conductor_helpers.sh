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

export PATH="${fakebin}:${PATH}"
export FAKE_GH_LOG="${log_path}"
export FAKE_ISSUE_2827_STATE="${issue_2827_state_file}"
export FAKE_ISSUE_2828_STATE="${issue_2828_state_file}"
export FAKE_PR_4001_STATE="${pr_4001_state_file}"

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
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-05 task.
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
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-06 task.
2. [pending] Inspect provider output such as `downloading... done` without treating prose ellipsis as truncation.
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

## Serial Gates

- `#2827` must close out before `#2828`

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
  --title "[v0.91.1][sprint-1][management] Trial sprint" \
  --goal "Run the narrow sprint-conductor trial" \
  --state "${state_path}" \
  >/dev/null

python3 - "${state_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
assert state["sprint_issue_number"] == 3001
assert state["sprint_issue_created_by_skill"] is True
assert state["current_issue_number"] == 2827
assert len(state["issue_records"]) == 2
assert state["structured_prompt_preflight"]["required_card_types"] == ["stp.md", "sip.md", "sor.md", "spp.md", "srp.md"]
assert state["truth_check"]["status"] == "not_run"
assert state["truth_check"]["gate_passed"] is False
bundle = state["local_bundle"]
assert bundle["bundle_dir"].endswith("issue-3001__sprint-1-management-trial-sprint")
PY

test -f "${fake_repo}/.adl/v0.91.1/bodies/issue-3001-sprint-1-management-trial-sprint.md"
test -f "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"
test -f "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/sip.md"
test -f "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/sor.md"
grep -q "Run the narrow sprint-conductor trial" "${fake_repo}/.adl/v0.91.1/bodies/issue-3001-sprint-1-management-trial-sprint.md"
grep -q "Run the narrow sprint-conductor trial" "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"
grep -q "# Structured Task Prompt" "${fake_repo}/.adl/v0.91.1/tasks/issue-3001__sprint-1-management-trial-sprint/stp.md"
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
assert preflight["required_card_types"] == ["stp.md", "sip.md", "sor.md", "spp.md", "srp.md"]
assert len(preflight["issue_results"]) == 2
assert all(result["status"] == "ready" for result in preflight["issue_results"])
assert all(result["canonical_slug"] for result in preflight["issue_results"])
PY

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_readiness.py" \
  --repo-root "${fake_repo}" \
  --ordered-issues "2827,2828" \
  --execution-mode hybrid \
  --execution-packet-path "${readiness_packet}" \
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
---

# Structured Plan Prompt

## Codex Plan

1. [pending] Execute the bounded WP-06 task.
EOF2

cat >"${fake_repo}/.adl/v0.91.1/tasks/issue-2828__trial-wp06/spp.md" <<'EOF2'
---
issue: 2828
status: approved
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

grep -Fq "issue create" "${log_path}"
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
  "closeout": {}
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
