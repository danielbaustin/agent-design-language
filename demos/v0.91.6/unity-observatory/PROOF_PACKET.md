# Unity Observatory Bounded Proof Packet

## Status

Current through ADL issue `#4529`.

## Project Surface

This project contains a Unity Observatory scaffold under
`demos/v0.91.6/unity-observatory/` with:

- `Assets/Scenes/UnityObservatory.unity`
- `Assets/Scripts/UnityObservatoryBootstrap.cs`
- `Assets/Scripts/UnityObservatoryShellController.cs`
- `Assets/Editor/UnityObservatoryBatchValidator.cs`
- `Assets/UI/ObservatoryShell.uxml`
- `Assets/UI/ObservatoryShell.uss`
- `Packages/manifest.json`
- `ProjectSettings/ProjectVersion.txt`
- `ProjectSettings/EditorBuildSettings.asset`

The current runtime shell is built programmatically from
`UnityObservatoryShellController.cs`. The UXML and USS assets are tracked
reference surfaces for the same bounded shell and are not claimed as
live-loaded runtime assets in this issue.

The active editor baseline for this bounded scaffold is Unity `6.5`, with
local proof targeting `6000.5.1f1`.

The Unity-facing contract seed now lives at:

- `Assets/Resources/observatory_contract.json`

This seed is the checked-in reference copy of the same bounded contract family
that ADL emits as `unity_observatory_contract.json` in the Observatory CLI
bundle.

## Launch Wiring

The scene seed contains `UnityObservatoryBootstrap`. At Play time the bootstrap:

- creates a main camera when needed
- creates explicit runtime UI Toolkit panel settings
- creates a runtime `UnityObservatoryShellController`
- populates a calm document-panel Observatory shell
- surfaces the governed packet reference
  `demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json`
- keeps the launch seam inside the bounded `adl.csm_visibility_packet.v1`
  contract family
- shows bounded counts, room/lens labels, and proposal-boundary copy
- falls back to deterministic bounded state if the checked-in contract resource
  is missing, empty, or malformed
- routes Unity `2022.3.x` through a compatibility canvas path instead of
  depending on runtime UI Toolkit theme availability
- carries a focused editor verifier at
  `Assets/Editor/UnityObservatoryCompatibilityVerifier.cs` for the
  compatibility canvas path

The current scaffold now loads a deterministic Unity-facing contract seed rather
than stopping at static summary literals. It remains the bounded launch surface
that later issues use for:

- `#4032` ADL evidence/data contract binding
- `#4033` inhabitant-facing world/status/checklist/redacted projection expansion
- `#4034` logging/OTel/security consumption proof
- `#4035` final working Observatory closeout truth

For `#4034`, the same seed now also carries one explicit observability/security
consumption section with:

- `#3999` OTel and event-stream boundary refs
- `#4000` logging-validation and redaction refs
- `#4023` security-floor linkage
- a reviewer packet ref for the issue-owned non-claim proof

## Validation Truth

Repository structure validation: passed by focused file, content, and proof
checks during issue execution.

Deterministic launch-baseline proof: passed by
`bash adl/tools/test_v0916_unity_observatory_baseline.sh`.

Deterministic Unity contract proof: passed by
`bash adl/tools/test_v0916_unity_observatory_contract.sh`
and focused bundle/contract Rust checks.

Deterministic Unity 6.5 working-scene proof: passed by
`bash adl/tools/test_v0916_unity_observatory_unity65_smoke.sh`, which compiles
the migrated project, opens `Assets/Scenes/UnityObservatory.unity`, loads the
checked-in Unity contract resource, and executes
`Assets/Editor/UnityObservatoryBatchValidator.cs` to confirm the scene contains
`UnityObservatoryBootstrap` and that the Observatory shell builds the expected
title, packet-contract, and observability surfaces.

Observability/security consumption proof: carried by the contract and reviewed
in
`docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md`.

Governed Observatory contract proof: passed by
`cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture`
and
`cargo test --manifest-path adl/Cargo.toml csm_observatory_cli_writes_fixture_backed_bundle -- --nocapture`.

Unity editor validation now has two bounded proving lanes:

- Unity `2022.3.62f3` compatibility fallback proof from `#4524`, exercised
  through the in-editor menu verifier `ADL -> Observatory -> Verify
  Compatibility Canvas`, which asserted
  `shouldUseCompatibilityCanvas=True`, a non-empty compatibility payload, and
  `sortingOrder=10`.
- Unity `6000.5.1f1` working-scene migration proof from `#4529`, exercised
  through `bash adl/tools/test_v0916_unity_observatory_unity65_smoke.sh` and
  the checked-in batch validator, which now drives the runtime bootstrap path,
  loads the checked-in contract resource, and asserts the theme/style-backed
  Observatory shell surfaces.

Unity build validation: not run.

C# compiler validation outside Unity: not run.

## Known Limitations

- The shell loads a checked-in Unity-facing contract seed rather than parsing
  the full governed packet directly inside Unity.
- No live Runtime v2 or ADL runtime API integration is claimed.
- No live OpenTelemetry collector or exporter integration is claimed.
- No inhabitant-safe identity/profile closure beyond redacted lane projections is claimed.
- The working-scene proof is limited to the checked-in scene and shell surface;
  it does not claim a standalone player build.
- No production Observatory readiness is claimed.

## Non-Claims

- This packet does not claim Unity build success.
- This packet does not claim live ADL runtime ingestion.
- This packet does not claim identity-safe inhabitant/profile closure.
- This packet does not claim production Observatory readiness.
