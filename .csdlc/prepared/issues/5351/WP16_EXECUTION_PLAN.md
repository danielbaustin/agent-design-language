# WP-16 Execution Plan

Issue: #5351
Branch/worktree: `codex/5351-v0918-preparation` in its bound issue worktree
Scope: execute the integrated quality gate without changing product code.

## Current Truth

- WP-14A #5384 is typed `closed_out` at generation 16 and accepted the integrated platform through PR #5726. The retained ledger is `.csdlc/evidence/5384/platform-acceptance-ledger.v1.json`.
- WP-15 #5354 has GitHub merged truth through PR #5731 at `97427f324c87d97cb1b36c7804c50bf80c9389d8` and retained convergence proof at `.csdlc/evidence/5354/convergence-proof.v1.json`.
- WP-15 demo-matrix reconciliation landed through PR #5747 at `ab4e9e2217c152df47b1754b66b01febb4a59549`; consume `docs/milestones/v0.91.8/review/wp15_demo_matrix_5733/RECONCILIATION_LEDGER_v1.md`.
- #5354 has a retained terminal receipt recording `closed_out`, no claim, PR #5731, and exact PR head `e8c63268429b0162671e7f1bfae5f560171d7099`.
- Squash-merge ancestry is proven with PR #5731 merge commit `97427f324c87d97cb1b36c7804c50bf80c9389d8`; the receipt's observed SHA remains the exact PR head identity.

## Source Evidence

- Live issue #5351 defines the WP-16 quality-gate outcome and dependency on WP-14/WP-15 completion.
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml` sequences WP-16 #5351 after WP-15 #5354 and before WP-17 #5360.
- `docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md` names #5351 as the integrated release gate and states that planning alone satisfies no gate.
- `.csdlc/evidence/5384/platform-acceptance-ledger.v1.json` and `.csdlc/issues/5384/index.json` provide WP-14A acceptance truth.
- `.csdlc/evidence/5354/convergence-proof.v1.json`, PR #5731, and PR #5747 provide current WP-15 merged proof inputs, subject to the typed closeout blocker above.

## Admission Gate

Run no quality lane until all predicates pass:

1. GitHub #5354 is closed by a merged PR.
2. Typed #5354 is `closed_out`.
3. #5354 has no active claim.
4. Retained receipt `csdlc-v2/closeout/5354.json` exists and validates terminal truth.
5. The receipt records merged disposition, PR identity, and exact PR-head identity.
6. PR #5731's squash merge and PR #5747's merge are ancestors of the exact #5351 execution head.
7. The #5384 platform acceptance ledger remains digest-stable and ancestral.

## Changed Surfaces

Future WP-16 execution may amend the #5351 claim only after admission passes.
Expected changed surfaces are issue-local lifecycle/evidence paths plus the
new quality-gate packet under `.csdlc/evidence/5351`. Product code, milestone
docs, release docs, demo matrices, deployment configuration, and dependency
records remain read-only inputs unless a typed reviewed claim amendment names a
specific path and reason.

## Packet Rows

Create one repository-relative quality packet with these required rows:

- product contracts and characterization;
- stable deployment identities from #5384 and #5354;
- rollback window, rollback evidence, and no-deferral policy;
- deletion eligibility and post-deletion validation status;
- demo convergence, demo matrix, and feature-proof coverage;
- docs checks: YAML, links, feature crosswalk, release notes, and blocker truth;
- exact revision matrix for WP-14A, WP-15, and WP-16 execution head;
- focused proving lane results;
- integrated proving lane results;
- failure routing and owner issue for every non-pass required row;
- budgets, COTS inventory, redaction, and path hygiene;
- WP-17 handoff predicate.

Required row states are `pass`, `fail`, `blocked`, `not_applicable`, and
`explicit_non_claim`. Only `pass` satisfies a required gate.

## Execution Lanes

1. `wp15-terminal-gate`: run `.csdlc/prepared/issues/5351/check-dependencies.rb`.
2. `focused-quality`: verify product contracts, stable deployment identities, rollback, deletion eligibility inputs, demos, docs, revision matrix, budgets, COTS, and redaction.
3. `integrated-platform`: run the accepted ADL v2, Runtime v3, and C-SDLC v2 integrated checks at the exact execution head.
4. `complete`: consume same-revision focused and integrated packets, then record blocker routing, exact review, and publication readiness without repeating the expensive suites.
5. `post-merge-exact`: after authorized serialized merge, rerun only the proof required by changed merge identity, then record blocker truth, CI, and the WP-17 release predicate.

## Focused Validation

The first executable validation after admission is the focused lane, not the
integrated lane. It must prove:

- dependency identity and ancestry for #5384, #5354, and #5747;
- product-contract rows from retained ADL v2, Runtime v3, C-SDLC v2, provider, workcell, Unity, and podcast evidence;
- rollback and deletion row classification;
- docs/YAML/link/feature-proof row classification;
- COTS, budget, redaction, and repo-relative artifact hygiene.

The integrated lane starts only after focused validation is green or has routed
all blockers.

## Failure Routing

- Product-contract or compiler failures route to the owning ADL v2 issue family.
- Runtime v3 failures route to Runtime v3 owners.
- C-SDLC v2 lifecycle failures route to C-SDLC v2 owner issues.
- Demo or claim-boundary failures route to WP-15 follow-up owners.
- Documentation drift routes to WP-17 only after #5351 records the blocker; WP-17 must not start early.
- Deletion failures route to WP-13 deletion owners and remain non-deferred blockers for internal review.

## Review And PR

After focused and integrated lanes pass, run an exact-revision bounded review
over the #5351 packet and changed surfaces. Fix actionable in-scope findings,
refresh the exact review if any substantive change lands, then publish a PR
only from the reviewed head. The PR must include `Closes #5351` only when the
quality gate itself has actually executed and passed; this preparation packet
must not open that PR.

## Rollback And Handoff

Before publication, rollback means deleting issue-local generated #5351 evidence
and restoring the prepared packet. After an authorized merge, rollback follows
the #5384/#5343 rollback contract and requires a reviewed follow-up issue.
Handoff to WP-17 #5360 is allowed immediately after #5351 has a merged PR,
passing exact-head integrated proof, and no required quality row outside `pass`
or justified `not_applicable`. Typed closeout follows asynchronously and does
not block WP-17.

## Non-Goals

- Do not edit product code, deployment configuration, demo matrices, milestone docs, or release docs during preparation.
- Do not use Runtime v2, AWS, provider credentials, paid services, hidden network authority, raw GitHub fallbacks, or hard-coded addresses.
- Do not turn failed, missing, stale, or unsupported evidence into a green documentation note.
- Do not begin WP-17 #5360 before #5351 has merged with passing exact-head
  integrated proof. Typed closeout is explicitly not a WP-17 admission gate.
