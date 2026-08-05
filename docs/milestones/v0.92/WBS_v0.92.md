# v0.92 Work Breakdown Structure

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-08-04`
- Owner: ADL maintainers
- Status: active WP sequence opened by `v0.92` WP-01
- Related issues: `#3377`, `#3434`, `#5359`, `#5765`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Active allocation. WP-01 reconciled the completed v0.91.8 release package, the
reviewed TBD input dispositions from `#5359`, the AEE completion tranche, and
`#3377`, then opened the GitHub issue wave and initialized all six C-SDLC cards
for every child issue. Closed issue `#3434` is retained only as the historical
v0.92 planning-document preparation input; it is not active milestone work.

## How To Use

Use this WBS with the opened issue wave. Live issues and typed issue records
remain execution authority; this document defines milestone allocation.

## WBS Summary

v0.92 should harden the execution substrate and develop the identity,
continuity, and first-birthday layer without stealing work from citizen-state,
moral-trace, or constitutional-governance milestones.

## Active WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Canonical docs and issue-wave readiness | Update canonical version/docs surfaces, consume reviewed v0.91.8 and TBD dispositions, open the final issue wave, and prepare cards. | Updated canonical docs, opened issue wave, and full SIP/STP/SPP/VPP/SRP/SOR card bundles. | v0.91.8 release truth, activation-test map, AEE completion tranche, `#3377`, and WP-22 planning dispositions from `#5359`. |
| WP-01B | Canonical documentation and version activation | Activate v0.92 across `docs/planning/ADL_FEATURE_LIST.md`, current documentation, READMEs, manifests, Cargo metadata, skills, and runbooks before substantive implementation. | Current v0.92 canonical documentation and version truth with a checked-surface inventory. | WP-01. |
| WP-02 | Agent Logic GitHub organization repository migration | Execute the reviewed five-repository transfer plan before substantive milestone work while retaining `danielbaustin/asksifu` as personal and excluding Horust. | Verified company-owned organization, five serially transferred repositories, preserved GitHub surfaces including issue/PR assignee retention or explicit reassignment, updated integrations, and migration report. | WP-01, WP-01B, reviewed #5815 plan, confirmed destination organization and owners, complete migration inventory, billing/security readiness. |
| WP-24 | Ten-article launch series | Write all ten planned ADL launch articles in parallel, bring every article to editorial-review-ready state, maintain them as milestone evidence lands, and finalize claims during WP-23. | Ten complete review-ready article drafts, one coherent series arc, claim/evidence matrix, and final release-grounded publication disposition. | WP-02 for parallel drafting; WP-23 for final release-truth alignment. |
| WP-24A | Podcast Studio first ten episodes | Produce all first ten Podcast Studio episodes in parallel as complete review-ready packages rather than topic placeholders. | Ten reviewed episode packages with scripts, transcripts, show notes, final audio and QA, guest metadata, artwork/ID3 metadata, and RSS-ready enclosure records. | WP-02; established Podcast Studio v2 proof; Agent Logic route and storage decisions. |
| WP-02A | CI and coverage reliability | Repair fast/slow routing, coverage aggregation, platform parity, and immediate-failure causes before broad execution. | Green proving substrate with focused and slow lanes separated. | WP-02. |
| WP-02B | Post-migration build acceleration experiment | Compare the same exact CI workloads on standard and one restricted 16-core GitHub-hosted Ubuntu runner without changing proof semantics. | Complete cold/warm measurement corpus, proof parity, one canary, lane decisions, and fallback or cleanup evidence. | WP-02, WP-02A, organization-owner budget and runner-access approval. |
| WP-03 | Runtime launch and resilience consolidation | Combine launch recovery and long-lived Agent OS plans into one bounded reliability tranche. | One Guardian-owned launch path, resilient kernel startup, configuration, recovery, and lifecycle proof. | WP-02A. |
| WP-04 | Distributed Guardian/polis runtime | Execute and complete the documented 16-issue distributed-runtime program within v0.92 with architecture and security gates. | Reviewed architecture/security gate, all 16 child issues landed, and integrated distributed proof. | WP-03. |
| WP-05 | C-SDLC estimation and cycle-time reduction | Join session estimation/reconnection and sprint cycle-time work into one workflow-efficiency track. | Measured cycle-time baseline, simplified lifecycle path, and regression proof. | WP-02A. |
| WP-06 | Remote validation/build runner | Pilot a portable remote validation runner without making local execution depend on network availability. | Bounded runner, provenance contract, failover, and platform proof. | WP-02A. |
| WP-07 | Prompt-card enum typing | Verify historical delivery and implement only the remaining typed-enum gap. | Delivery audit plus bounded enum/schema/tooling correction if needed. | WP-01, WP-05. |
| WP-08 | Birthday contract and negative cases | Define what counts as birth and what does not. | Feature contract, negative fixtures, and validation rules. | WP-01, WP-02A. |
| WP-09 | Stable name and identity architecture | Define identity root, stable name, aliases, provenance, and continuity head. | Identity record contract and fixtures. | WP-08 and prior citizen-state lineage. |
| WP-10 | Continuity across bounded cycles | Prove identity survives multiple bounded cycles with evidence. | Continuity record, cycle fixtures, and validation. | WP-09. |
| WP-11 | Implement the first Memory Palace context-topology slice | Bind identity to witnessed artifacts and implement deterministic Memory Palace topology and bounded working-set behavior without raw private-state exposure. | Working topology and retrieval behavior, stale-context negatives, witnessed memory references, and redaction-safe provenance proof. | WP-09, WP-10, ObsMem/trace baseline. |
| WP-12 | Capability envelope | Declare provider, model, tool, skill, authority, and limit context at birth without absorbing the deferred taxonomy program. | Capability envelope and validation fixtures. | WP-09 and governed-tool evidence where tool actions are in scope. |
| WP-13 | ACP / cognitive profiles | Define runtime-visible cognitive profile records grounded in memory, capability, continuity, ToM, and intelligence evidence. | ACP/profile contract, update rules, privacy boundary, and fixtures. | WP-10 through WP-12 plus v0.91.1 evidence. |
| WP-13A | Implement the Adaptive Learning DAG | Build evaluation bindings, durable adaptation deltas, policy-governed graph modification, and replay-safe execution on the requalified Runtime v3 loop substrate. | Working Runtime v3 implementation, deterministic replay proof, accepted/rejected mutation paths, and required negative tests. | WP-01, WP-13, `#5104` merge evidence, current Runtime v3 qualification. |
| WP-14 | ACIP and A2A contract reconciliation | Reconcile ACIP/A2A semantics, then define protobuf schema, public catalog, deterministic JSON projection, and authenticated full-duplex WSS carrier proof. | One versioned contract family, schemas, fixtures, and real carrier proof. | WP-04, ACIP substrate, and trace/replay baseline. |
| WP-15 | Birth witnesses and receipts | Define witness set and citizen-facing receipt for the birthday event. | Witness schema, receipt schema, and validation. | WP-09 through WP-12. |
| WP-16 | Birthday review packet | Assemble identity, continuity, memory, capability, profile, protocol, witness, and moral context into one review surface. | Reviewer packet and fixture. | WP-08 through WP-15. |
| WP-17 | Migration and cross-polis continuity planning | Define birthday-identity movement semantics without duplicating WP-04 infrastructure implementation. | Cross-polis continuity feature note, design note, and non-goals. | WP-09, WP-10, WP-16. |
| WP-18 | First birthday demo | Build a flagship demo showing a real birthday record and negative cases. | Runnable proof demo and artifacts. | WP-08 through WP-16. |
| WP-18A | Observatory/Unity consumer integration | Integrate the separate Observatory and Unity consumers with the versioned Runtime API and WSS surfaces. | Working consumer integration and compatibility proof without moving UI code into Runtime. | WP-03, WP-14, WP-18. |
| WP-18B | Provider-neutral multi-agent proof | Prove the birthday/runtime contract across multiple providers without provider-specific success substitutions. | Provider-neutral multi-agent proof matrix, artifacts, and negative cases. | WP-14, WP-16, WP-18. |
| WP-19 | Birthday-to-governance handoff | Produce the evidence map v0.93 governance will consume. | Handoff packet mapping identity evidence to governance and ADR-plan updates. | WP-16, WP-17, v0.93 allocation. |
| WP-20 | Demo matrix, AEE proof, and proof coverage | Align demos and AEE proof expectations with milestone claims. | Demo matrix, AEE proof routing or packet, commands, artifacts, and validation notes. | WP-18, WP-18A, WP-18B, WP-19. |
| WP-21 | Repository-wide code reduction cleanup | Complete the remaining behavior-preserving repository reduction tranche with an exact deletion denominator. | Reviewed deletion manifest, parity proof, and retained-surface inventory. | WP-20. |
| WP-21A | Rust refactoring and maintainability pass | Simplify active Rust ownership boundaries and maintainability hotspots without changing supported behavior or hiding feature work in refactoring. | Refactoring inventory, focused parity proof, before/after LoC, and reviewed maintainability improvements. | WP-20, WP-21. |
| WP-22 | Quality gate | Validate implementation, docs, platform behavior, and claim boundaries; block internal review until every indexed v0.92 feature has accepted exact-revision proof. | Feature-completion matrix, quality-gate record, and blocker disposition. | WP-04 through WP-21A. |
| WP-23 | Docs and release-truth pass | Align README, changelog, feature list, ADR plan, release notes, skills, agent guidance, and milestone docs. | Docs review packet, ADR candidate packet if needed, and updated release docs. | WP-22. |
| WP-25 | Internal review | Run internal code/docs/tests/process/publication review. | Internal review report and finding register. | WP-23, WP-24, WP-24A. |
| WP-26 | External / third-party review | Prepare and run external review. | External review handoff and received review packet. | WP-25. |
| WP-27 | Review findings remediation | Fix or route review findings. | Finding disposition record and remediation PRs. | WP-26. |
| WP-28 | Next milestone planning | Prepare the v0.93 handoff. | Next-milestone handoff and downstream planning update. | WP-27. |
| WP-28A | Next-milestone closeout plan | Prepare the exact terminal issue/PR/receipt and ceremony sequence before final review. | Reviewed closeout plan and issue universe. | WP-28. |
| WP-29 | Next milestone review pass | Review v0.93 planning and closeout readiness before ceremony. | Next-milestone review findings and disposition note. | WP-28A. |
| WP-30 | Release ceremony | Close the milestone with exact release evidence. | Evidence package, release notes, and ceremony closeout. | WP-29. |

