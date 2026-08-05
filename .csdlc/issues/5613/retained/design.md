# Issue 5613 design

Status: preparation review findings remediated; awaiting typed approval.

## Objective

Add one narrow typed C-SDLC v2 operation that repairs exactly one validation
result in a terminal SOR without reopening the target issue. The operation must
preserve the target's original merged pull-request identity, terminal
disposition, claim-free state, and initialization identity while atomically
regenerating cards, projections, audit truth, record digest, and retained
terminal receipt.

The same corrective issue then materializes the already-proven terminal
projections for issues 5337, 5339, and 5591 from their exact terminal commits.
It uses the new operation to replace machine-local validation entries in issue
5591 with truthful portable entries and omits the unsupported, unbound
`guardian-soak.json` evidence file.

## Contract

The typed request identifies:

- a distinct active authority issue and claim;
- one closed-out, claim-free target issue;
- exact authority record generation and digest;
- exact target record generation and digest;
- exact retained receipt digest;
- one complete expected old validation result;
- one complete replacement validation result;
- an actor and reason;
- an optional transaction failpoint used only by tests.

The operation fails closed when authority is absent, expired, stale, or does
not protect the target issue directory; when target or receipt CAS is stale;
when the target is not closed-out and claim-free; when the old result has zero
or multiple exact matches; when the replacement is malformed or
machine-local; or when the terminal transaction cannot update both projection
and receipt. A failed transaction restores the prior record, cards, audit, and
receipt bytes.

## Portability

Validation commands and evidence references retained in a terminal SOR must be
repository-relative or stable symbolic descriptions. They may not contain
machine-local build roots such as `/Volumes/FastWork`, `/private/tmp`, home
directories, or platform-specific checkout prefixes. Replacement validation
results remain truthful: portable wording records what ran without inventing a
new command, outcome, or proof claim.

## Terminal projection materialization

The corrective branch consumes terminal state only from these exact commits
and their existing retained shared-Git receipts:

- issue 5337: `461713dc10d26fa5336a054c07ef1844f804ec8f`;
- issue 5339: `817126889942fc57820bf9f05f5cc40e2debd683` from
  `origin/codex/5339-v0918-wp04-language-core`;
- issue 5591: `8cfb7b25ad246dd411a57ecc4fda8e47665912fc` from
  `origin/codex/5591-runtime-v3-parity-a-preparation`.

Materialization uses Git commit ancestry/cherry-pick mechanics and typed repair;
terminal records are never hand-copied. Original receipts are preserved except
for the typed, atomic #5591 validation repair, which regenerates that receipt
under exact CAS.

## COTS and dependencies

No new dependency is allowed. The implementation reuses the existing
`serde`, `serde_json`, and `schemars` contracts plus the repository's current
transaction journal, digest, card renderer, terminal receipt validator, and
filesystem primitives. Adding a crate is a stop condition.

## Budgets

- implementation: at most 800 new or changed non-test Rust lines;
- focused tests and fixtures: at most 1,000 new or changed lines;
- focused test lane: 300 seconds;
- strict all-target Clippy: 600 seconds;
- complete C-SDLC v2 test lane: 600 seconds;
- fresh-checkout projection proof: 180 seconds.

Exceeding a budget requires review and explicit issue-local evidence; it does
not silently widen authority.

## Non-goals

- no Runtime source or Runtime v2 changes;
- no ADL-v2 product changes;
- no AWS or provider execution;
- no reopening of issues 5337, 5339, or 5591;
- no replacement of original PR identities or terminal dispositions;
- no generic terminal-card editor or broad lifecycle rewrite.
