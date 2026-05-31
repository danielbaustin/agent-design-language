# v0.91.4 Quality Gate

## Status

`blocked`

## Gate Decision

`v0.91.4` is not ready to advance past the quality gate yet.

This is a successful truthful result for WP-14: the gate is now explicit about what is already proven, what is routed out of scope, and what still blocks Sprint 4 from moving into the internal-review tail.

## Scope Boundary

This gate applies to Sprint 4 (`#3362`) and its ordered closeout children `#3363` through `#3371`.

Work outside Sprint 4 should not be treated as a `v0.91.4` release blocker unless a later WP explicitly pulls it back in.

Routed out of the `v0.91.4` Sprint 4 release path:

- multi-agent stabilization and follow-on proving work
- provider/model matrix follow-ons
- public prompt-record follow-ons
- demo-readiness follow-ons outside the Sprint 4 closeout path
- `#3526` / draft PR `#3528` audit work on feature-completion dates and AEE
  closure unless a later Sprint 4 issue explicitly reclaims it as a release
  blocker

Non-core sidecar work that is not required as Sprint 4 release proof:

- CodeFriend sidecar completion
- WildClawBench sidecar completion

## Inputs Reviewed

- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
- `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
- `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
- `docs/milestones/v0.91.4/WP_EXECUTION_READINESS_v0.91.4.md`
- `docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md`
- GitHub issue truth for Sprint 4 closeout-tail issues `#3365` through `#3371`, checked at `2026-05-30T19:58:36Z`

## Gap-Analysis Role

This quality gate is also the bounded gap-analysis surface for the current
`v0.91.4` milestone state.

It does not re-review every implementation artifact from first principles.
Instead, it compares:

- the current Sprint 4 release claim
- the landed proof surfaces already available in the repo
- the remaining open release-tail work
- the milestone docs that define release readiness

Against those inputs, the gate classifies:

- what is already strong enough to count as release input
- what remains a true blocker
- what is routed out of `v0.91.4`
- what is a quality concern but not yet a release blocker

## Landed Evidence Consumed

The gate can already rely on the following proof inputs:

- lifecycle validator / doctor / conductor / editor hardening from the earlier issue wave
- software-development-polis proof
- merge-readiness proof
- ObsMem transition-memory proof
- five-minute sprint repeatability plus validation-tail / proof-latency evidence
- Parallel Validation Fabric landing, including explicit docs-only and release-gate separation
- process-drift regression proof
- WP-13 demo/proof coverage refresh and best-available showcase routing

## Validation Summary

WP-14 did not rerun broad runtime, browser, or release-tail suites.

That is intentional and truthful.

This issue is a release-readiness gate and blocker-disposition surface. It consumes already-landed proof and current milestone state rather than inventing test theater.

Focused validation used for this issue should remain bounded to:

- milestone-doc patch hygiene
- structured-prompt validation for local SRP/SOR updates
- GitHub issue-state inspection for the Sprint 4 closeout tail

Broad validation remains a release-tail requirement, not a prerequisite for writing the gate record.

## Pass Surfaces

The following gate dimensions are currently in a good enough state to count as available release input rather than blockers by themselves:

1. `WP-13` demo/proof coverage is complete enough for release-tail planning.
2. Earlier core C-SDLC control-plane proof is present and reviewer-discoverable.
3. Parallel Validation Fabric is no longer only implicit repeatability residue; it has explicit landed proof surfaces.
4. Remaining non-Sprint-4 proving work has a truthful route to `v0.91.5` rather than silently bloating `v0.91.4`.

## Quality Gaps Identified

The current milestone-quality picture has four distinct categories:

1. release blockers still open in Sprint 4
2. durable-proof gaps still preventing ceremony truth
3. routed bridge work that should not distort `v0.91.4`
4. maintenance-quality gaps that should be tracked but are not themselves release blockers yet

This gate records all four so the milestone does not confuse unfinished quality
work with either release proof or unrelated future cleanup.

