#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

fakebin="${tmpdir}/bin"
mkdir -p "${fakebin}"
log_path="${tmpdir}/gh.log"
touch "${log_path}"

cat >"${fakebin}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' "$*" >> "${FAKE_GH_LOG}"

if [[ "$1" == "issue" && "$2" == "view" ]]; then
  issue_number="$3"
  case "${issue_number}" in
    2827)
      echo '{"number":2827,"title":"[v0.91.1][WP-05][runtime] Citizen standing model","state":"OPEN","url":"https://github.com/danielbaustin/agent-design-language/issues/2827"}'
      ;;
    2828)
      echo '{"number":2828,"title":"[v0.91.1][WP-06][runtime] Citizen state substrate","state":"OPEN","url":"https://github.com/danielbaustin/agent-design-language/issues/2828"}'
      ;;
    3001)
      echo '{"number":3001,"title":"[v0.91.1][sprint-1][management] Trial sprint","state":"OPEN","url":"https://github.com/danielbaustin/agent-design-language/issues/3001"}'
      ;;
    *)
      echo "unexpected gh issue view ${issue_number}" >&2
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
  echo '{"state":"OPEN","isDraft":true,"url":"https://github.com/danielbaustin/agent-design-language/pull/4001"}'
  exit 0
fi

echo "unexpected gh invocation: $*" >&2
exit 1
EOF
chmod +x "${fakebin}/gh"

export PATH="${fakebin}:${PATH}"
export FAKE_GH_LOG="${log_path}"

state_path="${tmpdir}/sprint-state.json"

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/create_missing_sprint_issue.py" \
  --repo-root "${repo_root}" \
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
assert state["truth_check"]["status"] == "not_run"
assert state["truth_check"]["gate_passed"] is False
PY

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
  --repo-root "${repo_root}" \
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
assert state["continuation"] == "waiting_for_review"
PY

if python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/update_sprint_state.py" \
  --state "${state_path}" \
  --sprint-issue 3001 \
  --ordered-issues "2827,2828" \
  --current-issue 2827 \
  --artifact-path ".adl/reviews/example.md" >/dev/null 2>&1; then
  echo "expected second update_sprint_state.py call to fail after gate consumption" >&2
  exit 1
fi

python3 "${repo_root}/adl/tools/skills/sprint-conductor/scripts/check_sprint_truth.py" \
  --repo-root "${repo_root}" \
  --state "${state_path}" \
  --require-match >/dev/null

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
PY

grep -Fq "issue create" "${log_path}"
grep -Fq "issue close 3001 --comment Sprint completed cleanly." "${log_path}"
grep -Fq "pr view https://github.com/danielbaustin/agent-design-language/pull/4001 --json state,isDraft,url" "${log_path}"

echo "PASS test_sprint_conductor_helpers"
