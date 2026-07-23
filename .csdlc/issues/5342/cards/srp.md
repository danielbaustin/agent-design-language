# Structured Review Prompt

Template: 1.0.0

Issue: 5342

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

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

Revision: Some("git-blake3:7fbc7200674250cda3e6a743bb5f3f0c77a4a855:a04296b217516695393d6b9be61a29330e98b8fc59167f9effa4cde1fbd8fb67")

Reviewer: Some("subagent:019f8a9c-c932-70d1-a78d-e1fd5ae66b18")

Result: pass
