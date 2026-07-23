# Structured Review Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_cargo_validation.sh
adl/tools/test_run_cargo_validation.sh
csdlc-v2/src/operator.rs
csdlc-v2/src/proof.rs
csdlc-v2/src/readiness.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate7.rs
csdlc-v2/tests/gate7_lifecycle.rs
.csdlc/issues/5624
.csdlc/prepared/issues/5624
.csdlc/evidence/5624

## Prompts

- Can `.` validate any checkout other than the exact current terminal branch worktree?
- Can two repositories with the same relative worktree suffix collide?
- Do malformed, missing, wrong, and dirty candidates fail with unsafe_checkout?
- Can validation alter the terminal record or retained receipt?
- Does command-level proof exercise the same path as issue 5340?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:0794c7282bcfe1d39fa21baa4b9b4a71d17890c6:9198f9170b47dce50a1a1054266f3c0d26aba7d7fad3e93cc6e999e0f9492ec1")

Reviewer: Some("subagent:019f8ab8-7865-7263-832c-2922bae1b9d3")

Result: pass
