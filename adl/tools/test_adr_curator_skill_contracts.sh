#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/adr-curator/SKILL.md" ]]
[[ -f "${skills_root}/adr-curator/adl-skill.yaml" ]]
[[ -f "${skills_root}/adr-curator/agents/openai.yaml" ]]
[[ -f "${skills_root}/adr-curator/references/adr-curation-playbook.md" ]]
[[ -f "${skills_root}/adr-curator/references/output-contract.md" ]]
[[ -x "${skills_root}/adr-curator/scripts/curate_adrs.py" ]]
[[ -f "${skills_root}/docs/ADR_CURATOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "adr-curator"' "${skills_root}/adr-curator/adl-skill.yaml"
grep -Fq 'id: "adr_curator.v1"' "${skills_root}/adr-curator/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ADR_CURATOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/adr-curator/adl-skill.yaml"
grep -Fq "policy.stop_before_acceptance_must_be_true" "${skills_root}/adr-curator/adl-skill.yaml"
grep -Fq "This skill is a curation lane, not a decision authority" "${skills_root}/adr-curator/SKILL.md"
grep -Fq "scripts/curate_adrs.py" "${skills_root}/adr-curator/SKILL.md"
grep -Fq "Do not accept, reject, supersede, publish, or commit ADRs." "${skills_root}/adr-curator/references/output-contract.md"
grep -Fq "Schema id: \`adr_curator.v1\`" "${skills_root}/docs/ADR_CURATOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "adr-curator" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
out_root="${tmpdir}/adr-curation"
mkdir -p "${packet_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "adr-curator-contract-test",
  "repo_name": "example-repo",
  "publication_allowed": false
}
JSON
cat >"${packet_root}/evidence_index.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.evidence.v1",
  "evidence": [
    {
      "path": "docs/adr/0001-provider-registry.md",
      "category": "docs",
      "line_count": 80,
      "reason": "accepted ADR documents provider registry decision",
      "specialist_lanes": ["architecture", "docs", "synthesis"]
    },
    {
      "path": "docs/migrations/runtime-boundary.md",
      "category": "migration_notes",
      "line_count": 60,
      "reason": "migration note records a decision to route runtime state through adapters",
      "specialist_lanes": ["architecture", "dependency", "synthesis"]
    }
  ]
}
JSON
cat >"${packet_root}/architecture-review.md" <<'MARKDOWN'
# Architecture Review

## Candidate ADRs

- Decision: Route provider calls through the provider registry.
  Status: proposed
  Source: docs/architecture/provider-registry.md
  Context: Provider calls currently need one stable ownership boundary.
  Decision: Runtime code should call providers through the provider registry.
  Consequences: Centralizes provider selection but requires registry tests.
  Alternatives: Direct provider imports; per-command provider construction.
  Validation: Review packet evidence only; no ADR was accepted.

- Decision: Replace the legacy runtime boundary ADR.
  Status: proposed
  Source: docs/migrations/runtime-boundary.md
  Supersedes: ADR-0001
  Context: Migration notes describe a new runtime adapter boundary.
  Decision: Runtime state should cross subsystem boundaries through adapters.
  Consequences: Clearer layering with additional adapter maintenance cost.
MARKDOWN

python3 "${skills_root}/adr-curator/scripts/curate_adrs.py" \
  "${packet_root}" --out "${out_root}" --max-adrs 8 >/tmp/adr-curator.out

[[ -f "${out_root}/adr_candidates.json" ]]
[[ -f "${out_root}/adr_candidates.md" ]]
grep -Fq '"schema": "codebuddy.adr_curator.v1"' "${out_root}/adr_candidates.json"
grep -Fq '"repo_name": "example-repo"' "${out_root}/adr_candidates.json"
grep -Fq '"status": "proposed"' "${out_root}/adr_candidates.json"
grep -Fq '"ADR-0001"' "${out_root}/adr_candidates.json"
grep -Fq 'Proposed ADR Drafts' "${out_root}/adr_candidates.md"
grep -Fq 'Supersession Map' "${out_root}/adr_candidates.md"
grep -Fq 'Human approval is required before ADR acceptance.' "${out_root}/adr_candidates.md"
grep -Fq 'No ADR files, issues, PRs, tests, docs, or remediation branches were created' "${out_root}/adr_candidates.md"
if grep -R "${tmpdir}" "${out_root}" >/dev/null; then
  echo "ADR curation artifacts should not leak absolute temp paths" >&2
  exit 1
fi
if find "${out_root}" -name '*.rs' -o -name '*.sh' -o -name '*.yml' | grep -q .; then
  echo "ADR curator should not write implementation files" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/adr-curator/SKILL.md"

echo "PASS test_adr_curator_skill_contracts"
