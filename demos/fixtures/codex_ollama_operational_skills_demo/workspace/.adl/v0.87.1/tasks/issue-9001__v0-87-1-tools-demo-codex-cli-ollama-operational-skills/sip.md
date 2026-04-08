# ADL Input Card

Task ID: issue-9001
Run ID: issue-9001
Version: v0.87.1
Title: [v0.87.1][tools] Demo Codex CLI + Ollama operational skills
Branch: codex/demo-codex-ollama-skills

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/9001
- PR: https://github.com/danielbaustin/agent-design-language/pull/9001
- Source Issue Prompt: .adl/v0.87.1/bodies/issue-9001-v0-87-1-tools-demo-codex-cli-ollama-operational-skills.md
- Docs: none
- Other: This is a local demo fixture; keep edits bounded to this task bundle.

## Agent Execution Rules
- Do not create or bind a new branch or worktree.
- Do not delete or recreate cards.
- Do not widen scope beyond card cleanup.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record only if explicitly asked.

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
  output_card: .adl/v0.87.1/tasks/issue-9001__v0-87-1-tools-demo-codex-cli-ollama-operational-skills/sor.md
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

Execution:
- Agent: codex
- Provider: local codex CLI
- Tools allowed: local file edits only
- Sandbox / approvals: workspace-write
- Source issue-prompt slug: v0-87-1-tools-demo-codex-cli-ollama-operational-skills
- Required outcome type: docs
- Demo required: false

## Goal

Clean up this demo input card so it is truthful and ready for bounded review.

## Required Outcome

- Keep the task docs-only.
- Keep the edits bounded to the local fixture cards.

## Acceptance Criteria

- The lifecycle truth is clear and not contradictory.
- The target surfaces are concrete enough for a bounded card-cleanup task.

## Inputs
- linked source issue prompt
- current STP and SIP in this task bundle

## Target Files / Surfaces
- maybe these cards

## Validation Plan
- Commands to run: maybe validate the cards
- Tests to run: none probably
- Artifacts or traces: none
- Reviewer checks: make sure the cards are clearer than before

## Demo / Proof Requirements
- Demo set: none
- Proof surfaces: cleaned cards
- No-demo rationale: this fixture is only for card cleanup

## Constraints / Policies
- Determinism: keep the edits stable and bounded.
- Security and privacy: do not add secrets or absolute host paths.
- Resource limits: keep the task small.

## System Invariants (must remain true)
- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: false
security_sensitive: true
ci_validation_required: false
```

## Non-goals / Out of scope

- code changes
- PR publication

## Notes / Risks

- This card is intentionally a little sloppy so the demo has a bounded cleanup task.

## Instructions to the Agent
- Read the linked source issue prompt before editing.
- Keep the task bounded to card cleanup.
- Do not invent implementation work.
