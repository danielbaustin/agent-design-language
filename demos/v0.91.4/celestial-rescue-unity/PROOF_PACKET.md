# Celestial Rescue Unity Project Proof Packet

## Status

Prepared for ADL issue `#3460`.

## Project Surface

This project contains a Unity scaffold under `demos/v0.91.4/celestial-rescue-unity/` with:

- `Assets/Scenes/CelestialRescue.unity`
- `Assets/Scripts/CelestialRescueBootstrap.cs`
- `Assets/Scripts/CelestialRescueGame.cs`
- `Assets/Scripts/ShipController.cs`
- `Assets/Scripts/OxygenTimer.cs`
- `Assets/Scripts/SatelliteBeacon.cs`
- `Assets/Scripts/HudController.cs`
- `Assets/UI/CelestialRescueHUD.uxml`
- `Assets/UI/CelestialRescueHUD.uss`
- `Packages/manifest.json`
- `ProjectSettings/ProjectVersion.txt`

## Demo Wiring

The scene seed contains `CelestialRescueBootstrap`. At Play time the bootstrap creates:

- main camera when needed
- game controller
- oxygen timer
- rescue ship with `Rigidbody2D` and `CircleCollider2D`
- five satellite beacons with `CircleCollider2D`
- HUD controller and runtime UI Toolkit visual tree

The UXML/USS files are tracked as the UI Toolkit reference surface for the adopted calm document-panel design direction.

## Validation Truth

Repository structure validation: passed by focused file/directory/content checks during issue execution.

Unity editor validation: not run.

Unity build validation: not run.

C# compiler validation outside Unity: not run.

## Non-Claims

- No Observatory behavior is claimed.
- No Unity editor success is claimed.
- No Unity build success is claimed.
- No external UI toolkit vendoring is claimed.
- This is not a v0.91.4 release-blocking proof surface.
