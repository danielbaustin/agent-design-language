# #4763 v0.91.8 WP-21 Preparation Design

## Metadata

- Issue: #4763, `[v0.91.8][WP-21][birthday-docs] Implement first-birthday docs and external launch surfaces`
- Worktree: `/Volumes/FastWork/adl-wp-4763`
- Branch: `codex/4763-v0918-wp14-preparation`
- Preparation baseline: `origin/main` at `51bc5ae51b57c19dbab693af1c5a45142995f4e5`
- Current merge head before preparation refresh: `90d1e00a2731ca7c70520a608438da15b4ab5aa0`
- Boundary: preparation only; no documentation implementation, external publication, PR, merge, or closeout

## Source Evidence

- `docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md` states that WP-14 leaves launch/birthday readiness as a map and routes implementation/proof to v0.91.8 children. It requires #4762 witness/receipt readiness before claims and keeps #4763 public docs claim-bounded until proof.
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml` lists #4762 and #4763 as WP-21 child issues and preserves the rule that implementation waits for issue-specific SIP/STP/SPP/VPP readiness.
- `docs/milestones/v0.91.8/WBS_v0.91.8.md` positions WP-21 as the activation-handoff package after WP-20.
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md` requires birth-witness receipt evidence and public launch docs evidence; missing evidence is consumed as blocker/non-claim.
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md` is a planning/readiness surface and forbids legal personhood, consciousness, or unsupported public-readiness claims.
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md` says planning packages are not complete evidence and activation surfaces need integrated proof, operator scope-out, blocker evidence, or implementation-required status.

## Dependency Gates

1. #4762 actual retained implementation proof for birth witnesses and receipt package is a later #4763 execution dependency.
2. #4762 claim acquisition, lifecycle receipt bookkeeping, PR publication, merge, and closeout are not blockers for this preparation branch.
3. #4762 claim/receipt/closeout state is not sufficient evidence for #4763 launch readiness.
4. Typed #4763 lifecycle reacquire/doctor must pass before later execution is lifecycle-clean; the current preparation attempt is blocked by unrelated #5332 terminal-authority reconciliation.
5. Public launch surfaces remain non-public and claim-bounded until operator publication authority exists.

## Exact Issue-Local Paths

- `.csdlc/issues/4763/index.json`
- `.csdlc/issues/4763/audit.jsonl`
- `.csdlc/issues/4763/cards/sip.md`
- `.csdlc/issues/4763/cards/stp.md`
- `.csdlc/issues/4763/cards/spp.md`
- `.csdlc/issues/4763/cards/vpp.md`
- `.csdlc/issues/4763/cards/srp.md`
- `.csdlc/issues/4763/cards/sor.md`
- `.csdlc/issues/4763/cards/sip.values.json`
- `.csdlc/issues/4763/cards/stp.values.json`
- `.csdlc/issues/4763/cards/spp.values.json`
- `.csdlc/issues/4763/cards/vpp.values.json`
- `.csdlc/issues/4763/cards/srp.values.json`
- `.csdlc/issues/4763/cards/sor.values.json`
- `.csdlc/locks/4763.lock`
- `.csdlc/prepared/issues/4763/design.md`
- `.csdlc/prepared/issues/4763/diagram.mmd`
- `.csdlc/prepared/issues/4763/preparation-review.md`
- `.csdlc/prepared/issues/4763/reacquire-claim-20260731.json`

## Intended Later Implementation Paths

- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`
- Optional future external-launch staging paths only after explicit operator authorization; none are authorized by this preparation branch.

## COTS Posture

- No new crates, binaries, provider SDKs, SaaS services, analytics tools, design tools, hosting targets, tracking scripts, or media assets are authorized.
- Later implementation should use existing Markdown, Mermaid, GitHub issue evidence, and repository documents.
- External launch surfaces are repository documentation/staging artifacts until publication is explicitly authorized.

## Budgets

- Preparation packet budget: at most 1,200 net issue-local LoC across cards, design, diagram, request, review, and index/audit metadata.
- Later documentation implementation budget: at most 800 net Markdown/docs LoC unless SPP is re-opened.
- Later issue-local validation/helper budget: at most 200 LoC.
- Preparation elapsed budget: 4 hours.
- Later execution elapsed budget: 6 hours after #4762 proof and typed lifecycle readiness exist.
- Local validation budget: 40 minutes for preparation, 90 minutes for later docs execution.
- Review budget: one bounded gpt-5.5 preparation review plus normal future lifecycle review before any PR/publication.

## PVF Lanes

| Lane | Phase | Role | Status |
| --- | --- | --- | --- |
| `prep-diff-hygiene` | preparation | `git diff --check` before commit | immediate |
| `prep-card-render-integrity` | preparation | render cards/design/diagram and update digests | immediate |
| `typed-lifecycle-reacquire-doctor` | preparation/future execution | typed #4763 reacquire and doctor | blocked by unrelated #5332 reconciliation |
| `dependency-proof-gate` | future execution | inspect #4762 actual retained implementation proof | deferred until #4762 proof exists |
| `future-doc-proof` | future execution | prove docs changed only intended paths and cite evidence | deferred |
| `public-claim-redaction` | future execution | reject unsupported launch/birthday overclaims | deferred |

## Rollback Criteria

- Revert only #4763 issue-local prep paths and the preparation commit if the packet needs rollback.
- Do not revert unrelated origin/main integration or unrelated worktree state without operator direction.
- Future implementation rollback must restore the intended docs paths and preserve evidence explaining why claims were withdrawn.

## No-Deferral Criteria

Future #4763 execution must fail closed if any of these remain true:

- #4762 actual retained implementation proof is missing, fixture-only, receipt-only, or claim-only.
- Typed #4763 reacquire/doctor remains blocked.
- Launch copy makes unsupported public-readiness, legal-status, personhood, consciousness, sentience, or autonomy claims.
- Implementation touches paths outside the intended docs set without replan.
- A new COTS dependency becomes necessary.
- Publication or external launch is requested without explicit operator authorization and current review truth.
