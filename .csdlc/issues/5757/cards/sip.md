# Structured Intent Prompt

Template: 1.0.0

Issue: 5757

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Correct the trusted-origin, monotonic generation, and shared-certificate proof defects from #5722 while preserving the approved Observatory design.

## Required Outcome

A ready PR for #5757 with focused UI/runtime/TLS proof and `Closes #5757` publication.

## Scope

- HTML Observatory client origin validation before bearer/WSS use
- Monotonic generation ordering across live, retained, WSS, and fallback completions
- Shared localhost certificate/browser-control/authenticated-WSS proof across ports 8765 and 20997

## Authority

- Use only typed C-SDLC v2 lifecycle binaries for lifecycle operations
- Never write tracked files on primary main
- Do not inspect, edit, claim, or depend on #5748 or its worktree
- Do not use /private/tmp
- Preserve the approved Observatory visual design

## Assumptions

- none

## Operator Constraints

- All request and evidence artifacts stay inside /Volumes/FastWork/adl-wp-5757
- Publish a ready PR and do not block on asynchronous closeout
