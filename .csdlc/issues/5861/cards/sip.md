# Structured Intent Prompt

Template: 1.0.0

Issue: 5861

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make issue creation, semantic preparation, readiness sealing, and execution binding simple, truthful, and recoverable without hidden claim identity coordination.

## Required Outcome

Operators can create a visibly draft issue, prepare and edit six typed cards claim-free, seal exact semantic readiness, and bind through one recoverable derived-identity command.

## Scope

- csdlc-v2 issue creation and initialization contracts
- csdlc-v2 prepared generation and readiness receipt storage
- csdlc-v2 bind, release, doctor, migration, and batch preparation behavior
- focused Rust tests and operator contract documentation

## Authority

- Issue 5861 owns the v0.92 sidecar redesign only
- Preparation receipts do not reserve product paths or authorize implementation
- The active WP-01 readiness repair does not depend on this issue
- Provider reviews are advisory and do not authorize lifecycle transitions

## Assumptions

- none

## Operator Constraints

- Use independent Rust v2 binaries only
- Do not reintroduce v1 wrappers, shell lifecycle logic, or retry loops
- Preserve issue-bound worktrees, overlap protection, schemas, design review, and evidence requirements
- Keep tracker metadata and semantic readiness truth distinct
- Fail closed on unsupported cross-host locking topology
