# v0.91.5 Next Milestone Handoff

## Metadata

- Milestone: `v0.91.5`
- Handoff issue: `#3581`
- Target downstream milestone: `v0.91.6`
- Planned milestone sequence: `v0.91.6` -> `v0.91.7` -> `v0.92`
- Status: `wp_20_ceremony_active`
- Last updated: `2026-06-17`

## Status

This is the closed WP-19 handoff surface now consumed by the active WP-20
release ceremony. It prepares v0.91.6 to open from tracked bridge evidence
instead of chat memory or implicit readiness, while preserving the planned
sequence through v0.91.7 and then v0.92.

v0.91.5 is not released yet. WP-19 does not perform ceremony or implement
v0.91.6, v0.91.7, or v0.92 work. It records what the next milestones may
consume from the closed review/remediation gates and what remains for ceremony.

## Current Release-Tail Truth

- Sprint 1 through Sprint 3 delivery is materially landed.
- The first internal-review remediation wave closed through `#3899`.
- WP-14 quality gate and WP-15 docs/review alignment are recorded as release-tail inputs.
- The second-pass internal review closed through `#3923`; its findings and
  remediation queue are tracked inputs for this handoff.
- External/third-party review closed through `#3580`.
- Final review remediation and pre-v0.92 readiness routing closed through
  `#3577`, with WP-18 disposition evidence recorded.
- WP-19 handoff closed through `#3581` / PR `#3962`.
- Release ceremony is active through `#3578`.
- Rust/GitHub transport-boundary cleanup follow-on `#3929` is closed and is
  consumed here as release-tail tooling truth rather than executed by WP-19.

## Expected Downstream Milestones

The expected next milestone is v0.91.6.

v0.91.6 should absorb the next planning and hardening tranche: feature-doc
completion, tooling follow-ons, account/setup planning, security and CAV
follow-through, resilience, curiosity, memory-palace planning, CodeFriend v1
planning, and remaining pre-v0.92 bridge documentation.

v0.91.7 is the planned second pre-v0.92 bridge tranche. It should absorb the
remaining bridge work that does not belong in the first v0.91.6 tranche, rather
than forcing v0.91.6 to become too broad or opening v0.92 with hidden bridge
debt.

v0.92 remains the later first true Gödel-agent birthday and activation
milestone. It should consume v0.91.5 closeout, v0.91.6/v0.91.7 bridge
completion, the final pre-v0.92 readiness ledger, and the first-birthday
readiness packet from `#3377`.

## Handoff Gate

Before v0.91.5 ceremony may proceed, this handoff must be reviewed against the
following gate:

| Gate | Current route | Required disposition before ceremony |
| --- | --- | --- |
| GitHub transport-boundary cleanup | `#3929` | closed and consumed as release-tail tooling truth |
| second-pass internal review | `#3923` | closed; findings register and dispositions consumed |
| external / third-party review | `#3580` | closed; external review handoff consumed |
| final remediation and pre-v0.92 routing | `#3577` | closed; WP-18 disposition consumed |
| next-milestone handoff | `#3581` | closed through PR `#3962`; this document and related release-tail docs reflect current truth |
| release ceremony | `#3578` | active; packages and publishes v0.91.5 release truth |

Operational interpretation: WP-19 review and handoff confirmation are complete.
The remaining work is WP-20 ceremony publication and final closeout. If new
implementation work appears after this point, it must be recorded as a review
finding or follow-on issue rather than hidden inside WP-20.

## Required Handoff Inputs

WP-19 consumes these inputs:

- v0.91.5 release evidence and quality-gate status.
- First internal-review findings register and remediation queue.
- Second-pass internal-review findings register and remediation queue from
  closed `#3923`.
- External/third-party review output from closed `#3580`.
- Final remediation and pre-v0.92 routing output from closed `#3577`.
- Provider/model matrix and OpenRouter disposition.
- Multi-agent completion or blocker truth.
- Public prompt packet/export/redaction disposition.
- Demo and Unity Observatory readiness disposition.
- [V092_ACTIVATION_TEST_MAP_v0.91.5.md](V092_ACTIVATION_TEST_MAP_v0.91.5.md).
- `#3377` first-birthday readiness packet.
- Closed bridge-planning tranche issues `#3800` and `#3801`, which own the
  pre-v0.92 bridge-doc routing that this handoff consumes rather than restates
  from scratch.
- Closed Rust/GitHub transport-boundary follow-on `#3929`, consumed as
  release-tail tooling cleanup before ceremony claims the implementation tail is
  clean.

## v0.91.6 Opening Inputs

When v0.91.6 opens, WP-01 should start from:

