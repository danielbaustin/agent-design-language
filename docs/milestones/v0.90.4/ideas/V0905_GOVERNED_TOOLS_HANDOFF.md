# v0.90.5 Governed Tools Handoff

v0.90.4 may discover that contracts, bids, delegation, and review summaries need
tool-mediated work. That is expected. It should not cause v0.90.4 to absorb the
governed-tools milestone.

## Boundary

v0.90.4 owns the contract-market question:

- what work was requested
- who may bid
- why a bid was selected
- what resources or tool-mediated work the bid expects
- who may accept, delegate, execute, dispute, complete, or cancel
- what evidence proves the contract lifecycle

v0.90.5 owns the governed-tool question:

- what portable tool shape is valid
- which capability contract applies
- who may call the tool
- who may see arguments, results, and traces
- what Freedom Gate mediation is required
- how denial is recorded
- how replay and redaction work
- how model proposals are normalized and rejected

## v0.90.4 Recording Rule

When a contract or bid requires a tool, v0.90.4 should record that requirement as
a constraint, resource estimate, adapter expectation, or evidence requirement.

It should not treat any of the following as permission to execute:

- valid JSON
- model confidence
- a named adapter
- a counterparty promise
- a parent contract's broad scope
- a human out-of-band request

## Handoff Evidence

WP-10, WP-14, WP-14A, and WP-19 should preserve enough evidence for v0.90.5 to
pick up the tool lane cleanly:

- example contract fields that mention tool needs
- negative cases where tool execution is denied or deferred
- resource estimates for tool-mediated work
- review-summary language that distinguishes contract proof from tool proof
- backlog notes for any tool requirement that blocked contract execution

## Non-Claims

This handoff does not implement UTS, ACC, the tool registry, the governed
executor, redaction, replay, or model testing. It exists so citizen economics can
make tool needs legible without weakening the tool-governance boundary.