## Work Packages

The work packages are the active `WP-01` through `WP-30` rows above,
including the bounded `WP-01B`, `WP-02B`, `WP-13A`, `WP-18A`, `WP-18B`, `WP-21A`, and `WP-28A` sidecars. They
are backed by concrete GitHub issues and initialized six-card bundles.

## Sequencing

The intended sequence is planning and proving-substrate repair first; Runtime
reliability and workflow-tooling tracks may then proceed in parallel with the
birthday contract. Identity and continuity precede evidence-bearing birthday
features. Consumer/demo work follows the integrated review packet. Cleanup,
quality, docs, publication, review, remediation, next-milestone planning,
closeout planning, next-milestone review, and ceremony form the release tail.

## Sequencing Pressure

1. Update canonical docs, complete repository migration, repair CI/coverage,
   and run the bounded build-acceleration experiment before broad execution credit.
2. Run Runtime reliability, distributed-runtime, C-SDLC efficiency, remote
   validation, and prompt-typing work in parallel where dependencies permit.
3. Start the birthday contract and negative cases, then add identity and
   continuity.
4. Add memory grounding, capability envelope, ACP/cognitive profile, Adaptive
   Learning DAG implementation, and reconciled ACIP/A2A transport readiness.
5. Add witnesses, receipts, and the integrated review packet.
6. Add migration semantics only after local birth semantics are stable.
7. Build the flagship birthday demo, consumer integrations, provider-neutral
   proof, governance handoff, and proof matrix.
