# WBS - v0.90.2

## Work Package Shape

v0.90.2 should use the standard 20-WP shape so it is not under-managed. The
first sprint should verify the v0.90.1 inheritance and define the CSM run
contract before implementation begins.

| WP | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- |
| WP-01 | Milestone package and issue wave | Promote and finalize this planning package | tracked v0.90.2 docs and issue cards | v0.90.1 closeout |
| WP-02 | Runtime v2 inheritance and compression audit | Verify actual v0.90.1 artifacts, open CSM Observatory work, and compression readiness | inheritance and compression report | WP-01 |
| WP-03 | CSM run packet contract | Define the first-run packet, fixture, and review target for `proto-csm-01` | CSM run contract and fixture | WP-02 |
| WP-04 | Invariant and violation artifact contract | Map inherited invariants and define stable violation artifacts | invariant map, violation schema, fixtures | WP-03 |
| WP-05 | Manifold boot and citizen admission | Boot `proto-csm-01` and admit two worker citizens with traceable identity handles | boot/admission artifacts and tests | WP-03, WP-04 |
| WP-06 | Governed episode and resource scheduling | Run one governed episode under resource pressure and explain scheduler choice | scheduling artifact and trace | WP-05 |
| WP-07 | Freedom Gate mediation | Route one non-trivial citizen action through the Freedom Gate | decision artifact and tests | WP-05, WP-06 |
| WP-08 | Invalid action rejection | Reject one invalid action through normal kernel/policy flow | violation packet and negative test | WP-04, WP-07 |
| WP-09 | Snapshot, rehydrate, and wake continuity | Snapshot the local manifold, rehydrate, and wake citizens without duplicate activation | continuity proof packet | WP-05-WP-08 |
| WP-10 | Observatory packet and operator report integration | Emit first-run visibility packet and operator report from the CSM run artifacts | Observatory packet/report | WP-03, WP-05-WP-09 |
| WP-11 | Recovery eligibility model | Define and test safe-resume versus reject/quarantine rules against first-run artifacts | recovery decision record | WP-08, WP-09 |
| WP-12 | Quarantine state machine | Implement quarantine state and evidence preservation for unsafe recovery | quarantine artifact and tests | WP-11 |
| WP-13 | Governed adversarial hook and hardening probes | Add one bounded adversarial probe plus duplicate activation, snapshot integrity, and trace/replay gap negative probes | adversarial hook packet and hardening proof packets | WP-11, WP-12 |
| WP-14 | Integrated first CSM run demo | Produce one end-to-end first-run and hardening proof packet | integrated CSM run demo packet | WP-05-WP-13 |
| WP-15 | Docs, quality, and review convergence | Align feature docs, demo matrix, README, quality posture, and reviewer entry surfaces | coherent docs/review package and quality report | WP-14 |
| WP-16 | Internal review | Review claims, artifacts, compression, quality, and boundaries | findings-first internal review packet | WP-15 |
| WP-17 | External / 3rd-party review | Execute external review against the stabilized v0.90.2 proof package | completed external review record | WP-16 |
| WP-18 | Review findings remediation | Fix accepted findings or defer explicitly with owner and rationale | patches, closure notes, and tracked deferrals | WP-16, WP-17 |
| WP-19 | Next milestone planning | Finalize v0.91/v0.92 handoff and next-milestone planning before release closeout | next-milestone planning and handoff packet | WP-18 |
| WP-20 | Release ceremony | Complete tag/release ceremony | release closure | WP-19 |

## Compression Candidate

v0.90.2 should inherit the v0.90.1 compression improvements. Compression is
allowed only when it preserves evidence, reviewability, and release truth.

Compression target:

- WP-02 through WP-04 should make execution mechanically crisp before coding
- WP-10 can run in parallel with the runtime implementation once the packet
  contract is stable
- WP-13 should stay bounded to one governed adversarial hook plus concrete
  hardening probes, not expand into a full security-ecology sprint
- WP-15 through WP-19 must preserve the v0.87.1 closeout pattern:
  docs/review convergence, internal review, external / 3rd-party review,
  findings remediation, next-milestone planning, then ceremony
