# Issue 5852 Design: v0.92 Release Ceremony

Status: design-time ready; ceremony waits for a passing WP-29 review.

## Authority And Sources

Issue #5852 and WP-30 own the final v0.92 evidence package, release notes,
annotated tag/release ceremony, sprint rollup, and milestone closeout. Inputs
are the exact WP-29-approved closeout sequence, all terminal child/sprint truth,
the passing quality/review/remediation chain, final docs and launch packages,
`docs/milestones/v0.92/RELEASE_PLAN_v0.92.md`, release notes, checklist, handoff,
and `adl/tools/release_ceremony.sh` plus its focused tests.

## Outcome Contract

Assemble a release evidence manifest that binds every release claim to exact
implementation, validation, review, merge, and terminal evidence. Reconcile
final notes and checklist from landed truth only. At the exact approved commit,
execute the reviewed split-step ceremony: preflight; annotated `v0.92` tag;
tag push; draft GitHub release; release publication; independent tag/release/
asset verification; typed #5852 closeout; sprint #5856 and milestone rollup.

Every network mutation must be idempotent and identity-checked before retry.
Partial state is recorded explicitly. The ceremony cannot repair product or
review findings, bypass required checks, or use a tag/release as evidence for
behavior that did not land.

## Execution Sequence

1. Verify WP-29 pass, all required child/sprint terminal truth, no active
   claims, clean exact head, green required checks, and absent/conflict-free tag
   and release identities.
2. Build and validate the final evidence package, release notes, checklist,
   handoff links, artifact hashes, and residual-risk/non-claim statement.
3. Run the ceremony script and tests in preflight/dry-run mode at the exact
   candidate head; obtain exact-head review.
4. After merge, create/push the annotated tag, create/publish the release, and
   verify commit, tag object, release target, notes, and assets independently.
5. Record partial failures without duplicate mutation; resume only after live
   identity checks.
6. Complete typed issue and sprint/milestone closeout from retained receipts
   and live release evidence, then hand v0.93 its accepted packet.

## Protected-Path Candidates

- `docs/milestones/v0.92/RELEASE_NOTES_v0.92.md`
- `docs/milestones/v0.92/RELEASE_PLAN_v0.92.md`
- `docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md`
- `docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/V092_RELEASE_CEREMONY_5852.md`
- `adl/tools/release_ceremony.sh` and its focused test only if preflight defects
  require a separately reviewed repair
- `.csdlc/evidence/5852`

Tags, releases, issues, and sprint state are live ceremony mutations, not
tracked-path ownership substitutes.

## Validation And Failure Policy

Required lanes are evidence-manifest completeness with nonempty required
fields, recomputed artifact hashes, residual-risk and non-claim evidence,
release-claim linkage,
notes/checklist/handoff consistency, tag/release absence and identity negative
cases, ceremony script focused tests and dry-run, duplicate/partial-failure
recovery, artifact hash verification, exact-head review, and post-publication
live readback. Any red check, active claim, missing receipt, unresolved finding,
dirty head, tag/release conflict, or partial verification blocks advancement.

## Non-Goals

- No product remediation, evidence invention, or feature completion by ceremony.
- No unreviewed tag/release mutation or silent retry of non-idempotent steps.
- No claim that v0.93 is active or that legal/governance promises are delivered.
