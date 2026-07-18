# WP-15 Demo Convergence And Proof Coverage

Status: convergence_recorded

Issue: #4642

WP: WP-15

Date: 2026-07-18

## Result

PASS: the v0.91.7 demo matrix and proof coverage now have a single
reviewable convergence packet for demo/Observatory truth.

This packet does not approve the milestone for release. It records that the
demo surfaces already landed for v0.91.7 are mapped to retained proof,
reviewer commands, current issue state, and public claim boundaries. Remaining
release-readiness decisions stay owned by later WP-16 through WP-23 gates.

## Scope

WP-15 consumes the demo and Observatory evidence that already landed in v0.91.7
and records the gap boundary before quality gate and review work.

In scope:

- demo matrix convergence for `docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md`;
- feature-proof coverage for demo-visible and launch-visible surfaces;
- Observatory, HTML, Unity, Runtime v2, and Runtime v3 explicit opt-in proof
  status;
- non-claims and follow-on boundaries for v0.91.8 and later work.

Out of scope:

- v0.91.7 release approval;
- v0.92 activation readiness;
- Runtime v3 default cutover;
- Unity player-build readiness;
- clean-checkout replay of third-party Unity asset packs;
- browser-owned AWS, SNS, SSM, or runtime mutation authority.

## Dependency Truth

| Dependency | Current truth | Evidence |
| --- | --- | --- |
| WP-14 #4641 | Closed on 2026-07-18 | GitHub issue state; PR #5493 and terminal closeout PR #5496 are merged |
| WP-09 #4636 | Closed on 2026-07-11 | `docs/milestones/v0.91.7/review/V0917_WP09_OBSERVATORY_DEMOS_BIRTHDAY_VISIBLE_PROOF_4636.md` |
| HTML proof #4690 | Closed on 2026-07-07 | `demos/v0.91.7/html-observatory/README.md`; retained validator command |
| Demo matrix proof #4691 | Closed on 2026-07-11 | `docs/milestones/v0.91.7/review/demo_matrix_4691/4691-birthday-visible-demo-matrix-proof.md` |
| Unity integrated proof #4689 | Closed on 2026-07-11 | `docs/milestones/v0.91.7/review/unity_observatory_4689/4689-unity-observatory-integrated-proof.md` |
| Runtime v3 Observatory proof #5286 | Closed on 2026-07-13 | `docs/architecture/runtime_v3_observatory_consumption_5286.v1.json` |

The retained sprint-review register still names some earlier WP-09 and WP-14
states as open. That register is stale for those rows; this packet uses current
issue state and the retained proof packets above for WP-15 convergence truth.

## Demo Convergence Matrix

| Surface | WP-15 classification | Proof / command | Claim boundary |
| --- | --- | --- | --- |
| Documentation package | ready | `find docs/milestones/v0.91.7 -maxdepth 2 -type f` | Filesystem presence only, not implementation proof. |
| Bridge overclaim scan | ready | text review over v0.91.7 milestone docs | Prevents planning-only readiness claims. |
| Runtime Soak 2 Observatory packet | proven-retained | `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/` | Retained runtime packet, not fresh soak or full product completion. |
| HTML Observatory integrated proof | proven | `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh` | Default is retained Runtime v2/CSM mirror; Runtime v3 is explicit opt-in. |
| Unity shell/stage/walkthrough proof | proven-limited | `#4652`, `#4703`, `#4704`, `#4689`, and `#4702` retained packets | Depends on operator-provisioned assets for full local replay; no player-build claim. |
| Unity asset and MCP publication boundary | proven-boundary | `#4745` policy and manifest | Does not grant redistribution rights or commit third-party assets. |
| Runtime v3 Observatory feed | proved-explicit-opt-in | `docs/architecture/runtime_v3_observatory_consumption_5286.v1.json` | No default cutover, no Runtime v2 decommission, no browser mutation authority. |

## Feature-Proof Coverage

Feature proof coverage is retained in
`docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md` and the machine
ledger at
`docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json`.

For WP-15, the coverage result is:

