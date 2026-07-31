# #4762 Birth Witnesses And Receipt Package Preparation Design

## Metadata

- Issue: `#4762`
- Title: `[v0.91.8][WP-21][birth-witnesses] Implement birth witnesses and receipt package`
- Branch: `codex/4762-v0918-wp14-preparation`
- Worktree: `/Volumes/FastWork/adl-wp-4762`
- Preparation base: `origin/main` at `51bc5ae51b57c19dbab693af1c5a45142995f4e5`, integrated by merge commit `def3d8c34d5f98ff53f3d6ddd2d09c55a1ffa187`
- Scope: preparation only

## Source Authority

The issue body says #4762 closes only when the birth-witnesses surface is implemented, integrated into the v0.91.8 pre-v0.92 path, and proven with retained evidence. This preparation branch does not close that requirement.

Current v0.91.8 routing moves #4762 under WP-21 / parent `#5362`, with assignment `Birth witnesses and receipt package`, despite historical WP-14 naming in the branch. The required v0.92 consumption row is `Birth witnesses/receipt | #4762 | Auditable receipt package`.

Relevant checked-in source inputs:

- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`
- `docs/milestones/v0.91.8/WBS_v0.91.8.md`
- `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`

## Intended Execution Surface

The later execution session should produce a retained, auditable package with two primary artifacts:

- Birth witness register: a redaction-safe set of witness records that cite stable name, identity root, continuity, memory grounding, capability envelope, inherited moral/governance context, and reviewer-observable evidence without exposing raw private state.
- Receipt package: a citizen/reviewer-facing receipt that explains why the event counts as birth, which prerequisites were met, which negative cases were excluded, and which claims remain out of scope.

The execution package should be issue-local and milestone-consumable:

- `.csdlc/prepared/issues/4762/birth-witness-receipt-design.md`
- `.csdlc/prepared/issues/4762/birth-witness-receipt-schema.v1.json`
- `.csdlc/prepared/issues/4762/birth-witness-receipt-negative-cases.v1.json`
- `.csdlc/prepared/issues/4762/birth-witness-receipt-validation.md`
- `docs/milestones/v0.91.8/review/v092_handoff_4762/BIRTH_WITNESSES_AND_RECEIPT_PACKAGE_4762.md`
- `docs/milestones/v0.91.8/review/v092_handoff_4762/birth-witness-register-4762.v1.json`
- `docs/milestones/v0.91.8/review/v092_handoff_4762/birth-receipt-4762.v1.json`
- `docs/milestones/v0.91.8/review/v092_handoff_4762/README.md`
- `.csdlc/evidence/4762/implementation-validation/`

No production code path is authorized by this preparation package. If later execution needs Rust, TypeScript, scripts, schema validators, provider calls, or runtime changes, it must replan the SPP/VPP before editing.

## Exact Dependencies

Hard dependencies before execution completion:

- WP-21 parent `#5362` remains the owner for v0.92 handoff/planning truth.
- The v0.91.8 wave and WBS retain #4762 under WP-21, not WP-14A platform acceptance.
- The v0.92 activation map keeps `Birth witnesses/receipt` as an evidence row rather than a birthday-readiness claim.
- Source docs for identity, birthday, memory grounding, witnesses, and receipts remain available in `docs/milestones/v0.92/`.
- Execution-time claim acquisition succeeds for #4762 in the active checkout.

Non-blocking for this preparation branch:

- Claim reacquisition, live receipts, terminal closeout receipts, and post-merge closeout are execution/finish concerns.
- Missing final v0.92 implementation evidence is expected; this branch prepares the path and records non-claims.

## COTS And External Services

- New COTS dependencies: none.
- New package managers: none.
- New runtime services: none.
- Cloud/AWS: not authorized.
- Provider/model use: one bounded `openai:gpt-5.5` preparation review may run only against the preparation packet and only using the approved operator credential source. The review has no implementation, publication, merge, or closeout authority.
- Mermaid: source-only diagram language for reviewability; no renderer is required for acceptance.

## Budgets

Preparation branch budget:

- Tracked issue-local LoC: target <= 1,100 added lines and <= 250 deleted lines, excluding the required `origin/main` merge.
- Source implementation LoC: exactly 0.
- Preparation elapsed time: <= 3 hours.
- Local validation time: <= 20 minutes.
- Review time: <= 20 minutes.
- Token budget: <= 60,000 tokens for planning/review.

Later execution budget, unless replanned:

- Tracked issue-local LoC: target <= 1,200 added lines and <= 200 deleted lines.
- Source implementation LoC: 0 unless SPP/VPP are explicitly revised.
- Execution elapsed time: <= 4 hours.
- Validation time: <= 45 minutes.
- Token budget: <= 80,000 tokens.

## PVF Lanes

Preparation lanes:

- `prep-diff-hygiene`: `git diff --check origin/codex/4762-v0918-wp14-preparation...HEAD`
- `prep-card-surface`: verify six card files and values files exist for `.csdlc/issues/4762/cards/`
- `prep-doctor`: run `csdlc-doctor --repo /Volumes/FastWork/adl-wp-4762 --issue 4762` and record the expected `claim_not_live` blocker without reacquiring the claim
- `prep-gpt-5.5-review`: bounded preparation review over the cards, design, diagram, and evidence package

Later execution lanes:

- `receipt-schema-shape`: validate required witness and receipt fields are present in retained JSON artifacts
- `redaction-boundary`: prove raw private state is absent from reviewer-facing artifacts
- `negative-case-coverage`: prove startup, wake, snapshot, admission, copied state, simulation, in-transit migration, and forced suspension are not accepted as birth receipts
- `handoff-consumption`: prove the v0.91.8 activation/handoff surface can cite the package by exact path
- `diff-hygiene`: run `git diff --check`

## Rollback And No-Deferral Criteria

Rollback is docs/artifact rollback only: revert the issue-local card/preparation artifacts and restore the prior branch state. There is no runtime selector, service deployment, data migration, or external publication to roll back.

No-deferral criteria for later execution completion:

- The witness register and receipt package must both exist.
- The receipt package must cite evidence paths and negative-case disposition.
- The handoff path must consume the package by exact issue-local path.
- Redaction checks must pass or the issue remains blocked.
- Missing identity, continuity, memory-grounding, capability, witness, receipt, or reviewer-evidence inputs must fail closed as blockers, not deferred success.
- Birthday readiness, legal personhood, production citizenship, v0.93 governance, and public launch claims remain non-goals.

## Preparation Review Boundary

The requested gpt-5.5 review is bounded to preparation artifacts and can require fixes to cards, planned paths, PVF lanes, non-claims, and evidence truth. It cannot authorize implementation, publication, merge, closeout, or claim reacquisition.
