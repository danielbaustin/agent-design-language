# Unity Observatory Bounded Proof Packet

## Status

Current through ADL issue `#4416`.

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

Observability/security consumption proof: carried by the contract and reviewed
in
`docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md`.

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
- No live OpenTelemetry collector or exporter integration is claimed.
- No inhabitant-safe identity/profile closure beyond redacted lane projections is claimed.
- No production Observatory readiness is claimed.

## Non-Claims

- This packet does not claim Unity editor success.
- This packet does not claim Unity build success.
- This packet does not claim live ADL runtime ingestion.
- This packet does not claim identity-safe inhabitant/profile closure.
- This packet does not claim production Observatory readiness.
