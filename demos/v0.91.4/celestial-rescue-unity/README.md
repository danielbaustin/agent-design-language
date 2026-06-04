# Celestial Rescue Unity Demo

## Status

Tracked Unity project scaffold for issue `#3460` in the v0.91.5 demo sprint.

This project is a demoable Unity game surface for the v0.91.4 C-SDLC showcase lane. It does not claim Observatory behavior and does not claim Unity editor/build validation unless the proof packet records a real editor/build run.

## Concept

Pilot a rescue craft through a quiet orbital field, recover five stranded satellites, and return the constellation online before oxygen runs out.

## Controls

- `W` / up arrow: thrust forward
- `S` / down arrow: reverse thrust
- `A` / left arrow: rotate left
- `D` / right arrow: rotate right

## Project Layout

- `Assets/Scenes/CelestialRescue.unity` - scene seed with `CelestialRescueBootstrap`
- `Assets/Scripts/` - gameplay loop, ship movement, satellite rescue, oxygen timer, HUD controller, runtime scene bootstrap
- `Assets/UI/CelestialRescueHUD.uxml` - UI Toolkit HUD/menu/result surface reference
- `Assets/UI/CelestialRescueHUD.uss` - calm, instrument-panel styling inspired by the adopted Unity UI Toolkit direction
- `Packages/manifest.json` - minimal Unity package manifest
- `ProjectSettings/` - project version and build-scene seed

## Open In Unity

1. Open Unity Hub.
2. Add project from disk: `demos/v0.91.4/celestial-rescue-unity`.
3. Use Unity `2022.3 LTS` or a compatible editor.
4. Open `Assets/Scenes/CelestialRescue.unity`.
5. Press Play. `CelestialRescueBootstrap` creates the camera, ship, five rescue satellites, colliders, game controller, oxygen timer, and HUD wiring at runtime.

## Validation Truth

Focused repository validation can verify structure and source presence without Unity. Unity editor/build validation must be recorded separately if actually run.
