# WP-12 Access And Activation Gate (#4660)

## Metadata

- Issue: `#4660`
- Parent sprint: `#4639`
- Milestone: `v0.91.7`
- Status: access gate recorded
- Machine-readable companion: `docs/milestones/v0.91.7/review/security/wp12_access_activation_gate_4660.json`

## Purpose

Record the enforceable WP-12 access-rule and activation-blocker gate consumed
by the `v0.92` readiness path. This packet closes the access-rule planning gap
without claiming that every WP-12 protocol or CAV surface is complete.

The gate makes external-agent, ACIP, WebSocket, SSM, custody, credential, and
CAV activation claims fail closed unless the named owner evidence is present or
an explicit operator non-claim/defer decision is recorded.

## Access Rules

| Rule | Decision | Required evidence before claim |
| --- | --- | --- |
| Schema access is not message-content access | fail_closed | ACIP schema/projection proof plus denied-access proof for protected message contents. |
| External-agent trust is not implied by transport reachability | fail_closed | Explicit access decision, authority basis, and denial behavior before external-agent trust claims. |
| WebSocket carrier proof is not runtime API activation | fail_closed | WebSocket transport proof may support carrier readiness only; live runtime API integration requires a later promoted issue. |
| SSM access is operations readiness only | bounded_allow | SSM evidence can support local operations claims, but not secret-value, live mutation, or governance-authority claims. |
| Custody signatures require trusted key anchors | fail_closed | Tamper-evident custody proof must verify signatures with an externally supplied trusted public key, not an embedded key alone. |
| Credential break-glass remains approval-bound | fail_closed | Missing, expired, denied, or stale credentials must produce denied/rebind/degraded evidence rather than ambient fallback. |
| CAV red/blue claims require retained scenario evidence | bounded_allow | `#4914` records retained bounded CSM red/blue proof; unbounded adversarial readiness remains a non-claim. |

## Activation Checklist

| Surface | Owner | Disposition | Evidence |
| --- | --- | --- | --- |
| Security/CAV boundary | `#4656` | `gate_recorded_child_blockers_remaining` | `WP12_SECURITY_CAV_PRE_V092_REQUIREMENTS_4656.md`, `wp12_security_cav_gate_4656.json` |
| SSM readiness | `#4657` | `integrated_proven` | `WP12_SSM_READINESS_4657.md`, `wp12_ssm_readiness_4657.json` |
| ACIP schema/protobuf projection | `#4658` | `integrated_proven` | `WP12_ACIP_SCHEMA_PROTOBUF_PROJECTION_4658.md`, `wp12_acip_schema_protobuf_projection_4658.json` |
| WebSocket transport path | `#4659` | `boundary_proven` | `WP12_ACIP_WEBSOCKET_TRANSPORT_4659.md`; merged PR `#5146` |
| Access rules and activation blockers | `#4660` | `access_gate_recorded` | this packet and `wp12_access_activation_gate_4660.json` |
| CAV red/blue runtime proof | `#4914` | `boundary_proven` | bounded retained CSM red/blue scenarios may be cited; do not claim broad live CAV readiness |
| Tamper-evident Polis custody | `#4917` | `integrated_proven` | `WP12_POLIS_CUSTODY_4917.md`, `wp12_polis_custody_4917.json` |
| Credential rotation and break-glass policy | `#4920` | `integrated_proven` | `WP12_CSM_CREDENTIAL_POLICY_4920.md`, retained credential-policy proof packet |

## v0.92 Consumption Rule

`v0.92` may consume a WP-12 row only when the row is one of:

- `integrated_proven`
- `boundary_proven` for the named bounded claim only
- `operator_scoped_out_with_evidence`
- `deferred_noncritical_with_operator_approval`
- `blocked_with_evidence`

Rows that remain `pr_open_pending_ci_review` or `blocked_with_evidence` cannot
support activation-readiness claims. `boundary_proven` rows support only their
named bounded claim; they do not imply production activation readiness.

## Current Result

The access gate is recorded and enforceable by
`adl/tools/validate_wp12_access_activation_gate_4660.py`.

Current WP-12 gate result: `access_gate_recorded` with no open owner-issue or
PR blockers. The bounded proof rows still constrain the claims that v0.92 may
consume.

Bounded rows and non-claims:

- `#4659` and merged PR `#5146` support the retained loopback WebSocket
  transport-path claim only.
- `#4914` now supports bounded retained CSM red/blue scenario claims only.
- Live WebSocket runtime API integration alongside HTTP is noncritical and
  backlog-only until the operator promotes it in the next milestone.

## Non-Claims

- This packet does not open a new issue.
- This packet does not implement live WebSocket runtime API integration.
- This packet does not claim production transport security, TLS, auth, or
  cross-polis networking.
- This packet does not claim x402 or payment-flow readiness.
- This bounded WP-12 packet does not by itself claim final v0.92 activation
  readiness.

## Validation

Focused validation for this gate:

```sh
python3 adl/tools/validate_wp12_access_activation_gate_4660.py \
  --access-gate docs/milestones/v0.91.7/review/security/wp12_access_activation_gate_4660.json \
  --parent-gate docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json
python3 -m json.tool docs/milestones/v0.91.7/review/security/wp12_access_activation_gate_4660.json >/dev/null
git diff --check
```
