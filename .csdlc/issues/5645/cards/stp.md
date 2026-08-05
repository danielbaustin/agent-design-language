# Structured Task Prompt

Template: 1.0.0

Issue: 5645

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and validate the typed csdlc-merge command.

## Deliverables

- csdlc-merge binary
- versioned request/result schema
- focused unit and fixture tests
- design and diagram

## Acceptance

1. Reject stale or non-merge-ready requests without a merge attempt
2. Pass expected head SHA to GitHub
3. Return exact merge commit SHA
4. Classify permission failures without leaking tokens

## Dependencies

- existing publication/readiness contracts
- existing GitHub token resolver and Octocrab client

## Inputs

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/src/store.rs

## Non Goals

- automatic merge on publish
- changing branch protection
- combining merge and closeout
