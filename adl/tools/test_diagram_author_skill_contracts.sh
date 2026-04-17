#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/diagram-author/SKILL.md" ]]
[[ -f "${skills_root}/diagram-author/adl-skill.yaml" ]]
[[ -f "${skills_root}/diagram-author/agents/openai.yaml" ]]
[[ -f "${skills_root}/diagram-author/references/diagram-playbook.md" ]]
[[ -f "${skills_root}/diagram-author/references/output-contract.md" ]]
[[ -f "${skills_root}/diagram-author/references/renderer-setup.md" ]]
[[ -x "${skills_root}/diagram-author/scripts/render_diagrams.sh" ]]
[[ -f "${skills_root}/docs/DIAGRAM_AUTHOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "diagram_author.v1"' "${skills_root}/diagram-author/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/DIAGRAM_AUTHOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/diagram-author/adl-skill.yaml"
grep -Fq "policy.backend_policy_must_be_explicit" "${skills_root}/diagram-author/adl-skill.yaml"
grep -Fq "policy.stop_before_publication_must_be_true" "${skills_root}/diagram-author/adl-skill.yaml"
grep -Fq "Mermaid, D2, PlantUML, Structurizr DSL" "${skills_root}/diagram-author/SKILL.md"
grep -Fq "Do not choose formal UML just because the request says \"diagram\"" "${skills_root}/diagram-author/SKILL.md"
grep -Fq "scripts/render_diagrams.sh" "${skills_root}/diagram-author/SKILL.md"
grep -Fq "Prefer SVG as the durable rendered artifact." "${skills_root}/diagram-author/references/diagram-playbook.md"
grep -Fq "brew install mermaid-cli d2 plantuml graphviz librsvg" "${skills_root}/diagram-author/references/renderer-setup.md"
grep -Fq "mermaid-rs-renderer" "${skills_root}/diagram-author/references/renderer-setup.md"
grep -Fq "Publication Attempted: true | false" "${skills_root}/diagram-author/references/output-contract.md"
grep -Fq "Rendered Artifacts" "${skills_root}/diagram-author/references/output-contract.md"
grep -Fq 'Schema id: `diagram_author.v1`' "${skills_root}/docs/DIAGRAM_AUTHOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "output_formats" "${skills_root}/docs/DIAGRAM_AUTHOR_SKILL_INPUT_SCHEMA.md"

"${skills_root}/diagram-author/scripts/render_diagrams.sh" --check-tools >/tmp/diagram-author-render-tools.tsv
"${skills_root}/diagram-author/scripts/render_diagrams.sh" --help | grep -Fq "Supported inputs:"
"${skills_root}/diagram-author/scripts/render_diagrams.sh" --help | grep -Fq "skip-backends"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/diagram-author/SKILL.md"

echo "PASS test_diagram_author_skill_contracts"
