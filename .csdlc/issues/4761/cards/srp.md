# Structured Review Prompt

Template: 1.0.0

Issue: 4761

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/4761/capability-envelope
docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md

## Prompts

- Later review should verify that every capability claim maps to retained evidence and unsupported claims are explicit.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The envelope is a pre-v0.92 consumed input only; downstream birthday/runtime work must still emit its own retained proof before claiming execution completion.
- C-SDLC metadata under .csdlc is lifecycle evidence and is excluded from the v2 substantive Git revision hash by design; it was inspected as part of this bounded review.

## Review Result

Revision: Some("git-blake3:e93b36cc996429783f68ac4bed5365fb30fe3e07:7521d72c060102bcbba651260a2f3ff9eaf83c2e28cb33aa2fc77c3943ba56cc")

Reviewer: Some("codex-review:4761-capability-envelope")

Result: pass
