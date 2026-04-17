#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/repo-diagram-planner/SKILL.md" ]]
[[ -f "${skills_root}/repo-diagram-planner/adl-skill.yaml" ]]
[[ -f "${skills_root}/repo-diagram-planner/agents/openai.yaml" ]]
[[ -f "${skills_root}/repo-diagram-planner/references/diagram-planning-playbook.md" ]]
[[ -f "${skills_root}/repo-diagram-planner/references/output-contract.md" ]]
[[ -x "${skills_root}/repo-diagram-planner/scripts/plan_repo_diagrams.py" ]]
[[ -f "${skills_root}/docs/REPO_DIAGRAM_PLANNER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "repo-diagram-planner"' "${skills_root}/repo-diagram-planner/adl-skill.yaml"
grep -Fq 'id: "repo_diagram_planner.v1"' "${skills_root}/repo-diagram-planner/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_DIAGRAM_PLANNER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-diagram-planner/adl-skill.yaml"
grep -Fq "plan_from_review_packet_requires_target.review_packet_path" "${skills_root}/repo-diagram-planner/adl-skill.yaml"
grep -Fq "Plan diagram work for a repository review without becoming the diagram author" "${skills_root}/repo-diagram-planner/SKILL.md"
grep -Fq "scripts/plan_repo_diagrams.py" "${skills_root}/repo-diagram-planner/SKILL.md"
grep -Fq "Do not author diagram source or rendered diagram assets" "${skills_root}/repo-diagram-planner/references/output-contract.md"
grep -Fq "Schema id: \`repo_diagram_planner.v1\`" "${skills_root}/docs/REPO_DIAGRAM_PLANNER_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo-diagram-planner" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
plan_root="${tmpdir}/diagram-plan"
mkdir -p "${packet_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "diagram-planner-contract-test",
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
      "reason": "architecture overview and runtime boundary surface",
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
      "path": "adl/tools/skills/repo-review-synthesis/SKILL.md",
      "category": "docs",
      "line_count": 90,
      "reason": "multi-agent specialist handoff and synthesis lane",
      "specialist_lanes": ["docs", "diagrams", "synthesis"]
    },
    {
      "path": "Cargo.toml",
      "category": "manifest",
      "line_count": 60,
      "reason": "dependency manifest",
      "specialist_lanes": ["dependency", "diagrams", "synthesis"]
    },
    {
      "path": "README.md",
      "category": "docs",
      "line_count": 20,
      "reason": "onboarding surface",
      "specialist_lanes": ["docs", "synthesis"]
    }
  ]
}
JSON

python3 "${skills_root}/repo-diagram-planner/scripts/plan_repo_diagrams.py" \
  "${packet_root}" --out "${plan_root}" --max-tasks 4 >/tmp/repo-diagram-planner.out
[[ -f "${plan_root}/repo_diagram_plan.json" ]]
[[ -f "${plan_root}/repo_diagram_plan.md" ]]
grep -Fq '"schema": "codebuddy.repo_diagram_plan.v1"' "${plan_root}/repo_diagram_plan.json"
grep -Fq '"repo_name": "example-repo"' "${plan_root}/repo_diagram_plan.json"
grep -Fq '"diagram_family": "system_context"' "${plan_root}/repo_diagram_plan.json"
grep -Fq '"diagram_family": "container_or_component"' "${plan_root}/repo_diagram_plan.json"
grep -Fq '"skill": "diagram-author"' "${plan_root}/repo_diagram_plan.json"
grep -Fq 'Diagram Tasks' "${plan_root}/repo_diagram_plan.md"
grep -Fq 'Source Evidence Map' "${plan_root}/repo_diagram_plan.md"
grep -Fq 'Family / Backend Rationale' "${plan_root}/repo_diagram_plan.md"
grep -Fq 'Diagram Author Handoff' "${plan_root}/repo_diagram_plan.md"
if grep -R "${tmpdir}" "${plan_root}" >/dev/null; then
  echo "diagram planner output should not leak absolute temp paths" >&2
  exit 1
fi
if grep -R '```mermaid\|@startuml\|<svg\|\.png' "${plan_root}" >/dev/null; then
  echo "diagram planner should not author diagram source or rendered assets" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/repo-diagram-planner/SKILL.md"

echo "PASS test_repo_diagram_planner_skill_contracts"
