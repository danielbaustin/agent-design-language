---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend"
title: "[v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend"
labels:
  - "track:roadmap"
  - "area:tools"
  - "area:quality"
  - "type:feature"
  - "version:v0.91.5"
issue_number: 3562
generated_at: "2026-06-04T17:59:30Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.91.5"
required_outcome_type:
  - "code"
repo_inputs:
  - ".adl/v0.91.5/bodies/issue-3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Versioned C-SDLC prompt template applied; source issue prompt remains the design-time intent source."
pr_start:
  enabled: true
  slug: "v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend"
---

Canonical Template Source: `docs/templates/prompts/1.0.0/stp.md`
Generated: 2026-06-04T17:59:30Z

# Structured Task Prompt

## Summary

Add a first-class external `review_provider` lane so ADL and CodeFriend can ask hosted or local model providers to perform bounded review work with typed inputs, typed outputs, provenance, logs, and model identity.

## Goal

Make external reviewer models such as Claude, Gemini, OpenAI, Ollama, and DeepSeek usable as repeatable review providers rather than ad hoc chat sessions. This should reuse the provider communication substrate and should be scheduled after the native DeepSeek provider work in `#3549`.

## Required Outcome

ADL has a documented and implementable `ReviewProviderV1` path that can run a bounded review packet through a configured model provider, capture the result as review evidence, and preserve enough provenance for CodeFriend-style synthesis.

## Deliverables

`ReviewProviderV1` design and contract covering request, provider route, result, run record, logs, and provenance.; A CLI or tool entrypoint proposal for running one bounded review lane from an issue, diff, file set, and rubric.; Provider sequencing plan that depends on `#3549` for native DeepSeek support and reuses existing hosted/local provider paths where possible.; CodeFriend integration notes describing how review-provider outputs become findings packets rather than authority claims.; Follow-on implementation slices if the issue proves too large for one PR.

## Acceptance Criteria

The issue is explicitly sequenced after `#3549`.; The contract distinguishes native Codex subagents from external provider-backed reviewer agents.; The design supports Claude, Gemini, OpenAI, Ollama, and DeepSeek without hard-coding one provider as special.; Outputs include model identity, timestamps, elapsed time, request identity, log location, redaction status, and review-result status.; The design includes fail-closed behavior for provider errors, empty outputs, timeout, auth failure, and malformed findings.; CodeFriend can consume the result as a review artifact with findings-first synthesis.

## Repo Inputs

`#3549` native DeepSeek API provider issue; Existing provider communication substrate; Existing podcast/multi-agent provider setup; Existing CodeFriend review packet expectations; Existing ADL review skills and internal-review artifact shapes

## Dependencies

Depends on `#3549` before implementation work that requires DeepSeek provider coverage.; Can start with design before `#3549` closes, but execution should not claim full provider coverage until DeepSeek is available.

## Target Files / Surfaces

`#3549` native DeepSeek API provider issue; Existing provider communication substrate; Existing podcast/multi-agent provider setup; Existing CodeFriend review packet expectations; Existing ADL review skills and internal-review artifact shapes

## Validation Plan

Use the normal ADL issue workflow. Keep credentials external, redact logs by default, and capture model identity using the emerging model identity contract.

## Demo Expectations

No user-facing demo is required for the first slice. A later proof can run one small review packet through one hosted provider and one local/Ollama provider.

## Non-goals

Do not replace native Codex subagents.; Do not make external provider output authoritative without synthesis/review.; Do not require all providers to be implemented in the first slice.; Do not expose credentials or raw provider logs in tracked artifacts.

## Issue-Graph Notes

Schedule after `#3549` for provider execution coverage.; This is expected to become necessary infrastructure for CodeFriend.; If the implementation grows, split into design, CLI, provider adapters, and CodeFriend ingestion issues.

## Notes

The target concept is an external reviewer lane: `ReviewRequestV1`, `ReviewProviderV1`, `ReviewResultV1`, and `ReviewRunRecordV1`. The lane should be boring, tail-friendly, provenance-rich, and safe to run repeatedly.

## Tooling Notes

Generated from docs/templates/prompts/1.0.0/.
