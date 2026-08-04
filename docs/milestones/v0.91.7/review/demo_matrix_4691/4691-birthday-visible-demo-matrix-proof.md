# #4691 Birthday-Visible Demo Matrix Proof

Date: 2026-07-11

## Result

PASS: the v0.91.7 demo matrix now records the birthday-visible Observatory
surfaces as a proof-backed review map instead of a planning placeholder.

The matrix is runnable enough for milestone review because each row names the
operator command or proof packet that a reviewer can use, and each activation
claim is bounded by retained evidence and explicit non-claims.

## Updated Surface

- `docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md`

## Evidence Inputs

The matrix consumes these landed proof surfaces:

| Surface | Evidence |
| --- | --- |
| HTML Observatory integrated proof | `demos/html-observatory/README.md`; `adl/tools/test_v0917_html_observatory_integrated_proof.sh` |
| HTML retained runtime API mirror | `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/status.json`; `health.json`; `ready.json`; `metrics.json`; `events.json` |
| HTML runtime/AWS/ACIP evidence | `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json`; `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json`; `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json` |
| Unity shell proof | `docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md`; `docs/milestones/v0.91.7/review/unity_observatory_4652/flagship-shell-main-camera-4652.png` |
| Unity flagship stage proof | `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-stage-proof.md`; `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-investor-hero.png` |
| Unity MCP walkthrough proof | `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`; `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md`; `docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png` |
| Asset and Unity-MCP publication boundary | `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md`; `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-manifest.json` |
| Runtime Soak 2 Observatory packet | `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/demo_manifest.json`; `operator_report.md` |

## Reviewer Commands

HTML Observatory retained/integrated proof:

```bash
bash adl/tools/test_v0917_html_observatory_integrated_proof.sh
python3 -m http.server 8765
```

Open the dashboard from the repository root:

```text
http://127.0.0.1:8765/demos/html-observatory/
```

Optional live-loopback HTML proof, after installing the stable owner binary:

```bash
bash adl/tools/install_owner_binaries.sh --bin csm
.adl/bin/csm api serve \
  --spec docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/agent.yaml \
  --bind 127.0.0.1:24645 \
  --max-requests 25 \
  --idle-timeout-ms 60000 \
  --json
```

Unity Observatory review:

- Read the retained proof summaries and images listed above.
- For a full local flagship scene replay, first provision the Unity Asset Store
  packs named by `#4745` into the recorded roots, then use the #4703/#4704
  Unity-MCP proof commands in their proof packets.

## Claim Boundary

This proof establishes a complete review matrix for birthday-visible demo
surfaces that have landed evidence. It does not claim:

- v0.92 activation readiness
- Unity player-build readiness
- clean-checkout replay of third-party Unity asset packs
- browser-owned AWS mutation authority
- full runtime completion beyond the retained runtime/API evidence linked by
  the HTML Observatory proof
- parent WP-09 closeout, which remains owned by #4702 and #4636

## Follow-On Watch

The matrix is compatible with the pending #4689 Unity integrated proof PR. Until
that PR merges, #4691 treats #4689 as an in-flight rollup rather than a landed
proof input.
