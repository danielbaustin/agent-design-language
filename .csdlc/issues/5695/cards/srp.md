# Structured Review Prompt

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/github.rs
csdlc-v2/src/merge.rs

## Prompts

- Does every supported octocrab MergeableState map explicitly?
- Can blocked or unstable ever become stale_base?
- Does csdlc-merge remain fail-closed while checks or ancestry are pending?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:ac0df9138d3b80d48d8480bf7761bd581da787bb:91b31c0fb64a9065135b6e8b340b714e18a66bea661a18a31bbcb8cdc956406c")

Reviewer: Some("codex-subagent:review-5695")

Result: pass
