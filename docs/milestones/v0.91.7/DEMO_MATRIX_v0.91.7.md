# v0.91.7 Demo Matrix

## Status

birthday-visible proof map ready

## Metadata

- Milestone: `v0.91.7`
- Version: `v0.91.7`
- Date: `2026-07-11`
- Owner: ADL maintainers
- Setup lineage: `#3801`, `#3825`, `#4368`
- Proof update: `#4691`, `#4642`

## Purpose

Record the birthday-visible Observatory and demo surfaces for the final
pre-`v0.92` implementation/readiness tranche. `v0.91.7` still does not claim
runnable demo completion from planning docs alone; this matrix links the
reviewable commands, retained proof packets, screenshots, runtime mirrors, and
claim boundaries that reviewers should use.

## Scope

In scope:

- birthday-visible Observatory proof classification;
- reviewer command and retained-artifact links;
- implementation/proof-surface classification;
- follow-on validation/proof assignments;
- public claim boundaries for runtime, AWS, Unity, and browser behavior.

Out of scope:

- Curiosity runtime proof;
- Constructability validator implementation;
- protocol implementation.
- v0.92 activation readiness;
- Unity player-build readiness;
- clean-checkout replay of third-party Unity Asset Store payloads;
- browser-owned AWS mutation authority.

## Reviewer Preconditions

Working directory:

```bash
git rev-parse --show-toplevel
```

No provider credentials, cloud credentials, or remote runtime services are
required to read the matrix, inspect retained proof, or run the HTML Observatory
retained-proof validator. Optional live-loopback HTML proof requires the local
repo `csm` binary and a loopback API process as documented by
`demos/v0.91.7/html-observatory/README.md`. Optional live Runtime v3 proof
uses the Runtime v3 kernel HTTPS control API on loopback port `20997` and the
explicit query `?runtime=v3&runtimeApiBase=https://localhost:20997&live=1`;
Runtime v2 remains the default.

## Related Docs

