# v0.92 Activation Test Map from v0.91.8

WP-21 `#5362` owns the handoff rows below after WP-14A accepts the platform.
These rows do not block WP-14A itself.

Issue `#4759` implements this file as the WP-21 activation bridge consumed by
the v0.91.8 pre-v0.92 handoff. The bridge consumes the accepted WP-14A
platform ledger at
`docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md`,
where issue `#5384` accepts baseline
`11151e0beab02b1667f6505b7f8992bfd47d2f8f` and records these accepted
merges:

| Accepted product | Owner | Accepted merge |
| --- | ---: | --- |
| C-SDLC v2 | #5358 | `fc75f4fc697262f89f99461679a406be0b4b3775` |
| Runtime v3 | #5361 | `f7258b07e9da414bfee518f0c89a76071bc03ee8` |
| ADL v2 soak and rollback | #5344 | `d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2` |
| ADL v2 reversible default | #5343 | `e1b6a34e4763a79d1c40c641e64c0c061a0aa96c` |

The activation bridge is a consumption map, not a readiness claim. A v0.92
consumer may cite a row only after the row has an exact evidence pointer and
one of these dispositions:

- `accepted_platform_input`: accepted by the WP-14A platform ledger.
- `handoff_owned`: routed to the named WP-21 issue before v0.92 consumption.
- `blocked_with_evidence`: preserved as an explicit blocker or non-claim.
- `deferred_non_claim`: deliberately outside the activation claim.

| v0.92 input | v0.91.8 source | Required evidence |
| --- | --- | --- |
| Platform install | #5345, #5343, #5384 | `accepted_platform_input`: WP-14A ledger accepts ADL v2 reversible default merge `e1b6a34e4763a79d1c40c641e64c0c061a0aa96c`; stable install and selector receipt remain anchored by #5345/#5343 evidence. |
| Runtime execution and canonical ingress | #5341, #5361, #5591 | `accepted_platform_input`: WP-14A ledger accepts Runtime v3 merge `f7258b07e9da414bfee518f0c89a76071bc03ee8`; Runtime v3 retained proof covers guardian-launched live ingress, HTTPS/WSS Observatory, rollback restore, and continuity restore. |
| Reasoning graphs, loops, affect control, and adaptive cognition | #5592, #5107 | `accepted_bridge_input`: #5592 and #5107 are closed retained inputs. Consume `docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md`; #5107 is a downstream Adaptive Learning DAG plan and does not by itself prove learning-driven graph mutation. |
| Governed runtime operations | #5589 | `accepted_bridge_input`: #5589 is closed with retained Runtime v3 Parity-C production-adapter, identity/private-state, provider/scheduler, checkpoint, and lifelog proof. Activation must consume that exact evidence rather than infer broader behavior. |
| Secure local/remote access and HTML Observatory | #5590 | `accepted_bridge_input`: #5590 is closed with retained Runtime v3 Parity-D configuration-driven HTTPS, authenticated HTTP/WebSocket, guardian, telemetry, and rollback proof. Activation must preserve its exact security and configuration boundaries. |
| Unity Observatory demo proof | #5354, #4739, #4741, #5332 | `handoff_owned`: WP-15 #5354 owns Unity demo convergence independently of WP-14A; v0.92 may consume only a reviewed proof packet or explicit Unity tooling disposition. |
| Lifecycle governance | #5358 | `accepted_platform_input`: WP-14A ledger accepts C-SDLC v2 merge `fc75f4fc697262f89f99461679a406be0b4b3775`; downstream lifecycle claims still require issue-local typed records. |
| Capability envelope | #4761 | `accepted_bridge_input`: #4761 is closed/merged/closed_out and retains the evidence-backed capability envelope at `.csdlc/evidence/4761/capability-envelope/envelope.v1.json` plus non-claims at `.csdlc/evidence/4761/capability-envelope/non-claims.v1.md`. |
| Memory Palace | #4760, #5007 | `accepted_bridge_input`: #4760 and #5007 are closed/merged/closed_out and retain the Memory Palace context handoff plus ADR acceptance. This bridge consumes their exact evidence and does not widen it into unbounded runtime Memory Palace behavior. |
| Birth witnesses/receipt | #4762 | `accepted_platform_input`: #4762 merged through PR #5744 at merge commit `021be8e33b486d9b66886ff299c20607ed8a071a` and supplies the retained witness/receipt package at `docs/milestones/v0.91.8/review/v092_handoff_4762/`. The package records `birth_event_status: not_claimed`; its merge is proof input, not proof that the birthday occurred. |
| Public launch docs | #4758, #4763 | `accepted_bridge_input`: #4758 and #4763 are closed/merged/closed_out and retain claim-bounded launch and first-birthday docs. #4763 may cite the merged #4762 retained package, but this bridge does not authorize a birthday-complete or public-launch claim. |
| Adaptive Learning DAG | #5107 | `accepted_bridge_input`: #5107 is closed/merged/closed_out as a retained downstream plan; bounded loop/runtime evidence still does not prove learning-driven graph mutation or v0.92 adaptive-learning implementation. |
| Distributed workcell | #5497, #5501 | `handoff_owned`: consume one reviewed live workcell and bounded context/output-contract proof before relying on distributed execution behavior. |
| Canonical feature preservation | #5594, #5362, #5355 | `handoff_owned`: every relevant canonical feature-list row needs an owner and terminal disposition before release-tail closeout; absent Runtime v3 implementation blocks Runtime v2 deletion, not this bridge. |
| Release-tail handoff routing | WP-21, WP-21A #5355, WP-22 | `handoff_owned`: WP-21 produces the exact handoff and activation bridge, WP-21A #5355 aligns next-milestone handoff review, and WP-22 reviews before v0.92 consumption. |

If any row lacks evidence, `v0.92` must consume it as a blocker or explicit
non-claim.

Current consumption truth: #5408 is closed/remediated via PR #5419, but #4906
remains retained blocked-with-evidence unless separately dispositioned.

## Downstream Consumption

The v0.91.8 handoff consumes this activation map through
`docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md` and
`docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`. The v0.92 planning
surface consumes it through
`docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`.

The immediate dependency successor #4762 merged through PR #5744 at
`021be8e33b486d9b66886ff299c20607ed8a071a`, with source head
`d736baca1c82c6ca9b770678ff2c04ce44458fc9`. Its auditable package is retained
at `docs/milestones/v0.91.8/review/v092_handoff_4762/`. Downstream #4763 may
consume that package as witness/receipt evidence, while the birthday event,
current exact-head review, and operator publication approval remain separate
gates.