8. Complete behavior-preserving cleanup before the quality and review tail.

## Sprint Umbrellas

The issue wave is grouped into five coordination-only sprint umbrellas. These
umbrellas define dependency routing, safe parallel lanes, integration proof,
and write-surface boundaries; they do not replace child-issue ownership or
authorize an umbrella agent to implement child code directly.

| Sprint | Members | Execution shape |
| --- | --- | --- |
| Foundation and throughput | WP-01B, WP-02, support `#5812`, WP-02A, WP-02B, WP-05, WP-06, WP-07 | Hybrid: migration and CI are serial gates; workflow and remote-runner work may proceed in parallel after CI stabilizes. |
| Runtime, Observatory, polis, and protocol | support `#5800`, WP-03, support `#5795`, WP-04, WP-14, WP-18A | Hybrid: browser trust and Runtime resilience establish the baseline; local Shepherd integration follows; distributed, protocol, and consumer lanes converge afterward. |
| Birthday core | WP-08 through WP-13A, WP-15, WP-16 | Hybrid dependency graph with contract and identity gates before integrated witness/review proof. |
| Demonstration, handoff, and publication | WP-17, WP-18, WP-18B, WP-19, WP-20, WP-24, WP-24A | Publication production starts early after migration; proof and handoff converge only after birthday and protocol dependencies. |
| Release tail | WP-21, WP-21A, WP-22, WP-23, WP-25 through WP-30 | Sequential quality, review, remediation, planning, and ceremony chain. |

All five umbrellas may be prepared in parallel. Live implementation may start
only for dependency-ready child issues, and each child keeps its own typed
lifecycle, validation, exact-head review, and publication authority. The exact
machine-readable membership and serial gates are in
`WP_ISSUE_WAVE_v0.92.yaml`.