- Design contract: `DESIGN_v0.91.7.md`
- WBS: `WBS_v0.91.7.md`
- Sprint plan: `SPRINT_PLAN_v0.91.7.md`
- Checklist: `MILESTONE_CHECKLIST_v0.91.7.md`
- Feature index: `FEATURE_DOCS_v0.91.7.md`

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D1 | Documentation package proof | `#3825` docs package exists and links truthfully | `find docs/milestones/v0.91.7 -maxdepth 2 -type f` | tracked docs | Expected planning, feature, review, and proof docs are present | deterministic filesystem check | ready |
| D2 | Bridge overclaim scan | Docs do not claim runtime or `v0.92` readiness from planning alone | text scan over `docs/milestones/v0.91.7` | this matrix plus issue-local proof packets | claims are bounded by non-goals, proof links, and consumption rules | deterministic text review | ready |
| D3 | Runtime Soak 2 Observatory packet | Runtime evidence has a retained Observatory visibility packet and operator report | inspect `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/` | `visibility_packet.json`, `demo_manifest.json`, `operator_report.md` | runtime/observatory packet exists for the HTML surface to consume | retained JSON/Markdown evidence; not a fresh soak run | proven-retained |
| D4 | HTML Observatory integrated proof | HTML Observatory renders a birthday-visible CSM polis panopticon over retained runtime/API/AWS/ACIP evidence, can optionally upgrade to loopback CSM API polling, and can consume Runtime v3 through explicit HTTPS opt-in on port `20997` | `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`; retained mode may use the documented static server; optional Runtime v3 path requires an HTTPS Observatory origin and `?runtime=v3&runtimeApiBase=https://localhost:20997&live=1` | `demos/v0.91.7/html-observatory/README.md`; `demos/v0.91.7/html-observatory/index.html`; `docs/architecture/runtime_v3_observatory_consumption_5286.v1.json`; retained `csm_liveness_4976` API mirror; WP-08 heartbeat and ACIP-SNS summaries | validator passes; dashboard loads retained proof mirror; Runtime v3 opt-in feed contract consumes runtime-owned `/v1/observatory`; no default Runtime v3 cutover is claimed | deterministic retained-proof validator plus Runtime v3 TLS/feed test; live loopback is optional and local-only | proven |
| D5 | Unity shell proof | Unity Observatory shell can open the flagship scene, render a presentable observatory environment, and instantiate the runtime polis shell | read #4652 packet; use its Unity-MCP commands for live editor replay when the local Unity project is available | `docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md`; `docs/milestones/v0.91.7/review/unity_observatory_4652/flagship-shell-main-camera-4652.png` | #4652 records shell validation success and retained camera evidence | live Unity-MCP proof plus retained image; copied full-asset proof project is not a Git payload | proven-limited |
| D6 | Unity flagship stage proof | Flagship Observatory stage scene exists, validates, and has retained investor-facing visual evidence | read #4703 packet; replay with operator-provisioned Unity asset packs and the proof commands in that packet | `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-stage-proof.md`; `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-investor-hero.png`; `demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryFlagshipStageBuilder.cs` | #4703 records validation pass, scene object counts, and retained 1920x1080 hero proof | deterministic owned scene/proof code; full local visual replay needs operator-provisioned third-party assets | proven-limited |
| D7 | Unity reproducible walkthrough proof | Unity-MCP proof can bind the proof project, load the flagship scene, find runtime/polis objects, and retain nonblank camera evidence | read #4704 packet and walkthrough | `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`; `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md`; `docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png` | #4704 records endpoint proof, scene proof, runtime/polis object names, image dimensions, and nonblank hash | live Unity-MCP proof plus retained image; batchmode replay and player build are not claimed | proven-limited |
| D8 | Unity asset and MCP publication boundary | Unity proofs are reviewable without committing third-party asset packs or generated Unity-MCP payloads | inspect #4745 policy and manifest | `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md`; `docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-manifest.json` | policy names local asset roots, package names, retained publishable payloads, and non-claims | deterministic docs/manifest check; does not grant redistribution rights | proven-boundary |
| D9 | Birthday-visible matrix proof | This matrix is no longer a planning-only list; it is backed by issue-local proof references and reviewer commands | inspect `docs/milestones/v0.91.7/review/demo_matrix_4691/4691-birthday-visible-demo-matrix-proof.md`; run `git diff --check` for this issue | #4691 proof packet and this file | proof packet links all landed visible-demo evidence and names unproven boundaries | retained docs proof; no fresh Unity replay claimed by #4691 | proven |
| D10 | WP-15 demo convergence and proof coverage | v0.91.7 demo-visible proof coverage is converged into a single issue-local packet with current issue state, retained proof, skipped checks, and non-claims | inspect `docs/milestones/v0.91.7/review/V0917_WP15_DEMO_CONVERGENCE_4642.md`; run `python3 -m json.tool docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json` | `docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md`; `docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json` | WP-15 records retained proof coverage and names later quality/review gates without claiming release readiness | deterministic docs/JSON validation plus retained proof references; no fresh Unity, Runtime v3, or AWS run | proven |

## Known Limits

- This matrix records birthday-visible review readiness and links to
  issue-local runtime/demo proof where that proof exists; rows without linked
  issue-local proof still do not claim runtime behavior.
- Curiosity, Constructability, ACIP, security, and reasoning-graph proofs require
  issue-local evidence or evidence-backed blockers before `v0.92` can consume them.
- The #4689 Unity integrated proof rollup has landed and is consumed by the
  WP-09 umbrella closeout packet.
- Full WP-09 closeout remains owned by the WP-09 umbrella #4636.
- Unity player-build readiness, clean-checkout third-party asset replay, and
  browser-owned AWS mutation remain explicit non-claims.
- Runtime v3 Observatory consumption is explicit opt-in only; the dashboard
  default remains retained Runtime v2/CSM mirror evidence, and Unity live
  Runtime v3 consumption is not claimed while #4739 and #4741 remain open.
