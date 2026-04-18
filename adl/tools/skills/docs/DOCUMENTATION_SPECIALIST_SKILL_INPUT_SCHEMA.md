# Documentation Specialist Skill Input Schema

Schema id: `documentation_specialist.v1`

This schema describes structured input for the `documentation-specialist` skill.
The skill plans, writes, audits, repairs, or polishes bounded repository
documentation from explicit source evidence and stops before publication,
approval claims, ADR acceptance, or broad unbounded rewrites.

## Required Top-Level Fields

- `skill_input_schema`: must be `documentation_specialist.v1`.
- `mode`: one of the supported modes below.
- `target`: bounded documentation target.
- `source_packet`: evidence used to support factual claims.
- `audience`: intended reader.
- `policy`: edit, validation, and stop-boundary policy.

## Supported Modes

- `write_doc`: create a bounded documentation target.
- `repair_doc`: fix stale, misleading, or incomplete documentation in a bounded
  target.
- `audit_doc`: produce findings or a handoff packet without editing by default.
- `polish_doc`: improve readability while preserving facts and scope.
- `plan_doc`: outline a future documentation target from source evidence.
- `refresh_doc`: update a bounded doc after source evidence changes.
- `handoff_packet`: emit a documentation packet for another skill or operator.

## Target Fields

- `path`: repository-relative target path when editing or auditing an existing
  file.
- `doc_type`: README, milestone, feature, ADR, demo, review, architecture,
  onboarding, runbook, skill, or another explicit type.
- `scope`: one bounded slice, path list, issue, PR, milestone, or review packet.
- `write_intent`: edit_existing, create_new, audit_only, plan_only, or
  handoff_only.

## Source Packet Fields

- `evidence_paths`: repository-relative paths that support factual claims.
- `issues`: issue numbers or URLs used as intent evidence.
- `prs`: PR numbers or URLs used as integration evidence.
- `commands`: commands whose output or existence is relevant.
- `known_gaps`: source gaps already known to the operator.
- `assumptions`: assumptions that must stay labeled in output.

## Audience Values

Use an explicit value such as:

- `operator`
- `reviewer`
- `new_contributor`
- `maintainer`
- `customer_private`
- `public_candidate`

## Policy Fields

- `bounded_target_required`: must be true.
- `source_evidence_required`: must be true.
- `allow_repo_edits`: whether the skill may edit tracked docs.
- `write_handoff_artifact`: whether to write a separate handoff packet.
- `check_commands`: whether safe bounded commands should be checked.
- `stop_before_publication`: must be true.
- `stop_before_broad_rewrite`: must be true.

## Output Contract

When editing documentation, the skill reports:

- target paths changed
- source evidence used
- claims clarified
- assumptions and gaps surfaced
- commands checked or skipped
- validation result
- residual risk

When producing an artifact, use:

- `references/output-contract.md`

## Boundaries

The skill must not publish externally, claim release approval, claim review
approval, accept ADRs, create issues, open PRs, rewrite broad doc sets without
an explicit bounded target, or present planned work as implemented behavior.
