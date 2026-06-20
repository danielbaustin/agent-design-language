# v0.91.6 Candidate Work Breakdown Structure

## Metadata

- Milestone: `v0.91.6`
- Version: `v0.91.6`
- Date: `2026-06-16`
- Status: candidate WP sequence for first pre-`v0.92` bridge tranche
- Setup issue: `#3800`

## Status

This WBS has been promoted into concrete `v0.91.6` issues. The WP-03
logging/tooling route is opened as umbrella `#3968` with child lanes
`#3995`-`#4001`, and the WP-04 public-records route is opened as umbrella
`#3969` with child lanes `#4002`-`#4006`.

WP-01 consumed this document and
[WP_ISSUE_WAVE_v0.91.6.yaml](WP_ISSUE_WAVE_v0.91.6.yaml) to begin opening
concrete GitHub issues with canonical C-SDLC cards. Additional WPs may still
remain in planning or queue-blocked state.

Current live state:

- WP-03 has merged through `#4001`
- WP-04 child issues `#4002`-`#4006` have merged through `#4006`
- WP-04 umbrella `#3969` has merged and closed after the merged child wave

## WBS Summary

`v0.91.6` should turn first-tranche bridge pressure into reviewed feature docs,
decision records, and proof-loop repairs before `v0.92` activation opens.

## Candidate WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Planning promotion and issue-wave readiness | Promote this candidate package, reconcile `#3778`, open the issue wave, and prepare SIP/STP/SPP/SRP/SOR cards. | Opened issue wave and card bundles. | `#3778`, `#3800`. |
| WP-02 | Resilience, persistence, and sleep/wake feature doc | Define retry/fault classes, health persistence, checkpoint/restore, sleep/wake, hibernation, simulation, migration, replay, and continuity proof. | Feature doc and acceptance checklist. | WP-01. |
| WP-03 | Logging/tooling proof-loop fixes | Plan and complete validation split, CI runtime-budget observability, logging/Otel consumption, PR proof-loop reliability, issues `#3802`-`#3805`, and card-to-GitHub projection convergence including `#3935`. | Tooling reliability feature doc, review packet, completed `#3935` convergence slice, and issue routes. | WP-01. |
| WP-04 | Public prompt records | Define local editable authoring, export, redaction, validation, indexing, evidence, and security review boundaries. | Public prompt-record feature doc. | WP-01, security review inputs. |
| WP-05 | Provider/model reliability and multi-agent readiness | Define hosted/local/remote/OpenRouter/Gemma lanes, role suitability, known failures, and proof limits. | Provider/model reliability feature doc and matrix update. | WP-01, prior provider/multi-agent evidence. |
| WP-06 | ACIP/A2A/provider communications decisions | Decide schema catalog, access rules, external-agent posture, provider communications, WebSocket boundary, JSON projection, and protobuf routing. | Decision record plus feature-doc route. | WP-01, WP-07 security input. |
| WP-07 | Security bridge readiness and CAV | Refresh threat model, CAV plan, provider/model trust, public-record security, ACIP access/security, and malformed-output expectations. | Security/CAV bridge feature doc. | WP-01, WP-04. |
| WP-08 | Identity/continuity and capability-selector bridge | Connect capability evidence, identity continuity, negative cases, and resilience/citizen persistence. | Bridge record for v0.92 consumption. | WP-02, WP-05. |
| WP-09 | Observatory/Unity consumption classification | Classify Observatory/Unity surfaces as proof, rehearsal, substrate, blocked, or deferred. | Consumption classification record. | WP-01, prior demo-readiness docs. |
| WP-10 | AEE, Memory/ObsMem, and ACP accounting | Record AEE completion boundary, residual runtime/provider action routing, ObsMem handoff, Memory Palace planning state, ACP/profile scope, privacy boundary, and what v0.92 may consume. | Bridge accounting record for AEE, Memory/ObsMem, and ACP. | WP-01, WP-02, WP-05. |
| WP-11 | Bridge ledger refresh and v0.91.7 handoff | Update first-tranche dispositions and route remaining second-tranche work into `#3801`. | Updated bridge-ledger addendum or handoff record. | WP-02 through WP-10. |
| WP-12 | Internal review | Review docs for missing bridge surfaces, overclaims, security gaps, and vague spillover. | Review packet and finding register. | WP-11. |
| WP-13 | Remediation and closeout | Fix review findings, update milestone docs, and produce closeout truth. | Remediation PRs, final checklist, and closeout packet. | WP-12. |

