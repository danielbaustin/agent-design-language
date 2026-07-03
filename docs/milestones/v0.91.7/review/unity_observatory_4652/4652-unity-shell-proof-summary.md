# #4652 Unity Observatory Shell Proof Summary

## Scope

This packet records the bounded #4652 proof that the Unity observatory demo surface can open in the flagship scene, render a presentable observatory environment, and instantiate the runtime polis shell through the same bootstrap path used by the Unity demo.

It does not claim a production runtime build, third-party asset redistribution readiness, or full mini-sprint closeout. #4703 owns environment staging, #4704 owns reproducible walkthrough capture, and #4702 owns parent wave closeout.

## Live Project

- Issue worktree: `.worktrees/adl-wp-4652`
- Unity project used for proof: `demos/v0.91.6/unity-observatory-4652-flagship`
- Hub-friendly symlink used during proof: `<host-git-dir>/adl-wp-4652-unity-observatory-flagship`
- Unity version: `6000.5.1f1`
- Unity-MCP endpoint observed during final proof: `http://localhost:24645`

The copied flagship project is retained as issue-local proof and is not proposed as a publishable repository payload in this issue because it includes large generated Unity state and imported third-party asset packs.

## Proof Commands

- `node <Unity-MCP>/cli/dist/cli.js wait-for-ready <host-git-dir>/adl-wp-4652-unity-observatory-flagship --timeout 90000 --interval 3000 --verbose`
  Verified MCP readiness on the live #4652 Unity editor.
- `node <Unity-MCP>/cli/dist/cli.js run-tool script-execute <host-git-dir>/adl-wp-4652-unity-observatory-flagship --input '{"csharpCode":"ADL.Demos.UnityObservatory.Editor.UnityObservatoryBatchValidator.ValidateFlagshipShellScene();",...}' --raw`
  Verified the loaded `FlagshipObservatoryStage` scene can instantiate the runtime shell and find the packet, observability, and runtime polis handoff labels.
- `node <Unity-MCP>/cli/dist/cli.js run-tool script-execute <host-git-dir>/adl-wp-4652-unity-observatory-flagship --input '{"csharpCode":"...scene summary probe..."}' --raw`
  Verified the polished scene contains the bootstrap, proof cameras, investor lighting, runtime polis projection, and runtime contract reference objects.
- `node <Unity-MCP>/cli/dist/cli.js run-tool screenshot-camera <host-git-dir>/adl-wp-4652-unity-observatory-flagship --input '{"cameraName":"Main Camera","width":1600,"height":900}' --raw`
  Captured and decoded the retained visual proof PNG.

## Observed Signals

- Flagship scene summary: `scene=FlagshipObservatoryStage;roots=9;cameras=3;lights=53;bootstrap=True;investorLighting=True;runtimePolisProjection=True;runtimeContractRefs=True`
- Shell validation: `Success`
- Editor state after proof: not playing, not paused, not compiling, not updating.
- Retained visual proof: `docs/milestones/v0.91.7/review/unity_observatory_4652/flagship-shell-main-camera-4652.png`

## Demo Surface Proven

The shell includes a dedicated runtime polis strip with:

- `Runtime polis surface`
- `Bounded polis state is inspectable, trace-backed, and still explicitly governed.`
- `proposal-only controls`
- `#4704 walkthrough capture`

The flagship scene includes investor-facing proof objects for:

- runtime polis projection
- runtime contract reference
- runtime artifact root
- runtime operator report
- investor lighting and proof cameras

## Non-Claims

- This proof does not claim a player build.
- This proof does not publish the copied third-party asset project.
- This proof does not claim #4704 walkthrough capture is complete on this branch.
- This proof does not claim full #4702 mini-sprint closeout.
- Unity-MCP tooling anomalies remain tracked in #4789.
