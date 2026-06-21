# Unity Observatory Launchable Baseline Proof Packet

## Status

Prepared for ADL issue `#4031`.

## Project Surface

This project contains a Unity Observatory scaffold under
`demos/v0.91.6/unity-observatory/` with:

- `Assets/Scenes/UnityObservatory.unity`
- `Assets/Scripts/UnityObservatoryBootstrap.cs`
- `Assets/Scripts/UnityObservatoryShellController.cs`
- `Assets/UI/ObservatoryShell.uxml`
- `Assets/UI/ObservatoryShell.uss`
- `Packages/manifest.json`
- `ProjectSettings/ProjectVersion.txt`
- `ProjectSettings/EditorBuildSettings.asset`

The current runtime shell is built programmatically from
`UnityObservatoryShellController.cs`. The UXML and USS assets are tracked
reference surfaces for the same bounded shell and are not claimed as
live-loaded runtime assets in this issue.

The Unity-facing contract seed now lives at:

- `Assets/Resources/observatory_contract.json`

This seed is the checked-in reference copy of the same bounded contract family
that ADL emits as `unity_observatory_contract.json` in the Observatory CLI
bundle.

## Launch Wiring

The scene seed contains `UnityObservatoryBootstrap`. At Play time the bootstrap:

- creates a main camera when needed
- creates a runtime `UnityObservatoryShellController`
- populates a calm document-panel Observatory shell
- surfaces the governed packet reference
  `demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json`
- keeps the launch seam inside the bounded `adl.csm_visibility_packet.v1`
  contract family
- shows bounded counts, room/lens labels, and proposal-boundary copy

The current scaffold now loads a deterministic Unity-facing contract seed rather
than stopping at static summary literals. It remains the bounded launch surface
that later issues use for:

- `#4032` ADL evidence/data contract binding
- `#4033` inhabitant-facing world/status/checklist/redacted projection expansion
- `#4034` logging/OTel/security consumption proof

## Validation Truth

Repository structure validation: passed by focused file, content, and proof
checks during issue execution.

Deterministic launch-baseline proof: passed by
`bash adl/tools/test_v0916_unity_observatory_baseline.sh`.

Deterministic Unity contract proof: passed by
`bash adl/tools/test_v0916_unity_observatory_contract.sh`
and focused bundle/contract Rust checks.

Governed Observatory contract proof: passed by
`cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture`
and
`cargo test --manifest-path adl/Cargo.toml csm_observatory_cli_writes_fixture_backed_bundle -- --nocapture`.

Unity editor validation: not run.

Unity build validation: not run.

C# compiler validation outside Unity: not run.

## Known Limitations

- The shell loads a checked-in Unity-facing contract seed rather than parsing
  the full governed packet directly inside Unity.
- No live Runtime v2 or ADL runtime API integration is claimed.
- No inhabitant-safe identity/profile closure beyond redacted lane projections is claimed.
- No HTML/mobile Observatory completion is claimed.

## Non-Claims

- This packet does not claim Unity editor success.
- This packet does not claim Unity build success.
- This packet does not claim completed ADL ingestion or inhabitant readiness.
- This packet does not claim WP-09 closeout readiness.
