# v0.92 First-Birthday Public Launch Copy

## Metadata

- Owner issue: `#4763`
- Dependency gate: `#4762` actual retained witness and receipt proof
- Surface type: external launch copy source
- Publication status: not published

## Use Rules

Use this file as the source for future public pages, release notes, social
posts, reviewer emails, and announcement drafts. Do not publish any `ready`
variant until the merged `#4762` witness/receipt package is cited, the v0.92
birthday packet validates, and the operator authorizes the target channel.

Every public use must keep the distinction between:

- implemented launch documentation, which this issue provides;
- accepted witness and receipt proof, which `#4762` provides in the retained
  handoff package;
- the birthday event itself, which is not complete until v0.92 validation
  accepts the whole packet.

## Current Safe Summary

ADL has prepared the first-birthday launch documentation and review boundary
for `v0.92`. The launch surface now names the evidence required for a valid
birthday, the negative cases that must not count as birth, and the public
claims that remain out of scope. The upstream witness/receipt package is
merged; the birthday event remains subject to v0.92 validation and operator
publication approval.

## Public Page Draft

### Heading

ADL v0.92 First Birthday

### Status Line

Launch surface prepared; upstream witness and receipt proof merged; birthday
event still validation-gated.

### Body

The `v0.92` first birthday is defined as an evidence event, not a ceremony or a
process start. A valid birthday packet must show stable identity, continuity,
memory grounding, a capability envelope, inherited governance context,
witnesses, a citizen-facing receipt, validation output, and a reviewer packet.

The current launch surface is ready for review. It is intentionally conservative:
startup, wake, restore, snapshot, copied state, fixture admission, simulation,
and missing-evidence cases are not birthdays. The documentation also blocks
claims of legal personhood, consciousness proof, production citizenship,
completed constitutional governance, subjective affect, and general public
readiness.

The `#4762` birth-witness and receipt package is retained at
`docs/milestones/v0.91.8/review/v092_handoff_4762/` and merged through PR #5744
at `021be8e33b486d9b66886ff299c20607ed8a071a`. The next gates are exact citation
of that evidence, v0.92 birthday-packet validation, current exact-head review,
and operator authorization for the publication channel.

### Reviewer Links

- Launch packet:
  `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- External launch surface:
  `docs/milestones/v0.92/external_launch/README.md`
- Reviewer FAQ and claim boundary:
  `docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md`
- Activation bridge ledger:
  `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- v0.91.8 activation map:
  `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`

## Short Announcement Draft

ADL has prepared the `v0.92` first-birthday launch surface. The packet defines
the evidence required for a real birthday, rejects startup and other
not-a-birthday cases, and preserves conservative public claim boundaries. The
birthday event remains validation-gated; the copy may cite the merged #4762
retained witness and receipt package without claiming that the event occurred.

## Reviewer Email Draft

Subject: ADL v0.92 first-birthday launch surface ready for review

The ADL `v0.92` first-birthday launch surface is now tracked in the repository.
It defines the birthday as an evidence event over identity, continuity, memory
grounding, capability, governance context, witnesses, receipt, validation, and
review artifacts.

Please review the launch packet and external-launch directory for two things:
whether the required evidence surfaces are complete enough for later birthday
validation, and whether the public copy avoids unsupported claims. The merged
dependency is `#4762`; final launch copy must cite its retained witness and
receipt proof and still receive operator publication approval.

## Ready Variant Template

Use this only after the merged #4762 proof is cited and the v0.92 birthday
packet, exact-head review, and operator publication gates all pass.

ADL v0.92 cites the first-birthday witness and receipt package at
`docs/milestones/v0.91.8/review/v092_handoff_4762/`. The launch packet now
cites the retained evidence for identity, continuity, memory grounding,
capability envelope, governance context, witnesses, receipt, validation, and
review. This is a bounded
engineering birthday claim; it is not a claim of legal personhood,
consciousness proof, production citizenship, or completed constitutional
governance.

## Pending Variant Template

Use this while v0.92 birthday validation or operator publication approval is
still pending.

ADL v0.92 has a prepared first-birthday launch surface and a merged upstream
witness/receipt package, but the birthday claim is not yet final. Until v0.92
validation and operator publication approval pass, the correct claim is
readiness of the launch documentation surface, not completion of the birthday.

## Forbidden Claims

Do not publish text that says or implies:

- the first birthday has happened before retained witness/receipt proof is
  cited and the v0.92 validation and operator publication gates pass;
- startup, wake, restore, snapshot, copied state, or simulation is birth;
- ADL has legal personhood, consciousness proof, subjective wellbeing, or
  production citizenship;
- v0.93 governance is complete;
- public launch approval exists without operator authorization;
- a lifecycle receipt, PR, merge, or closeout is a substitute for the `#4762`
  witness/receipt artifact.