## Required Blockers

The milestone remains blocked for the reasons below.

### B1. Sprint 4 closeout tail is still open.

The following required release-tail issues remain open:

- `#3365` Docs + adoption review pass
- `#3366` Internal review
- `#3367` External / 3rd-party review
- `#3368` Review findings remediation
- `#3369` Next milestone planning
- `#3370` Next milestone review pass
- `#3371` Release ceremony

Open `v0.91.4` work outside that Sprint 4 closeout tail should not be treated
as a release blocker by this gate unless explicitly re-routed back into the
Sprint 4 lane. At the current snapshot, that includes:

- `#3526` `[v0.91.4][docs] Audit ADL feature completion dates and AEE closure`
  with draft PR `#3528`

State snapshot recorded at `2026-05-30T19:58:36Z`:

| Issue | State |
| --- | --- |
| `#3365` | `OPEN` |
| `#3366` | `OPEN` |
| `#3367` | `OPEN` |
| `#3368` | `OPEN` |
| `#3369` | `OPEN` |
| `#3370` | `OPEN` |
| `#3371` | `OPEN` |

The quality gate must not approve release before those ordered tail steps are complete.

### B2. Durable workflow-state migration proof is not complete.

The milestone still expects durable C-SDLC truth to move out of local-only `.adl` state into tracked review/evidence surfaces. That migration is not complete enough yet to call release truth durable.

This remains a blocker because the release claim is that C-SDLC is the default lane, not merely a local operator habit.

### B3. Signed trace and release-evidence convergence are incomplete.

The release tail still lacks a completed signed-trace verification surface and final release-evidence packet tying feature proof, review proof, residual risk, and routed follow-on work together.

Until that evidence exists, the gate must stay blocked.

### B4. Docs/adoption and review proof are incomplete.

The milestone does not yet have:

- the docs/adoption review packet
- the internal review packet
- the external / third-party review packet
- the remediation/disposition record

That means the release claim has not yet survived the required review sequence.

### B5. Release-readiness inputs still need final scope alignment.

The current release-plan and checklist surfaces still contain broader release-tail checklist items for sidecar completion/truthful routing alongside the narrower Sprint 4 closeout path.

Under current milestone truth, Sprint 4 is the release-blocking lane and the `v0.91.5` bridge is explicitly called out only for the multi-agent/provider/public-prompt/demo-readiness follow-on set.

That means release-readiness surfaces should be kept aligned with the narrowed Sprint 4 boundary before ceremony, and any sidecar expectations should be named explicitly as complete, blocked, or routed rather than treated as implicit release blockers.

## Non-Blocking Quality Concern

### Q1. The Rust maintainability hotspot tracker should be referenced from its maintained manual surface.

The current Rust maintainability tracker is maintained at:

- `.adl/reports/manual/rust_module_watch_list.md`

This is a real quality/maintenance signal, but it is not a `v0.91.4` Sprint 4
release blocker by itself because:

- WP-14 is a release-readiness gate, not a broad maintainability refactor lane
- the maintained tracker does not currently invalidate the landed Sprint 4 proof surfaces
- aligning milestone/release surfaces to reference the maintained tracker should be handled as explicit follow-on maintenance work rather than hidden inside the release-tail gate

Current posture: `follow_on_needed_non_blocking`

## Manual Control-Tower Checks

WP-14 should also record the current state of a few recurring milestone-quality
checks, even when they are performed manually rather than through a dedicated
future workflow.

Manual checks performed on 2026-05-30:

1. `Issue closeout truth`
   - command:
     `bash adl/tools/check_milestone_closed_issue_sor_truth.sh --version v0.91.4`
   - result:
     `PASS check_milestone_closed_issue_sor_truth version=v0.91.4 checked=77`
   - interpretation: the earlier broad stale closed-issue `SOR` drift is no
     longer an active milestone-level blocker
