# Structured Task Prompt

Template: 1.0.0

Issue: 5852

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver release evidence package and ceremony closeout.

## Deliverables

- release evidence package and ceremony closeout
- release evidence packet, release notes, and closeout record

## Acceptance

1. AC-1: WP-29 passed at the exact ancestral SHA; every required child/sprint is terminal and claim-free; the candidate head is clean with green required checks and conflict-free tag/release identity.
2. AC-2: The release evidence manifest binds every final claim to exact implementation, validation, review, merge, terminal receipt, artifact hash, and residual-risk/non-claim evidence.
3. AC-3: Final notes, plan, checklist, handoff, launch assets, and evidence links describe landed reviewed behavior only and pass format/link/redaction/claim-boundary checks.
4. AC-4: Ceremony script tests and dry-run pass at the exact reviewed head; tag push, draft release, publication, and verification execute in the approved order with identity checks before retry.
5. AC-5: Independent live readback proves annotated tag object/target, release target/state/notes/assets/hashes, and records any partial failure without duplicate mutation.
6. AC-6: Typed #5852, sprint #5856, and milestone closeout use retained receipts/live release truth, and v0.93 receives the accepted handoff without activation or unsupported claims.

## Dependencies

- WP-29

## Inputs

- Passing terminal WP-29 review and its approved closeout/ceremony sequence
- All required terminal child/sprint records, quality/internal/external/remediation evidence, final docs and launch packages
- docs/milestones/v0.92/RELEASE_PLAN_v0.92.md, RELEASE_NOTES_v0.92.md, MILESTONE_CHECKLIST_v0.92.md, NEXT_MILESTONE_HANDOFF_v0.92.md, and adl/tools/release_ceremony.sh

## Non Goals

- Product remediation, evidence invention, or feature completion by tag/release ceremony
- Unreviewed network mutation or blind retry of partial/non-idempotent release steps
- Activating v0.93 or claiming legal personhood, production governance, consciousness, or unproven delivery
