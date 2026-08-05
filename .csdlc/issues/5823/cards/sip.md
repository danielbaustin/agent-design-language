# Structured Intent Prompt

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-06: Remote validation/build runner.

## Required Outcome

portable bounded runner with provenance and failover

## Scope

- Provider-neutral typed validation request, result, provenance, artifact, timeout, cancellation, cleanup, and fallback contracts
- Existing provider adapters in tools/aws_remote_validation, adl/tools/run_aws_spot_remote_validation_lane.sh, and adl/tools/run_nessus_remote_validation.sh
- Local no-network execution of the same declared command profile
- Linux remote, local macOS, and Windows path/quoting or approved live-runner proof
- Issue-local portable fixtures and redacted evidence under .csdlc/evidence/5823

## Authority

- Issue 5823 owns the provider-neutral runner contract and bounded adapter integration
- Existing AWS and Nessus tools remain provider-specific adapters
- Local validation remains authoritative and usable without network availability
- AWS runs require explicit authorization and the agent-logic-admin business profile
- No credential values or arbitrary secret payloads enter requests or evidence

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