## Companion Planning Queue

These items are queued for `v0.91.6` readiness but are not activation surfaces
and should not disturb the first-tranche bridge sequence above:

| Item | Route | Required v0.91.6 disposition |
| --- | --- | --- |
| `agent-logic.ai` AWS account setup | `#3902` | Account setup, AWS credits application guidance, Terraform boundary, and hosting/security non-claims are recorded in `review/AGENT_LOGIC_AWS_ACCOUNT_DECISION_RECORD_3902.md`; AWS Activate review and private credit visibility remain post-close external follow-up. |
| CodeFriend v1 / portable adapter v2 | `docs/milestones/v0.95/features/CODEFRIEND_V1_PORTABLE_ADAPTER_V2_PROOF_v0.95.md` | Route remains visible as post-v0.92 / pre-v0.95 proof work without pulling product implementation into v0.91.6. |
| Guilds / MVP governance route | `docs/milestones/v0.93/features/GUILDS_AND_COLLECTIVE_ORGANIZATION_v0.93.md` and v0.95 MVP consumption | Route remains visible as MVP-scoped governance work without pulling governance implementation into v0.91.6 or v0.92 birthday activation. |
| Runtime integration soak sprint | `#4185` and `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md` | Plan Soak #1 as the post-Tokio walking-skeleton proof in `v0.91.6`. Route Soak #2 as the `v0.91.7` full feature-list integration proof, with Soak #3 only if needed before `v0.92`. |

## Acceptance Mapping

- Resilience must cover continuity, sleep/wake, recovery, migration, replay, and
  proof, not just retry language.
- Tooling reliability must include issues `#3802`-`#3805` or explicitly route
  them out.
- Tooling reliability must classify whether PR/issue GitHub surfaces are
  card-owned managed projections, drift-checked mirrors, linked-only review
  surfaces, or card-local only.
- Tooling reliability must complete the `SOR`-owned PR body and closing-linkage
  convergence slice in `v0.91.6`, not leave it as an unbounded later follow-on.
- Public prompt records must preserve local editable authoring and public export
  boundaries.
- Provider/model reliability must include Gemma reliability and multi-agent
  suitability limits.
- ACIP/A2A must make a protobuf/JSON/WebSocket/access-rule decision or route a
  named residual to `v0.91.7`.
- Security/CAV must remain on the activation path.
- AEE completion, Memory/ObsMem, and ACP/cognitive profiles must be accounted
  before `v0.92` consumes them.
- `#3902` must remain visible as a `v0.91.6` account/setup planning item, not
  as hidden infrastructure work inside the birthday milestone. Its operational
  setup is complete; AWS Activate review and private credit visibility remain
  post-close external follow-up.
- CodeFriend v1 / adapter v2 and guilds must remain visible as companion
  planning routes, not as first-tranche activation proof.
- Runtime Soak #1 must distinguish Tokio substrate completion from integrated
  runtime proof. Runtime coherence requires Soak #2 in `v0.91.7` to close every
  required feature-list row as working, blocked, deferred, or operator-approved
  out of scope, with Soak #3 only if Soak #2 exposes blockers.
- `v0.91.7` residuals must be named, not left as "future work."

## Exit Criteria

- WP-01 can open concrete issues without reconstructing the plan from chat.
- Every first-tranche feature-like WP has a tracked feature-doc route.
- `v0.92` activation remains blocked until bridge truth is reviewed.
