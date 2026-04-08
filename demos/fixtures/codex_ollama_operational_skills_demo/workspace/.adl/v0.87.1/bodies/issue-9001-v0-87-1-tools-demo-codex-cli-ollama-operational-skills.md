---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-87-1-tools-demo-codex-cli-ollama-operational-skills"
title: "[v0.87.1][tools] Demo Codex CLI + Ollama operational skills"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.87.1"
issue_number: 9001
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Demo fixture"
required_outcome_type:
  - "docs"
repo_inputs:
  - "adl/tools/skills/stp-editor/SKILL.md"
  - "adl/tools/skills/sip-editor/SKILL.md"
  - "adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Fixture source prompt for the Codex CLI + Ollama operational-skills demo."
pr_start:
  enabled: false
  slug: "v0-87-1-tools-demo-codex-cli-ollama-operational-skills"
---

## Summary

This fixture represents a small workflow-skill task that only needs bounded card cleanup. The demo should show Codex CLI using the tracked editor skills to tighten the STP and SIP without widening scope or inventing implementation work.

## Goal

Use the tracked editor skills to make the paired STP and SIP clearer, more truthful, and ready for qualitative review.

## Required Outcome

This fixture should remain a docs/card-only task.

## Deliverables

- a tightened STP with concrete deliverables and acceptance criteria
- a truthful SIP with cleaner targets and validation guidance

## Acceptance Criteria

- the STP is bounded and reviewable without changing issue intent
- the SIP reflects truthful lifecycle state and concrete validation guidance
- no branch/worktree creation or implementation claims are introduced

## Repo Inputs

- `adl/tools/skills/stp-editor/SKILL.md`
- `adl/tools/skills/sip-editor/SKILL.md`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`

## Dependencies

- none

## Demo Expectations

- no external demo artifact is required; the proof is the cleaned cards plus the Codex CLI transcript

## Non-goals

- implementing application code
- publishing a PR
- editing unrelated files

## Issue-Graph Notes

- This fixture is intentionally small so the Codex CLI + Ollama demo stays bounded and repeatable.

## Notes

- Keep the improvements card-local and deterministic.

## Tooling Notes

- The demo should install skills from the tracked repo path rather than relying on preexisting local skills.