## Issue Wave

The opened issue wave and live issue-number mapping live in
[WP_ISSUE_WAVE_v0.92.yaml](WP_ISSUE_WAVE_v0.92.yaml).

Before publishing WP-01, it must:

- reconcile the active sequence with the final v0.91.8 release package,
  the activation-test map, the WP-22 TBD dispositions, and `#3377`
- consume `docs/milestones/v0.91.8/evidence/wp16/ISSUE_OUTCOME_AUDIT.md`,
  `docs/milestones/v0.91.8/evidence/wp16/QUALITY_GATE.md`, and
  `docs/milestones/v0.91.8/evidence/wp16/issue-outcome-audit.v1.json`
  as quality inputs while using the final v0.91.8 release package as the
  release-truth authority
- reconcile the AEE completion tranche and either seed concrete
  AEE proof work or record why existing v0.92 WPs cover it
- requalify the post-`#5104` loop-runtime merge evidence against current
  Runtime v3 contracts before treating bounded recurrent execution over
  reasoning graphs as a consumed prerequisite for the Adaptive Learning DAG
  implementation
- verify all feature docs remain linked and scoped, including the Memory
  Palace context-topology proof slice
- create all six lifecycle cards for every opened issue from the active prompt-template
  registry
- keep `SIP`, `STP`, and `SPP` design-time ready before execution
- keep `SRP` and `SOR` truthful to review and output lifecycle state
- preserve the explicit deferred and later-backlog source pointers below

## Deferred And Later Backlog

The following inputs were reviewed by v0.91.8 WP-22 and intentionally excluded
from v0.92 execution. Deferral is not a delivery claim and the source pointers
must be retained in the issue-wave package.

| Disposition | Track | Source inputs | Reason |
| --- | --- | --- | --- |
| Deferred technical track | Capability taxonomy integration | `.adl/docs/TBD/capability_testing/ADL_CAPABILITY_TAXONOMY.md` | Useful but not critical to first-birthday execution; schedule under a later capability milestone. |
| Deferred technical track | MLX local provider and OCI model packaging | `.adl/docs/TBD/MLX_APPLE_METAL_PROVIDER_PLAN.md`; `.adl/docs/TBD/OCI_MODEL_PACKAGING_METHOD_PLAN.md` | Keep as one later provider/model-distribution track. |
| Later product-publication backlog | Agent Logic website and investor publication | `.adl/docs/TBD/AGENT_LOGIC_WEBSITE_DESIGN_v2.1.md`; `.adl/docs/TBD/AGENT_LOGIC_INVESTOR_MATERIAL_TRACKING_AND_PUBLICATION_PLAN.md` | Separate Agent Logic launch lane outside the ADL v0.92 WBS. |
| Later research/publication backlog | General-intelligence paper program | `.adl/docs/TBD/general-intelligence-paper/` | Requires its own research/publication milestone disposition. |

## Acceptance Mapping

- Birth must be distinguishable from startup, wake, snapshot, admission, and
  copied state.
- Identity must include stable name, identity root, continuity, memory
  grounding, capability, bounded cognitive profile, witnesses, and receipt.
- Continuity must be evidence-based and reviewable.
- Memory grounding must not expose raw private state, and Memory Palace work
  must land its named first working slice before completion.
- Capability envelope must record limits and authority context.
- Cognitive profiles must be evidence-grounded, privacy-bounded, and distinct
  from identity, reputation, and public standing.
- Adaptive-learning work remains incomplete until WP-13A lands evaluation
  bindings, stateful adaptation, policy-governed graph mutation, replay proof,
  and fail-closed negative cases.
- Binary ACIP must remain inspectable through public schemas and deterministic
  JSON projection while message contents remain governed by authority and
  visibility policy.
- v0.93 governance must consume v0.92 evidence rather than redefine birth.
- Demos must show behavior and artifacts, not just narrative.

## Sequencing Notes

- The release tail order must remain quality, docs, publication, internal
  review, external review, remediation, next-milestone planning, closeout
  planning, next-milestone review, and ceremony.
- Side work discovered during v0.92 should be routed explicitly rather than
  hidden inside birthday implementation WPs.
- Any divergence from this active WBS should be recorded in the owning issue cards and
  milestone planning docs.

## Exit Criteria

- The WBS validates against the active planning-template set.
- WP-01 can use this document and `WP_ISSUE_WAVE_v0.92.yaml` without
  reconstructing work packages from chat.
- Every implementation WP names a reviewable deliverable.
- Every feature-like implementation tranche has a tracked feature doc or an
  explicit reason it is release/review/process work instead.
- v0.92 ADR candidates are planned before review-tail execution.
