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
import re
import sys

todo_marker = "[" + "TODO"
host_markers = ["/" + "Users/", "/" + "private/tmp"]
for raw in sys.argv[1:]:
    text = Path(raw).read_text()
    if todo_marker in text:
        raise SystemExit(f"TODO residue in {raw}")
    if any(marker in text for marker in host_markers):
        raise SystemExit(f"host path leakage in {raw}")

manifest = Path(sys.argv[2]).read_text()
contract = Path(sys.argv[4]).read_text()

def yaml_list_after(key):
    lines = manifest.splitlines()
    result = []
    in_list = False
    for line in lines:
        if line.strip() == f"{key}:":
            in_list = True
            continue
        if not in_list:
            continue
        if line.startswith("  ") and line.strip().startswith("- "):
            result.append(line.split("- ", 1)[1].strip().strip('"'))
            continue
        if line.startswith("  ") and line.strip().endswith(":"):
            break
        if line and not line.startswith("  "):
            break
    return result

def contract_list(section):
    match = re.search(rf"## {re.escape(section)}\n\n(.*?)(?:\n## |\Z)", contract, re.S)
    if not match:
        raise SystemExit(f"missing contract section: {section}")
    return re.findall(r"- `([^`]+)`", match.group(1))

manifest_sections = yaml_list_after("required_sections")
contract_sections = contract_list("Required Markdown Sections")
if manifest_sections != contract_sections:
    raise SystemExit(
        "manifest required_sections drift from output contract: "
        f"{manifest_sections!r} != {contract_sections!r}"
    )

manifest_json = yaml_list_after("required_json_fields")
contract_json = contract_list("Required JSON Shape")
if manifest_json != contract_json:
    raise SystemExit(
        "manifest required_json_fields drift from output contract: "
        f"{manifest_json!r} != {contract_json!r}"
    )
PY

printf '%s\n' "OK: planning-doc-editor skill contract"
