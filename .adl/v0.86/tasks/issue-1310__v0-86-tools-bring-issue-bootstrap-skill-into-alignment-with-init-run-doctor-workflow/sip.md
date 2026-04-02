# ADL Input Card

Task ID: issue-1310
Run ID: issue-1310
Version: v0.86
Title: [v0.86][tools] Bring issue-bootstrap skill into alignment with init-run-doctor workflow
Branch: codex/1310-v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/1310
- PR:
- Source Issue Prompt: .adl/v0.86/bodies/issue-1310-v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow.md
- Docs: docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md; docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md
- Other: .adl/skills/issue-bootstrap/; .gitignore

## Agent Execution Rules
- Do not run `pr start`; the branch and worktree already exist.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .adl/v0.86/tasks/issue-1310__v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow/sor.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

Reviewer protocol IDs are versioned and order-sensitive:
1. checklist contract
2. output artifact contract
3. reviewer behavior contract

Prompt Spec contract notes:
- Supported section IDs and machine-readable field semantics are defined in `docs/tooling/prompt-spec.md`.
- Missing required Prompt Spec keys or required boolean `automation_hints` fields should fail lint.
- Prompt generation must preserve declared section order rather than heuristic extraction.

Execution:
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug:
- Required outcome type:
- Demo required:

## Goal

Bring the issue-bootstrap skill bundle into line with the current bootstrap-review-run-doctor model without changing the control-plane commands again.

## Required Outcome

- Update the skill, manifest, playbook, and output contract so they agree on the current workflow truth.
- Remove or correct references that treat ignored or missing planning paths as canonical when the repo does not actually provide them.
- Keep the linked issue prompt, repository changes, and output record aligned.

## Acceptance Criteria

- The skill describes Step 1 as mechanical bootstrap for new or existing issues without teaching `start` as the later binder.
- The handoff after bootstrap is qualitative card review, then `run`, with `doctor` preserved as the diagnostic surface.
- The skill bundle points at source docs that actually exist in the repository.
- The manifest, playbook, and output contract agree with the main skill file on boundaries, commands, and handoff language.

## Inputs
- linked source issue prompt
- .adl/skills/issue-bootstrap/SKILL.md
- .adl/skills/issue-bootstrap/adl-skill.yaml
- .adl/skills/issue-bootstrap/agents/openai.yaml
- .adl/skills/issue-bootstrap/references/bootstrap-playbook.md
- .adl/skills/issue-bootstrap/references/output-contract.md
- docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md
- docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md
- .gitignore

## Target Files / Surfaces
- .adl/skills/issue-bootstrap/SKILL.md
- .adl/skills/issue-bootstrap/adl-skill.yaml
- .adl/skills/issue-bootstrap/agents/openai.yaml
- .adl/skills/issue-bootstrap/references/bootstrap-playbook.md
- .adl/skills/issue-bootstrap/references/output-contract.md
- .adl/v0.86/tasks/issue-1310__v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow/sor.md

## Validation Plan
- Commands to run:
  - inspect the skill bundle and current feature docs
  - verify tracked-vs-ignored path truth for any referenced planning files
  - run the smallest repo-native validation that proves the skill remains aligned with current lifecycle commands
- Tests to run:
  - any bounded command or lint checks needed to prove the updated skill reflects current repo truth
- Artifacts or traces:
  - updated issue-bootstrap skill bundle
  - completed output card
- Reviewer checks:
  - confirm the skill can be used as the final manual issue-bootstrap source without obsolete `create/start` long-term teaching

## Demo / Proof Requirements
- Demo set: follow the linked issue prompt.
- Proof surfaces: updated skill bundle plus output card.
- No-demo rationale: if no demo is required, explain why in the output card.

## Constraints / Policies
- Determinism: keep behavior stable for identical inputs unless the issue explicitly changes semantics.
- Security and privacy: do not introduce secrets, tokens, prompts, tool arguments, or absolute host paths.
- Resource limits: prefer the smallest command and test surface that proves the issue is complete.

## System Invariants (must remain true)
- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Artifacts remain replay-compatible with the replay runner.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.
- Artifact schema changes are explicit and approved.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: true
security_sensitive: true
ci_validation_required: true
```

## Card Automation Hooks (prompt generation)
- Prompt source fields:
  - Goal
  - Required Outcome
  - Acceptance Criteria
  - Inputs
  - Target Files / Surfaces
  - Validation Plan
  - Demo / Proof Requirements
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical input card content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

- unrelated repository repair
- changing the control-plane commands again
- creating the other three workflow skills in this issue

## Notes / Risks

- Refine this card if the linked source issue prompt changes materially before implementation begins.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do the work described above.
- Write results to the paired output card file.
