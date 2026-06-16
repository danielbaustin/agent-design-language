# v0.91.7 Candidate Work Breakdown Structure

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-06-16`
- Status: candidate WP sequence for second pre-`v0.92` bridge tranche
- Setup issue: `#3801`

## Status

Candidate allocation only. `v0.91.7` issues have not been opened from this WBS.

WP-01 should consume this document and
[WP_ISSUE_WAVE_v0.91.7.yaml](WP_ISSUE_WAVE_v0.91.7.yaml), then open concrete
GitHub issues with canonical C-SDLC cards.

## WBS Summary

`v0.91.7` should turn remaining pre-`v0.92` conceptual surfaces into reviewed
feature docs and decision records without claiming the birthday implementation.

## Candidate WP Sequence

| WP | Work Package | Description | Primary deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Planning promotion and issue-wave readiness | Promote this candidate package, reconcile `#3778` and `v0.91.6`, open the issue wave, and prepare cards. | Opened issue wave and card bundles. | `#3778`, `#3800`, `#3801`. |
| WP-02 | Curiosity Engine / Discovery Substrate | Define curiosity artifacts, detection hooks, hypotheses, experiment plans, discovery budget, governance, Freedom Gate integration, ObsMem/reasoning-graph update, and proof. | Curiosity feature doc and proof expectations. | WP-01. |
| WP-03 | Constructability Gate | Define construction-event schema, external anchors, admissibility validator, shared-reality boundary, and proof path. | Constructability feature doc and validation plan. | WP-01, WP-02. |
| WP-04 | Reasoning graph, loop runtime, and `adl.skill.v1` | Define the pre-v0.92 bridge among prompts, skills, loops, trace, ObsMem, PVF, AEE, Runtime v2, UTS, ACC, and `adl.skill.v1`. | Reasoning graph / skill-standard bridge feature doc. | WP-01, WP-02. |
| WP-05 | Residual security readiness | Account for security/CAV residuals after v0.91.6 and define any activation-path blockers. | Security residual addendum. | WP-01, v0.91.6 security output. |
| WP-06 | Residual ACIP/A2A/protobuf decisions | Decide remaining protobuf/JSON/WebSocket/access-rule questions after v0.91.6. | ACIP/A2A residual decision record. | WP-01, WP-03, WP-05. |
| WP-07 | Affect and happiness bridge accounting | Define safe tests, non-claims, and public-evidence limits for affect, humor, happiness, and wellbeing. | Affect/happiness bridge record. | WP-01. |
| WP-08 | Godel mechanics bridge accounting | Map experiment, hypothesis, mutation, evaluation, promotion, and proof boundaries for the birthday path. | Godel mechanics bridge record. | WP-01, WP-02, WP-04. |
| WP-09 | Economics-context decision | Decide whether economics is context-only for v0.92 or needs explicit tests. | Economics-context decision record. | WP-01. |
| WP-10 | Bridge ledger refresh and v0.92 handoff | Update second-tranche dispositions and define what `#3780` may consume. | Bridge-ledger addendum or v0.92 handoff record. | WP-02 through WP-09. |
| WP-11 | Internal review | Review docs for missing bridge surfaces, overclaims, security gaps, and vague spillover. | Review packet and finding register. | WP-10. |
| WP-12 | Remediation and closeout | Fix or route review findings and record closeout truth. | Remediation PRs, final checklist, and closeout packet. | WP-11. |

## Acceptance Mapping

- Curiosity must include governed discovery-cycle proof expectations.
- Constructability must distinguish provisional cognition from shared reality.
- Reasoning graph / `adl.skill.v1` must be a bridge, not a hand-waved standard.
- Security residuals must wait for the v0.91.6 security/CAV output.
- ACIP/A2A residuals must have explicit v0.92 dispositions and must consume
  both security and constructability boundaries.
- Affect/happiness must preserve safe-test and non-claim boundaries.
- Godel mechanics must be mapped before birthday evidence consumes it.
- Economics must be explicitly context-only, test-required, blocked, or routed.

## Exit Criteria

- WP-01 can open concrete issues without reconstructing the plan from chat.
- Every second-tranche feature-like WP has a tracked feature-doc route.
- `#3780` can refresh v0.92 activation docs from tracked bridge truth.
