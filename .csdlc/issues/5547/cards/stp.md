# Structured Task Prompt

Template: 1.0.0

Issue: 5547

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Decide and record the C-SDLC revision identity contract, implement or route the identity residual, and produce an ownership-first split plan for the named large modules without starting broad refactors.

## Deliverables

- C-SDLC review identity disposition for IR-4645-011.
- Focused implementation and tests if the chosen identity contract changes code, or exact v0.91.8 residual routing if it is deferred.
- Ownership-first split plan for large modules from IR-4645-012.
- Truthful validation and residual-risk record.

## Acceptance

1. AC1: Decide whether C-SDLC review identity honors scope pathspecs or explicitly advertises whole-tree revision identity.
2. AC2: Implement the chosen C-SDLC identity contract or record operator-approved v0.91.8-bound residual with exact issue routing.
3. AC3: Produce ownership-first split plan for the large modules called out by #4645, without hiding behavior changes inside cosmetic moves.
4. AC4: Record which parts, if any, are safe to defer until v0.91.8 before WP-19 begins.

## Dependencies

- Issue #4645 retained internal review evidence.
- Issue #4647 parent remediation routing.
- PR #5543 retained review context.
- Current v0.91.7 C-SDLC v2 source and docs.

## Inputs

- docs/reviews/v0.91.7/internal-review-4645/FINDINGS_REGISTER.md from the retained #4645 worktree.
- csdlc-v2/src/git.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/store.rs
- adl/src/long_lived_agent.rs
- adl/src/csm_runtime_api.rs
- adl/src/scheduler.rs
- adl/src/provider_adapter.rs

## Non Goals

- Do not perform broad module refactors unless explicitly widened.
- Do not claim residuals fixed if they are deferred.
- Do not start v0.91.8 implementation work.
- Do not use sunset v1 C-SDLC lifecycle wrappers.
