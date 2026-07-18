# Structured Review Prompt

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/test_ci_runtime_contracts.sh

## Prompts

- Are all nextest install steps updated?
- Does every step fail closed instead of falling back?
- Does the static contract detect partial or future drift?
- Is the hosted warning genuinely absent?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:441632856110b1d7c1e115d640d07b9c7c092185:873cf72ac653f14e6049437bf06275fe7219cd5a8998e2ac8d9314e33dcad9fe")

Reviewer: Some("bounded-subagent-review-5464")

Result: pass
