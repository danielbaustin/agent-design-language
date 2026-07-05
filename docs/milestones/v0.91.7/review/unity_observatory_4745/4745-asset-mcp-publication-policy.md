# #4745 Unity Observatory Asset And MCP Publication Policy

Date: 2026-07-04

## Result

PASS: #4745 resolves the current publication boundary for the flagship Unity
observatory demo.

The repository publication route is:

- keep the canonical Unity Observatory scaffold, scripts, contracts, project
  settings, proof summaries, and retained visual evidence in Git
- keep imported third-party asset-pack roots out of Git until a separate
  license, storage, and redistribution decision approves a specific subset
- keep Unity-MCP as proof tooling, not as runtime demo state or a player-build
  dependency
- keep local investor-demo staging reproducible for an operator who has access
  to the same Unity Asset Store packages, with exact root folders and checks
  recorded below rather than hidden machine-local state

## Evidence Inputs

The decision consumes the current #4703 live proof packet:

- `.worktrees/adl-wp-4703/.adl/reviews/unity-observatory-4703-live-proof/20260704-mcp-local-server-blocker.md`
- `.worktrees/adl-wp-4703/demos/v0.91.6/unity-observatory/Proof/flagship-observatory-investor-hero.png`

Observed #4703 imported payload sizes:

| Local path | Observed size | Publication decision |
| --- | ---: | --- |
| `Assets/Creepy_Cat` | 5.3G | External/operator-provisioned only |
| `Assets/ScifiOfficeLite` | 274M | External/operator-provisioned only |
| `Assets/Sci-Fi Styled Modular Pack` | 13M | External/operator-provisioned only until license is recorded |
| `Assets/Plugins/NuGet` | included in 17M plugin root | Proof-tooling payload only; do not vendor |
| `Proof` | 2.1M | Retain selected reviewer-facing screenshots under `docs/` |

Local metadata inspection found these Unity Asset Store package names under the
imported roots, all with `licenseType: Store` in `.meta` files:

| Package name from local metadata | Expected root after import |
| --- | --- |
| `3D Scifi Kit Starter Kit` | `Assets/Creepy_Cat/3D Scifi Kit Starter Kit_HD` |
| `3D Showroom Level Kit Vol 11` | `Assets/Creepy_Cat/ShowRoom_Vol 11` |
| `3D Showroom Level Kit Vol 32` | `Assets/Creepy_Cat/ShowRoom_Vol 32` |
| `Free Sci-Fi Office Pack` | `Assets/ScifiOfficeLite` |
| `Sci-Fi Styled Modular Pack` | `Assets/Sci-Fi Styled Modular Pack` |

Existing retained downstream proof packets:

- `docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md`
- `docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md`

## Asset Publication Decision

The imported packs are not committed in this issue.

Reasons:

- the largest observed imported root is 5.3G
- no complete license or redistribution notice surface was found for the local
  #4703 imported asset set
- the existing #4704 proof already retained a pruned local proof project as a
  worktree artifact while publishing only reviewable summaries and screenshots
- committing the asset roots would make the Unity demo path review-hostile and
  would not by itself prove license safety

The accepted route for #4703 publication is therefore:

1. Publish the deterministic Unity scripts, scene-builder/editor proof code,
   project metadata, docs, and selected visual proof artifacts that are owned by
   ADL.
2. Treat the third-party asset roots as local operator-provisioned inputs.
3. Reproduce the full local staging environment by importing the package names
   listed above through Unity Package Manager / My Assets into the exact
   expected roots, then verify those roots and sizes before running #4703
   staging proof.
4. Record any future asset subset by source, license, exact root path, size,
   and reason before it becomes a Git or LFS payload.
5. Prefer a reduced subset over a full-pack import if later publication needs
   repository-contained scene replay.

Local acquisition verification commands:

