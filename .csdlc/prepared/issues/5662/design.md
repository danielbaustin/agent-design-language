# Issue 5662 Design: Flagship Polis Observatory Polish

## Intent

Transform the existing functional `FlagshipObservatoryStage` into a coherent
investor-quality Observatory without importing licensed payloads into Git or
claiming runtime behavior that direct Unity proof does not establish.

## Ownership Boundary

The operator-provisioned Unity project on FastWork is the live authoring
surface. Its licensed asset roots remain external and untracked. The
repository-owned source of truth is limited to:

- `Assets/Scenes/FlagshipObservatoryStage.unity`
- `Assets/Editor/UnityObservatoryFlagshipStageBuilder.cs`
- `Assets/Scripts/UnityObservatoryBootstrap.cs`
- `Assets/Scripts/UnityObservatoryShellController.cs`
- `Assets/UI/ObservatoryShell.uxml`
- `Assets/UI/ObservatoryShell.uss`
- `Assets/Resources/ObservatoryShellRuntime.uss`
- `Assets/Resources/observatory_contract.json`

Repository-owned files are edited in the bound worktree and synchronized to
the live FastWork project for Unity import and proof. Scene mutation is
performed through Unity-MCP or editor tooling, then the resulting publishable
scene is synchronized back into the worktree. Licensed pack directories are
never copied into Git.

## Experience Architecture

### Spatial composition

The first Play Mode frame must read as a deliberate Observatory campus:

1. A grounded arrival foreground establishes scale and a visible route.
2. The central operations chamber is the primary focal structure.
3. Secondary platforms and silhouettes provide depth without competing with
   the central chamber.
4. Terrain, ramps, stairs, and platforms meet cleanly with no visible voids,
   floating edges, or accidental staging geometry.
5. The hero camera uses stable framing suitable for 16:9 proof at 1920x1080
   and 2560x1440.

### Lighting and material hierarchy

- Neutral key and fill lighting preserve asset materials and architectural
  depth.
- Cyan emission identifies control pathways and active systems but does not
  determine the whole exposure.
- Warm amber is reserved for events, caution, and human-scale wayfinding.
- Green communicates healthy runtime truth only.
- Background and terrain remain readable enough to explain the campus shape.

### Operator shell

The screen-space shell is a fixed dashboard overlay with internal scrolling
only. It exposes:

- connection and truth mode: live, degraded, disconnected, or demo;
- runtime readiness and system health;
- agent selection and inspector state;
- recent event flow;
- governance and evidence state when supported;
- bounded operator communication when supported;
- explicit non-claims for unavailable runtime fields.

The scene remains visible behind and between control surfaces. UI text and
icons must stay crisp and non-overlapping at both proof resolutions.

## Runtime Truth Model

The repository-owned Observatory contract is authoritative for supported
fields. Presentation state is derived as follows:

- **Live:** a current runtime response passes the contract and is visibly
  identified as live.
- **Degraded:** the runtime responds but one or more required health or
  capability checks fail.
- **Disconnected:** no current runtime response is available.
- **Demo:** fixture or demonstrative data is explicitly enabled and visibly
  labeled; it is never presented as live.

Unsupported governance, evidence, or communication capabilities remain
visible as unavailable rather than being fabricated.

## Interaction

- Select an agent or runtime subsystem from the scene or shell.
- Inspect current identity, state, health, and supported metrics.
- Observe bounded event activity without obscuring the hero environment.
- Send an operator message only through a supported runtime contract.
- Fail closed with an exact visible reason when communication is unsupported
  or disconnected.

## Execution Sequence

1. Re-prove the intended project, active MCP endpoint, loaded scene, and clean
   editor state.
2. Capture a baseline at 1920x1080 and identify the highest-impact composition
   defects.
3. Correct hero camera, terrain/platform continuity, focal hierarchy, and
   staging voids through Unity-MCP/editor operations.
4. Tune lighting, emission, environment, and material readability.
5. Refine the fixed operator shell and runtime truth states.
6. Exercise agent inspection, event flow, and operator communication against
   the supported contract or retain exact fail-closed evidence.
7. Capture 1920x1080 and 2560x1440 Play Mode proof from the intended project.
8. Synchronize only repository-owned changes back to the bound worktree and
   run focused contract, diff, and review proof.

## Proof Boundary

Success requires direct evidence from the intended FastWork project and
flagship scene. Static source checks alone do not prove visual quality, Play
Mode behavior, runtime connectivity, or investor readiness. No player binary
is built.

## Tooling Anomalies

Every repeatable Unity-MCP, editor, runtime, asset, or proof-tool anomaly is
recorded in #5662 with reproduction evidence or routed to a named follow-up
owner. Retrying around an anomaly is not resolution.