2. `Rust module watch-list review`
   - source: `.adl/reports/manual/rust_module_watch_list.md`
   - result: tracker updated on `2026-05-30`; top current review targets are:
     - `adl/src/runtime_v2/contract_market_demo.rs` (`1382` LoC)
     - `adl/src/cli/tests/run_state/persistence.rs` (`1381` LoC)
     - `adl/src/long_lived_agent.rs` (`1377` LoC)
   - interpretation: maintainability hotspots remain real and visible, but
     they are being tracked and do not by themselves block Sprint 4 closeout
3. `Internal review plan existence/readiness`
   - source:
     `docs/milestones/v0.91.4/review/internal_review/V0914_INTERNAL_REVIEW_PLAN_2026-05-30.md`
   - result: present, scoped, and reviewer-facing; status remains
     `planned_not_started`
   - interpretation: the internal review surface exists and is structurally
     ready, but WP-16 is still a real open blocker because execution has not
     started
4. `Test coverage gap analysis`
   - artifact:
     `docs/milestones/v0.91.4/review/quality_gate/V0914_TEST_COVERAGE_GAP_ANALYSIS_2026-05-30.md`
   - result: dedicated packet now exists; verdict `partial`
   - interpretation: the missing artifact gap is fixed, but final
     release-authoritative coverage proof still remains part of the open
     release-tail work
5. `PVF lane health`
   - source snapshot: PR `#3527` check state at `2026-05-30T20:48:53Z`
   - result:
     - `adl-ci`: `pass`
     - `adl-coverage`: `pass`
     - `adl-slow-proof`: `skipping`
   - interpretation: current docs-only release-tail work is being classified
     truthfully enough that broad runtime proof is not being forced into this
     PR path; PVF remains usable as a release input rather than a fresh blocker
6. `Changed-file risk review`
   - current open Sprint 4 PRs: only `#3527`
   - current tracked WP-14 delta:
     - `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
     - `docs/milestones/v0.91.4/review/quality_gate/V0914_TEST_COVERAGE_GAP_ANALYSIS_2026-05-30.md`
     - `docs/milestones/v0.91.4/review/quality_gate/V0914_TEST_RUNTIME_REGRESSION_CHECK_2026-05-30.md`
     - `docs/milestones/v0.91.4/review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md`
   - interpretation: current open release-tail delta is low-risk and docs-only;
     no additional runtime/provider/workflow-control code is presently in flight
     in the Sprint 4 release-tail lane
7. `Test runtime regression check`
   - artifact:
     `docs/milestones/v0.91.4/review/quality_gate/V0914_TEST_RUNTIME_REGRESSION_CHECK_2026-05-30.md`
   - result: dedicated packet now exists; verdict `partial`
   - interpretation: runtime-regression posture is now explicitly recorded, but
     a richer cross-milestone comparison workflow still remains future process
     work
8. `Prompt/card lifecycle audit`
   - manual sample: `pr doctor --mode ready` for `#3365` through `#3371`
   - result: all sampled release-tail issues report `ready_status: PASS`,
     `pr_run_readiness: ready`, and `lifecycle_state: pre_run`
   - interpretation: remaining Sprint 4 issue bundles are structurally ready;
     the current blocker is release-tail sequencing/execution, not missing card
     scaffolding
9. `PR stack/base hygiene`
   - current open release-tail PRs: only `#3527`
   - result: base `main`, merge state `CLEAN`, checks green, draft still open
     at `2026-05-30T20:48:53Z`
   - interpretation: current stack/base hygiene looks clean; no hidden conflict
     or stale-base problem is visible in the active release-tail PR snapshot
10. `Docs truth / staleness scan`
   - source surfaces:
     - `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
     - `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
     - `docs/milestones/v0.91.4/RELEASE_NOTES_v0.91.4.md`
   - result: still treated as partial gap by the local milestone gap review;
     some proof/demo/release wording remains too conservative or stale relative
     to already closed work
   - interpretation: this remains a real docs/adoption cleanup concern for
     WP-15 rather than a reason to reopen the earlier implementation wave
