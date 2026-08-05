# Structured Intent Prompt

Template: 1.0.0

Issue: 5838

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prove the same versioned birthday multi-agent scenario across at least two real providers without provider-specific success substitution.

## Required Outcome

A redacted exact-revision matrix of real provider runs, equivalent ACIP semantics, negative/provider-loss cases, and visible rejection of fixture, cached, receipt-only, or synthetic substitution.

## Scope

- adl/tools/demo_v092_provider_neutral_birthday.sh
- adl/tools/validate_v092_provider_neutral_proof.py
- adl/tools/test_v092_provider_neutral_proof.sh
- demos/v0.92/provider-neutral-birthday/
- docs/milestones/v0.92/features/PROVIDER_NEUTRAL_MULTI_AGENT_PROOF_v0.92.md
- .csdlc/evidence/5838/

## Authority

- WP-18B owns provider comparison and no-substitution proof
- #5832 owns ACIP protocol/transport
- #5836 owns the birthday scenario
- Provider credentials and private payloads remain outside retained artifacts

## Assumptions

- #5832 supplies the versioned ACIP contract
- #5834 supplies the integrated birthday review packet
- #5836 supplies the runnable scenario

## Operator Constraints

- At least two independently configured real providers must pass
- Provider unavailability is a non-pass and cannot trigger substitution
- Never retain credentials, private prompts, or unredacted provider payloads
- Release the preparation claim before handoff
