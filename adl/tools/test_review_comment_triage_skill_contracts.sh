#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

skill_root="${skills_root}/review-comment-triage"
fixtures_root="${skill_root}/fixtures"
python_script="${skill_root}/scripts/triage_review_comments.py"

[[ -f "${skill_root}/SKILL.md" ]]
[[ -f "${skill_root}/adl-skill.yaml" ]]
[[ -f "${skill_root}/agents/openai.yaml" ]]
[[ -f "${skill_root}/references/review-comment-triage-playbook.md" ]]
[[ -f "${skill_root}/references/output-contract.md" ]]
[[ -x "${python_script}" ]]
[[ -f "${skills_root}/docs/REVIEW_COMMENT_TRIAGE_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "review-comment-triage"' "${skill_root}/adl-skill.yaml"
grep -Fq 'id: "review_comment_triage.v1"' "${skill_root}/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REVIEW_COMMENT_TRIAGE_SKILL_INPUT_SCHEMA.md"' "${skill_root}/adl-skill.yaml"
grep -Fq "route_follow_on_as_issue_planning" "${skill_root}/adl-skill.yaml"
grep -Fq "This is optional orchestration guidance only" "${skill_root}/SKILL.md"
grep -Fq "review-comment-triage" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "review-comment-triage" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "blocked_or_operator_decision" "${skill_root}/references/output-contract.md"
grep -Fq "Schema id: \`review_comment_triage.v1\`" "${skills_root}/docs/REVIEW_COMMENT_TRIAGE_SKILL_INPUT_SCHEMA.md"

for fixture in \
  "actionable_comments.json" \
  "already_fixed_comments.json" \
  "stale_comments.json" \
  "follow_on_comments.json" \
  "mixed_comments.json"; do
  [[ -f "${fixtures_root}/${fixture}" ]]
done

python3 - "${skills_root}" "${tmpdir}" <<'PY'
import json
import pathlib
import subprocess
import sys

skills_root = pathlib.Path(sys.argv[1])
tmpdir = pathlib.Path(sys.argv[2])
script = skills_root / "review-comment-triage" / "scripts" / "triage_review_comments.py"
fixtures_root = skills_root / "review-comment-triage" / "fixtures"

expected_counts = {
    "actionable_comments.json": {"actionable_now": 2},
    "already_fixed_comments.json": {"already_fixed": 2},
    "stale_comments.json": {"stale_or_not_reproducible": 2},
    "follow_on_comments.json": {"follow_on_issue_needed": 2},
    "mixed_comments.json": {
        "actionable_now": 1,
        "blocked_or_operator_decision": 1,
        "already_fixed": 1,
        "stale_or_not_reproducible": 1,
        "follow_on_issue_needed": 1,
    },
}

for name, expected in expected_counts.items():
    payload = fixtures_root / name
    out = tmpdir / f"{name}.out.json"
    proc = subprocess.run(
        ["python3", str(script), str(payload), "--out", str(out)],
        check=True,
        capture_output=True,
        text=True,
    )
    assert proc.returncode == 0
    with out.open(encoding="utf-8") as fh:
        triage = json.load(fh)
    for category, count in expected.items():
        assert triage["counts"].get(category, 0) == count, (name, category, triage["counts"], expected)
    for required in {
        "actionable_now",
        "blocked_or_operator_decision",
        "already_fixed",
        "stale_or_not_reproducible",
        "follow_on_issue_needed",
    }:
        assert required in triage["triage"], required
    assert "recommended_handoffs" in triage

    if name == "actionable_comments.json":
        assert triage["counts"]["actionable_now"] > 0
        assert triage["recommended_handoffs"]["actionable_now"] == "pr-janitor"
PY

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skill_root}/SKILL.md"

echo "PASS test_review_comment_triage_skill_contracts"
