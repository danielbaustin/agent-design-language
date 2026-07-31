# Structured Review Prompt

Template: 1.0.0

Issue: 5683

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/5683/LIVE_UNITY_PROOF.md
.csdlc/evidence/5683/TOOLING_ANOMALIES.md
.csdlc/evidence/5683/capture_unity_mcp_image.mjs
.csdlc/evidence/5683/final-full-hd-game-view.png
.csdlc/evidence/5683/final-qhd-game-view.png
.csdlc/evidence/5683/raw-environment-full-hd-pass-3.png
.csdlc/evidence/5683/raw-environment-qhd-pass-3.png
.csdlc/evidence/5683/record-execution-request.json
.csdlc/evidence/5683/record-validation-alignment-request.json
.csdlc/evidence/5683/record-validation-contract-request.json
.csdlc/evidence/5683/record-validation-diff-request.json
.csdlc/evidence/5683/record-validation-live-unity-request.json
.csdlc/issues/5683/audit.jsonl
.csdlc/issues/5683/cards/sip.md
.csdlc/issues/5683/cards/sip.values.json
.csdlc/issues/5683/cards/sor.md
.csdlc/issues/5683/cards/sor.values.json
.csdlc/issues/5683/cards/spp.md
.csdlc/issues/5683/cards/spp.values.json
.csdlc/issues/5683/cards/srp.md
.csdlc/issues/5683/cards/srp.values.json
.csdlc/issues/5683/cards/stp.md
.csdlc/issues/5683/cards/stp.values.json
.csdlc/issues/5683/cards/vpp.md
.csdlc/issues/5683/cards/vpp.values.json
.csdlc/issues/5683/index.json
.csdlc/locks/5683.lock
.csdlc/prepared/issues/5683/advance-implemented.json
.csdlc/prepared/issues/5683/design.md
.csdlc/prepared/issues/5683/diagram.mmd
adl/tools/test_v0916_unity_observatory_contract.sh
demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryFlagshipStageBuilder.cs
demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs

## Prompts

- Does either target-resolution hero frame retain any corrupted-looking cyan or green geometry, particle, material, or lighting artifact?
- Are the architecture, terrain, camera, and lighting composition coherent and investor-presentable?
- Does the operator shell remain fixed, legible, non-overlapping, and truthful about runtime state?
- Does direct evidence prove the intended Unity project and scene rather than a repository-only approximation?
- Does the proof index link the actual demo evidence rather than lifecycle metadata?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Game View sizing relies on undocumented UnityEditor reflection and may require maintenance after Unity upgrades.
- The static contract test is source-oriented; behavioral confidence depends on the retained live Unity proof.
- The scene remains intentionally sparse because problematic imported geometry was deactivated.
- Runtime presentation remains fixture/contract-based, not proof of live CSM integration.

## Review Result

Revision: Some("git-blake3:6a8b2175311f45642e255f4d41af8afd6b1ca630:eb2303a210af1579d18b45a03a6aa232e4050f09678259c6835b24ecddde3e05")

Reviewer: Some("Hooke")

Result: pass
