# Structured Task Prompt

Template: 1.0.0

Issue: 5780

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only issue #5780 terminal-authority deletion and the minimum active contracts, tests, and documentation required to prove it.

## Deliverables

- Deleted closeout binary and skill
- Deleted terminal projection and receipt-authority writers
- Legacy read-compatibility proof
- Negative authority and clean-room installation guards
- Before-and-after command, source-line, fixture, and artifact metrics

## Acceptance

1. AC-1: no supported workflow creates a second PR or tracked projection solely to record an earlier merge
2. AC-2: legacy v0.91.7 and v0.91.8 records and receipts remain readable with unchanged compatibility outcomes
3. AC-3: supported terminal operator surface is finish, status, clean, and bounded read-only compatibility inspection
4. AC-4: public APIs and schemas expose no current terminal receipt writer, repair, transport, or reconciliation authority
5. AC-5: complete standalone tests, strict Clippy, schema compatibility, and clean-room install verification pass
6. AC-6: deletion metrics prove real command, source-line, fixture, and artifact reduction with no hidden wrapper

## Dependencies

- Issue #5778 and PR #5782 merged
- Issue #5779 and PR #5794 merged
- Derived terminal finish and standalone cleanup parity proofs

## Inputs

- GitHub issue #5780
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/operator/skills/csdlc-v2-closeout/SKILL.md
- csdlc-v2/tests/gate7_lifecycle.rs
- .csdlc/evidence/5780/refactor-plan/refactor_plan.json

## Non Goals

- No redesign of planning, validation, review, provider, runtime, or ADL language systems
- No destructive rewrite of historical tracked records or Git history
- No cleanup of unrelated legacy shell tooling