1. `docs/milestones/v0.91.6/README.md`
2. `docs/milestones/v0.91.6/WBS_v0.91.6.md`
3. `docs/milestones/v0.91.6/SPRINT_PLAN_v0.91.6.md`
4. `docs/milestones/v0.91.6/WP_ISSUE_WAVE_v0.91.6.yaml`
5. feature-doc planning package for all known pre-v0.92 bridge surfaces
6. tooling follow-on queue from v0.91.5 release-tail execution
7. security/CAV follow-through queue
8. resilience, curiosity, and Memory Palace planning inputs
9. CodeFriend v1 and adapter v2 planning inputs
10. `#3902` account/setup planning for `agent-logic.ai`
11. first-birthday readiness inputs from `#3377`, consumed as planning evidence
    rather than as v0.92 implementation approval

The v0.91.6 opening issue should verify that every known pre-v0.92 activation
surface is complete, blocked, deferred, or explicitly routed into v0.91.6 or
v0.91.7 before v0.92 begins.

## v0.91.7 Second-Tranche Rule

Use v0.91.7 as the planned second bridge tranche after v0.91.6. It is not an
emergency patch created after v0.92 starts.

The default second-tranche candidates are:

- remaining feature docs that are required before v0.92
- unresolved security/CAV follow-through
- remaining resilience or curiosity implementation planning
- Memory Palace context-problem work that is not mature enough for v0.91.6
- CodeFriend v1 proof planning that depends on adapter v2 readiness
- any unresolved review finding that is not release-blocking for v0.91.5 but is
  blocking for v0.92 activation

## v0.92 Opening Inputs

When v0.92 eventually opens, WP-01 should start from:

1. `docs/milestones/v0.92/README.md`
2. `docs/milestones/v0.92/WBS_v0.92.md`
3. `docs/milestones/v0.92/SPRINT_v0.92.md`
4. `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
5. `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
6. `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
7. `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
8. v0.91.5 closeout and final pre-v0.92 routing from `#3577`
9. v0.91.6 and v0.91.7 closeout truth
10. first-birthday readiness from `#3377`

## Activation Surfaces To Preserve

The v0.92 activation map names these bridge surfaces. WP-19 must preserve their
routing through v0.91.6 and v0.91.7 rather than silently compressing them into
release notes:

- AEE completion
- Memory / ObsMem handoff
- ACP / cognitive profiles
- provider/model matrix
- Observatory / Unity readiness
- ACIP / provider communications
- public prompt records
- Memory Palace context topology
- Guilds / MVP governance route
- CodeFriend v1 / adapter v2 route before v0.95
- security and CAV threat-model follow-through

## Follow-On Routing

The following work is not implemented in WP-19, but must remain visible:

- v0.91.6 is the immediate next milestone and absorbs account/setup,
  tooling-planning, security, feature-doc, resilience, curiosity, and
  memory/context planning follow-ons, including `#3902` for the
  `agent-logic.ai` AWS account plan.
- v0.91.7 is the planned second bridge tranche after v0.91.6.
- v0.92 owns first-birthday activation only after v0.91.6/v0.91.7 close the
  required bridge work and `#3377` readiness is consumable.
- v0.93 owns later governance expansion beyond the MVP-critical guild route.
- v0.95 remains MVP convergence, not first implementation of major product
  surfaces.
- Aptitude Atlas remains post-v0.95 productization; v0.95 consumes capability
  evidence only.

## WP-20 Review Requirement

Before v0.91.5 release, review this handoff and confirm:

- v0.91.6 dependencies are explicit, with v0.91.7 second-tranche criteria recorded;
- no first-birthday implementation is claimed early;
- activation surfaces are complete, blocked, or deferred;
- closed bridge tranches `#3800` / `#3801` have been consumed as routing inputs,
  not left as orphan planning work;
- AEE completion has a named v0.91.6/v0.91.7 owner/proof route instead of being
  deferred implicitly to v0.95 convergence;
- `#3929` is consumed as closed release-tail tooling truth;
- remaining v0.92 go/no-go work stays explicitly routed through `#3577`,
  `#3377`, v0.91.6, and v0.91.7 as needed;
- residual risks have owners.

## Release Ceremony Readiness Note

WP-20 may begin only after this document reflects the final closed state of
`#3929`, `#3923`, `#3580`, and `#3577`.

Those surfaces are closed and consumed by WP-19. Ceremony can proceed from
release evidence and release notes. If new contradictions appear, WP-20 should
record a blocked release handoff rather than publishing v0.91.5.

After ceremony completes, schedule a 15-minute operator break before starting
v0.91.6 WP-01. The break is an operational handoff pause, not a change to the
v0.91.6 -> v0.91.7 -> v0.92 milestone sequence.

## Non-Claims

This handoff does not claim:

- v0.91.5 is released;
- v0.91.6 is approved to open;
- v0.91.7 is complete;
- v0.92 is approved to open;
- the first true Gödel-agent birthday is implemented;
- second-pass internal review or external review remains open;
- `#3929` remains open;
- bridge planning is still pending in closed `#3800` / `#3801`.

`#3800`, `#3801`, `#3929`, `#3923`, `#3580`, `#3577`, and `#3581` are closed
routing inputs. Final ceremony truth remains open release-tail work until
`#3578` closes.
