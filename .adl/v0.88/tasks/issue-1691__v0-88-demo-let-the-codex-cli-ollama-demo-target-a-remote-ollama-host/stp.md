---
issue_card_schema: adl.issue.v1
wp: "unassigned"
queue: "tools"
slug: "v0-88-demo-let-the-codex-cli-ollama-demo-target-a-remote-ollama-host"
title: "[v0.88][demo] Let the Codex CLI + Ollama demo target a remote Ollama host"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:demo"
  - "area:tools"
  - "version:v0.88"
issue_number: 1691
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "demo"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Mirrored from the authored GitHub issue body during bootstrap/init."
pr_start:
  enabled: false
  slug: "v0-88-demo-let-the-codex-cli-ollama-demo-target-a-remote-ollama-host"
---

## Summary

Update the existing Codex CLI + Ollama operational-skills demo so it can target an Ollama service running on a remote machine through explicit host configuration, while keeping the demo's bounded truth model and reviewer flow intact.

## Goal

Make the demo truthfully support a non-local Ollama HTTP endpoint for the current demo wrapper and docs, so operators can point the existing demo at a remote Ollama host without implying that the full ADL runtime already has first-class remote Ollama transport.

## Required Outcome

The current Codex CLI + Ollama demo accepts explicit remote-host configuration, validates it clearly, documents the operator path, and remains bounded to the demo wrapper surface rather than widening into runtime/provider redesign.

## Deliverables

- remote-host support for the existing Codex CLI + Ollama demo wrapper
- explicit validation/error handling for configured remote Ollama endpoints
- updated demo/operator docs explaining local vs remote-host usage truthfully
- bounded tests covering non-default host configuration on the demo path

## Acceptance Criteria

- the existing demo can target a configured remote Ollama HTTP host without requiring local Ollama on the same machine
- the demo docs clearly distinguish demo-layer remote host support from full runtime/provider support
- operator-visible errors are explicit when the remote host is unreachable or misconfigured
- bounded tests cover the remote-host configuration path
- no runtime/provider transport redesign is introduced in this issue

## Repo Inputs

- `adl/tools/demo_codex_ollama_operational_skills.sh`
- `adl/tools/test_demo_codex_ollama_operational_skills.sh`
- `adl/tools/test_demo_codex_ollama_semantic_fallback.sh`
- `demos/v0.87.1/codex_ollama_operational_skills_demo.md`
- `demos/README.md`

## Dependencies

- none

## Demo Expectations

- the updated proof remains the existing Codex CLI + Ollama operational-skills demo
- no new flagship demo surface is required

## Non-goals

- adding first-class remote Ollama transport to the ADL runtime provider layer
- changing `adl/src/provider.rs` from local CLI execution to remote HTTP transport in this issue
- broad provider architecture redesign

## Issue-Graph Notes

- bounded follow-on from the current Ollama demo/operator surface
- the runtime/provider transport follow-on should remain a separate backlog item

## Notes

- prefer explicit host configuration such as `OLLAMA_HOST` / `OLLAMA_HOST_URL` over ambient inference
- keep the truth boundary crisp: demo support first, provider-runtime support later

## Tooling Notes

- validate the configured host through the existing demo HTTP checks where possible
- keep any new environment-surface naming consistent with current demo/operator docs

