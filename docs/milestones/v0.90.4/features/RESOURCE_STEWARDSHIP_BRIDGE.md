# Resource Stewardship Bridge - v0.90.4

## Purpose

Connect contract-market execution to compute, memory, attention, bandwidth, and
tool-resource requirements without pretending v0.90.4 ships full economics,
payments, or governed tool execution.

## Resource Types

The bridge should track:

- compute
- memory
- attention
- bandwidth
- artifact storage
- review/operator time where relevant
- tool adapter budget or tool-mediated work estimate, where relevant

## Bridge Rule

Contracts may request resources and bids may estimate resource use. The market
runner may validate those claims as fixture data. No real payment settlement is
required.

Tool-resource estimates are not tool-call grants. A contract may say that work
expects a search, repository inspection, diagram render, local model call, or
other tool-mediated step, but v0.90.4 should record that as a resource and
evidence requirement. The actual authority to call tools belongs to the
governed-tools layer.

## Governance Boundary

Resource allocation remains policy-bound. Contracts cannot override standing,
access control, private-state protection, quarantine, sanctuary, or challenge
rules.

Contracts also cannot override future ACC authority, visibility, redaction, or
denial rules for tools. If a requested resource would require unsafe tool
execution, the correct v0.90.4 result is a fixture-backed deferral or denial,
not a bypass.

## Later Work

Reputation, pricing, payment settlement, inter-polis economics, and production
market behavior should be handed off after the contract-market proof is stable.
