#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/adl-milestone-creator/SKILL.md" ]]
[[ -f "${skills_root}/adl-milestone-creator/adl-skill.yaml" ]]
[[ -f "${skills_root}/adl-milestone-creator/agents/openai.yaml" ]]
[[ -f "${skills_root}/docs/ADL_MILESTONE_CREATOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: adl-milestone-creator' "${skills_root}/adl-milestone-creator/SKILL.md"
grep -Fq 'id: "adl-milestone-creator"' "${skills_root}/adl-milestone-creator/adl-skill.yaml"
grep -Fq 'id: "adl_milestone_creator.v1"' "${skills_root}/adl-milestone-creator/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ADL_MILESTONE_CREATOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/adl-milestone-creator/adl-skill.yaml"
grep -Fq "require_full_planning_package" "${skills_root}/adl-milestone-creator/adl-skill.yaml"
grep -Fq "Do not create a skinny milestone package" "${skills_root}/adl-milestone-creator/SKILL.md"
grep -Fq "docs/templates/prompts/current.json" "${skills_root}/adl-milestone-creator/SKILL.md"
grep -Fq 'Schema id: `adl_milestone_creator.v1`' "${skills_root}/docs/ADL_MILESTONE_CREATOR_SKILL_INPUT_SCHEMA.md" || \
  grep -Fq 'skill_input_schema: adl_milestone_creator.v1' "${skills_root}/docs/ADL_MILESTONE_CREATOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "adl-milestone-creator" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

python3 - <<'PY' "${skills_root}/adl-milestone-creator/SKILL.md" "${skills_root}/adl-milestone-creator/adl-skill.yaml" "${skills_root}/adl-milestone-creator/agents/openai.yaml"
from pathlib import Path
import sys

for raw in sys.argv[1:]:
    text = Path(raw).read_text()
    todo_marker = "[" + "TODO"
    if todo_marker in text:
        raise SystemExit(f"TODO residue in {raw}")
    host_markers = ["/" + "Users/", "/" + "private/tmp"]
    if any(marker in text for marker in host_markers):
        raise SystemExit(f"host path leakage in {raw}")

skill = Path(sys.argv[1]).read_text()
if len(skill.splitlines()) > 500:
    raise SystemExit("SKILL.md exceeds compact skill size budget")
PY

printf '%s\n' "OK: adl-milestone-creator skill contract"
