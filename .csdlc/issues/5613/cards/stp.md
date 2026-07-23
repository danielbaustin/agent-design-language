# Structured Task Prompt

Template: 1.0.0

Issue: 5613

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add the narrow atomic operation, repair only issue 5591 terminal validation portability, and integrate the three exact terminal projections.

## Deliverables

- Typed repair request schema and closeout command
- Atomic store implementation with exact CAS and rollback
- Focused terminal repair regression suite
- Portable issue 5591 SOR and regenerated receipt
- Integrated terminal projections for 5337, 5339, and 5591
- Fresh-checkout collision-free proof

## Acceptance

1. AC-1: Distinct active authority and exact authority/target/receipt CAS are required
2. AC-2: Target must remain closed-out and claim-free
3. AC-3: Exactly one complete old validation result must match
4. AC-4: Malformed or machine-local replacement results fail closed
5. AC-5: Success atomically regenerates SOR, projections, audit, digest, and retained receipt
6. AC-6: Interrupted receipt update restores prior record, cards, audit, and receipt bytes
7. AC-7: Issue 5591 retained SOR contains no machine-local absolute path
8. AC-8: Unsupported guardian-soak JSON is absent from the corrective branch
9. AC-9: Issues 5337, 5339, and 5591 preserve original terminal identities and dispositions
10. AC-10: Fresh checkout sees all three targets closed-out, claim-free, doctor-clean, and non-colliding

## Dependencies

- Retained terminal receipts for issues 5337, 5339, 5358, 5591, and 5602
- Terminal commits 461713dc10d26fa5336a054c07ef1844f804ec8f, 817126889942fc57820bf9f05f5cc40e2debd683, 23ea342fdc1e4080a4e8d2236c8514ab4a9fc15f, 8cfb7b25ad246dd411a57ecc4fda8e47665912fc, and 5fc1ec96e5f1e8f2080af16caf1344295fb13064
- Existing terminal transaction and receipt-refresh implementation

## Inputs

- csdlc-v2 existing serde, serde_json, and schemars dependencies
- csdlc-v2 terminal repair operations and transaction journal
- .git/csdlc-v2/closeout/5337.json
- .git/csdlc-v2/closeout/5339.json
- .git/csdlc-v2/closeout/5591.json

## Non Goals

- No Runtime implementation changes
- No Runtime v2 or ADL-v2 changes
- No broad terminal card editor
- No issue reopening or terminal identity replacement
- No new third-party crate
- No AWS or external provider execution
