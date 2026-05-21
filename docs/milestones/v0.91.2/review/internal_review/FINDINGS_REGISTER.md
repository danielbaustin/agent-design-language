# v0.91.2 WP-20 Internal Review Findings Register

## Supersession Status

This file is historical `WP-20` review context. The first `WP-20` packet was
too thin for external handoff and is superseded for readiness decisions by
`docs/milestones/v0.91.2/review/internal_review_full/`. Do not use this file as
standalone approval to start `WP-21`.

## Findings

### P2-1 Retained `#3121` closeout residue remains an explicit release-tail truth gap

Evidence:

- `docs/milestones/v0.91.2/QUALITY_GATE_v0.91.2.md` records closed-issue closeout truth as `partial` because known retained `#3121` residue remains explicitly deferred.
- `docs/milestones/v0.91.2/review/quality_gate/QUALITY_GATE_PACKET_v0.91.2.md` records the same retained closeout-truth caveat.

Impact:

This does not invalidate the UTS benchmark work or WP-18 quality gate, but it must remain visible before release closeout. If it is silently forgotten, the release could claim cleaner lifecycle truth than the records support.

Recommended route:

Route to WP-22 remediation or a bounded follow-on issue before release ceremony.

### P2-2 Release readiness is correctly blocked, but WP-21/WP-22 handoff must preserve non-claims

Evidence:

- `docs/milestones/v0.91.2/RELEASE_READINESS_v0.91.2.md` says accepted
  `WP-20B` findings must be fixed and rechecked before external review.
- `docs/milestones/v0.91.2/QUALITY_GATE_v0.91.2.md` says the current judgment is `NOT_READY`.

Impact:

This was correct lifecycle truth for the first `WP-20` packet, but it is no
longer sufficient. `WP-20B` found accepted blockers that must be fixed and
rechecked before clean external review.

Recommended route:

Carry this finding into the top-level
`docs/milestones/v0.91.2/ADL_v0.91.2_THIRD_PARTY_REVIEW_HANDOFF.md` and
`WP-22` remediation records as a release-tail guardrail.

## Non-Findings Worth Preserving

- WP-17, WP-17A, WP-18, and WP-19 are closed.
- Demo/proof coverage is mapped to concrete milestone evidence surfaces.
- The milestone does not claim release readiness prematurely.
- UTS benchmark historical evidence is separated from runbook/current proof language.
