# v0.91.6 VPP and PVF Lane-Template Mini-Sprint SEP

Status: `child_wave_complete`
Date: 2026-06-21
Sprint umbrella: `#4332`
Execution mode: `hybrid`
Sprint review path:
`docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md`

This Sprint Execution Packet records the bounded execution contract and final
child-wave truth for the VPP / PVF lane-template mini-sprint. It does not
replace issue-local `SIP -> STP -> SPP -> SRP -> SOR` truth, and it does not
absorb related residue outside the listed child issues and explicitly routed
related issues.

## Sprint Goal

Make validation planning a first-class C-SDLC phase by coordinating:

- per-issue execution metrics
- first-class nested goal accounting design
- VPP / Validation Planning Prompt surfaces
- externalized PVF lane registry truth
- versioned prompt-template integration
- truthful lane assignment and lane-parallelization planning

## Scope Boundary

In scope:

- `#4329`
- `#4331`
- `#4308`
- `#4309`
- inherited already-landed prerequisite slices `#4277`, `#4281`, `#4278`,
  `#4279`, and `#4300`
- umbrella execution, review, and closeout truth for `#4332`

Out of scope:

- full validation-manager rewrite
- retroactive migration of old issues
- silent weakening of failed, skipped, blocked, or pending validation truth
- fabrication of missing token/time telemetry
- unrelated GitHub projection backlog beyond explicitly accounted routing

## Child Issue Wave

| Issue | Role | Final status | Notes |
|---|---|---|---|
| `#4277` | initial PVF lane assignment during issue creation/planning | completed | Closed on 2026-06-20T20:35:01Z; satisfied prerequisite before the umbrella resumed. |
| `#4281` | opportunistic lane-parallelization planning | completed | Closed on 2026-06-20T23:17:13Z; satisfied prerequisite before the umbrella resumed. |
| `#4278` | SPP estimates and SOR actuals | completed | Closed on 2026-06-21T00:11:41Z; provides the core estimate/actual field substrate. |
| `#4279` | variance analysis for estimate misses | completed | Closed on 2026-06-21T01:01:20Z; provides the variance-analysis substrate. |
| `#4300` | sprint umbrella / child issue goal requirement reconciliation | completed | Closed on 2026-06-21T02:43:34Z; establishes supported sprint-goal versus child-goal policy. |
| `#4329` | per-issue execution metrics foundation | merged_closed | PR `#4362` merged at 2026-06-21T05:56:07Z; issue closed at 2026-06-21T05:56:09Z. |
| `#4331` | first-class nested goal accounting design | merged_closed | PR `#4364` merged at 2026-06-21T05:17:32Z; issue closed at 2026-06-21T05:17:33Z. |
| `#4308` | VPP prompt plus externalized PVF lane registry | merged_closed | PR `#4365` merged at 2026-06-21T05:59:18Z; issue closed at 2026-06-21T05:59:19Z. |
| `#4309` | next prompt-template version with VPP/time/token/goal fields | merged_closed | PR `#4366` merged at 2026-06-21T06:33:31Z; issue closed at 2026-06-21T06:35:51Z after non-closing lifecycle merge. |

## Related Issues Accounted For

| Issue | Relationship | Current truth |
|---|---|---|
| `#4196` | adjacent validation-lane routing input consumed by the broader PVF direction | closed on 2026-06-20; treated as satisfied adjacent substrate, not a blocking child of this sprint |
| `#4286` | adjacent PR closing-linkage Rust/PVF follow-on | still open; explicit related residue outside the core child wave and therefore not silently absorbed into `#4332` |

## Recommended Execution Order

The source prompt recommended:

1. `#4329`
2. `#4331`
3. `#4308`
4. `#4309`
5. `#4277`
6. `#4281`

Observed final truth:

1. prerequisite slices `#4277`, `#4281`, `#4278`, `#4279`, and `#4300`
   were already closed before umbrella execution resumed
2. `#4309` published first as draft PR `#4366`
3. `#4329` published next as draft PR `#4362`
4. `#4331` published next as draft PR `#4364`
5. `#4308` published next as draft PR `#4365`
6. the child wave then converged to terminal merged/closed truth across all
   four core issues

This remains an order divergence from the original ideal sequence, but not a
scope or correctness violation because the prerequisite metrics/goal/lane
slices were already landed and the four core children still respected the
intended dependency shape.

## Candidate Parallel Lanes

| Lane | Issues | Candidate reason | Risk to watch |
|---|---|---|---|
| metrics + goal design | `#4329`, `#4331` | Both are design/field-contract heavy and can progress in parallel if they avoid touching the same prompt-template surfaces at the same time. | Shared field vocabulary drift between per-issue metrics and nested goal aggregation. |
| template + lane-registry | `#4308`, `#4309` | The issues are tightly related and can move close together when the template-set boundary is explicit. | Conflicting edits across prompt-template, schema, and validation-routing surfaces. |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| metrics substrate versus goal-hierarchy design | `#4329`, `#4331` | The two issues can remain design-aligned without forcing a single implementation patch if the per-issue metrics layer stays foundational. | Reconcile field names, unknown/not_collected rules, and sprint/issue aggregation notes before publication. |

Actual outcome:

- The child wave executed serially in the observed publication sequence.
- The retained outputs still satisfy the planning intent because the sprint
  never promised autonomous parallel launch; it promised bounded
  parallelization planning and truthful watcher ownership.

## Serial Gates

| Gate | Blocked | Exit condition |
|---|---|---|
| prerequisite metrics/goal/lane gate | core child publication sequence | Satisfied by already-closed `#4277`, `#4278`, `#4279`, `#4281`, and `#4300`. |
| child PR review gate | umbrella closeout | Satisfied; all four core child PRs reached merged/closed truth. |
| related residue gate | final sprint non-claims | `#4286` remains explicit external residue and must not be silently treated as completed. |

## Watcher Policy

- Every child PR-open state in this mini-sprint was watcher-owned work, not a
  reason to pretend the umbrella was complete prematurely.
- The final watch set covered PRs `#4362`, `#4364`, `#4365`, and `#4366` until
  each reached terminal merged/closed truth.
- The umbrella may now claim `child_wave_complete`, but it may not claim that
  related residue such as `#4286` is also complete.

## PVF Notes

- `#4329` establishes the per-issue metric substrate and unknown/not_collected
  truth for later sprint aggregation.
- `#4331` defines the nested goal accounting ownership model with ADL, not
  Codex, owning the hierarchy.
- `#4308` externalizes at least a proving slice of lane selection into a
  machine-readable registry and validates fail-closed behavior.
- `#4309` stages the template-set evolution needed to host VPP and
  time/token/goal fields without silently patching locked prompt semantics.

Validation and closeout continue to preserve failed, skipped, blocked, and
pending lane truth rather than hiding nonterminal states behind aggregate
success claims.

## Sprint Review And Closeout

Sprint review is tracked at
`docs/milestones/v0.91.6/review/V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md`.

The child wave is complete. Remaining work for the umbrella is limited to:

- publishing this retained packet
- merging the umbrella PR
- closing issue `#4332`
- running repo-native local closeout for the umbrella
