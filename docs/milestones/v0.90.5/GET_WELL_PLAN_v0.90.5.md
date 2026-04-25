# Get-Well Plan - v0.90.5

## Purpose

`v0.90.5` must continue the Governed Tools v1.0 implementation, but it also
must avoid repeating the `v0.90.4` validation-cost failure mode. This get-well
plan is the milestone-root pointer to the runtime and CI recovery work that
supports the milestone without replacing its core scope.

## Status

Active execution-support wave for `v0.90.5`.

This is separate from the canonical WP wave. It does not consume WP numbering
and must not reorder the Governed Tools v1.0 implementation state machine.

- WP-01 records the plan in the tracked milestone package.
- GW-00 produces the runtime baseline, budget, tracking artifact, and recovery
  wave coordination.
- GW-01 through GW-05 are the bounded runtime-reduction slices opened from the
  detailed source plan.
- WP-20, Coverage / quality gate, records final disposition and evidence from
  the get-well wave before release closeout.
- WP-25 captures any follow-on runtime-reduction work that remains after
  `v0.90.5`.

## Source Plan

Detailed source plan:

- [Test Runtime Reduction Plan](ideas/TEST_RUNTIME_REDUCTION_PLAN_v0.90.5.md)

The source plan identifies the remaining heavyweight proof-family test problem:

- repeated expensive setup across runtime proof families
- duplicated CLI/demo proof-matrix cost
- authoritative coverage wall time dominated by concentrated hotspot families

## Operating Rule

The get-well plan can create bounded follow-on slices when validation cost is a
real blocker, but those slices must preserve the Governed Tools v1.0 proof
boundaries.

It must not:

- replace UTS, ACC, compiler, policy, executor, demo, or review work
- weaken negative safety, redaction, authority, or denial evidence
- create a hidden parallel milestone
- turn `v0.90.5` into a CI-only milestone

## Open Get-Well Wave

| Slice | Issue | Scope |
| --- | --- | --- |
| GW-00 | #2592 | runtime baseline, budget, tracking artifact, and recovery-wave coordination |
| GW-01 | #2593 | collapse external-counterparty proof-family tests |
| GW-02 | #2594 | collapse private-state observatory proof-family tests |
| GW-03 | #2595 | collapse delegation-subcontract proof-family tests |
| GW-04 | #2596 | collapse contract-market and resource-stewardship proof-family tests |
| GW-05 | #2597 | shrink CLI and demo proof-matrix tail |

The GW slices should run as early as practical, ideally before WP-02 starts
accumulating more validation cost. They are execution-support work, not
additional Governed Tools feature WPs.

## WP-20 Disposition Requirement

WP-20 must record one of these outcomes:

- the opened GW slices completed and their measurable effect is recorded
- specific GW slices were deferred with issue references and rationale
- remaining runtime-reduction work was deferred to the next milestone with
  issue references and rationale

No release closeout should claim that the validation-cost problem is solved
unless the coverage/quality evidence supports that claim.
