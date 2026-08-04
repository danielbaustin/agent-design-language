# Structured Task Prompt

Template: 1.0.0

Issue: 5356

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare the complete WP-18 internal-review lifecycle and execution contract; do not perform the review or publish it.

## Deliverables

- six current-registry issue-specific typed cards
- reviewed design and Mermaid diagram
- exact preparation-only protected paths and WP-17 terminal dependency gate
- complete review corpus and six-lane code/security/tests/docs/architecture/evidence matrix
- exact-revision identity, finding severity/disposition, redaction/provenance, rollback, and publication contracts
- budgets, COTS reuse, PVF, no-deferral, stop conditions, bounded preparation review and fixes

## Acceptance

1. AC-1: No review execution starts until #5360 is GitHub merged, typed closed_out, claim-free, retained-receipt-backed, and its merged SHA is ancestral to the exact #5356 execution revision
2. AC-2: Review identity freezes repository, base, head, exact commit, corpus digest, and WP-17 receipt digest; any change invalidates all lane results
3. AC-3: The corpus covers canonical planning/features/proof/quality/release/handoff plus landed WP-02 through WP-17 code, tests, deployment, lifecycle, CI, issue/PR, and closeout evidence
4. AC-4: Mandatory code, security, tests, docs, architecture, and evidence lanes independently review the same frozen identity and return complete machine-routable results
5. AC-5: Findings use stable IDs, P0-P3 severity, exact evidence, impact/invariant/failure mode/fix/route/disposition/residual risk; unresolved blockers prevent WP-19
6. AC-6: Missing corpus, stale identity, skipped lane, secret/private/host-bound/untracked evidence, unsupported claims, and inconsistent specialist results fail closed
7. AC-7: Existing COTS and repository review tooling are reused; preparation stays within 1400 nonblank lines/file<500 and future harness within 2500 lines/<250 assertions and declared time budgets unless exactly reviewed
8. AC-8: Complete review, synthesis, review-quality, typed review, redaction/provenance, exact recheck, CI, authorized serialized merge, post-merge proof, and closeout occur without deferral before WP-19 begins

## Dependencies

- WP-17 #5360 merged, typed closed_out, claim-free, backed by a retained merged receipt, and ancestral to the exact #5356 execution revision

## Inputs

- AGENTS.md
- GitHub issues #5356 and #5360
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/SPRINT_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/review/README.md
- docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md
- .csdlc/prepared/issues/5356/review-corpus.json
- .csdlc/prepared/issues/5356/specialist-lanes.json

## Non Goals

- performing internal review, producing findings, remediating findings, or sending external review
- product, Runtime, deployment, milestone-document, feature, release, or issue-graph implementation
- Runtime v2, AWS, raw gh, provider credentials, paid services, hard-coded addresses, private prompt retention, or host-bound evidence
- PR, publication, merge, release approval, v0.92 activation, or closeout during preparation
