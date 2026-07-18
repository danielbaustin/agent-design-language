# Structured Review Prompt

Template: 1.0.0

Issue: 5495

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/git.rs
csdlc-v2/src/review.rs
csdlc-v2/src/doctor.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/5495/retained/design.md
.csdlc/issues/5495/retained/diagram.mmd

## Prompts

- Can a source or retained-design change bypass review?
- Are all normal typed publication metadata surfaces covered without allowing arbitrary files?
- Does merged reconciliation still require exact identity and final reviewed intent?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Typed request manifests are treated as lifecycle metadata only when they use a numeric issue prefix and JSON suffix.

## Review Result

Revision: Some("git-blake3:1ef7bb7eebcc0c6421d1e02c319ca4b45614149c:a2c283b0d7ecc77ffcd29fa9bebc43a59e99086c80b3df20a90ef6ef2664824b")

Reviewer: Some("review_5495")

Result: pass
