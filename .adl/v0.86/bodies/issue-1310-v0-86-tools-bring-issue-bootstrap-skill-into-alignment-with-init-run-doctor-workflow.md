---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow"
title: "[v0.86][tools] Bring issue-bootstrap skill into alignment with init-run-doctor workflow"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.86"
issue_number: 1310
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs:
  - ".adl/skills/issue-bootstrap/"
  - "docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md"
  - "docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md"
  - "adl/tools/pr.sh"
canonical_files:
  - ".adl/skills/issue-bootstrap/SKILL.md"
  - ".adl/skills/issue-bootstrap/adl-skill.yaml"
  - ".adl/skills/issue-bootstrap/agents/openai.yaml"
  - ".adl/skills/issue-bootstrap/references/bootstrap-playbook.md"
  - ".adl/skills/issue-bootstrap/references/output-contract.md"
demo_required: false
demo_names: []
issue_graph_notes:
  - "Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet."
pr_start:
  enabled: true
  slug: "v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow"
---

## Summary
Bring the issue-bootstrap skill and its supporting contracts up to date with the current PR lifecycle so it becomes a trustworthy source for subagent-driven issue bootstrap.

## Goal
Make the issue-bootstrap skill reflect the current public model truthfully: init for mechanical bootstrap, qualitative card review as a separate step, run as the execution-time binder, and doctor as the diagnostic surface.

## Required Outcome
This issue must ship:
- a corrected issue-bootstrap skill that no longer teaches the superseded create-start lifecycle as the long-term model
- aligned supporting files so the skill, manifest, playbook, and output contract agree with each other
- corrected canonical doc references so the skill points at real source material in the repository
- proof that the skill can be used as the durable source for automated issue bootstrap without leaking obsolete workflow steps

## Deliverables

- updated .adl/skills/issue-bootstrap/SKILL.md
- updated skill manifest and supporting agent metadata
- updated bootstrap playbook and output contract
- validation notes proving the skill boundaries, handoff, and references are truthful

## Acceptance Criteria

- the skill teaches init as the bootstrap step and run as the later execution-time binder
- the skill preserves doctor as the review and drift-diagnostic surface
- the skill no longer points at missing or misplaced source docs
- the manifest, playbook, and output contract are consistent with the main skill file
- the skill is explicit enough to become the last manually created issue before issue-bootstrap is delegated to subagents

## Repo Inputs

- .adl/skills/issue-bootstrap/
- docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md
- docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md
- adl/tools/pr.sh
- current git tracking and ignore rules for `.adl/`

## Dependencies

- issue #1303 for the init-run workflow shift
- issue #1304 for the broader issue-bootstrap skill direction

## Demo Expectations

- no runtime demo is required
- proof should be a coherent skill bundle plus validation against current repo truth

## Non-goals

- creating the other three workflow skills in this issue
- changing the actual control-plane commands again here
- implementing qualitative review or execution behavior inside the bootstrap skill

## Issue-Graph Notes

- This should become the final manual issue-creation cleanup before issue-bootstrap can be delegated permanently.
- The result should be strong enough that later skills can be written directly from the docs and skill bundle.

## Notes

- Treat the current repo truth as authoritative over any stale language inside the existing skill files.
- Do not preserve references to ignored planning files as canonical sources unless this issue also makes those files real and durable.

## Tooling Notes

- The skill must stop at mechanical bootstrap and hand off cleanly to qualitative review.
