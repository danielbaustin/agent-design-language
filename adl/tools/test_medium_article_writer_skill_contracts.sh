#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/medium-article-writer/SKILL.md" ]]
[[ -f "${skills_root}/medium-article-writer/adl-skill.yaml" ]]
[[ -f "${skills_root}/medium-article-writer/agents/openai.yaml" ]]
[[ -f "${skills_root}/medium-article-writer/references/medium-writing-playbook.md" ]]
[[ -f "${skills_root}/medium-article-writer/references/output-contract.md" ]]

grep -Fq 'id: "medium_article_writer.v1"' "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq 'reference_doc: "/Users/daniel/git/agent-design-language/adl/tools/skills/docs/MEDIUM_ARTICLE_WRITER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq "policy.writing_mode_must_be_explicit" "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq "policy.stop_before_publish_must_be_true" "${skills_root}/medium-article-writer/adl-skill.yaml"
grep -Fq "reviewer-friendly Medium article packet" "${skills_root}/medium-article-writer/SKILL.md"
grep -Fq "stopping before publication" "${skills_root}/medium-article-writer/SKILL.md"
grep -Fq "This skill stops at a reviewable draft packet." "${skills_root}/medium-article-writer/references/medium-writing-playbook.md"
grep -Fq "Publication Attempted: true | false" "${skills_root}/medium-article-writer/references/output-contract.md"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/medium-article-writer/SKILL.md"

echo "PASS test_medium_article_writer_skill_contracts"
