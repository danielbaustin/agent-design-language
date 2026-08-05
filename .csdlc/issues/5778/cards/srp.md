# Structured Review Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.

## Prompts

- Does finish preserve exact reviewed-head, required-check, publication identity, and expected-SHA merge guarantees?
- Can any interruption or concurrent call create conflicting terminal results or require a second PR?
- Does terminal claim release avoid weakening active nonterminal collision safety?
- Are legacy records readable without becoming competing current authority?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The repaired exact head still requires GitHub Actions integration proof before merge.

## Review Result

Revision: Some("git-blake3:a13bac366e2e9fe228b13ae058543ca3c3cd61fe:f74554d056eb0dcf1de41c5b3b64ac58b912b6bcee9254f2a97a3159da195dfd")

Reviewer: Some("codex-subagent:review_5778_exact_head")

Result: pass
