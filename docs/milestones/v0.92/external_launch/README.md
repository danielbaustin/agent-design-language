# v0.92 First-Birthday External Launch Surfaces

## Metadata

- Milestone: `v0.92`
- Surface owner: `#4763`
- Upstream dependency: `#4762` actual retained birth-witness and receipt proof
- Status: implemented launch-document surface; publication gated
- Last dependency check: `2026-07-31`; #4762 merged through PR #5744 at `021be8e33b486d9b66886ff299c20607ed8a071a`

## Purpose

This directory is the reviewable external-launch surface for the first
birthday milestone. It gives maintainers, reviewers, and later public channels
one bounded source for what may be said, what must still wait for proof, and
which repository evidence a launch claim must cite.

The surface is implemented as repository documentation. It is not a website
deployment, social post, press release submission, or operator approval to
publish. External use remains gated until the launch copy cites the merged
`#4762` retained package, the launch owner records current exact-head review
truth, the v0.92 birthday packet validates, and the operator authorizes the
target channel.

## Surface Inventory

| Surface | Path | Audience | Current use |
| --- | --- | --- | --- |
| Launch copy packet | `PUBLIC_LAUNCH_COPY_v0.92.md` | Maintainers, external reviewers, future public page or announcement owner | Canonical copy source with ready, pending, and forbidden variants. |
| Reviewer FAQ and claim boundary | `REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md` | Internal/external reviewers, publication approver | Review questions, redaction checks, and no-overclaim rules. |
| Milestone launch packet link | `../FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md` | v0.92 WP-01 and birthday implementation owners | Consumes this directory as the concrete launch-doc surface. |
| Activation bridge ledger row | `../V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md` | v0.92 activation reviewers | Records that public launch docs are implemented and may consume merged #4762 proof, while birthday and publication claims remain gated. |
| v0.91.8 activation map row | `../../v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md` | pre-v0.92 release-tail reviewers | Shows how WP-21 hands this surface forward. |

## Publication Gate

Publication is allowed only when all of these are true:

- `#4762` has an accepted exact result for the auditable witness and receipt
  package.
- The launch copy cites the exact retained witness/receipt artifact paths or
  issue result.
- The final publication branch has current review truth at its exact head.
- No text claims legal personhood, consciousness proof, production
  citizenship, completed constitutional governance, subjective affect, or
  general public readiness.
- The operator explicitly authorizes publication in the target channel.

## Current Launch Status

`#4763` provides a real external-launch documentation surface now: the copy,
FAQ, claim boundary, publication gate, and consumption wiring are tracked in
the repo. #4762 supplied the retained witness/receipt package through PR #5744,
but that proof input does not establish the birthday event or authorize
publication.

The correct public posture is therefore:

> ADL has prepared the first-birthday launch surface and review boundary, and
> the upstream witness and receipt package is retained. The birthday event
> remains validation-gated and is not claimed by this surface.

## Evidence Inputs

These repository inputs define the allowed claim boundary:

- `docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md`
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`
- `docs/milestones/v0.91.8/review/v092_handoff_4762/`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`

## Validation Expectations

Changes to this directory should run:

- `git diff --check`
- a link/path scan for every referenced repository path
- a claim-boundary scan for forbidden terms and unsupported readiness claims
- current dependency inspection for `#4762`
- one bounded exact-head review before PR publication
