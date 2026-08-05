# Structured Task Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement, validate, review, publish, merge, and close the bounded Rust GitHub action surface.

## Deliverables

- csdlc-github Rust binary
- typed request/result schemas
- reconciled issue mutations
- focused tests and operator contract

## Acceptance

1. AC-1: issue create is idempotent and reconciles exact repository/title/body identity using a durable operation key
2. AC-2: issue update, labels, assignees, comments, and close are typed, validated, and reconciled
3. AC-3: permission, identity, stale-request, ambiguous-outcome, and mismatch failures are fail-closed
4. AC-4: focused Rust tests cover every mutation and no external connector is required

## Dependencies

- existing csdlc-v2 publication, merge, closeout, and token resolver code
- GitHub issue #5655

## Inputs

- csdlc-v2/src/github.rs
- csdlc-v2/src/github_token.rs
- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/bin/csdlc-merge.rs
- csdlc-v2/src/bin/csdlc-closeout.rs

## Non Goals

- Runtime or AWS work
- GitHub connector integration
- legacy wrapper restoration
- changes to closed issue records
