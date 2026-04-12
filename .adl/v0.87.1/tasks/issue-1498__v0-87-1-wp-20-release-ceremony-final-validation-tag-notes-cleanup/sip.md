# ADL Input Card

Task ID: issue-1498
Run ID: issue-1498
Version: v0.87.1
Title: [v0.87.1][WP-20] Release ceremony (final validation + tag + notes + cleanup)
Branch: codex/1498-v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/1498
- PR: none
- Source Issue Prompt: .adl/v0.87.1/bodies/issue-1498-v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup.md
- Docs:
  - CHANGELOG.md
  - docs/milestones/v0.87.1/README.md
  - docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md
  - docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md
  - docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md
- Other:
  - adl/tools/README.md
  - adl/tools/release_ceremony.sh

## Agent Execution Rules
- This issue is execution-bound in its dedicated worktree; do not work on `main`.
- Do not run the release ceremony script yet.
- Do not create or push the release tag yet.
- Do not draft or publish the GitHub release yet.
- Only modify files needed to make the release ceremony truthful and ready.
- Keep repository-relative paths in cards and docs; avoid absolute host paths.
- Record only validations actually run.

## Goal

Prepare `v0.87.1` for final release ceremony execution without performing the mutating ceremony actions yet.

## Required Outcome

- align the release-tail docs with the real closed/open issue boundary
- add the canonical release-ceremony helper and tool docs
- keep internal/generated proof spill out of public repo-root surfaces
- leave the milestone ready for the final preflight/tag/release execution pass

## Acceptance Criteria

- release-tail docs no longer imply that `WP-17` through `WP-19` are still open
- `artifacts/` is treated as local internal/generated output, not repo content to publish
- the release-ceremony helper exists and is documented, but has not yet been run for mutation
- the output card truthfully records a preparation-only state if ceremony execution has not yet happened

## Inputs
- linked source issue prompt
- current `v0.87.1` milestone docs
- current `#1498` branch/worktree state
- current tool README and release-helper draft

## Target Files / Surfaces
- .gitignore
- adl/tools/README.md
- adl/tools/release_ceremony.sh
- docs/milestones/v0.87.1/README.md
- docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md
- docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md
- docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md
- .adl/v0.87.1/tasks/issue-1498__v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup/sor.md

## Validation Plan
- Required now:
  - shell syntax validation for `adl/tools/release_ceremony.sh`
  - release-doc wording and status review
  - diff/consistency checks for the touched release-tail surfaces
- Explicitly deferred until final ceremony execution:
  - running `adl/tools/release_ceremony.sh`
  - creating/pushing `v0.87.1`
  - drafting/publishing the GitHub Release

## Demo / Proof Requirements
- No standalone demo is required.
- Proof is a truthful release-preparation bundle: aligned release docs, helper script, ignore policy, and a ready execution surface for the final ceremony pass.

## Constraints / Policies
- Determinism: release-tail wording must match actual issue/review state.
- Security and privacy: internal/generated local outputs remain outside the public tracked surface.
- Resource limits: keep the work bounded to ceremony readiness, not a broader release redesign.

## System Invariants (must remain true)
- No hidden state or undeclared side effects.
- Release docs must not overclaim completed ceremony actions.
- Internal/generated artifact roots must not silently become public tracked repo content.
- Final release mutation happens only when explicitly requested.

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

- performing the final tag/release mutation steps
- reopening review or remediation scope
- regenerating or publishing the internal proof artifact corpus
- changing milestone scope beyond truthful release-tail alignment

## Notes / Risks

- The main risk is overstating release completion before the final ceremony run.
- The second risk is allowing repo-root `artifacts/` spill to masquerade as a publishable surface.
