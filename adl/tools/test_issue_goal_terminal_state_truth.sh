#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

check_script="${repo_root}/adl/tools/check_issue_goal_terminal_state.py"
record_script="${repo_root}/adl/tools/skills/sprint-conductor/scripts/record_issue_goal_metrics.py"
state_path="${tmpdir}/state.json"
sink_path="${tmpdir}/goal-metrics.jsonl"

cat >"${state_path}" <<'JSON'
{
  "sprint_issue_number": 9000,
  "ordered_issue_numbers": [4470],
  "issue_records": [
    {
      "issue_number": 4470,
      "status": "in_progress",
      "artifact_paths": []
    }
  ]
}
JSON

python3 - "${check_script}" <<'PY'
import json
import subprocess
import sys

check_script = sys.argv[1]

def run(*args, expect=0):
    proc = subprocess.run(
        ["python3", check_script, *args],
        text=True,
        capture_output=True,
    )
    if proc.returncode != expect:
        raise SystemExit(
            f"unexpected exit {proc.returncode} for {args}\nstdout={proc.stdout}\nstderr={proc.stderr}"
        )
    return json.loads(proc.stdout)

pending = run(
    "--goal-kind", "implementation",
    "--pr-state", "open",
    "--checks-state", "pending",
    "--review-truth", "current",
    expect=1,
)
assert pending["truth_status"] == "not_satisfied"
assert "pending" in pending["reason"]

red = run(
    "--goal-kind", "implementation",
    "--pr-state", "open",
    "--checks-state", "red",
    "--review-truth", "current",
    expect=1,
)
assert "failing" in red["reason"]

draft = run(
    "--goal-kind", "implementation",
    "--pr-state", "draft",
    "--checks-state", "green",
    "--review-truth", "current",
    expect=1,
)
assert "draft" in draft["reason"]

missing_checks = run(
    "--goal-kind", "implementation",
    "--pr-state", "open",
    "--checks-state", "missing",
    "--review-truth", "current",
    expect=1,
)
assert "missing" in missing_checks["reason"]

conflicted = run(
    "--goal-kind", "implementation",
    "--pr-state", "open",
    "--checks-state", "green",
    "--review-truth", "current",
    "--merge-conflicts",
    expect=1,
)
assert "conflicts" in conflicted["reason"]

stale_review = run(
    "--goal-kind", "implementation",
    "--pr-state", "open",
    "--checks-state", "green",
    "--review-truth", "stale",
    expect=1,
)
assert "current" in stale_review["reason"]

green = run(
    "--goal-kind", "implementation",
    "--pr-state", "open",
    "--checks-state", "green",
    "--review-truth", "current",
    expect=0,
)
assert green["completion_allowed"] is True

setup = run(
    "--goal-kind", "setup_only",
    expect=0,
)
assert setup["declared_boundary"] == "handoff_only"
assert setup["completion_allowed"] is True
PY

if python3 "${record_script}" \
  --state "${state_path}" \
  --issue-number 4470 \
  --sink "${sink_path}" \
  --capture-stage review_handoff \
  --data-source manual_entry \
  --completion-state completed \
  --goal-kind implementation \
  --pr-state open \
  --checks-state pending \
  --review-truth current >/dev/null 2>&1; then
  echo "expected completed goal metrics record to fail when terminal state is pending" >&2
  exit 1
fi

python3 "${record_script}" \
  --state "${state_path}" \
  --issue-number 4470 \
  --sink "${sink_path}" \
  --capture-stage review_handoff \
  --data-source manual_entry \
  --completion-state completed \
  --goal-kind implementation \
  --pr-state open \
  --checks-state green \
  --review-truth current >/dev/null

python3 - "${state_path}" "${sink_path}" <<'PY'
import json
import sys
from pathlib import Path

state = json.loads(Path(sys.argv[1]).read_text())
rows = [json.loads(line) for line in Path(sys.argv[2]).read_text().splitlines() if line.strip()]
assert len(rows) == 1
record = rows[0]
assert record["goal_terminal_state"]["completion_allowed"] is True
assert record["goal_terminal_state"]["declared_boundary"] == "pr_green"
summary = state["issue_records"][0]["goal_metrics"]
assert summary["goal_terminal_state"]["truth_status"] == "satisfied"
assert state["closeout"]["goal_metrics_rollup"]["terminal_truth_status_counts"]["satisfied"] == 1
PY

echo "PASS test_issue_goal_terminal_state_truth"
