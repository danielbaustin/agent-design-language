# v0.92 Candidate Work Breakdown Structure

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-08-04`
- Owner: ADL maintainers
- Status: reviewed candidate WP sequence for `v0.92` WP-01 seeding
- Related issues: `#3377`, `#3434`, `#5359`, `#5765`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Candidate allocation only. v0.92 has no opened GitHub issue wave yet.

The candidate WP sequence below should be consumed by the v0.92 WP-01 planning
pass. WP-01 must reconcile the completed v0.91.8 release package, the reviewed
TBD input dispositions from `#5359`, the AEE completion tranche, and `#3377`,
then seed the actual GitHub issue wave and full six-card C-SDLC set. This review
does not itself open v0.92 execution issues.

## How To Use

Use this WBS as WP-01 seed input, not as proof that v0.92 issues are already
open. WP-01 must verify live prerequisite truth, update the canonical docs,
generate the real issue wave, and create all six C-SDLC cards for each opened
issue.

## WBS Summary

v0.92 should harden the execution substrate and develop the identity,
continuity, and first-birthday layer without stealing work from citizen-state,
moral-trace, or constitutional-governance milestones.

## Candidate WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Canonical docs and issue-wave readiness | Update canonical version/docs surfaces, consume reviewed v0.91.8 and TBD dispositions, open the final issue wave, and prepare cards. | Updated canonical docs, opened issue wave, and full SIP/STP/SPP/VPP/SRP/SOR card bundles. | v0.91.8 release truth, AEE completion tranche, `#3377`, and `#5359`. |
| WP-01A | Agent Logic GitHub organization repository migration | Execute the reviewed six-repository transfer plan before substantive milestone work while retaining `danielbaustin/asksifu` as personal. | Verified company-owned organization, six serially transferred repositories, preserved GitHub surfaces, updated integrations, and migration report. | WP-01, confirmed destination organization and owners, complete seven-repository inventory, billing/security readiness. |
| WP-24 | Ten-article launch series | Write all ten planned ADL launch articles in parallel, bring every article to editorial-review-ready state, maintain them as milestone evidence lands, and finalize claims during WP-23. | Ten complete review-ready article drafts, one coherent series arc, claim/evidence matrix, and final release-grounded publication disposition. | WP-01A for parallel drafting; WP-23 for final release-truth alignment. |
| WP-24A | Podcast Studio first ten episodes | Produce all first ten Podcast Studio episodes in parallel as complete review-ready packages rather than topic placeholders. | Ten reviewed episode packages with scripts, transcripts, show notes, final audio and QA, guest metadata, artwork/ID3 metadata, and RSS-ready enclosure records. | WP-01A; established Podcast Studio v2 proof; Agent Logic route and storage decisions. |
| WP-02 | CI and coverage reliability | Repair fast/slow routing, coverage aggregation, platform parity, and immediate-failure causes before broad execution. | Green proving substrate with focused and slow lanes separated. | WP-01A. |
| WP-03 | Runtime launch and resilience consolidation | Combine launch recovery and long-lived Agent OS plans into one bounded reliability tranche. | One Guardian-owned launch path, resilient kernel startup, configuration, recovery, and lifecycle proof. | WP-02. |
| WP-04 | Distributed Guardian/polis runtime | Execute the documented 16-issue distributed-runtime program with architecture and security gates. | Reviewed architecture/security gate followed by bounded distributed Guardian/polis child issues and integrated proof. | WP-03. |
| WP-05 | C-SDLC estimation and cycle-time reduction | Join session estimation/reconnection and sprint cycle-time work into one workflow-efficiency track. | Measured cycle-time baseline, simplified lifecycle path, and regression proof. | WP-02. |
| WP-06 | Remote validation/build runner | Pilot a portable remote validation runner without making local execution depend on network availability. | Bounded runner, provenance contract, failover, and platform proof. | WP-02. |
| WP-07 | Prompt-card enum typing | Verify historical delivery and implement only the remaining typed-enum gap. | Delivery audit plus bounded enum/schema/tooling correction if needed. | WP-01, WP-05. |
| WP-08 | Birthday contract and negative cases | Define what counts as birth and what does not. | Feature contract, negative fixtures, and validation rules. | WP-01, WP-02. |
| WP-09 | Stable name and identity architecture | Define identity root, stable name, aliases, provenance, and continuity head. | Identity record contract and fixtures. | WP-08 and prior citizen-state lineage. |
| WP-10 | Continuity across bounded cycles | Prove identity survives multiple bounded cycles with evidence. | Continuity record, cycle fixtures, and validation. | WP-09. |
| WP-11 | Memory grounding and Memory Palace proof slice | Bind identity to witnessed artifacts and memory references without raw private-state exposure, and implement or block the first Memory Palace context-topology slice. | Memory-grounding contract, redacted packet, and Memory Palace proof-slice disposition. | WP-09, WP-10, ObsMem/trace baseline. |
| WP-12 | Capability envelope | Declare provider, model, tool, skill, authority, and limit context at birth without absorbing the deferred taxonomy program. | Capability envelope and validation fixtures. | WP-09 and governed-tool evidence where tool actions are in scope. |
| WP-13 | ACP / cognitive profiles | Define runtime-visible cognitive profile records grounded in memory, capability, continuity, ToM, and intelligence evidence. | ACP/profile contract, update rules, privacy boundary, and fixtures. | WP-10 through WP-12 plus v0.91.1 evidence. |
| WP-13A | Adaptive Learning DAG queue | Requalify historical loop-runtime evidence and queue evaluation bindings, stateful adaptation, governed graph modification, and replay proof. | Adaptive-learning feature contract, status checklist, work queue, and negative-test plan. | WP-01, WP-13, `#5104` merge evidence, current Runtime v3 qualification. |
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
| WP-22 | Quality gate | Validate implementation, fixtures, docs, platform behavior, and claim boundaries. | Quality-gate record and blocker disposition. | WP-04 through WP-21. |
| WP-23 | Docs and release-truth pass | Align README, changelog, feature list, ADR plan, release notes, skills, agent guidance, and milestone docs. | Docs review packet, ADR candidate packet if needed, and updated release docs. | WP-22. |
| WP-25 | Internal review | Run internal code/docs/tests/process/publication review. | Internal review report and finding register. | WP-23, WP-24, WP-24A. |
| WP-26 | External / third-party review | Prepare and run external review. | External review handoff and received review packet. | WP-25. |
| WP-27 | Review findings remediation | Fix or route review findings. | Finding disposition record and remediation PRs. | WP-26. |
| WP-28 | Next milestone planning | Prepare the v0.93 handoff. | Next-milestone handoff and downstream planning update. | WP-27. |
| WP-28A | Next-milestone closeout plan | Prepare the exact terminal issue/PR/receipt and ceremony sequence before final review. | Reviewed closeout plan and issue universe. | WP-28. |
| WP-29 | Next milestone review pass | Review v0.93 planning and closeout readiness before ceremony. | Next-milestone review findings and disposition note. | WP-28A. |
| WP-30 | Release ceremony | Close the milestone with exact release evidence. | Evidence package, release notes, and ceremony closeout. | WP-29. |