```bash
test -d demos/v0.91.6/unity-observatory/Assets/Creepy_Cat
test -d demos/v0.91.6/unity-observatory/Assets/ScifiOfficeLite
test -d "demos/v0.91.6/unity-observatory/Assets/Sci-Fi Styled Modular Pack"
rg -n "packageName: (3D Scifi Kit Starter Kit|3D Showroom Level Kit Vol 11|3D Showroom Level Kit Vol 32|Free Sci-Fi Office Pack|Sci-Fi Styled Modular Pack)" demos/v0.91.6/unity-observatory/Assets -g "*.meta"
rg -n "licenseType: Store" demos/v0.91.6/unity-observatory/Assets/Creepy_Cat demos/v0.91.6/unity-observatory/Assets/ScifiOfficeLite "demos/v0.91.6/unity-observatory/Assets/Sci-Fi Styled Modular Pack" -g "*.meta"
du -sh demos/v0.91.6/unity-observatory/Assets/Creepy_Cat demos/v0.91.6/unity-observatory/Assets/ScifiOfficeLite "demos/v0.91.6/unity-observatory/Assets/Sci-Fi Styled Modular Pack"
```

## Unity-MCP Dependency Decision

Unity-MCP is accepted as local editor/proof tooling for this mini-sprint.

Unity-MCP is not accepted as runtime demo state for the canonical Observatory
project until a separate issue proves player/build behavior and dependency
policy.

Publication rules:

- `com.ivanmurzak.unity.mcp` may appear in issue-local Unity proof projects or
  worktrees when the proof needs live editor automation.
- Do not commit generated `Assets/Plugins/NuGet` payloads as the source of
  truth for Unity-MCP.
- Do not claim the investor demo requires a cloud MCP endpoint.
- Do not claim player build readiness from Unity-MCP editor proof.
- Use the repo-pinned Unity-MCP CLI at `<host-git-dir>/Unity-MCP/cli/dist/cli.js`
  for local proof in this environment; do not rebuild tooling binaries for this
  sprint.

Current local MCP proof truth:

- project config was restored to Custom/local mode
- Codex client config was repaired to the live local endpoint
- local tool calls succeeded against `scene-list-opened`
- live session native Codex Unity MCP `script_execute` returned
  `dataPath=.worktrees/adl-wp-4703/demos/v0.91.6/unity-observatory/Assets`,
  `scene=Assets/Scenes/FlagshipObservatoryStage.unity`, `roots=12`, and
  `playing=False`; this session proof is recorded here and should be repeated
  by #4704 if a durable walkthrough proof needs the same exact output
- residual symlink/process detection mismatch remains tracked under #4739/#4741

## Downstream Routing

#4703 environment staging:

- may keep the local full-asset worktree as the operator demo source
- should publish owned scene-builder/editor logic, proof packet, selected
  retained screenshots, and this policy reference
- must not publish the 5.3G asset root without a later license/storage decision

#4652 shell/demo-surface integration:

- may consume the retained shell proof and runtime-polis surface
- must not treat its copied flagship project as a publishable repository
  payload
- should reference this policy for third-party asset and Unity-MCP boundaries

#4704 reproducible proof/walkthrough:

- remains the proof/walkthrough owner
- should keep retained visual evidence under `docs/`
- should treat endpoint numbers as session-local unless the proof explicitly
  requires and verifies a fixed endpoint

#4702 parent mini-sprint:

- can truthfully claim live local Unity proof and retained screenshots
- cannot yet claim a self-contained repository replay of the full imported
  flagship environment
- cannot yet claim player-build or full investor-polish readiness

## Fresh Reviewer Reproduction Contract

A fresh reviewer can consume the publishable proof without hidden local state
by reading:

1. this policy packet
2. the #4652 and #4704 retained proof summaries
3. the retained PNGs under `docs/milestones/v0.91.7/review/unity_observatory_*`
4. the canonical Unity scaffold in `demos/v0.91.6/unity-observatory`

A fresh reviewer can reproduce the full local flagship environment only after
operator-provisioning the same Unity Asset Store package names into an
issue-local Unity worktree at the exact roots listed in this packet. That route
is intentionally not represented as repository self-containment.

## Non-Claims

- This packet does not grant redistribution rights for any third-party asset.
- This packet does not claim the full imported environment is replayable from a
  clean Git checkout.
- This packet does not claim Unity player-build readiness.
- This packet does not claim Unity-MCP cloud connectivity is required.
- This packet does not close #4703, #4652, #4704, or #4702.
