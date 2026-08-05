# Structured Intent Prompt

Template: 1.0.0

Issue: 5878

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts.

## Required Outcome

Register and integrate the distributed module, then prove real multi-node Guardian behavior, API/WSS continuity, adversarial failures, and native macOS/Linux/Windows receipts.

## Scope

- adl-runtime/src/distributed/mod.rs
- adl-runtime/src/lib.rs
- adl-runtime/tests/distributed_guardian.rs
- adl/tools/validate_v092_distributed_guardian.sh
- adl/tools/validate_v092_distributed_native_receipts.rb

## Authority

- Issue 5878 exclusively owns the declared paths
- WP-04-IMP issue 5862 coordinates only
- WP-04.16 alone owns final module registration
- No sibling, Runtime v2, or v0.93 authority

## Assumptions

- none

## Operator Constraints

- Do not start before #5821 is terminal
- Bind only the exact exclusive paths
- Use nonzero exact test selection
- Fix all actionable pre-PR findings
