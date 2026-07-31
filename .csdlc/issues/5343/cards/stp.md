# Structured Task Prompt

Template: 1.0.0

Issue: 5343

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare durable reviewed #5343 cutover contracts and an executable fail-closed dependency gate; do not execute the selector switch, implement product code, install binaries, publish, open a PR, merge, delete legacy code, use AWS/raw gh, or edit Runtime v2.

## Deliverables

- All six issue-specific current-registry typed cards
- Reversible default-switch design and Mermaid diagram
- Exact #5344 merged, typed-closed-out, claim-release, retained-receipt, ancestry, and accepted soak/rollback handoff gate
- Exact #5345 selector/installer identity and authority gate
- Fresh-install, locked compare-and-swap, explicit-v1, failure-preservation, rollback-window, and no-deletion contract
- Preparation-only protected paths and exact ownership boundary
- COTS decision, LoC/module/test/time budgets, PVF classification, no-deferral matrix, CI, exact-review, and post-merge plan
- Deterministic redacted exact-revision cutover evidence schema

## Acceptance

1. AC-1: No selector command or cutover work starts until #5344 is GitHub merged, typed closed_out with no active claim, backed by a retained merged terminal receipt whose observed merge SHA is ancestral, and accompanied by accepted exact-revision soak and rollback evidence
2. AC-2: The #5344 handoff binds reviewed revision, manifest digest, exact prior and restored selector digests, fresh-install receipt, zero unresolved rows, accepted residual risks, and an operator-approved rollback-window contract; prior and restored digests must match
3. AC-3: The exact intended generation and executable digest are verified from a fresh isolated installation and deterministic installation receipt before selection, with no implicit network, credential, fallback, or production-state discovery
4. AC-4: Default selection performs one locked atomic compare-and-swap through the #5345 interface from an expected prior digest, emits a deterministic receipt, and re-reads exact selected generation, executable digest, and installation identity before success
5. AC-5: Stale expectation, contention, malformed or missing receipt, executable mismatch, unsupported schema, interruption, re-read mismatch, and smoke failure preserve prior selector bytes or trigger explicit rollback through the same transaction; direct storage editing and silent fallback are forbidden
6. AC-6: Explicit v1 override is proven before and after selection and at required rollback-window checkpoints while the prior v1 executable and receipt remain intact; exact restoration failure blocks publication, closeout, WP-13, and WP-14A
7. AC-7: The retained cutover packet is deterministic, redacted, repo-relative, exact-revision and exact-tool-version bound, records selector and installation receipts, rollback-window start/end, checkpoint results, and claim boundaries, and contains no secret or absolute host path
8. AC-8: #5343 edits no Runtime v2 or incumbent ADL source, deletes no legacy surface, adds no crate or production dependency, and owns only issue-local records and its normalized cutover evidence
9. AC-9: Preparation, dependency, transaction, failure, fresh-install, explicit-v1, rollback-window, budget, CI, exact-review, and post-merge PVF lanes are executable with no deferred acceptance; orchestration stays within 500 nonblank lines, tests/fixtures within 800, modules below 400, and 120/300/600/1200-second budgets unless exact reviewed variance is recorded

## Dependencies

- #5344 WP-12 soak/rollback live merged, typed closed_out, claim released, retained merged receipt present, merge SHA ancestral, and exact handoff accepted
- #5345 authoritative selector and installer live merged, typed closed_out, retained merged receipt present, merge SHA ancestral, and exact installation contract available
- WP-11 parity and Runtime v3 acceptance inherited through and truthfully bound by the terminal #5344 handoff
- Operator-approved rollback-window duration and checkpoint cadence recorded before selector mutation

## Inputs

- AGENTS.md
- GitHub issue #5343 source prompt
- docs/templates/prompts/current.json
- docs/milestones/v0.91.8/DESIGN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/features/DELETION_AND_CUTOVER_v0.91.8.md
- future terminal #5344 receipt and accepted exact-revision soak/rollback handoff
- future terminal #5345 selector/installer receipt and public command contract

## Non Goals

- Selector, installer, CLI, lock, compare-and-swap, signing, language, compiler, engine, records, Runtime, provider, governed-tool, or lifecycle implementation
- Soak or rollback execution owned by #5344
- Runtime v2 or incumbent ADL source edit, import, copying, linking, deletion, or cleanup
- WP-13 deletion, WP-14A acceptance, milestone closeout, v0.92 activation, cloud provisioning, AWS, live credentials, or production-provider claims
- Any selector transaction, installation, product edit, publication, PR, merge, or closeout during this preparation task
