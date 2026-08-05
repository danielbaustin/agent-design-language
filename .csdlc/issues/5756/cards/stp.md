# Structured Task Prompt

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement the smallest provider-aware fix for the global non-2xx bare substring 1008 bug and add positive MiniMax plus negative cross-provider tests.

## Deliverables

- provider-aware classification fix
- MiniMax positive regression
- OpenAI, Anthropic, DeepSeek, and generic negative regressions
- focused provider tests and strict Clippy evidence
- ready PR closing #5756

## Acceptance

1. AC-1: Only a validated MiniMax response can produce MiniMax insufficient-balance classification.
2. AC-2: OpenAI, Anthropic, DeepSeek, and generic provider errors containing 1008 retain their provider-specific retry/classification behavior.
3. AC-3: Focused provider-adapter tests and strict Clippy pass.
4. AC-4: The PR body includes Closes #5756.
5. AC-5: No AWS use and no /private/tmp; all work remains in the issue worktree.

## Dependencies

- current origin/main at 85f0aa3d1f6b442acb61ada97fb3a5a73b50a444

## Inputs

- GitHub issue #5756
- adl/src/provider_adapter.rs
- .csdlc/designs/5756/design.md
- .csdlc/designs/5756/diagram.mmd

## Non Goals

- provider adapter refactor
- AWS proof
- issue #5748 inspection
- issue #5757 Observatory work
- merge or asynchronous closeout
