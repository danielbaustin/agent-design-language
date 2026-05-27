# v0.91.4 Work Breakdown Structure

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-25`
- Owner: ADL maintainers
- Related issues: `#3210`, seeded v0.91.4 issue wave with Sprint 1 closeout recorded

## Status

Tracked WBS. The v0.91.4 issue wave is seeded through Sprint 4, the sidecar
waves, and the standalone first-birthday readiness issue; Sprint 1 has already
closed cleanly.

## How To Use

Use this WBS as the planning source for v0.91.4 issue/sprint sequencing and
issue-wave scope. The table is a planned sequence, while execution and closeout
truth live in the sprint state artifacts and milestone status docs.

## WBS Summary

v0.91.4 hardens the v0.91.3 C-SDLC vertical slice into repeatable operation.
The goal is not more theory; it is a dependable development control plane.

## Candidate WP Sequence

| WP | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Design pass and issue-wave readiness | docs | v0.91.4 issue wave, sprint umbrellas, validated cards | v0.91.3 closeout |
| WP-02 | Lifecycle validator hardening | tools | strict card and transition lifecycle validators | WP-01 |
| WP-03 | Doctor and conductor state truth | tools | `pr doctor`, workflow-conductor, and sprint-conductor aligned on C-SDLC stages | WP-02 |
| WP-04 | Editor skill repair reliability | tools | card editor skills repair drift without hand edits | WP-02 |
| WP-05 | Software Development Polis, shard ownership, and interface freeze | tools/docs | actor standing, enforceable shard write boundaries, and synchronization rules | WP-03 |
| WP-06 | Evidence convergence, review synthesis, and signed trace proof | tools | repeatable evidence bundle, review synthesis flow, and minimal signed trace bundle | WP-05 |
| WP-07 | Merge-readiness and PR gate hardening | tools | transition-aware merge gate preserving GitHub truth | WP-06 |
| WP-08 | ObsMem transition memory integration | tools/docs | SRP/SOR/signed-trace memory handoff write/read contract | WP-06, WP-07 |
| WP-09 | Sprint conductor default C-SDLC lane | tools | sprint flow that cannot skip child closeout or umbrella truth | WP-03, WP-04 |
| WP-10 | Five-minute-sprint repeatability and Parallel Validation Fabric | demo/tools | repeated transition metrics, validation-tail/proof-latency report, explicit Parallel Validation Fabric feature/proof packet, and coordination-latency report | WP-05 through WP-09 |
| WP-11 | Active issue migration policy | docs/tools | policy for existing open cards and future issue defaults | WP-02 through WP-09 |
| WP-12 | Regression fixtures for process drift | tests/tools | fixtures for SRP drift, stale SORs, skipped closeout, and env/global-state hazards | WP-10, WP-11 |
| WP-13 | Demo matrix and proof coverage | demo | demo matrix, feature-proof coverage, and proof-evidence map | WP-12 |
| WP-14 | Coverage / quality gate | quality | validation gate covering lifecycle, tools, tests, traces, and release blockers | WP-13 |
| WP-15 | Docs + adoption review pass | docs | C-SDLC default-operation docs, tracked-source docs, onboarding updates, and docs-review findings | WP-14 |
| WP-16 | Internal review | review | code/docs/tests/process review packet | WP-15 |
| WP-17 | External / 3rd-party review | review | independent review packet and handoff surface | WP-16 |
| WP-18 | Review findings remediation | review | fixes, finding dispositions, and follow-on routing | WP-17 |
| WP-19 | Next milestone planning | docs | `NEXT_MILESTONE_HANDOFF_v0.91.4.md` refresh and downstream planning update | WP-18 |
| WP-20 | Next milestone review pass | docs | final review pass over next-milestone planning before ceremony | WP-19 |
| WP-21 | Release ceremony | release | evidence package, signed-trace proof, and closeout record | WP-20 |

## Work Packages

The candidate WP sequence above is the canonical work-package table for
v0.91.4. The work-package issue wave is already seeded; later status truth now
depends on sprint execution and closeout rather than issue-number assignment.

## CodeFriend Sidecar Mini-Sprint

The CodeFriend pre-alpha setup work is planned for v0.91.4 as a sidecar
mini-sprint. It is product setup work, not C-SDLC core machinery, and it does
not add or remove any release closeout-tail step.

