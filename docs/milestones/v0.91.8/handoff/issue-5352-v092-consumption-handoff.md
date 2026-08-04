# WP-21 Exact-Revision v0.92 Consumption Handoff

Status: final #5352 publication candidate

Issue: #5352
Integrated baseline: `origin/main` at `c34f0c9412495039a6374f7ce88fa39e34bb5042`
Final path: `docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md`

This handoff assembles the exact reviewed inputs that v0.92 may consume. It
does not claim the v0.92 birthday occurred, production readiness, public launch
approval, Adaptive Learning implementation, or terminal C-SDLC closeout. The
active typed claim owns the final handoff directory. #5558 is closed by PR
#5749, whose merge commit is ancestral to the publication baseline; publication
still requires focused validation and one exact-head review with no unresolved
actionable findings.

## Accepted Platform Revisions

| Product | Issue / PR | Reviewed head | Accepted merge | Contract |
| --- | --- | --- | --- | --- |
| WP-14A platform | #5384 / #5726 | `71e3b70b8f0d235d768ced0383074345547811d4` | `72fbf30c74a5193ea41f042c76c5986a48e59d6c` | Platform acceptance ledger |
| C-SDLC v2 | #5358 / #5606 | `e048230245b1ad101c8056678123a2747faa4b60` | `fc75f4fc697262f89f99461679a406be0b4b3775` | Typed v2 lifecycle generation |
| Runtime v3 | #5361 / #5650 | `f7fc71421f4bcf70039b910c9b88b538bb111400` | `f7258b07e9da414bfee518f0c89a76071bc03ee8` | Bounded operational Runtime v3 proof |
| ADL v2 soak / rollback | #5344 / #5703 | `141dfa20ccc3753060687259ad933397331df9c7` | `d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2` | Opt-in soak and rollback proof |
| ADL v2 reversible default | #5343 / #5704 | `e4bbc988cad682cbb2ff8d24085e1a99bccec1ce` | `e1b6a34e4763a79d1c40c641e64c0c061a0aa96c` | Reversible selector cutover |

The machine-readable authority is
`.csdlc/evidence/5384/platform-acceptance-ledger.v1.json`. Its accepted baseline
is `11151e0beab02b1667f6505b7f8992bfd47d2f8f`, and every accepted merge above
must remain ancestral to the final publication baseline.

## WP-21 Integration Matrix

| Concern | Issue / PR | Exact reviewed head | Merge revision | Consumption surface |
| --- | --- | --- | --- | --- |
| Launch readiness | #4758 / #5739 | `c9b5c625ccfb17b1a75fd3a1a93f4810baf4a3e2` | `038f718c377549db21df3a1eb08402867beb2cd5` | `.csdlc/evidence/4758/launch-readiness/launch-readiness.v1.json` |
| Activation bridge | #4759 / #5738 | `32957a21a3fc3fc8a8efb3c3c6ad198db9b0ddd7` | `471db0c35dc34c2497682993378948481bdfa213` | `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md` |
| Memory Palace MVP | #4760 / #5740 | `9719252262913351144a20adf0affb7ed4b5480d` | `d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e` | `adl/src/memory_palace.rs` and retained runtime proof |
| Capability envelope | #4761 / #5741 | `8c3ef0336570238d26eab0fd49a9a2ff9c1a0c09` | `97d4036e0b5c21786d13cd1301b33038d95e3b98` | `.csdlc/evidence/4761/capability-envelope/envelope.v1.json` |
| Birth witness package | #4762 / #5744 | `d736baca1c82c6ca9b770678ff2c04ce44458fc9` | `021be8e33b486d9b66886ff299c20607ed8a071a` | `docs/milestones/v0.91.8/review/v092_handoff_4762/birth-receipt-4762.v1.json` |
| Birthday / launch docs | #4763 / #5734 | `313268e09b8d9906f61b0e12ac05cce4deea1e3c` | `d2b19b3aba092aff871b315d60590731e730cb4a` | `docs/milestones/v0.92/external_launch/README.md` |
| Memory Palace ADR | #5007 / #5743 | `426d0a53fb2b7b0be571b236ca5d0a248b32e1f8` | `1bd6f73b1c449ffd132ad9a34c739e16c39186c2` | `docs/adr/0058-memory-palace-context-handoff-architecture.md` |
| Adaptive Learning queue | #5107 / #5742 | `8bf36c9d214a54212e7c483fb29872e9be9e92b3` | `b77d020c5c5274e7b64b6ef8f36eed888f34fb4c` | `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md` |

All eight issues are observed merged/closed and all eight merge revisions are
ancestors of the integrated baseline. Their artifacts preserve their own
non-claims; this handoff does not turn planning, witness packaging, or launch
copy into an observed birthday event.

The child merge order differed from the planning order: these eight changes
landed before the final #5558 sunset-guidance repair. This handoff consumes the
later #5558 merge as an exact ancestral predecessor; it does not rewrite the
earlier merge sequence or claim that the planned ordering was followed.

## WP-20 Predecessor Gate

