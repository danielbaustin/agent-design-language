# Issue 5695 design

## Intent

Correct `csdlc-pr-state` mergeability classification so GitHub's `behind`
state means stale base ancestry only. `blocked` and `unstable` remain pending
merge-policy/check states, while `dirty` remains conflicted and `unknown`
remains waiting.

## Scope

- `csdlc-v2/src/github.rs` mergeability normalization and classification
- focused GitHub-state classification tests
- issue-local typed cards, evidence, and design/diagram records

## Non-goals

- no provider, Runtime, AWS, or CI workflow changes
- no changes to GitHub merge authority or required-check policy
- no reopening or implementation changes for #5683

## Acceptance

1. Every supported mergeability variant has an explicit classification.
2. Pending `blocked` and `unstable` states cannot be reported as `stale_base`.
3. A truly `behind` PR remains `stale_base`; `dirty` remains conflicted.
4. Focused tests prove the table and `csdlc-merge` remains fail-closed until
   clean ancestry and required checks are observed.

## Validation posture

Use one focused deterministic C-SDLC v2 GitHub-state test lane plus formatting
and strict Clippy for the touched crate. Hosted CI remains separate proof.
