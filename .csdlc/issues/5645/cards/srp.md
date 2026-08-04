# Structured Review Prompt

Template: 1.0.0

Issue: 5645

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

csdlc-v2/src/merge.rs
csdlc-v2/src/bin/csdlc-merge.rs
csdlc-v2/src/schema.rs
csdlc-v2/Cargo.toml

## Prompts

- Check exact-head and readiness enforcement
- Check no token leakage
- Check publish/merge separation

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Canonical gate matrix and mocked Octocrab request tests remain future hardening; the exact-head merge gate and focused contract proofs are present.

## Review Result

Revision: Some("git-blake3:dd1972aa2c544a3f8ad51dde2e7611fcc0c5fc26:eb9f2645132e816e0aa8d051667cf87826d3b15dbe215881fcfbe3e0def27125")

Reviewer: Some("subagent:review-5632")

Result: pass
