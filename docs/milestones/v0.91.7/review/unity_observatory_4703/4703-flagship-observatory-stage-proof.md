# #4703 Flagship Unity Observatory Stage Proof

Date: 2026-07-04

## Result

PASS: the bound #4703 Unity project contains and validates a flagship
observatory stage scene for the v0.91.7 demo wave.

## Project And Scene

- Unity project: `demos/v0.91.6/unity-observatory`
- Scene: `Assets/Scenes/FlagshipObservatoryStage.unity`
- Symlink used for Unity Hub visibility:
  `/Users/daniel/git/adl-unity-observatory-4703`

## Live Unity MCP Proof

The proof was run through the connected Unity editor using the local
Unity-MCP bridge, not through a player build or cloud endpoint.

Command surface:

- `mcp__ai_game_developer.script_execute`
- method:
  `ADL.Demos.UnityObservatory.Editor.UnityObservatoryFlagshipStageBuilder.CaptureInvestorHeroProof()`

Observed Unity log evidence:

- `ADL flagship observatory stage validation passed. scene=Assets/Scenes/FlagshipObservatoryStage.unity; prefabInstances=43; gameObjects=79; cameras=4; lights=7`
- `ADL flagship observatory hero proof captured. path=Proof/flagship-observatory-investor-hero.png`
- `#4703 flagship proof complete scene=Assets/Scenes/FlagshipObservatoryStage.unity roots=12 dirty=False`

## Retained Visual Evidence

- `docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-investor-hero.png`
- PNG dimensions: 1920 x 1080
- Source local working artifact:
  `demos/v0.91.6/unity-observatory/Proof/flagship-observatory-investor-hero.png`

## Publication Boundary

This issue follows #4745:

- publish owned Unity scene/staging/proof logic and retained review evidence
- keep imported third-party asset roots local/operator-provisioned
- keep generated Unity-MCP/NuGet payloads out of git
- treat Unity-MCP as editor proof tooling, not runtime demo state or player
  build readiness

The full local flagship environment can be reproduced by an operator with the
same Unity Asset Store packages imported into the roots listed in the #4745
asset/MCP publication policy.

## Non-Claims

- This proof does not grant redistribution rights for third-party Unity assets.
- This proof does not claim a clean Git checkout can replay the full imported
  environment without operator-provisioned asset packs.
- This proof does not claim Unity player-build readiness.
- This proof does not claim cloud MCP connectivity is required.