| Dependency | Live state | Exact revision truth | Effect on #5352 |
| --- | --- | --- | --- |
| #5548 Gate 2 fixture repair | closed | No merge revision claimed from the available typed issue observation | Retained predecessor disposition |
| #5558 sunset-guidance repair / PR #5749 | closed, completed | Branch head `033b28cffa6bdf191b1d013aa5a730ce7b10d9df`; GitHub merge commit `c34f0c9412495039a6374f7ce88fa39e34bb5042` is ancestral to the integrated baseline | Releases the final-path promotion gate; exact-head validation, review, and publication remain required |

WP-21 depends on WP-20 in
`docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`. The #5558 branch tip is
not itself an ancestor of `origin/main`, so this handoff uses the GitHub PR
merge commit rather than treating the branch tip as merge proof.

## Stable Contracts And Schemas

- C-SDLC generation authority:
  `csdlc-v2/operator/generation-selector.json`, selecting `v2`.
- Stable operator install directory: `.adl/bin/csdlc-v2/`. It is operational,
  repo-local generated state rather than a tracked handoff dependency; a local
  installation receipt is audit evidence only and never gates this handoff.
- C-SDLC card registry: `docs/templates/prompts/current.json`.
- Native card shape contract: `csdlc-v2/operator/native-card-shape.json`.
- Runtime v3 proof index:
  `.csdlc/evidence/5361/acceptance-proof-summary.json`.
- ADL v2 cutover contract:
  `docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json`.
- WP-21 activation bridge:
  `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`.

The local installed binary receipt is execution tooling evidence, not the
accepted C-SDLC product revision. Final validation must keep those two concepts
separate.

## Rollback Boundaries

- WP-14A records rollback window end `2026-08-12T09:04:24Z` and
  `deletion_authorized: false`.
- The ADL v2 cutover report retains ADL v1, proves exact prior selector bytes
  were restored during rollback, and ends with reversible default `adl-v2`.
- Runtime v3 retained proof covers rollback restore and cryptographic continuity
  restore; it does not claim a continuously running background guardian.
- C-SDLC publication remains exact-head and review-current. Any rebase, merge,
  or substantive fix invalidates stale review/publication evidence.
- This handoff authorizes consumption of accepted evidence only. It does not
  authorize incumbent deletion, production deployment, or closeout.

## Residual Risks

| Risk | Required disposition |
| --- | --- |
| #5558 was squash-merged rather than branch-tip merged | Validate PR #5749 by merge commit `c34f0c9412495039a6374f7ce88fa39e34bb5042` and retain branch head as observed review input only |
| Typed claim could become stale after final integration | Recheck claim, generation, and protected path before final validation |
| Installed C-SDLC receipt predates the final baseline | Record exact receipt source provenance and keep it distinct from accepted product or final-baseline truth |
| Launch documentation may be mistaken for an observed event | Retain `birth_event_status: not_claimed` and explicit publication gates |
| Adaptive Learning planning may be mistaken for runtime behavior | Keep #5107 as queue/planning truth only |
| Runtime operational proof may be read as background availability | Retain the background guardian non-claim from the Runtime v3 proof summary |
| Some merged child C-SDLC projections have not completed asynchronous terminal reconciliation | Treat GitHub merge/closure as integration truth, retain the lifecycle debt in the sprint review, and do not block independent work on closeout |
| #4762 retained proof used a machine-local validation binary | Retain this as historical provenance debt; do not use that path as portable proof for v0.92 execution |

## Explicit Non-Claims

- No v0.92 birthday event is claimed to have occurred.
- No public launch, identity, consciousness, or production-provider readiness
  is claimed.
- No Adaptive Learning runtime, learning-driven graph mutation, AWS, GPU, or
  credentialed remote-provider execution is claimed.
- No Unity proof is added or altered by #5352.
- No C-SDLC closeout is requested or recorded by this handoff.
- Typed closeout is asynchronous after GitHub closure and must not block
  independent implementation or release work.
- Closed child issues and retained receipts do not substitute for exact merge
  ancestry at the publication revision.

## Final Promotion Checklist

1. Observed #5558 closed completed by PR #5749 and recorded merge commit
   `c34f0c9412495039a6374f7ce88fa39e34bb5042`.
2. Integrated current `origin/main` once in the #5352 worktree.
3. Verified the active execution claim owns `docs/milestones/v0.91.8/handoff`.
4. Promoted the handoff to the final path and replaced pending snapshot truth.
5. Resolve the installed C-SDLC generation through the tracked selector; do not
   rebuild operational binaries when their source provenance is unchanged.
6. Run dependency ancestry, ledger contract, links, typed-record, and diff checks.
7. Commit a stable publication candidate and run exactly one GPT-5.5 review of
   that exact revision; fix all actionable findings and refresh proof if the
   revision changes.
8. Publish a ready PR to `main` with `Closes #5352`, shepherd required CI to
   green, merge, and observe issue closure. Route typed closeout asynchronously.
9. Complete the findings-first sprint review recorded in
   `WP21_SPRINT_REVIEW_5352.md`; fix all current blockers before merge while
   preserving historical child evidence and explicit non-claims.
