# Structured Task Prompt

Template: 1.0.0

Issue: 5804

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Populate and validate the packet, obtain one bounded review, and publish a docs-only corrective PR.

## Deliverables

- Concrete implementation and proof manifest
- Current issue-truth corrections
- Portable reviewer commands
- Complete validation and review evidence

## Acceptance

1. AC-1: The handoff manifest names concrete tracked implementation, test, and evidence entrypoints
2. AC-2: Current issue-state statements match live GitHub truth
3. AC-3: The sendable corpus requires no workstation-local root
4. AC-4: Corpus structure, links, schemas, paths, redaction, and diff hygiene pass

## Dependencies

- Issue #5804 is open
- WP-16, WP-17, and both WP-18 review passes remain merged and ancestral
- WP-19 #5357 remains open

## Inputs

- docs/milestones/v0.91.8/evidence/wp16/
- .csdlc/evidence/5360/documentation-alignment.v1.json
- .csdlc/evidence/5791/focused-5791-validation.log
- docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md

## Non Goals

- No external review execution
- No WP-19 closeout
- No product code changes
- No release approval or v0.92 activation
