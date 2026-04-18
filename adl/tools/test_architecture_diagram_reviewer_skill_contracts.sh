#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/architecture-diagram-reviewer/SKILL.md" ]]
[[ -f "${skills_root}/architecture-diagram-reviewer/adl-skill.yaml" ]]
[[ -f "${skills_root}/architecture-diagram-reviewer/agents/openai.yaml" ]]
[[ -f "${skills_root}/architecture-diagram-reviewer/references/diagram-review-playbook.md" ]]
[[ -f "${skills_root}/architecture-diagram-reviewer/references/output-contract.md" ]]
[[ -x "${skills_root}/architecture-diagram-reviewer/scripts/review_architecture_diagrams.py" ]]
[[ -f "${skills_root}/docs/ARCHITECTURE_DIAGRAM_REVIEWER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "architecture-diagram-reviewer"' "${skills_root}/architecture-diagram-reviewer/adl-skill.yaml"
grep -Fq 'id: "architecture_diagram_reviewer.v1"' "${skills_root}/architecture-diagram-reviewer/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ARCHITECTURE_DIAGRAM_REVIEWER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/architecture-diagram-reviewer/adl-skill.yaml"
grep -Fq "review_diagram_against_packet_requires_target.review_packet_path" "${skills_root}/architecture-diagram-reviewer/adl-skill.yaml"
grep -Fq "quality gate after \`diagram-author\`" "${skills_root}/architecture-diagram-reviewer/SKILL.md"
grep -Fq "scripts/review_architecture_diagrams.py" "${skills_root}/architecture-diagram-reviewer/SKILL.md"
grep -Fq "Do not author or edit diagram source" "${skills_root}/architecture-diagram-reviewer/references/output-contract.md"
grep -Fq "Schema id: \`architecture_diagram_reviewer.v1\`" "${skills_root}/docs/ARCHITECTURE_DIAGRAM_REVIEWER_SKILL_INPUT_SCHEMA.md"
grep -Fq "architecture-diagram-reviewer" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
diagram_root="${tmpdir}/diagrams"
review_root="${tmpdir}/diagram-review"
mkdir -p "${packet_root}" "${diagram_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "architecture-diagram-review-contract-test",
  "repo_name": "example-repo",
  "publication_allowed": false
}
JSON
cat >"${packet_root}/evidence_index.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.evidence.v1",
  "evidence": [
    {
      "path": "docs/architecture/runtime.md",
      "category": "architecture_docs",
      "line_count": 42,
      "reason": "runtime architecture boundary surface",
      "specialist_lanes": ["architecture", "docs", "diagrams", "synthesis"]
    },
    {
      "path": "adl/src/execute/state.rs",
      "category": "code",
      "line_count": 220,
      "reason": "runtime state lifecycle implementation",
      "specialist_lanes": ["architecture", "code", "tests", "diagrams", "synthesis"]
    },
    {
      "path": "adl/tools/skills/diagram-author/SKILL.md",
      "category": "docs",
      "line_count": 120,
      "reason": "diagram author source-grounded handoff",
      "specialist_lanes": ["docs", "diagrams", "synthesis"]
    }
  ]
}
JSON
cat >"${diagram_root}/runtime.mmd" <<'MMD'
flowchart TD
  Runtime["runtime state lifecycle"] --> DiagramAuthor["diagram author"]
  Runtime --> UnsupportedThing["unsupported-node needs evidence"]
MMD

python3 "${skills_root}/architecture-diagram-reviewer/scripts/review_architecture_diagrams.py" \
  "${packet_root}" "${diagram_root}" --out "${review_root}" >/tmp/architecture-diagram-reviewer.out
[[ -f "${review_root}/architecture_diagram_review_scaffold.json" ]]
[[ -f "${review_root}/architecture_diagram_review_scaffold.md" ]]
grep -Fq '"schema": "codebuddy.architecture_diagram_review.scaffold.v1"' "${review_root}/architecture_diagram_review_scaffold.json"
grep -Fq '"repo_name": "example-repo"' "${review_root}/architecture_diagram_review_scaffold.json"
grep -Fq 'runtime.mmd' "${review_root}/architecture_diagram_review_scaffold.json"
grep -Fq 'unsupported/evidence marker' "${review_root}/architecture_diagram_review_scaffold.md"
grep -Fq 'Renderer Status Checks' "${review_root}/architecture_diagram_review_scaffold.md"
grep -Fq 'Correction Handoffs' "${review_root}/architecture_diagram_review_scaffold.md"
if grep -R "${tmpdir}" "${review_root}" >/dev/null; then
  echo "architecture diagram review scaffold should not leak absolute temp paths" >&2
  exit 1
fi
if find "${review_root}" -name '*.svg' -o -name '*.png' | grep -q .; then
  echo "architecture diagram reviewer should not render visual assets" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/architecture-diagram-reviewer/SKILL.md"

echo "PASS test_architecture_diagram_reviewer_skill_contracts"

