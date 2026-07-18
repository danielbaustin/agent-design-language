# WP-12 Security And CAV Pre-v0.92 Requirements Gate (#4656)

## Metadata

- Issue: `#4656`
- Parent sprint: `#4639`
- Milestone: `v0.91.7`
- Status: gate recorded; bounded child proofs retained; no open WP-12 owner-issue or PR blocker remains
- Machine-readable companion: `docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json`

## Purpose

Record the WP-12 security and CAV gate that prevents `v0.92` activation from
silently inheriting unresolved security, CAV, protocol, access-rule, or public
evidence claims.

This packet is not a launch-readiness claim. It is the issue-local control
surface for `#4656`: the security/CAV requirements are now named, owner-bound,
and claim-limited to retained child evidence while production transport
activation remains an explicit non-claim.

## Findings

### F-4656-01: WP-12 security/CAV readiness is bounded by retained child proofs

Severity: blocker

Evidence:

- `docs/milestones/v0.91.7/review/runtime/SOAK2_REVIEW_BLOCKER_REGISTER_4844.md`
  assigns `capability_envelope` and `security_cav_boundary` to `#4656` and
  keeps them blocked before final activation claims.
- `docs/milestones/v0.91.7/review/runtime/soak2_4682/security_cav_boundary/proof_packet.json`
  records a fail-closed paused-boundary proof, not a complete CAV readiness
  proof.
- `docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json`
  records bounded retained CSM red/blue proof for #4914.
- `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`
  requires threat-model review and explicit blocker or approval records before
  security work can move out of activation.

Disposition:

- `#4656` records the gate and requirement ledger.
- `#4914`, `#4917`, and `#4920` now provide bounded retained evidence for
  adversarial CAV, tamper-evident custody, key-management, witness, and receipt
  claims, limited by their recorded non-claims.

### F-4656-02: ACIP/A2A security cannot be claimed until protocol and access owners settle

Severity: blocker

Evidence:

- `docs/milestones/v0.91.7/features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`
  states that unresolved activation-path protocol decisions block `v0.92`
  unless the operator explicitly scopes them out.
- `docs/milestones/v0.91.7/review/runtime/ACIP_RUNTIME_STREAM_SUBSTRATE_4900.md`
  selects WebSocket and `tokio-tungstenite` for carrier mechanics, but
  explicitly excludes protobuf, production WebSocket authentication, reconnect
  scheduling, cross-polis transport, and access-rule closure.
- `docs/milestones/v0.91.7/review/runtime/SOAK2_REVIEW_BLOCKER_REGISTER_4844.md`
  keeps `acip_a2a_path` blocked under `#4658`.

Disposition:

- `#4658` records integrated proof for schema/protobuf projection and
  consumption posture.
- Closed `#4659` and merged PR `#5146` retain the bounded WebSocket transport
  path that consumes the #4900 carrier decision.
- Closed `#4660` retains external-agent access rules, denial behavior, and
  trust boundaries.

### F-4656-03: Public evidence, profile privacy, and launch narrative need custody and key proof

Severity: high

Evidence:

- `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md` says capability envelope,
  witnesses, receipt, and activation-path security may be consumed only as
  integrated proof, operator-scoped-out evidence, or blocked evidence.
- `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`
  keeps public evidence and profile privacy requirements in scope.

Disposition:

- `#4917` must provide tamper-evident custody proof before public evidence or
  profile privacy language relies on retained artifacts.
- `#4920` must provide key rotation and break-glass policy before durable
  signing, custody, or recovery claims become activation evidence.

## Requirement Ledger

