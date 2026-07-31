# Structured Review Prompt

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/src/bin/csdlc-bind.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/lib.rs
csdlc-v2/tests/gate2.rs
.csdlc/issues/5648

## Prompts

- Check operator authority and CAS boundaries
- Check phase/protected-path truth
- Check no direct state or secret leakage

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The operator authority marker is validated as explicit non-empty provenance; its external authorization remains an operator responsibility.

## Review Result

Revision: Some("git-blake3:da05119e80b0cb0b6f768cf935ec9e0e10d7055e:a6ca8d1765314aed2615d3b45135cd78a4fdb9006f2ee1b311bdea0dd2b09879")

Reviewer: Some("bounded-subagent-review-5648")

Result: pass
