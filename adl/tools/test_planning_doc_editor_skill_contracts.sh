#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/planning-doc-editor/SKILL.md" ]]
[[ -f "${skills_root}/planning-doc-editor/adl-skill.yaml" ]]
[[ -f "${skills_root}/planning-doc-editor/agents/openai.yaml" ]]
[[ -f "${skills_root}/planning-doc-editor/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/PLANNING_DOC_EDITOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: planning-doc-editor' "${skills_root}/planning-doc-editor/SKILL.md"
grep -Fq 'id: "planning-doc-editor"' "${skills_root}/planning-doc-editor/adl-skill.yaml"
grep -Fq 'id: "planning_doc_editor.v1"' "${skills_root}/planning-doc-editor/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/PLANNING_DOC_EDITOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/planning-doc-editor/adl-skill.yaml"
grep -Fq "policy.no_card_edits_must_be_true" "${skills_root}/planning-doc-editor/adl-skill.yaml"
grep -Fq "Planning Doc Editor Output Contract" "${skills_root}/planning-doc-editor/references/output-contract.md"
grep -Fq "skill_input_schema: planning_doc_editor.v1" "${skills_root}/docs/PLANNING_DOC_EDITOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "planning-doc-editor" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

python3 - <<'PY' "${skills_root}/planning-doc-editor/SKILL.md" "${skills_root}/planning-doc-editor/adl-skill.yaml" "${skills_root}/planning-doc-editor/agents/openai.yaml" "${skills_root}/planning-doc-editor/references/output-contract.md"
from pathlib import Path
import sys

todo_marker = "[" + "TODO"
host_markers = ["/" + "Users/", "/" + "private/tmp"]
for raw in sys.argv[1:]:
    text = Path(raw).read_text()
    if todo_marker in text:
        raise SystemExit(f"TODO residue in {raw}")
    if any(marker in text for marker in host_markers):
        raise SystemExit(f"host path leakage in {raw}")
PY

printf '%s\n' "OK: planning-doc-editor skill contract"
