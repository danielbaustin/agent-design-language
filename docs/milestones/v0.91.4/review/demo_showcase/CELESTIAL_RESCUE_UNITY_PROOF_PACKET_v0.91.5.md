# Celestial Rescue Unity Proof Packet v0.91.5

## Status

Prepared for issue `#3460`.

## Summary

This packet records the Unity demo project created under `demos/v0.91.4/celestial-rescue-unity/` for the Celestial Rescue showcase lane.

## What Exists

- Unity project scaffold with `Assets`, `Packages`, and `ProjectSettings`.
- Gameplay scripts for ship movement, satellite rescue, oxygen timer, game state, and HUD updates.
- Runtime bootstrap script that creates the camera, ship, five satellites, 2D colliders, game controller, oxygen timer, and HUD wiring when the scene enters Play mode.
- UI Toolkit UXML/USS HUD, menu, and result reference surfaces.
- Scene seed and project settings seed.
- Runbook in the project README.
- Project-local proof packet at
  `demos/v0.91.4/celestial-rescue-unity/PROOF_PACKET.md`.

## Adopted Toolkit Direction

The project uses Unity UI Toolkit UXML/USS surfaces and a calm document-panel visual direction aligned with the adopted `sinanata/unity-ui-document-design-system` reference. This issue records the adoption direction but does not vendor that external toolkit or claim pixel-perfect implementation of the reference.

## Validation Recorded

Focused structure validation should check:

- `demos/v0.91.4/celestial-rescue-unity/Assets`
- `demos/v0.91.4/celestial-rescue-unity/Packages`
- `demos/v0.91.4/celestial-rescue-unity/ProjectSettings`
- gameplay scripts under `Assets/Scripts`
- UI Toolkit surfaces under `Assets/UI`
- scene seed under `Assets/Scenes`
- `CelestialRescueBootstrap` runtime assembly path

Unity editor/build validation: not run in this issue unless a later SOR explicitly records a real Unity editor/build command and result.

## Non-Claims

- No Observatory behavior is claimed.
- No Unity build success is claimed unless separately recorded.
- No external toolkit vendoring is claimed.
- This is not a v0.91.4 release-blocking proof surface.
