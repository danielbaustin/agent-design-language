#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
docs_root="${skills_root}/docs"
guide="${docs_root}/OPERATIONAL_SKILLS_GUIDE.md"

missing=0

fail() {
  echo "FAIL: $*" >&2
  missing=1
}

skill_schema_doc() {
  local skill="$1"
  local upper
  upper="$(printf '%s' "$skill" | tr '[:lower:]-' '[:upper:]_')"
  printf '%s/%s_SKILL_INPUT_SCHEMA.md' "$docs_root" "$upper"
}

for skill_dir in "${skills_root}"/*; do
  [[ -d "$skill_dir" ]] || continue
  skill="$(basename "$skill_dir")"
  [[ "$skill" != "docs" ]] || continue

  [[ -f "${skill_dir}/SKILL.md" ]] || fail "${skill}: missing SKILL.md"
  [[ -f "${skill_dir}/adl-skill.yaml" ]] || fail "${skill}: missing adl-skill.yaml"

  schema_doc="$(skill_schema_doc "$skill")"
  [[ -f "$schema_doc" ]] || fail "${skill}: missing input schema doc ${schema_doc#"$repo_root"/}"

  if [[ "$skill" == "stp-editor" || "$skill" == "sip-editor" || "$skill" == "sor-editor" ]]; then
    grep -Fq "${skill}" "${repo_root}/adl/tools/test_card_editor_skill_contracts.sh" \
      || fail "${skill}: missing card-editor grouped contract coverage"
    grep -Fq 'card-editor shared contract' "$guide" \
      || fail "${skill}: missing guide note for shared card-editor output contract"
  else
    [[ -f "${skill_dir}/references/output-contract.md" ]] \
      || fail "${skill}: missing references/output-contract.md"
  fi

  grep -Fq "\`${skill}\`" "$guide" \
    || fail "${skill}: missing OPERATIONAL_SKILLS_GUIDE mention"
done

grep -Fq 'diff -qr adl/tools/skills/test-generator "$CODEX_HOME/skills/test-generator"' "$guide" \
  || fail "guide: missing deployed skill diff verification recipe"

grep -Fq 'bash adl/tools/test_install_adl_operational_skills.sh' "${repo_root}/adl/tools/README.md" \
  || fail "adl/tools/README.md: missing install verification command"

if [[ "$missing" -ne 0 ]]; then
  exit 1
fi

echo "PASS test_skill_documentation_completeness"
