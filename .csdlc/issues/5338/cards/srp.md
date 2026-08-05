# Structured Review Prompt

Template: 1.0.0

Issue: 5338

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl-v2/crates/adl-compiler
.csdlc/issues/5338
.csdlc/prepared/issues/5338
.csdlc/evidence/5338

## Prompts

- Does the compiler consume only the landed typed #5339 document and avoid duplicating parser or language-policy authority?
- Are reference resolution, composition and pattern expansion, bounds, graph lowering, and diagnostic ordering completely specified and deterministic?
- Is the stable node identity preimage versioned, domain-separated, length-delimited, meaning-complete, and free of traversal or machine state?
- Can equivalent input syntax, nested metadata order, collection permutation, hash collision, or clean-process replay change plan bytes or diagnostics?
- Is ExecutionPlan inert data with a clean WP-06 boundary and no hidden scheduling, retry, runtime, provider, IO, or lifecycle authority?
- Are every #5339 fixture mapping, COTS choice, protected path, source/test budget, and time budget explicit, executable, and fail closed?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Legacy pattern fixtures remain intentionally outside the compiler input model because the landed language boundary rejects them; this boundary is explicitly classified and tested.

## Review Result

Revision: Some("git-blake3:44e8470ce3fcf0396885c74c0dd0347279449ecd:a8a7ef14f7dd8856a0c09e3a089cd3994d2cc9f943197fe326ad5e9bfacfe466")

Reviewer: Some("subagent:/root/review_5338_exact")

Result: pass
