# Structured Review Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/github.rs
.csdlc/issues/5655/index.json
.csdlc/issues/5655/cards/sor.md
.csdlc/issues/5655/cards/srp.md

## Prompts

- Does one Rust command surface cover every declared issue mutation without connector or wrapper fallback?
- Are ambiguous remote outcomes reconciled before retry or local state mutation?
- Are repository, issue, operation key, labels, assignees, comments, and close identity checked exactly?
- Are tokens bounded and never emitted?
- Do tests prove failure behavior rather than only happy paths?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was read-only and did not rerun tests; validation evidence is the implementation owner strict Clippy, focused gate, and full suite run recorded in SOR.

## Review Result

Revision: Some("git-blake3:1eb3e59819222fe59a590abbbc3b18bbe19643d1:3e0ffb11a77571d35b2c1d0a65dde2b0ecba9be381994fcdd52ef0df40434435")

Reviewer: Some("Boole")

Result: pass
