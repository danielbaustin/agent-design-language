# Resource Stewardship Bridge - v0.90.4

## Purpose

Connect contract-market execution to compute, memory, attention, and bandwidth
without pretending v0.90.4 ships full economics or payments.

## Resource Types

The bridge should track:

- compute
- memory
- attention
- bandwidth
- artifact storage
- review/operator time where relevant

## Bridge Rule

Contracts may request resources and bids may estimate resource use. The market
runner may validate those claims as fixture data. No real payment settlement is
required.

## Governance Boundary

Resource allocation remains policy-bound. Contracts cannot override standing,
access control, private-state protection, quarantine, sanctuary, or challenge
rules.

## Later Work

Reputation, pricing, payment settlement, inter-polis economics, and production
market behavior should be handed off after the contract-market proof is stable.