| Requirement | Owner | Current state | v0.92 impact |
| --- | --- | --- | --- |
| Capability envelope, witness, and receipt readiness | `#4656` with `#4914`, `#4917`, `#4920` | bounded child proofs retained | Supports bounded evidence claims; final activation remains outside this packet. |
| Security/CAV activation boundary | `#4656` with `#4914`, `#4917`, `#4920` | bounded child proofs integrated | Supports bounded CAV, custody, and credential policy claims; no destructive cloud, secret-retention, or production WebSocket runtime API claim. |
| SSM and local polis operations readiness | `#4657` | integrated proven | Supports SSM operations claims; secret values, provider/model execution, governance authority, and unattended mutation remain non-claims. |
| ACIP/A2A schema and protobuf projection | `#4658` with `#4900` | integrated proven | Schema/projection ready; bounded transport and fail-closed access evidence are retained without a production activation claim. |
| ACIP WebSocket transport path | `#4659` with `#4900` | boundary proven | Supports the retained loopback transport-path claim; production transport remains a non-claim. |
| External-agent access rules | `#4660` | access gate recorded | Defines fail-closed activation checklist and bounded allow/non-claim surfaces. |
| CAV runtime red-blue proof | `#4914` | boundary proven | Supports bounded retained CSM red/blue scenario claims only. |
| Tamper-evident evidence custody | `#4917` | integrated proven | Supports tamper-evident custody claims within #4917 non-claims. |
| Key rotation and break-glass policy | `#4920` | integrated proven | Supports policy and local negative-case claims within #4920 non-claims. |
| Curiosity/Constructability security gates | `#4637` with `#4692`, `#4693` | blocked until promoted or non-claimed | Blocks public claims if promoted into activation. |

## Activation Rule

WP-12 and `v0.92` may consume a row only when it is one of:

- `integrated_proven`: implementation runs in the integrated path with retained
  evidence;
- `boundary_proven`: retained evidence supports only the named bounded claim and
  does not establish production activation readiness;
- `operator_scoped_out`: implementation proof is outside `v0.92` activation
  scope, with evidence, risk, and operator approval recorded;
- `blocked_with_evidence`: named missing evidence or decision prevents
  activation use.

Any row still marked `child_issue_open`, `blocked_until_child_proofs`, or
`blocked_until_promoted_or_non_claimed` is not activation-ready. The WP-12
owner issues are closed; boundary-proven rows remain limited to their named
claims and do not establish production activation readiness.

## Non-Claims

- This packet does not claim `v0.92` security readiness.
- This packet does not claim ACIP/A2A/protobuf protocol completion.
- This packet does not approve external-agent trust, production WebSocket
  authentication, or launch-scope CAV claims.
- This packet does not move unresolved activation-path work to `v0.93` without
  explicit operator approval.

## Validation

Focused local validation for this packet:

```sh
python3 -m json.tool docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json >/dev/null
python3 - <<'PY'
import json
path = "docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json"
data = json.load(open(path, encoding="utf-8"))
assert data["schema"] == "adl.wp12.security_cav_gate.v1"
assert data["issue"] == 4656
assert data["requirements"]
expected_states = {
    "capability_envelope_witness_receipt_readiness": "boundary_proven",
    "security_cav_activation_boundary": "integrated_proven",
    "ssm_and_local_polis_secret_readiness": "integrated_proven",
    "acip_a2a_schema_and_protobuf_projection": "integrated_proven",
    "acip_websocket_transport_path": "boundary_proven",
    "external_agent_access_rules": "access_gate_recorded",
    "cav_runtime_red_blue_proof": "boundary_proven",
    "tamper_evident_evidence_custody": "integrated_proven",
    "key_rotation_and_break_glass_policy": "integrated_proven",
    "curiosity_constructability_security_gates": "blocked_until_promoted_or_non_claimed",
}
assert {row["id"] for row in data["requirements"]} == set(expected_states)
for row in data["requirements"]:
    for key in ("id", "owner_issue", "state", "v092_disposition", "evidence", "required_before_claim"):
        assert row.get(key), (row.get("id"), key)
    assert row["state"] == expected_states[row["id"]], row["id"]
PY
git diff --check
```

This validation proves the retained ledger is parseable and pins every row to
its reviewed integrated, bounded, access-gate, or blocked state.
