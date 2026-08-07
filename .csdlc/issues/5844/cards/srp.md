# Structured Review Prompt

Template: 1.0.0

Issue: 5844

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.92/publication/articles
.csdlc/evidence/5844/validate-article-series.rb
.csdlc/evidence/5844/rollback-manifest.json
.csdlc/evidence/5844/ROLLBACK_PROCEDURE.md
.csdlc/evidence/5844/claude-final-01-05-result.json
.csdlc/evidence/5844/claude-final-06-10-result.json
.csdlc/evidence/5844/gemini-exact-closing-result.json
.csdlc/evidence/5844/gemini-exact-provider-invocations.json

## Prompts

- Are all ten artifacts complete articles with bounded source packets rather than outlines?
- Is every material claim and citation supportable without exposing private information?
- Does the series remain coherent and avoid repeating the same argument under different titles?
- Are #5843-dependent claims and publication status explicitly gated?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Before any external publication, recheck #5843 release truth, UTS/ACC versions, CodeFriend status, cryptographic wording, CAV boundaries, and economic/payment posture.
- Gemini supplied no upstream request IDs; retained exact prompts, complete results, and matching invocation metadata provide the bounded association evidence.

## Review Result

Revision: Some("git-blake3:10c004c3150e1eb3fd2a7ddf1108a109631a8ecb:7c48d57a75d3fed8be83962234f42f636fbc81dd7b4590d01bdb5d94135565c3")

Reviewer: Some("codex-subagent:Nash")

Result: pass
