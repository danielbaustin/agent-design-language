# v0.91.6 Pre-v0.92 Burn-Down Checklist

Date: `2026-06-27`
Owner issue: `#4582`
Status: `reviewed_burn_down_for_release_tail_consumption`

## Classification Legend

- `complete_in_v0.91.6`: review evidence is sufficient for this milestone's
  bounded claim.
- `owned_by_open_v0.91.6`: still owned by an open release-tail issue.
- `routed_to_v0.91.7`: should be carried into the next bridge milestone.
- `v0.92_blocker`: must be resolved before v0.92 activation or birthday claims.
- `non_activation_companion`: visible but not required for v0.92 activation.
- `deferred_beyond_v0.92`: intentionally later than v0.92.

## Burn-Down Table

| Surface | Classification | Evidence / route | Notes |
| --- | --- | --- | --- |
| Release-tail sprint | `owned_by_open_v0.91.6` | `#4604`, `CLOSEOUT_TAIL_SPRINT_v0.91.6.md` | Ordered release tail remains active through WP-19. |
| Internal review | `owned_by_open_v0.91.6` | `#4582`, this packet set | WP-14A is executing now. |
| External review | `owned_by_open_v0.91.6` | `#3980` | Must consume this internal review first. |
| Remediation / final preflight | `owned_by_open_v0.91.6` | `#3981` | Must classify/fix accepted findings. |
| Next milestone planning | `owned_by_open_v0.91.6` | `#3982`, `#3983` | Carries v0.91.7/v0.92 handoff truth. |
| Release ceremony | `owned_by_open_v0.91.6` | `#3984` | Must not run before review/remediation/handoff settle. |
| Resilience / persistence / sleep-wake | `routed_to_v0.91.7` | `RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md`; runtime soak docs | Phase 1 exists; checkpoint/restore, sleep/wake, migration, replay, and durable continuity remain residual. |
| Logging/tooling reliability | `owned_by_open_v0.91.6` | WP-03 packets; `#3981` remediation | Strong progress; PR inventory/tooling truth still has residual gaps. |
| Public prompt records | `complete_in_v0.91.6` | `PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md`; public prompt review packets | Bounded export/redaction/indexing bridge is reviewable; broad future ingestion remains separate. |
| Provider/model reliability | `routed_to_v0.91.7` | provider review packets and suitability matrices | Evidence exists, but role-provider/runtime integration and full model-identity maturity remain future work. |
| ACIP/A2A/provider communications | `routed_to_v0.91.7` | `ACIP_A2A_PROVIDER_COMMUNICATIONS_v0.91.6.md`; ACIP runtime review | Protocol/runtime slices exist; full integration and protobuf/JSON closure remain routed. |
| Security/CAV | `v0.92_blocker` | security review packets; `SECURITY_BRIDGE_AND_CAV_v0.91.6.md` | Security remains on activation path and must be consumed by v0.92 planning. |
| Identity/continuity | `v0.92_blocker` | identity bridge doc and retained review | v0.92 is the identity/continuity milestone; v0.91.6 provides bridge evidence only. |
| Observatory/Unity | `routed_to_v0.91.7` | `DEMO_MATRIX_v0.91.6.md`; WP-09 closeout and Observatory proof packets | Bounded proof exists; Unity/editor/build and live runtime consumption remain residual. |
| AEE / Memory / ObsMem | `v0.92_blocker` | `AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`; WP-10 review | Bridge/accounting exists; activation consumption still requires v0.92-level integration truth. |
| ACP / cognitive profiles | `routed_to_v0.91.7` | AEE/Memory/ACP bridge doc | Privacy/profile completion remains bounded and future-facing. |
| GitHub convergence / Octocrab | `owned_by_open_v0.91.6` | review packets plus this finding register | Core issue/PR paths improved, but repo-native PR inventory is still incomplete. |
| Release-helper/tooling reliability | `owned_by_open_v0.91.6` | pr.sh lifecycle hardening review; `#3981` | Strong, but final preflight should consume residual code/tooling findings. |
| C-SDLC adoption | `routed_to_v0.91.7` | `V0916_CSDLC_ADOPTION_AUDIT_4434.md` | Operational, but not fully fail-closed by default. |
| VPP/PVF lane templates | `routed_to_v0.91.7` | `V0916_VPP_PVF_LANE_TEMPLATE_MINI_SPRINT_REVIEW_4332.md` | Substrate exists; universal registry-backed validation is not complete everywhere. |
| Scheduler / cognitive economics | `non_activation_companion` | scheduler review docs | Useful for future orchestration; not a v0.92 activation blocker by itself. |
| Build throughput / remote validation | `non_activation_companion` | build-throughput review packets | Helps validation latency; not required for v0.92 identity activation. |
| CodeFriend / portable adapter v2 | `non_activation_companion` | checklist route | Visible post-v0.92 / pre-v0.95 proof route. |
| Guilds | `non_activation_companion` | checklist route | MVP/governance planning, not v0.92 activation proof. |

## v0.92 Gate Statement

`v0.92` should remain blocked until WP-16/WP-17 consume this burn-down and name
which rows are truly activation blockers, which are v0.91.7 work, and which are
non-activation companion items. `v0.91.6` may close as a bridge/review milestone
only if its release ceremony does not convert bridge evidence into birthday or
runtime-coherence claims.
