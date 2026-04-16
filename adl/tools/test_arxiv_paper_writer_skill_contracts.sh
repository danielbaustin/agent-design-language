#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/arxiv-paper-writer/SKILL.md" ]]
[[ -f "${skills_root}/arxiv-paper-writer/adl-skill.yaml" ]]
[[ -f "${skills_root}/arxiv-paper-writer/agents/openai.yaml" ]]
[[ -f "${skills_root}/arxiv-paper-writer/references/arxiv-writing-playbook.md" ]]
[[ -f "${skills_root}/arxiv-paper-writer/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "arxiv_paper_writer.v1"' "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq "policy.citation_policy_must_be_explicit" "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq "policy.stop_before_submission_must_be_true" "${skills_root}/arxiv-paper-writer/adl-skill.yaml"
grep -Fq "without submitting, publishing, inventing citations" "${skills_root}/arxiv-paper-writer/SKILL.md"
grep -Fq "no arXiv submission was attempted" "${skills_root}/arxiv-paper-writer/SKILL.md"
grep -Fq "This skill stops at a reviewable manuscript packet." "${skills_root}/arxiv-paper-writer/references/arxiv-writing-playbook.md"
grep -Fq "Submission Attempted: true | false" "${skills_root}/arxiv-paper-writer/references/output-contract.md"
grep -Fq 'Schema id: `arxiv_paper_writer.v1`' "${skills_root}/docs/ARXIV_PAPER_WRITER_SKILL_INPUT_SCHEMA.md"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/arxiv-paper-writer/SKILL.md"

echo "PASS test_arxiv_paper_writer_skill_contracts"
