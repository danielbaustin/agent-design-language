# Contract Market Demo And Runner - v0.90.4

## Purpose

Define the first bounded proof that the CSM can run a reviewable contract
market.

## Demo Story

One issuer publishes a parent contract. Two bidders submit bids. An evaluator
selects one bid. The awarded party accepts, begins execution, delegates one
bounded subtask, receives delegated output, integrates it, completes the parent
contract, and emits a reviewer-facing summary.

## Required Artifacts

- parent contract
- bid A
- bid B
- evaluation artifact
- award transition event
- acceptance transition event
- subcontract
- delegated output
- parent integration output
- completion event
- trace bundle
- review summary
- demo manifest

## Runner Contract

The runner should:

- load the fixture set
- validate parent contract
- validate bids
- validate that any tool requirements are constraints and not execution grants
- validate evaluation
- emit award transition
- validate acceptance
- validate subcontract
- validate delegated output
- integrate parent output
- emit completion
- assemble review bundle

## Proof Boundary

The runner proves bounded artifact integrity and lifecycle authority. It does
not prove production scheduling, payment settlement, legal contract enforcement,
autonomous market optimization, or governed tool execution.

If the fixture mentions tool-mediated work, the runner should emit review
evidence that the requirement was recognized and kept inside the contract
boundary. It should not run the tool unless a later milestone supplies governed
tool authority.