## Work Packages

The work packages are the candidate `WP-01` through `WP-30` rows above,
including the bounded `WP-13A`, `WP-18A`, `WP-18B`, and `WP-28A` sidecars. They
remain candidate rows until v0.92 WP-01 opens concrete GitHub issues and
copies the final issue numbers back into milestone tracking docs.

## Sequencing

The intended sequence is planning and proving-substrate repair first; Runtime
reliability and workflow-tooling tracks may then proceed in parallel with the
birthday contract. Identity and continuity precede evidence-bearing birthday
features. Consumer/demo work follows the integrated review packet. Cleanup,
quality, docs, publication, review, remediation, next-milestone planning,
closeout planning, next-milestone review, and ceremony form the release tail.

## Sequencing Pressure

1. Update canonical docs and repair CI/coverage before broad execution credit.
2. Run Runtime reliability, distributed-runtime, C-SDLC efficiency, remote
   validation, and prompt-typing work in parallel where dependencies permit.
3. Start the birthday contract and negative cases, then add identity and
   continuity.
4. Add memory grounding, capability envelope, ACP/cognitive profile, Adaptive
   Learning DAG queue, and reconciled ACIP/A2A transport readiness.
5. Add witnesses, receipts, and the integrated review packet.
6. Add migration semantics only after local birth semantics are stable.
7. Build the flagship birthday demo, consumer integrations, provider-neutral
   proof, governance handoff, and proof matrix.
8. Complete behavior-preserving cleanup before the quality and review tail.

## Issue-Wave Preflight

The candidate issue-wave seed lives in
[WP_ISSUE_WAVE_v0.92.yaml](WP_ISSUE_WAVE_v0.92.yaml). WP-01 should treat it as
draft input, not as an already-opened issue wave.

Before opening v0.92 issues, WP-01 must:

- reconcile the candidate sequence with the final v0.91.8 release package,
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
  queue
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
  must remain a named bridge slice until implementation proof lands.
- Capability envelope must record limits and authority context.
- Cognitive profiles must be evidence-grounded, privacy-bounded, and distinct
  from identity, reputation, and public standing.
- Adaptive-learning work must remain queued until evaluation bindings,
  stateful adaptation, policy-governed graph mutation, replay proof, and
  fail-closed negative cases have concrete implementation WPs.
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
- Any divergence from this candidate WBS should be recorded in WP-01 cards and
  milestone planning docs.

## Exit Criteria

- The WBS validates against the active planning-template set.
- WP-01 can use this document and `WP_ISSUE_WAVE_v0.92.yaml` without
  reconstructing work packages from chat.
- Every candidate implementation WP names a reviewable deliverable.
- Every feature-like implementation tranche has a tracked feature doc or an
  explicit reason it is release/review/process work instead.
- v0.92 ADR candidates are planned before review-tail execution.
