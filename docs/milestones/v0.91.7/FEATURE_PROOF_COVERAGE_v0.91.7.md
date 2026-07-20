# v0.91.7 Feature Proof Coverage

Status: wp15_demo_convergence_recorded

Issue: #4642

Date: 2026-07-18

## Purpose

This document is the WP-15 feature-proof coverage index for v0.91.7. It links
the demo-visible and launch-visible milestone surfaces to retained evidence,
current issue truth, validation posture, and public claim boundaries.

It does not approve v0.91.7 release readiness and does not claim v0.92
activation readiness. Later gates still own quality, documentation alignment,
internal review, external review, remediation, next-milestone review, and
release ceremony truth.

Current 2026-07-20 refresh: WP-16 through WP-22, including WP-21A, are complete. WP-19
reviewed the exact 70-file corpus through one Fable 5 lane and three independent
shadow lanes, returning 22 findings. WP-20 fixed all 22 through merged PR #5588.
#5571 and #5573 are closed, with their retained publication-boundary and
closeout-audit evidence preserved; WP-23 #4650 is the ceremony integration gate.

## Coverage Table

| Surface | WP-15 status | Current issue truth | Evidence | Non-claims |
| --- | --- | --- | --- | --- |
| Demo matrix | proven | #4691 closed | `docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md`; `docs/milestones/v0.91.7/review/demo_matrix_4691/4691-birthday-visible-demo-matrix-proof.md` | Release readiness and v0.92 activation readiness. |
| HTML Observatory | proven | #4690 closed | `demos/v0.91.7/html-observatory/README.md`; `adl/tools/test_v0917_html_observatory_integrated_proof.sh` | Browser-owned AWS mutation, runtime mutation, and default Runtime v3 cutover. |
| Runtime v2 Observatory packet | proven-retained | #4682 closed | `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/` | Fresh soak rerun by WP-15 and full product completion. |
| Runtime v3 Observatory consumption | proved-explicit-opt-in | #5286 closed | `docs/architecture/runtime_v3_observatory_consumption_5286.v1.json` | Runtime v3 default runtime, Runtime v2 decommission, and browser mutation authority. |
| Unity Observatory shell, stage, and walkthrough | proven-limited | #4652, #4689, #4702, #4703, #4704, and #4745 closed | Unity proof packets under `docs/milestones/v0.91.7/review/unity_observatory_*` | Player-build readiness, clean-checkout third-party asset replay, and Unity live Runtime v3 consumption. |
| Launch and birthday handoff | retained-handoff | #4641 closed | `docs/milestones/v0.91.7/review/V0917_WP14_LAUNCH_BIRTHDAY_HANDOFF_4641.md` | Release approval and v0.92 activation readiness. |

## New-Feature Demo Audit

Not every new v0.91.7 feature has a reviewer-facing demo. WP-15 keeps this
truth explicit by separating demos from implementation proof and boundary
handoff evidence.

| Feature family | Owning issues | Demo coverage | Proof coverage | WP-15 classification |
| --- | --- | --- | --- | --- |
| Observatory / HTML / Unity | #4636, #4652, #4689, #4690, #4691, #4702, #4703, #4704, #4745 | Yes: HTML Observatory plus retained Unity editor/image proof | `docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md`, #4691 proof, WP-09 packet, Unity packets, HTML validator | `demo_backed` with explicit Unity limitations |
| Curiosity Engine / Discovery Substrate | #4692 | No visible demo row | #4692 Runtime v2 core, deterministic packet, CLI, budget/gate, and negative-case proof summarized by `docs/milestones/v0.91.7/review/V0917_WP10_CURIOSITY_CONSTRUCTABILITY_REVIEW_4637.md` | `proof_backed_no_demo` |
| Constructability Anchor Validator | #4693 | No visible demo row | #4693 Runtime v2 validator, CLI, fail-closed packets, negative cases, compiled smoke, and owner-lane proof summarized by `docs/milestones/v0.91.7/review/V0917_WP10_CURIOSITY_CONSTRUCTABILITY_REVIEW_4637.md` | `proof_backed_no_demo` |
| Reasoning graph / loop runtime / `adl.skill.v1` / AEE-ObsMem-PVF | #4694, #4695, #4696, #4697, #4912, #5096, #5136 | Partial: runtime proof packets and CLI/proof artifacts, not a polished demo matrix row | `docs/milestones/v0.91.7/review/V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`; `docs/milestones/v0.91.7/review/V0917_WP11_RUNTIME_V2_COGNITIVE_CONTROL_EVIDENCE_4638.md` | `proof_backed_partial_demo` |
| Affect and happiness boundary | #4752 | No | `docs/milestones/v0.91.7/review/wp13_affect_happiness_boundary_4752.md` | `boundary_handoff_only` |
| Godel mechanics boundary | #4753 | No standalone visible demo | `docs/milestones/v0.91.7/review/wp13_godel_constructability_boundary_4753.md`; retained Runtime v2 Godel evidence consumed by WP-13 | `boundary_handoff_only` |
| Economics and civilization boundary | #4754 | No | `docs/milestones/v0.91.7/review/wp13_economics_civilization_boundary_4754.md` | `scoped_out_non_claim` |
| Guild foundation boundary | #4755 | No | `docs/milestones/v0.91.7/review/wp13_guild_foundation_boundary_4755.md` | `boundary_handoff_only` |
| CodeFriend v1 / adapter v2 obligations | #4756 | No ADL demo | `docs/milestones/v0.91.7/review/wp13_codefriend_adapter_obligations_4756.md`; `docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md` | `handoff_only` |
| Publication and paper surfaces | #4757 | No | `docs/milestones/v0.91.7/review/wp13_publication_boundary_4757.md`; `docs/milestones/v0.91.7/review/wp13_publication_boundary_4757/boundary_packet.json` | `scoped_out_non_claim` |

WP-15 does not promote `proof_backed_no_demo`, `boundary_handoff_only`, or
`scoped_out_non_claim` rows into demos. If the release wants a demo for any of
those families, that is new tracked work, not a WP-15 closeout inference.

## Machine Ledger

The machine-readable WP-15 coverage ledger is retained at:

```text
docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json
```

## Downstream Gate History

The following downstream gates were required after WP-15. WP-23 is the sole
open gate before ceremony integration; the other rows retain completed history.

| Gate | Issue | Role |
| --- | ---: | --- |
| WP-18 | #4645 | Closed through merged PR #5543; internal remediation #5408 and #5544-#5547 closed |
| WP-19 | #4646 | Closed; provider-degraded review complete with 22 findings retained |
| WP-20 | #4647 | Closed through merged PR #5588 after fixing all 22 WP-19 findings |
| WP-21A | #5489 | Closed with retained v0.91.8 planning and review-handoff package |
| WP-23 | #4650 | Release ceremony |

Closed downstream gates retained after WP-15:

| Gate | Issue | Current truth |
| --- | ---: | --- |
| WP-16 | #4643 | Closed quality gate with downstream gates open. |
| WP-17 | #4644 | Closed documentation alignment; PR #5539 merged and terminal projection materialized by #5544. |

Closed downstream planning evidence:

| Gate | Issue | Current truth |
| --- | ---: | --- |
| WP-22 | #4649 | Closed on 2026-07-10; keep as retained next-milestone review evidence rather than remaining work. |

## Validation Posture

Fresh WP-15 validation is documentation and ledger integrity:

```bash
git diff --check
python3 -m json.tool docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json
.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 4642
```

Retained proof remains attached to the source issues named in the table. WP-15
does not rerun Unity editor proof, Runtime v3 loopback service proof, or AWS
lanes.
