# Unity Observatory Bounded Proof Packet

## v0.91.8 Flagship Presentation Proof (`#5662`)

Issue `#5662` advances the operator-provisioned flagship Observatory from the
earlier asset-staging composition into a coherent investor presentation while
keeping runtime and publication claims fail closed.

The tracked source layer now provides:

- a restrained hero camera aimed through the arrival causeway at the
  Observatory core
- a grounded plinth, undercroft, compact foundation core, structural supports,
  restrained arrival causeway, emissive guide rails, and deep-space backdrop
  that hide unfinished imported-pack edges
- reduced lighting and emission so the architecture, bridge, and core remain
  readable without clipping
- a fixed, translucent command-room shell with compact status metrics,
  Panopticon topology, event stream, inspector, operator communication, and
  footer status surfaces
- explicit `DEMO DATA`, `CONNECTING`, `LIVE`, `DEGRADED`, and `DISCONNECTED`
  state labels
- an HTTPS-only Runtime v3 adapter that constructs bearer-authenticated reads
  for the repository-owned `GET /v1/observatory` contract, plus retained
  loopback-only legacy CSM compatibility
- parsed Runtime v3 agent, health, topology, revision, event, continuity, and
  CloudWatch-route projection when a valid current feed is ingested
- exact current control truth: Runtime v3 exposes signed `/v1/control`, requires
  signed commands for mutation, and grants no browser mutation authority
- interactive navigation and a fail-closed operator-send result:
  `NOT SENT: runtime control exists, but Unity has no governed signed operator-proposal mapping.`

The live Unity `6000.5.1f1` editor was aligned to the exact
operator-provisioned project at:

`/Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory`

The retained Play Mode evidence is:

- `.csdlc/prepared/issues/5662/final-playmode-1920x1080.png`
- `.csdlc/prepared/issues/5662/final-playmode-2560x1440.png`

The files are exact `1920x1080` and `2560x1440` PNG captures. Both show a
fixed dashboard with no page scrolling or incoherent overlap. The scene and UI
remain readable at both target resolutions.

Direct Unity-MCP validation rebuilt the stage, exercised the `Agents`
navigation control and active icon state, verified bounded event and inspector
scroll surfaces, and returned
`flagship-stage-and-runtime-shell-valid`. The final scene validator observed
`43` prefab instances, `90` game objects, `4` cameras, and `7` lights. MCP
entered unpaused Play Mode, observed advancing rendered frames, and captured
the live Game View render texture at exact `1920x1080` and `2560x1440`.

At capture time the permission-safe process helper reported `unbound_port` for
both the Runtime v3 canonical local endpoint, `127.0.0.1:20997`, and the legacy
CSM endpoint, `127.0.0.1:19997`. The shell therefore remained in explicit
fixture mode; no unavailable runtime was presented as connected.

The final code also received direct in-editor Runtime v3 adapter proof. The
validator proves HTTPS endpoint normalization, bearer-header construction,
exact `adl.runtime_v3.observatory_feed.v2` parsing, exact `/v1/observatory` and
signed `/v1/control` capability truth, protocol-error degradation, and visible
agent, event, health, revision, continuity, proof, and CloudWatch-route
projection. It does not claim a network exchange with a live listener. The
validator also proves that parsed runtime events replace retained demo rows.
Legacy CSM compatibility now requires exact endpoint schema equality and
rejects wrong-version suffixes.

The operator action remains fail closed because Unity does not yet have a
governed signed proposal mapping, not because Runtime v3 lacks control:
`NOT SENT: runtime control exists, but Unity has no governed signed operator-proposal mapping.`

The Unity Web Request and Particle System built-in modules are explicitly
present in `Packages/manifest.json`. The final clean launch had no missing-module
component deletions. The bootstrap also enables
`Application.runInBackground` so a Play Mode demo continues to advance when
Unity is on another macOS desktop. A fresh off-screen Play Mode start observed
`runInBackground=True` and an advancing frame counter.

This proof does not claim a standalone player build, a live Runtime v3
connection, an operator message transport, or redistribution rights for the
imported licensed asset packs. The stable repo binary install did not contain
`adl-runtime-kernel`, so the issue did not build or substitute a replacement
owner binary. The retained screenshots truthfully present the checked-in
contract fixture; current Runtime v3 consumption is executable parser,
classifier, auth-policy, and UI-projection proof, not listener proof.

