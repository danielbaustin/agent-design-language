# Structured Intent Prompt

Template: 1.0.0

Issue: 5762

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the C-SDLC v2 terminal SOR validation repair tests independent of mutable tracked issue active-claim state.

## Required Outcome

The three terminal SOR validation repair tests synthesize deterministic issue-local repair authority in their temporary repository and pass when the terminal target is closed_out and claim-free.

## Scope

- csdlc-v2/src/store.rs test fixture construction
- issue-local lifecycle and evidence for #5762

## Authority

- typed C-SDLC v2 binaries own lifecycle state
- no production lifecycle semantic change
- no dependency on /private/tmp
- no dependency on #5613 retaining an active claim

## Assumptions

- none

## Operator Constraints

- work only in /Volumes/FastWork/adl-wp-5762
- do not write tracked files on primary main
- publish a ready PR with Closes #5762
- do not block on post-merge typed closeout