- demo-visible Observatory and HTML surfaces are proof-backed;
- Unity-visible surfaces are proof-backed but limited to retained/editor proof;
- not every new v0.91.7 feature has a reviewer-facing demo; several have
  implementation proof, CLI proof, boundary proof, or handoff proof instead;
- Runtime v3 Observatory consumption is proof-backed only through explicit
  opt-in;
- launch/birthday handoff evidence is available from WP-14, but does not
  authorize release readiness;
- security, quality gate, internal review, external review, remediation, next
  milestone review, and release ceremony remain later WP gates.

## New-Feature Demo Audit

| Feature family | Owning issues | Demo present? | Proof status | WP-15 disposition |
| --- | --- | --- | --- | --- |
| Observatory / HTML / Unity demo surfaces | #4636, #4652, #4689, #4690, #4691, #4702, #4703, #4704, #4745 | Yes | Retained HTML validator, Unity editor/image proof, asset-publication boundary, and demo matrix proof | Demo-backed with explicit Unity limitations. |
| Curiosity Engine / Discovery Substrate | #4692 | No visible demo row | Runtime v2 core, deterministic packets, CLI exposure, budget/gate/negative-case proof | Add as proof-backed but not demo-backed. |
| Constructability Anchor Validator | #4693 | No visible demo row | Runtime v2 validator, CLI exposure, fail-closed packets, negative cases, and owner-lane proof | Add as proof-backed but not demo-backed. |
| Reasoning graph, loop runtime, `adl.skill.v1`, AEE/ObsMem/PVF | #4694, #4695, #4696, #4697 plus #4912, #5096, #5136 | Partial | Merged implementation PRs and retained WP-11 cognitive-control evidence; some original proof roots are local/ignored or card-superseded | Add as proof-backed partial-demo/handoff, not broad demo coverage. |
| Affect, Godel, economics, guild, CodeFriend, publication boundaries | #4752-#4757 | Mostly no | WP-13 retained boundary and handoff packets | Add as boundary/handoff proof, not demos. |

The answer to "do we have a demo for every new feature we created?" is no.
WP-15 therefore records a distinction between `demo_backed`,
`proof_backed_no_demo`, `boundary_handoff_only`, and `scoped_out_non_claim`
instead of upgrading all proof packets into demos.

## Validation

Fresh validation run for this packet:

```bash
/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 4642
git diff --check
python3 -m json.tool docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json
```

Retained validation consumed:

- #4691 ran the HTML Observatory proof validator and docs integrity checks.
- #4636 consumed merged WP-09 child PR checks and proof packets.
- #5286 records the Runtime v3 HTTPS/feed validation lane.
- #4641 landed launch/birthday handoff evidence and terminal closeout.

Skipped validation:

- Fresh Unity editor replay was not rerun for WP-15; retained #4652/#4703/#4704
  and #4689 proof packets are the evidence.
- Fresh Runtime v3 loopback service was not started for WP-15; #5286 is the
  retained proof for explicit opt-in consumption.
- Fresh Curiosity, Constructability, WP-11, or WP-13 demos were not added by
  WP-15; those surfaces are classified by retained proof/boundary evidence.
- No AWS command was run for WP-15.

## Non-Claims

- This packet does not claim v0.91.7 release readiness.
- This packet does not claim v0.92 activation readiness.
- This packet does not claim Runtime v3 as the default runtime.
- This packet does not claim Runtime v2 decommission.
- This packet does not claim Unity player-build readiness.
- This packet does not claim clean-checkout replay of third-party Unity assets.
- This packet does not claim browser-owned AWS, SNS, SSM, or runtime mutation
  authority.

## Follow-On Routing

- WP-16 #4643 owns the next quality gate.
- WP-17 #4644 owns documentation alignment after quality-gate truth.
- WP-18 through WP-20 own internal/external review and remediation.
- WP-22 #4649 is already closed and remains retained next-milestone review
  evidence.
- WP-23 #4650 owns release ceremony.
- v0.91.8 Unity live-project proof remains outside v0.91.7 WP-15 and is tracked
  by #4739 and #4741.
