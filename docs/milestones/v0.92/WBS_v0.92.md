# v0.92 Candidate Work Breakdown Structure

## Metadata

- Milestone: `v0.92`
- Version: `v0.92`
- Date: `2026-05-27`
- Owner: ADL maintainers
- Status: candidate WP sequence for `v0.92` WP-01 seeding
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Status

Candidate allocation only. v0.92 has no opened GitHub issue wave yet.

The candidate WP sequence below should be consumed by the v0.92 WP-01 planning
pass. WP-01 should verify prerequisite truth, reconcile `v0.91.5`
release-tail closeout, the `v0.91.6` / `v0.91.7` bridge tranches, the AEE
completion tranche, and `#3377`, then seed the
actual GitHub issue wave and full C-SDLC card set.

## How To Use

Use this WBS as WP-01 seed input, not as proof that v0.92 issues are already
open. WP-01 should reconcile the sequence with `v0.91.5` release-tail
closeout, the `v0.91.6` / `v0.91.7` bridge tranches, the AEE completion
tranche, and `#3377`, then generate the
real issue wave and all five C-SDLC cards for each opened issue.

## WBS Summary

v0.92 should develop the identity, continuity, and first-birthday layer without
stealing work from citizen-state, moral-trace, or constitutional-governance
milestones.

## Candidate WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Design pass and issue-wave readiness | Promote reviewed v0.92 planning docs, consume `v0.91.5` release-tail closeout, the `v0.91.6` / `v0.91.7` bridge tranches, the AEE completion tranche, and `#3377`, seed issue wave, and prepare cards. | Opened issue wave and full SIP/STP/SPP/SRP/SOR cards, including AEE proof routing if required. | `v0.91.5` release-tail closeout, activation-test map, `v0.91.6` / `v0.91.7` bridge outputs, AEE completion tranche, and `#3377` readiness packet. |
| WP-02 | Birthday contract and negative cases | Define what counts as birth and what does not. | Feature contract, negative fixtures, and validation rules. | WP-01. |
| WP-03 | Stable name and identity architecture | Define identity root, stable name, aliases, provenance, and continuity head. | Identity record contract and fixtures. | WP-02 and prior citizen-state lineage. |
| WP-04 | Continuity across bounded cycles | Prove identity survives multiple bounded cycles with evidence. | Continuity record, cycle fixtures, validation. | WP-03. |
| WP-05 | Memory grounding and Memory Palace bridge | Bind identity to witnessed artifacts and memory references without raw private-state exposure, and route the first Memory Palace context-topology slice. | Memory-grounding contract, redacted packet, and Memory Palace bridge feature route. | WP-03, WP-04, ObsMem/trace baseline. |
| WP-06 | Capability envelope | Declare provider, model, tool, skill, authority, and limit context at birth. | Capability envelope and validation fixtures. | WP-03, governed-tool evidence where tool actions are in scope. |
| WP-07 | ACP / cognitive profiles | Define runtime-visible cognitive profile records grounded in memory, capability, continuity, ToM, and intelligence evidence. | ACP/profile contract, update rules, privacy boundary, and fixtures. | WP-04 through WP-06 plus v0.91.1 evidence. |
| WP-08 | ACIP binary schema and WebSocket carrier | Define protobuf ACIP wire schema, public schema catalog, deterministic JSON projection, and optional binary ACIP-over-WebSocket mock proof. | ACIP `.proto`, schema catalog rules, JSON/protobuf fixtures, mock WebSocket carrier proof. | ACIP substrate and trace/replay baseline. |
| WP-09 | Birth witnesses and receipts | Define witness set and citizen-facing receipt for the birthday event. | Witness schema, receipt schema, validation. | WP-03 through WP-06. |
| WP-10 | Birthday review packet | Assemble identity, continuity, memory, capability, witness, and moral context into one review surface. | Reviewer packet and fixture. | WP-02 through WP-09. |
| WP-11 | Migration and cross-polis continuity planning | Define bounded design notes for future movement without production migration claims. | Cross-polis continuity feature note, design note, and non-goals. | WP-03, WP-04, WP-10. |
| WP-12 | First birthday demo | Build a flagship demo showing a real birthday record and negative cases. | Runnable proof demo and artifacts linked to the first-birthday demo feature doc. | WP-02 through WP-10. |
| WP-13 | Birthday-to-governance handoff | Produce the evidence map v0.93 governance will consume. | Handoff packet mapping identity evidence to governance and ADR-plan updates. | WP-10, WP-11, v0.93 allocation. |
| WP-14 | Demo matrix, AEE proof, and proof coverage | Align demos and AEE proof expectations with milestone claims. | Demo matrix rows, AEE proof routing or proof packet, commands, artifacts, and validation notes. | WP-12, WP-13, v0.91.5 AEE completion tranche. |
| WP-15 | Quality gate | Validate implementation, fixtures, docs, and claim boundaries. | Quality-gate record and blocker disposition. | WP-14. |
| WP-16 | Docs and release-truth pass | Align README, changelog, feature list, ADR plan, release notes, and milestone docs. | Docs review packet, ADR candidate packet if needed, and updated release docs. | WP-15. |
| WP-17 | Internal review | Run internal code/docs/tests/process review. | Internal review report and finding register. | WP-16. |
| WP-18 | External / third-party review | Prepare and run external review. | External review handoff and received review packet. | WP-17. |
| WP-19 | Review findings remediation | Fix or route review findings. | Finding disposition record and remediation PRs. | WP-18. |
| WP-20 | Next milestone planning | Prepare the v0.93 handoff. | Next-milestone handoff and downstream planning update. | WP-19. |
| WP-21 | Next milestone review pass | Review v0.93 planning before ceremony. | Next-milestone review findings and disposition note. | WP-20. |
| WP-22 | Release ceremony | Close milestone with release evidence. | Evidence package, release notes, and ceremony closeout. | WP-21. |