| ID | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| CF-PRE-00 | CodeFriend pre-alpha site mini-sprint umbrella | docs/product | ordered sidecar sprint state, child issue orchestration, and final handoff | WP-01 |
| CF-PRE-01 | Product repo bootstrap | product/docs | private CodeFriend repo with README, license posture, source map, and no ADL runtime dependency | CF-PRE-00 |
| CF-PRE-02 | Static welcome page | product/frontend | minimal `CodeFriend - Because your code needs a friend. Coming soon from Agent Logic, Inc.` page | CF-PRE-01 |
| CF-PRE-03 | AWS S3, CloudFront, ACM, and Route 53 machinery | product/infra | S3 asset origin, CloudFront HTTPS, ACM certificate path, DNS plan, deployment/rollback docs | CF-PRE-02 |
| CF-PRE-04 | Publication safety, verification, and handoff | product/docs | public-safety review, verification record, blocked/completed handoff, and follow-on routing | CF-PRE-03 |

## Sequencing

- Phase 1: WP-01 through WP-04 establish lifecycle, validator, conductor, and
  editor readiness.
- Phase 2: WP-05 through WP-08 establish transition operation, signed trace,
  merge readiness, and memory handoff.
- Phase 3: WP-09 through WP-12 make sprint execution default-safe and measure
  repeatability, validation-tail, proof-latency, and regression risk.
- Phase 4: WP-13 through WP-21 perform proof, quality, docs/adoption review,
  internal review, external review, remediation, next-milestone planning,
  next-milestone review, and release ceremony.
- Sidecar: CF-PRE-00 through CF-PRE-04 may run after WP-01 without changing the
  C-SDLC release-tail sequence.

## Sequencing Notes

Validator and conductor truth must land before repeatability metrics,
validation-tail/proof-latency measurement, and Parallel Validation Fabric
planning. Otherwise the milestone risks timing an unreliable process and
calling that a proof.
The repeatability work must also account for validation-tail latency: a
five-minute sprint with a long blocking test cycle is useful evidence of the
next bottleneck, not proof that the operating loop is complete.

The sprint-conductor lane should explicitly consume the v0.91.2 and v0.91.3
lessons:

- no next child before closeout truth
- no umbrella closeout without sprint review and state artifact truth
- no isolated-test pass when combined-lane validation is the risk
- no SRP/SOR memory handoff over stale card semantics
- no actor-standing or shard-ownership claim without tracked transition evidence
- no durable C-SDLC proof without tracked signed trace evidence
- no default operation while durable workflow records still live only in local
  `.adl` state
- no release ceremony before proof coverage, quality gate, docs/adoption review,
  internal review, external review, remediation, next-milestone planning, and
  next-milestone review pass have completed in order
- no CodeFriend sidecar launch claim without repo, DNS/HTTPS, publication
  safety, and handoff evidence, or an explicit blocked state

## Acceptance Mapping

- WP-01 -> v0.91.4 opens with validated issue-wave and sprint setup.
- WP-02 through WP-04 -> lifecycle validators, conductor, doctor, and editors
  agree on the C-SDLC card state model.
- WP-05 through WP-08 -> actor standing, shard ownership, evidence, trace,
  merge readiness, and memory handoff are repeatable.
- WP-09 through WP-12 -> sprint execution is default-safe, measured, and
  guarded by drift fixtures. WP-10 must explicitly own Parallel Validation
  Fabric planning/proof rather than leaving it implicit inside repeatability.
- WP-13 through WP-15 -> demo/proof coverage, quality gate, and docs/adoption
  review support the milestone claims.
- WP-16 through WP-18 -> internal review, external review, and remediation are
  completed or truthfully routed.
- WP-19 through WP-21 -> next-milestone planning, next-milestone review, and
  release ceremony complete in order.
- CF-PRE sidecar -> CodeFriend setup is complete, blocked with handoff, or
  routed without becoming C-SDLC core proof.

## Planned Side Issues

The following items remain list-only v0.91.4 side issues or mini-sprints unless
explicitly promoted into the tracked issue wave. Already-seeded sidecars such as
WildClawBench `#3378` and first-birthday readiness `#3377` are no longer part
of this list-only bucket.

- outstanding demo work, including the best available Unity-facing C-SDLC demo
  proof
- benchmarking mini-sprint candidates beyond the already-seeded WildClawBench sidecar
- root README rewrite
- repo cleanup pass
- partial drafting of Medium articles
- HTML editors for planning docs
- first-birthday readiness follow-on work beyond the already-promoted `#3377`

## Exit Criteria

- Every work package has a seeded issue or an explicit routing decision.
- The closeout tail preserves the required sequence and does not skip the
  next-milestone review pass before ceremony.
- Sidecar work remains visibly separate from C-SDLC default-operation proof.
