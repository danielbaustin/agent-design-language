# #4704 Unity Observatory MCP Proof Summary

Date: 2026-07-03

## Result

PASS: Unity-MCP served the #4704 proof project on the required endpoint
`http://localhost:29779`, with the prior wrong-port `24645` endpoint unbound.

#4745 now records the publication boundary for this proof: Unity-MCP is local
editor/proof tooling, imported third-party assets are local
operator-provisioned inputs, and retained proof summaries/screenshots are the
reviewable repository payload.

## Required Endpoint Proof

`script-execute` on `http://localhost:29779` returned:

```text
dataPath=.worktrees/adl-wp-4704/demos/v0.91.6/unity-observatory-4704-proof/Assets
scene=Assets/Scenes/FlagshipObservatoryStage.unity
playMode=False
compiling=False
```

Permission-safe port checks showed:

```text
29779 => bound_port
24645 => unbound_port
```

## Scene Proof

The flagship scene was opened through MCP:

```text
scene=Assets/Scenes/FlagshipObservatoryStage.unity
roots=6
cameras=3
lights=50
rootNames=Main Camera|Directional Sun|Unity Observatory Bootstrap|ADL Flagship Observatory Proof Rig|Wide Observatory Camera|Runtime Detail Camera
```

Runtime/polis proof objects found in the loaded scene include:

```text
Polis Projection Governance
Polis Projection Metrics
Runtime Detail Camera
Polis Evidence Flow
Investor Runtime Banner
Polis Operator Guardrail
Runtime Polis Ribbon
Investor Runtime Backplate
Polis Evidence Wall
Runtime Polis Projection
```

## Retained Visual Evidence

Unity rendered the retained proof image from `Wide Observatory Camera`:

```text
docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png
```

Image verification:

```text
PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced
pixelWidth: 1920
pixelHeight: 1080
mean: [132.82, 148.32, 149.31]
min: [1, 3, 3]
max: [255, 255, 255]
nonblank: True
sha256: 24d71b8b767f1dcc4e96f596045c4949e25e0ff316ee5ae02c572737d07ef7a5
```

## Local Proof Project

After subagent review, the copied Unity proof project was pruned from the full
imported asset payload to the flagship scene dependency closure plus issue-owned
scripts, resources, project settings, README, and proof packet. Oversized
texture inputs were reduced so no retained project file is above 50 MB.

```text
demos/v0.91.6/unity-observatory-4704-proof => 309 MB
```

The copied proof project remains a local issue-worktree proof artifact for this
session. The reviewable PR carries the retained proof summary, walkthrough,
demo-matrix update, and rendered PNG; it does not publish the copied imported
asset folders because no local license or notice surface was found in that
project copy.

Post-prune Unity batchmode validation is not claimed. Three Unity `6000.5.1f1`
batchmode attempts against the pruned MCP-enabled project stalled at Unity's IL
post-processor socket retry before the #4704 validator ran. A disposable
no-MCP validation copy under a temporary workspace also reached the same ILPP
retry, so the blocker appears to be broader Unity batchmode IL post-processing
in this local environment rather than only Unity-MCP. The tooling packet was
attached to #4789.

## Non-Claims

This proof establishes project binding, flagship scene loading, runtime/polis
surface presence, and retained camera-rendered visual evidence. It does not yet
claim post-prune batchmode replay, full investor walkthrough readiness,
build-player readiness, third-party asset redistribution rights, or complete
mini-sprint closeout.
