# Structured Intent Prompt

Template: 1.0.0

Issue: 5495

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Permit approved typed lifecycle metadata commits after review without weakening substantive review guards.

## Required Outcome

A reviewed publication remains publishable after a commit containing only recognized C-SDLC lifecycle metadata, while source changes still require re-review.

## Scope

- csdlc-v2/src/git.rs
- csdlc-v2/src/review.rs
- csdlc-v2/tests/gate5.rs

## Authority

- Only bounded C-SDLC metadata paths qualify for automatic non-substantive proof
- Substantive repository paths remain review-authority controlled
- Merged-publication reconciliation retains exact remote identity checks

## Assumptions

- none

## Operator Constraints

- none