## Work Packages

The work packages are the candidate `WP-01` through `WP-22` rows above. They
remain candidate rows until v0.92 WP-01 opens concrete GitHub issues and
copies the final issue numbers back into milestone tracking docs.

## Sequencing

The intended sequence is planning first, birthday contract second, identity and
continuity third, evidence-bearing feature work fourth, demos and handoff
fifth, and the normal review/remediation/release tail last.

## Sequencing Pressure

1. Start with the birthday contract and negative cases.
2. Add stable name and identity architecture.
3. Add continuity, memory grounding, capability envelope, ACP/cognitive profile
   contract, and ACIP binary/schema-catalog transport readiness.
4. Add witnesses, receipts, and review packets.
5. Add migration planning only after local birth semantics are stable.
6. Build the flagship birthday demo and governance handoff last.

## Issue-Wave Preflight

The candidate issue-wave seed lives in
[WP_ISSUE_WAVE_v0.92.yaml](WP_ISSUE_WAVE_v0.92.yaml). WP-01 should treat it as
draft input, not as an already-opened issue wave.

Before opening v0.92 issues, WP-01 must:

- reconcile the candidate sequence with `v0.91.5` release-tail closeout, the
  activation-test map, the `v0.91.6` / `v0.91.7` bridge tranches, and `#3377`
- reconcile the AEE completion tranche from v0.91.5 and either seed concrete
  AEE proof work or record why existing v0.92 WPs cover it
- verify all feature docs remain linked and scoped, including the Memory
  Palace context-topology bridge
- create all five cards for every opened issue from the active prompt-template
  registry
- keep `SIP`, `STP`, and `SPP` design-time ready before execution
- keep `SRP` and `SOR` truthful to review and output lifecycle state

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
- Binary ACIP must remain inspectable through public schemas and deterministic
  JSON projection while message contents remain governed by authority and
  visibility policy.
- v0.93 governance must consume v0.92 evidence rather than redefine birth.
- Demos must show behavior and artifacts, not just narrative.

## Sequencing Notes

- The release tail order must remain internal review, external review,
  remediation, next-milestone planning, next-milestone review, and ceremony.
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
