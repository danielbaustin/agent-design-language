# Structured Review Prompt

Template: 1.0.0

Issue: 5342

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl-v2/crates/adl-records
.csdlc/issues/5342
.csdlc/prepared/issues/5342

## Prompts

- Can two semantically different records produce the same canonical bytes, digest, or signed preimage?
- Can an envelope select or modify the trust policy, key permissions, profile, kind, validity, or revocation decision that authorizes it?
- Does every malformed, tampered, oversized, replayed, unknown-field, duplicate-key, and wrong-key/profile/kind case fail closed?
- Are channel and fresh-process proofs genuinely independent of in-process object identity and implicit host state?
- Are all cryptographic operations delegated to reviewed COTS and all product inputs explicitly bounded?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:5f48b0854cd36f64bef38ed0d09a09d84fa2e158:87461a25829d37c8b92b27c8053e46fae3f92112b490d7761a66e8395eea4c25")

Reviewer: Some("subagent:019f8a9c-c932-70d1-a78d-e1fd5ae66b18")

Result: pass
