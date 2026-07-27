# Issue 5662 Live Unity Proof

Date: 2026-07-26

## Alignment

- Unity editor: `6000.5.1f1`
- known editor PID: `39287`
- local MCP endpoint: `http://localhost:29779`
- project:
  `/Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory`
- scene: `Assets/Scenes/FlagshipObservatoryStage.unity`
- alignment result: exact project, editor, MCP endpoint, and loaded scene agreed

## Scene And Play Mode

- scene validator:
  `prefabInstances=43; gameObjects=90; cameras=4; lights=7`
- full shell-and-stage validator:
  `Unity Observatory flagship shell verification passed. scene=FlagshipObservatoryStage`
- fresh Play Mode bootstrap log:
  `ADL Observatory shell built synchronously before the first bootstrap yield.`
- post-install MCP validator result:
  `flagship-stage-and-runtime-shell-valid`
- Unity compile errors after the final source refresh: `0`
- final clean launch after enabling the built-in Particle System module:
  no missing-module component deletions
- background execution:
  `runInBackground=True`; MCP profiler proof observed unpaused Play Mode,
  advancing rendered frames, and `21.17 FPS` while Unity remained on another
  macOS desktop

## Runtime Truth

- Runtime v3 mode reads `ADL_RUNTIME_OBSERVATORY_URL` and
  `ADL_RUNTIME_OBSERVATORY_TOKEN`, requires HTTPS, and constructs
  `Authorization: Bearer` reads for `GET /v1/observatory`
- the accepted schema is exactly `adl.runtime_v3.observatory_feed.v2`; wrong
  versions, missing bearer configuration, HTTP protocol errors, malformed
  control truth, and unhealthy snapshots fail to `Degraded`; connection and
  data-processing failures fail to `Disconnected`
- executable in-editor proof exercised bearer-header construction, parsed
  current Runtime v3 agent, health, revision, continuity, proof, CloudWatch
  route, and event fields, applied a `Live` projection, and then restored the
  explicit demo state
- legacy CSM loopback compatibility remains available, but exact schema equality
  is required; the validator rejects a `vgarbage` suffix
- current Runtime v3 control truth is explicit:
  `/v1/control` requires signed commands and browser mutation authority is false
- communication remains fail closed with:
  `NOT SENT: runtime control exists, but Unity has no governed signed operator-proposal mapping.`

No Runtime v3 listener was running on the canonical local port during the final
visual capture, and the stable repo binary install does not contain
`adl-runtime-kernel`. Per the issue boundary, no replacement owner binary was
built or substituted. The retained images therefore remain explicitly
`DEMO / FIXTURE`; the current Runtime v3 parser, classifier, auth policy, and UI
projection are direct executable in-editor proof, not a claim of a live network
exchange or listener.

## Final Visual Evidence

- `final-playmode-1920x1080.png`
  - exact dimensions: `1920x1080`
  - SHA-256:
    `d363009f87726387659f3d2193eb1f8009f9ad4db6183c5a0801f8a719ef6b9c`
- `final-playmode-2560x1440.png`
  - exact dimensions: `2560x1440`
  - SHA-256:
    `8d4605d0352911e6584780d6b673c2c95c671c030f27fb67ffbc3632ca367e49`

Both captures were produced from the live Game View render texture through
Unity-MCP after selecting exact fixed-resolution profiles and were visually
inspected. The dashboard is fixed to the viewport, navigation icons and labels
remain legible, only the event and inspector interiors scroll, and the active
foundations, guide rails, plinth, undercroft, and support masts close the former
foreground void.

## Boundaries

- no standalone player or binary was built
- no unavailable runtime was shown as connected
- no operator message was sent
- no licensed third-party asset payload is published by this issue
- no player or replacement owner binary was built
