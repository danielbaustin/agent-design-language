# #4704 Unity Observatory Operator Walkthrough

Date: 2026-07-03

## Purpose

This walkthrough is the operator-facing proof path for #4704. It demonstrates
that the flagship Unity observatory scene can be opened through the required
Unity-MCP endpoint and that retained visual evidence exists for the runtime
polis/observatory surface.

## Preconditions

- Unity editor is opened to `demos/v0.91.6/unity-observatory-4704-proof`.
- AI Game Developer is configured for custom HTTP endpoint
  `http://localhost:29779`.
- No wrong-port Unity-MCP endpoint is serving the proof session; `24645` was
  verified unbound during this session.

## Walkthrough Path

1. Confirm the live endpoint is the #4704 proof project by running
   `script-execute` through Unity-MCP and checking that `Application.dataPath`
   resolves to `.worktrees/adl-wp-4704/demos/v0.91.6/unity-observatory-4704-proof/Assets`.
2. Open `Assets/Scenes/FlagshipObservatoryStage.unity` through Unity-MCP.
3. Confirm the loaded scene reports 6 roots, 3 cameras, 50 lights, and roots
   including `ADL Flagship Observatory Proof Rig`, `Wide Observatory Camera`,
   and `Runtime Detail Camera`.
4. Confirm runtime/polis proof objects are present, including `Polis Projection
   Governance`, `Polis Projection Metrics`, `Polis Evidence Flow`, `Investor
   Runtime Banner`, and `Runtime Polis Projection`.
5. Render `Wide Observatory Camera` to retained proof image
   `docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png`.
6. Verify the retained image is a nonblank 1920x1080 PNG.

## Retained Evidence

- Proof summary:
  `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`
- Visual evidence:
  `docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png`

## Non-Claims

This walkthrough proves the #4704 endpoint binding, scene load, runtime/polis
object presence, and retained visual proof. It does not claim build-player
readiness, final investor polish, or mini-sprint closeout.