### Issue-owned tooling anomaly ledger

Detailed reproductions are retained in issue `#5662` comments. The bounded
execution encountered and preserved:

- CLI process discovery reporting a false negative while exact PID, project,
  scene, and MCP evidence agreed
- source import retaining a stale assembly until a forced synchronous refresh
- Game View resolution changes retaining the prior layout until Play Mode was
  restarted
- screenshot CLI output embedding a large base64 payload instead of retaining a
  file directly
- the Unity Web Request built-in module being disabled until the package was
  added through Unity-MCP
- the Particle System built-in module being disabled until the package was
  added through Unity-MCP
- standard managed HTTP fallback tasks remaining in `CONNECTING`, after which
  the fallback was removed rather than retained
- Play Mode remaining at frame `1` when Unity moved to another macOS desktop
  until the tracked bootstrap enabled `Application.runInBackground`
- the intended project missing the `com.ivanmurzak.unity.mcp` Editor package,
  which prevented the documented server auto-start until the package was
  installed in the operator project
- `bootstrap-local` requiring a token despite documenting it as optional
- `console-clear` returning HTTP 500 with `Response data is null`
- Play Mode stop returning a stale `IsPlaying: true` response before a bounded
  wait and successful Edit Mode validator proved the transition completed

## Status

Current through ADL issue `#5662`.

Historical issue `#4745` adds the publication boundary for the v0.91.7
flagship staging work:
imported third-party asset roots and generated proof/plugin payloads are not
repository publication payloads until a later issue records license, storage,
and subset approval. The retained review surface for that decision is
`docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md`.
Issue `#5662` identifies two bounded rendered Play Mode frames as its current
publication evidence and retains the baseline and restart captures only as
superseded intermediate history. It does not publish an imported asset root,
reusable licensed source asset, generated plugin payload, or redistribution
bundle.

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

For `#4548`, the proof lane now also stages one explicitly local runtime-derived
bundle into a disposable Unity project copy before batch validation. The
checked-in seed remains the normal project baseline; the runtime-derived swap is
proof-only.

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

Deterministic Unity 6.5 local-runtime consumption proof: passed by
`bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh`,
which:

- generates a fresh Observatory bundle from
  `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`
- stages `unity_observatory_contract.json` into a disposable Unity project copy
- runs the Unity batch validator against that staged project
- asserts the runtime shell renders:
  - title `Prototype CSM 01`
  - packet ref
    `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`
  - artifact root `runtime_v2`
  - report ref `runtime_v2/observatory/operator_report.md`
  - evidence-level note `artifact_backed_fixture`

This proof shows the Unity shell consumes an explicitly local runtime-derived
contract instead of only the older canned checked-in seed, while still stopping
short of live runtime/network ingestion claims.

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

- The shell still loads a Unity-facing contract bundle rather than parsing the
  full governed packet directly inside Unity.
- The v0.91.8 loopback adapter consumes the five current CSM read contracts but
  does not claim governance-decision or ACIP/operator write endpoints that the
  current CSM API does not expose.
- The retained final screenshots use fixture mode because no CSM listener was
  bound during capture; they are not a live-runtime capture.
- No live OpenTelemetry collector or exporter integration is claimed.
- No inhabitant-safe identity/profile closure beyond redacted lane projections is claimed.
- The working-scene proof is limited to the checked-in scene and shell surface;
  it does not claim a standalone player build.
- No production Observatory readiness is claimed.
- The v0.91.7 flagship local staging has used operator-provisioned imported
  asset roots. Those roots are not clean-checkout repository replay proof until
  a later issue records an approved redistribution or reduced-subset route.
- #4745 records the Unity Asset Store package names, expected import roots, and
  local metadata checks needed for an operator to reproduce the full local
  flagship staging outside the repository payload.
- Unity-MCP editor automation is proof tooling only in this packet. It is not
  claimed as runtime demo state, player-build readiness, or a cloud dependency.

## Non-Claims

- This packet does not claim Unity build success.
- This packet does not claim a successful connection to a running ADL runtime
  during the retained screenshot capture.
- This packet does not claim identity-safe inhabitant/profile closure.
- This packet does not claim production Observatory readiness.
- This packet does not grant redistribution rights for third-party Unity asset
  packs.
- This packet does not claim the full imported flagship environment is
  replayable from a clean Git checkout.
