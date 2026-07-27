# Structured Review Prompt

Template: 1.0.0

Issue: 5662

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

demos/v0.91.6/unity-observatory/Assets/Editor/UnityObservatoryFlagshipStageBuilder.cs
demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs
demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs
demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uxml
demos/v0.91.6/unity-observatory/Assets/UI/ObservatoryShell.uss
demos/v0.91.6/unity-observatory/Assets/Resources/ObservatoryShellRuntime.uss
demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json
demos/v0.91.6/unity-observatory/Assets/Scenes/FlagshipObservatoryStage.unity
demos/v0.91.6/unity-observatory/PROOF_PACKET.md
.csdlc/issues/5662
.csdlc/prepared/issues/5662

## Prompts

- Does the first Play Mode frame read as one deliberate Observatory rather than a staging pile?
- Are camera, terrain, ramps, lighting, materials, UI, and runtime state coherent at both target resolutions?
- Can every live, degraded, disconnected, demo, governance, evidence, and communication claim be traced to direct supported proof?
- Does the diff contain only repository-owned publishable source and exclude licensed payloads and binary output?
- Were all tooling anomalies retained or routed with current reproduction evidence?

## Findings

[
  {
    "id": "R5662-01",
    "severity": "p1",
    "summary": "The Unity adapter supports only unauthenticated loopback HTTP CSM reads and cannot consume the current bearer-authenticated HTTPS Runtime v3 Observatory feed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-02",
    "severity": "p1",
    "summary": "The communication surface falsely says no ACIP endpoint exists even though current Runtime v3 exposes authenticated control channels; Unity must report the missing signed proposal mapping instead.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-03",
    "severity": "p1",
    "summary": "Legacy CSM schema validation accepts any suffix after the version prefix, including invalid schemas such as vgarbage, so aggregate Live classification remains fail open.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-04",
    "severity": "p2",
    "summary": "Both retained captures expose a large floating foreground wedge and open void because the builder deactivates the foundations and supports it creates, contradicting the grounded composition acceptance criterion.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-05",
    "severity": "p2",
    "summary": "The fixed dashboard lacks the required clear navigation iconography, bounded internal scroll surfaces, and dynamic active navigation state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-06",
    "severity": "p1",
    "summary": "Runtime v3 Live classification does not require the authoritative signed control endpoint to equal /v1/control.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-07",
    "severity": "p1",
    "summary": "Runtime v3 proof overstates bearer transport and UI projection because it does not execute shared header construction or visibly assert agent, proof, continuity, and CloudWatch-route projections.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-08",
    "severity": "p2",
    "summary": "Runtime v3 HTTP protocol errors such as 401 and 500 are passed to the classifier as transport failures and become Disconnected instead of Degraded.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-09",
    "severity": "p1",
    "summary": "Publication truth remains pre-review with open findings and historical intermediate capture references not explicitly identified as superseded.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-10",
    "severity": "p2",
    "summary": "The executable Runtime v3 projection validator does not assert the parsed proof state even though the publication packet claims proof projection coverage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  },
  {
    "id": "R5662-11",
    "severity": "p3",
    "summary": "The proof packet says only two PNGs are retained although the commit also retains baseline and restart captures as superseded intermediates.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Runtime v3 validation is executable in-editor parser, classifier, bearer-auth construction, and UI projection proof, not a live HTTPS listener exchange.
- The retained publication captures truthfully show DEMO / FIXTURE data rather than a live Polis.
- Full rendering depends on operator-provisioned licensed Unity assets outside Git.

## Review Result

Revision: Some("git-blake3:eb7403a86b2d4a9649eaff8ee247cf978e76d5d1:4ca901916f76c6397493dce538b6cfff7df290cd123e9f29dd02f4d70fb6ec92")

Reviewer: Some("subagent:lagrange:019fa08a-85f5-75c1-a411-db74c1d1e031")

Result: pass
