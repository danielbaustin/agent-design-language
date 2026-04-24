#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

skill_root="${skills_root}/issue-splitter"
python_script="${skill_root}/scripts/plan_issue_split.py"

[[ -f "${skill_root}/SKILL.md" ]]
[[ -f "${skill_root}/adl-skill.yaml" ]]
[[ -f "${skill_root}/agents/openai.yaml" ]]
[[ -f "${skill_root}/references/issue-splitter-playbook.md" ]]
[[ -f "${skill_root}/references/output-contract.md" ]]
[[ -x "${python_script}" ]]
[[ -f "${skills_root}/docs/ISSUE_SPLITTER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "issue-splitter"' "${skill_root}/adl-skill.yaml"
grep -Fq 'id: "issue_splitter.v1"' "${skill_root}/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ISSUE_SPLITTER_SKILL_INPUT_SCHEMA.md"' "${skill_root}/adl-skill.yaml"
grep -Fq "policy.stop_before_card_mutation_must_be_true" "${skill_root}/adl-skill.yaml"
grep -Fq "bounded issue split planner" "${skill_root}/SKILL.md"
grep -Fq "issues_created: false" "${skill_root}/references/output-contract.md"
grep -Fq "Schema id: \`issue_splitter.v1\`" "${skills_root}/docs/ISSUE_SPLITTER_SKILL_INPUT_SCHEMA.md"

python3 - "${tmpdir}" "${python_script}" <<'PY'
import json
import subprocess
import sys
from pathlib import Path

tmpdir = Path(sys.argv[1])
python_script = Path(sys.argv[2])


def run_case(name: str, source_text: str, stp_text: str = ""):
    root = tmpdir / name
    report = tmpdir / f"{name}-report"
    (root / "task").mkdir(parents=True)
    (root / "source.md").write_text(source_text, encoding="utf-8")
    if stp_text:
        (root / "task" / "stp.md").write_text(stp_text, encoding="utf-8")
    subprocess.run(
        [
            "python3",
            str(python_script),
            "--task-bundle",
            str(root / "task"),
            "--source-prompt",
            str(root / "source.md"),
            "--out",
            str(report),
            "--run-id",
            f"issue-splitter-{name}-test",
        ],
        check=True,
        stdout=subprocess.DEVNULL,
    )
    return report


split_report = run_case(
    "split",
    """## Summary

- runtime: implement the split classifier
- docs: add a new operator guide section
- release: create a release-tail follow-on issue
- review: split the review-lane hardening into a follow-on issue
""",
    "review: split this concern into a later issue after the core classifier lands.\n",
)
split_json = json.loads((split_report / "issue_splitter_report.json").read_text(encoding="utf-8"))
assert split_json["status"] == "split_now"
assert split_json["classification"] == "split_now"
assert split_json["recommended_handoff"] == "finding-to-issue-planner"
assert split_json["proposed_follow_ons"]
assert any(item["bucket"] == "docs" for item in split_json["proposed_follow_ons"])
split_md = (split_report / "issue_splitter_report.md").read_text(encoding="utf-8")
assert "## Proposed Follow-Ons" in split_md
assert "issues_created: false" in split_md

keep_report = run_case(
    "keep",
    """## Summary

- tooling: add the split-planning helper
- tooling: document invocation and stop boundary
- tooling: add one deterministic contract test
""",
)
keep_json = json.loads((keep_report / "issue_splitter_report.json").read_text(encoding="utf-8"))
assert keep_json["status"] == "keep_as_is"
assert keep_json["classification"] == "keep_as_is"
assert keep_json["recommended_handoff"] == "workflow-conductor"
assert not keep_json["proposed_follow_ons"]

defer_report = run_case(
    "defer",
    """## Summary

- tooling: implement the helper
- docs: add the later documentation pass
- docs: defer the documentation split until after the helper proves out
""",
)
defer_json = json.loads((defer_report / "issue_splitter_report.json").read_text(encoding="utf-8"))
assert defer_json["status"] == "defer"
assert defer_json["classification"] == "defer"

blocked_report = run_case(
    "blocked",
    """## Summary

- runtime: split this into a follow-on issue now
- process: this must stay together as a single issue
""",
)
blocked_json = json.loads((blocked_report / "issue_splitter_report.json").read_text(encoding="utf-8"))
assert blocked_json["status"] == "blocked"
assert blocked_json["classification"] == "blocked"
assert blocked_json["recommended_handoff"] == "operator-review"

for report_root in (split_report, keep_report, defer_report, blocked_report):
    for path in report_root.iterdir():
        if str(tmpdir) in path.read_text(encoding="utf-8"):
            raise AssertionError("issue splitter artifacts should not leak absolute temp paths")
PY

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skill_root}/SKILL.md"

echo "PASS test_issue_splitter_skill_contracts"
