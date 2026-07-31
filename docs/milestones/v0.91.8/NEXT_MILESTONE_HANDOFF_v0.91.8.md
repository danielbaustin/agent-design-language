# v0.91.8 to v0.92 Handoff

`v0.92` remains the birthday milestone. `v0.91.8` may only hand off exact
reviewed platform truth.

## Required Handoff Contents

- Accepted ADL v2 revision and stable install path.
- Accepted Runtime v3 revision and operational proof.
- Accepted C-SDLC v2 revision and lifecycle proof.
- Selector state and rollback receipt.
- Deleted/retained incumbent-surface disposition.
- WP-14A accepted revision ledger and WP-21 handoff issue ledger.
- Demo and public-claim boundaries.
- Explicit non-claims and blockers.

## Implemented Activation Bridge

WP-21 issue `#4759` implements the activation bridge at
`docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`. The bridge
consumes the WP-14A platform acceptance ledger
`docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md`,
including accepted baseline `11151e0beab02b1667f6505b7f8992bfd47d2f8f` and
the accepted C-SDLC v2, Runtime v3, ADL v2 soak/rollback, and reversible
default-switch merge revisions recorded there.

This handoff therefore exposes one concrete v0.91.8 consumption path for
v0.92 activation planning:

1. accepted platform revisions from WP-14A `#5384`
2. activation-surface dispositions in
   `V092_ACTIVATION_TEST_MAP_v0.91.8.md`
3. v0.92 bridge consumption in
   `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
4. downstream birth-witness and receipt packaging in #4762

The bridge does not claim v0.92 birthday readiness, public launch readiness,
Memory Palace runtime completion, Unity demo proof, or unresolved Runtime v3
Parity-B/C/D proof. Missing proof remains a named blocker or non-claim on the
activation map.

## Review Routing

The release tail must preserve `WP-21 -> WP-21A -> WP-22` ordering before
`v0.92` consumes this handoff. WP-21 owns the exact handoff, Memory Palace,
launch/identity, capability, Adaptive Learning, feature-list, and planning truth;
future v0.91.8 WP-21A `#5355` prepares the next-milestone handoff/review
alignment, and WP-22 reviews that packet before release ceremony work. Current
v0.91.7 WP-21A `#5489` is only preparing this documentation package.

Current blocker and non-claim truth must be consumed explicitly: `#5408` is
closed/remediated via PR #5419, while #4906 remains retained
blocked-with-evidence unless separately dispositioned.

## Non-Claims

This handoff must not claim identity, consciousness, birthday readiness,
production-provider readiness, or public launch readiness unless corresponding
issues have closed with evidence and review.
