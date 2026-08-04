# Structured Intent Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Run the final WP-18 internal review second pass for v0.91.8 on the exact current revision.

## Required Outcome

A tracked review packet records exact-revision issue truth, closed-since-prior-review emphasis, actual code/test/CI/docs/evidence review, deduplicated findings, remediation/routing truth, validation, and publication readiness for #5791.

## Scope

- v0.91.8 final WP-18 internal review second pass
- issues closed since the prior WP-18 review
- actual code, test, CI, docs, lifecycle, and release evidence surfaces

## Authority

- Issue #5791 owns review artifacts and in-scope remediation routing.
- The review may not start v0.92 implementation.
- The root main checkout remains inspection-only.

## Assumptions

- none

## Operator Constraints

- Use FastWork for the issue worktree and build output.
- Do not mutate the root main checkout.
- Do not use AWS.
- Use review skills and typed C-SDLC v2 lifecycle.
- Review actual code, not only docs or issue narrative.
- PR must include Closes #5791.
