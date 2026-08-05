# Structured Task Prompt

Template: 1.0.0

Issue: 5762

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the terminal SOR validation fixture authority setup and any directly necessary issue-local records.

## Deliverables

- deterministic temporary repair authority in store.rs tests
- three focused terminal SOR validation repair tests passing together
- full locked all-target csdlc-v2 tests passing
- strict Clippy and diff hygiene evidence
- ready PR closing #5762

## Acceptance

1. AC-1: No terminal SOR validation repair test borrows active-claim truth from #5613 or another mutable tracked issue projection.
2. AC-2: The three terminal SOR validation repair tests pass together.
3. AC-3: The full locked all-target C-SDLC v2 test suite passes.
4. AC-4: Strict Clippy and git diff hygiene pass.
5. AC-5: A bounded gpt-5.5 review finds no actionable issue before publication.
6. AC-6: The PR body includes Closes #5762.

## Dependencies

- current origin/main at 57d115741f32b945217ee3cb14188b41ebde9b3f

## Inputs

- GitHub issue #5762
- csdlc-v2/src/store.rs

## Non Goals

- production lifecycle semantic change
- terminal repair control-plane redesign
- closeout blocking
- work outside the issue worktree