11. `ADR / decision readiness`
   - source surfaces:
     - `docs/milestones/v0.91.4/ADR_PLAN_v0.91.4.md`
     - `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md`
   - result: candidate ADR packet exists, decisions log exists, and candidate
     boundaries are explicit
   - interpretation: architectural decision routing appears present and
     reviewable; no missing ADR-plan blocker was found in this manual pass
12. `Demo / proof artifact readiness`
   - source surfaces include:
     - `review/demo_showcase/`
     - `review/merge_readiness/`
     - `review/obsmem_transition_memory/`
     - `review/browser_automation/`
   - result: substantial reviewer-facing proof packets exist for the current
     best showcase, browser proof, merge-readiness, and ObsMem handoff surfaces
   - interpretation: demo/proof inputs are materially present, but the milestone
     still does not claim a polished great demo for every feature
13. `Security / privacy / redaction pass`
   - artifact:
     `docs/milestones/v0.91.4/review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md`
   - result: dedicated packet now exists; verdict `pass_with_caveats`
   - interpretation: no obvious tracked secret or host-path leakage was found in
     the reviewed public milestone/review surfaces
14. `Follow-on issue hygiene`
   - source truth:
     - Sprint 4 umbrella `#3362` and children `#3363`-`#3371`
     - v0.91.5 bridge split and routed follow-on work
   - result: the remaining release tail is explicitly tracked, and non-Sprint-4
     work is already routed away from `v0.91.4`
   - interpretation: known large follow-on work is not floating unplanned;
     routing hygiene is substantially better than it was earlier in the
     milestone

## Blockers That Are No Longer Required Here

The gate explicitly does **not** block on the following anymore:

- live provider-backed multi-agent completion
- Unity-facing best-demo completion
- CodeFriend sidecar completion as default-operation proof
- WildClawBench sidecar completion as default-operation proof

Those may still be useful follow-on surfaces. They are not required to close `v0.91.4`.

## Treatment Of Specific Proof Dimensions

### Signed traces

Required before release closeout. Current posture: `blocked_pending_release_tail_evidence`.

### Tracked evidence

Required before release closeout. Current posture: `blocked_pending_durable_workflow_record_migration`.

### PVF

Available as landed proof input. Current posture: `pass_as_release_input`, not a remaining stabilization blocker for `v0.91.4`.

### Validation-tail / proof-latency posture

Available as landed evidence from the repeatability/PVF work. Current posture: `pass_as_release_input`, with the reminder that slow or deferred proof must remain explicit rather than being hidden behind aggregate green summaries.

## Required Remediation Before Internal Review

Before WP-16 internal review can be considered ready, the milestone should at minimum have:

1. docs/adoption review package completed under WP-15
2. release-readiness surfaces aligned to the Sprint 4-only scope boundary
3. durable workflow-state migration proof made explicit enough to review
4. signed-trace / release-evidence expectation made concrete for the remaining tail

## Follow-on Routing

- Release-tail blockers remain in Sprint 4 and should flow through WP-15 to WP-21.
- Non-Sprint-4 bridge work remains routed to `v0.91.5`; sidecar work remains non-core and should be classified explicitly as complete, blocked, or routed if later release-tail docs still mention it.
- Rust hotspot / refactoring-tracker reference alignment should be routed as separate maintenance planning rather than silently absorbed into the Sprint 4 gate.
- Carry these new WP-14 packet types forward into the future v0.91.5
  control-tower workflow so they become standard outputs instead of ad hoc
  one-offs.
- Any newly discovered defect that widens beyond the gate record should route into WP-18 or a follow-on issue rather than being hidden in the quality-gate doc.

## Gate Outcome

`blocked`

This gate should move to `pass` only when the Sprint 4 closeout tail is complete, durable workflow-state truth is reviewable from tracked repo state, signed trace / release evidence is present, and the remaining docs/review/remediation tail has actually landed.
