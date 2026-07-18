# Structured Task Prompt

Template: 1.0.0

Issue: 5404

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Resolve the four #5403 WP-12 findings without redesigning WP-12 or moving unrelated security work.

## Deliverables

- Corrected CAV/access/protocol truth records
- Focused validator execution or explicit downgrade proof
- Credential-policy synthetic/proof classification
- Regression validation for the corrected behavior

## Acceptance

1. CAV integrated claims are replaced with real boundary-crossing proof or downgraded truth
2. Activation gate, WBS, v0.92 consumers, and validator expectations agree with live issue state
3. Selected CI lanes execute focused #4657/#4660 validators or fail closed when they should
4. Synthetic credential-policy events are unmistakably classified and excluded from operational audit streams
5. Updated proof and review evidence is retained

## Dependencies

- #5403 review packet
- WP-12 security/protocol review packets
- Current #4657/#4659/#4660/#4914 issue and proof state

## Inputs

- docs/milestones/v0.91.7/review/security/WP12_ACCESS_ACTIVATION_GATE_4660.md
- docs/milestones/v0.91.7/review/security/WP12_SECURITY_CAV_PRE_V092_REQUIREMENTS_4656.md
- docs/milestones/v0.91.7/review/security/WP12_CSM_CREDENTIAL_POLICY_4920.md
- docs/milestones/v0.91.7/review/runtime/WP12_ACIP_WEBSOCKET_TRANSPORT_4659.md
- adl/tools/validate_wp12_cav_red_blue_4914.py
- adl/tools/validate_wp12_ssm_readiness_4657.py
- adl/tools/run_pr_fast_coverage_lane.sh

## Non Goals

- Full production CAV red/blue runtime expansion
- Production WebSocket authentication/TLS/cross-polis networking
- Unrelated WP-12 or runtime refactors
