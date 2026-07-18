# Structured Task Prompt

Template: 1.0.0

Issue: 5495

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Classify typed publication metadata commits and preserve fail-closed substantive drift behavior.

## Deliverables

- Bounded metadata path classifier
- Automatic metadata-only review proof
- Regression tests for publication metadata and substantive drift

## Acceptance

1. Recognized lifecycle metadata commits do not stale a clean review
2. Substantive changes still produce review_stale
3. Malformed explicit proof remains fail closed
4. Merged reconciliation remains identity protected

## Dependencies

- Review evidence and publication guard
- Git revision and changed-path helpers

## Inputs

- csdlc-v2/src/git.rs
- csdlc-v2/src/review.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/tests/gate5.rs

## Non Goals

- Changing substantive review policy
- Broadly allowing arbitrary .csdlc files
- AWS or remote infrastructure work
