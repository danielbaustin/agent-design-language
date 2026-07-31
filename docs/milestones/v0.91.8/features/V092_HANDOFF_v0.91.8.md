# v0.92 Handoff Feature

The `v0.92` handoff feature is an exact-revision consumption packet, not
birthday implementation.

It must name accepted revisions, stable install paths, rollback receipts,
claim boundaries, non-claims, and unresolved blockers. WP-21 `#5362` owns this
packet after consuming WP-14A platform acceptance.

The activation portion is implemented by #4759 in
`docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`. That bridge
consumes the accepted WP-14A baseline
`11151e0beab02b1667f6505b7f8992bfd47d2f8f` from
`docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md` and
routes each v0.92 activation input to an accepted platform input, a WP-21 owner,
an explicit blocker, or a non-claim. The v0.92 ledger
`docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md` is the
downstream consumption surface; #4762 consumes this activation bridge before it
authors birth-witness and receipt evidence.

Owning issues must remain visible in the packet: `#5352` prepares the exact
handoff; `#4758`, `#4759`, and `#4761` own launch/activation/capability;
`#4760` and `#5007` own Memory Palace; `#4762` and `#4763` own identity and
birthday documentation; and `#5107` owns Adaptive Learning planning.
`#5355` owns the future
v0.91.8 WP-21A handoff review alignment, and `#5359` owns release-tail
closeout truth. Do not confuse future v0.91.8 WP-21A `#5355` with current
v0.91.7 WP-21A `#5489`.

#4762 contributes the auditable birth-witness and receipt handoff package at
`docs/milestones/v0.91.8/review/v092_handoff_4762/`. WP-21 and v0.92 consumers
may cite that directory for the witness/receipt row, but must preserve its
explicit `birth_event_status: not_claimed` boundary until a future v0.92 birth
packet supplies the required identity, continuity, memory, capability,
witness, receipt, activation trace, validation output, and reviewer evidence.
