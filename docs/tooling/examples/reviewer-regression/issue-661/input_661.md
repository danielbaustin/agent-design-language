# ADL Input Card

Task ID: issue-0661
Run ID: issue-0661
Version: v0.3
Title: canonical-v08-milestone-index-and-navigation
Branch: codex/661-canonical-v08-milestone-index-and-navigation

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/661
- PR:
- Docs:
- Other:

Execution:
- Agent: Codex
- Provider: local workspace
- Tools allowed: git, filesystem, editor
- Sandbox / approvals: repo-local only, no network required

## Goal
Create the canonical navigation and index surface for the v0.8 milestone documentation under `docs/milestones/v0.8/` so contributors can quickly understand the scope, document structure, and reading order for the milestone.

This task should:
- ensure `docs/milestones/v0.8/README.md` functions as the canonical entry point
- organize links to all major v0.8 design, architecture, and planning docs
- define a clear reading/navigation order for contributors
- remove or avoid redundant or confusing navigation references

No functional or architectural changes should be introduced; this is a documentation navigation pass only.

## Acceptance Criteria
- `docs/milestones/v0.8/README.md` clearly serves as the canonical milestone index.
- All major v0.8 milestone documents are linked from the README.
- Documents are grouped into logical sections such as:
  - Vision / overview
  - Architecture
  - Epics
  - Execution / planning
  - Supporting design documents
- Navigation links use repo-relative paths.
- No broken internal links remain in the v0.8 milestone directory.
- No duplicate or conflicting “source of truth” references remain.
- Changes remain limited to documentation navigation and linking.

## Inputs
- Existing v0.8 milestone documents under `docs/milestones/v0.8/`
- Prior reconciliation work completed in issues #659 and #660
- Repository documentation conventions (README-based navigation)

## Constraints / Policies
- Determinism requirements:
  - The navigation structure should be stable and deterministic for identical repository state.
- Security / privacy requirements:
  - Documentation must not include secrets, tokens, prompts, tool arguments, or absolute host paths.
- Resource limits (time/CPU/memory/network):
  - Local repository editing only; no network or external services required.

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
  - Acceptance Criteria
  - Inputs
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical input card content
  - No secrets, tokens, or absolute host paths in generated prompt text

## Non-goals / Out of scope
- Rewriting or substantially modifying existing v0.8 design documents.
- Changing architectural decisions captured in milestone docs.
- Introducing new epics or features.
- Moving documents outside the `docs/milestones/v0.8/` directory.

## Notes / Risks
- Earlier planning work placed milestone documents in both `.adl/` planning locations and canonical `docs/` paths; this task assumes reconciliation from #659 and #660 is complete.
- Care should be taken not to reintroduce ambiguous “source of truth” references.
- Navigation structure should remain simple and maintainable.

## Instructions to the Agent
- Read this file.
- Do the work described above.
- Write results to the paired output card file.
