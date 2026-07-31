# Structured Review Prompt

Template: 1.0.0

Issue: 5678

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/tooling/OPUS_REVIEW_RUNBOOK.md
adl/tools/test_opus_review_runbook.sh

## Prompts

- Does the runbook invoke the actual JSON CLI?
- Can an operator verify provider/model identity and exact revision without exposing secrets?
- Does the contract check fail when the CLI interface drifts?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The ignored operator-local .adl/docs/TBD mirror remains outside the tracked canonical runbook and must be refreshed separately when an operator needs that local convenience path.

## Review Result

Revision: Some("git-blake3:df6c1ba075d262917aa71cea2f40ecfe71abced1:150c1020942ff8c3fe690eebbcf51f95dc7e6a65db7dcdc19f86fef79ddcd3b1")

Reviewer: Some("codex-subagent:review-5678-runbook")

Result: pass
