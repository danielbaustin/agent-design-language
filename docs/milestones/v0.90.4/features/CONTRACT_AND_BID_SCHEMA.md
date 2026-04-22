# Contract And Bid Schema - v0.90.4

## Purpose

Define the two primary market artifacts: contract and bid.

Contracts describe bounded work. Bids describe a qualified actor's proposed way
to satisfy that work. Neither artifact grants citizen standing by itself.

## Contract Requirements

A contract should include:

- contract id and version
- contract type
- lifecycle state
- parties
- scope, inputs, and deliverables
- process rules
- constraints
- tool or adapter requirements, if the work expects tool-mediated execution
- timeline
- evaluation criteria
- artifact references
- identity and trust requirements
- trace requirements
- extension slot for later economics

## Bid Requirements

A bid should include:

- bid id and version
- target contract id
- bidding agent or counterparty
- proposal
- cost or resource claim
- expected tool or adapter usage, expressed as a requirement rather than an
  execution grant
- confidence
- commitments
- exceptions
- trace and signature requirements
- extension slot for later pricing or payment rails

## Validation Rules

Minimum validation should reject:

- missing required identifiers
- bid against the wrong contract
- bid after bidding closes
- bid from ineligible counterparty
- contract without trace requirements
- bid without commitments or exceptions
- artifact references without reviewable paths
- tool requirement that implies direct execution authority

## Non-Claims

These schemas do not settle payment, legal status, tax, or production identity.
They create reviewable ADL artifacts for bounded market execution.

These schemas also do not authorize tool calls. Tool requirements are contract
constraints and review evidence only until the governed-tools layer supplies
UTS, ACC, registry binding, executor authority, denial records, redaction, and
replay semantics.
