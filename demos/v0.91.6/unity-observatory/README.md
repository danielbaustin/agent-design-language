# Unity Observatory

## Status

Launchable-baseline scaffold for WP-09 issue `#4031`.

This directory now contains the first Unity Observatory project scaffold for
`v0.91.6`. It is intended to open as a Unity `2022.3 LTS` project and provide
one launchable-equivalent shell for the richer Observatory lane.

This issue still does not claim:

- completed ADL evidence ingestion into Unity
- completed inhabitant-facing readiness
- completed logging/OTel/security consumption proof
- final WP-09 closeout truth

Those remain owned by `#4032`, `#4033`, `#4034`, and `#4035`.

## Purpose

Provide one concrete Unity project scaffold that downstream WP-09 issues can use
instead of reconstructing the launch surface from planning prose alone.

## Project Surface

- Unity project root: `demos/v0.91.6/unity-observatory`
- Intended editor family: Unity `2022.3 LTS`
- Primary scene path: `Assets/Scenes/UnityObservatory.unity`
- Primary bootstrap script: `Assets/Scripts/UnityObservatoryBootstrap.cs`
- Runtime controller: `Assets/Scripts/UnityObservatoryShellController.cs`
- Reference UI asset: `Assets/UI/ObservatoryShell.uxml`
- Reference style asset: `Assets/UI/ObservatoryShell.uss`
- Proof packet: `PROOF_PACKET.md`

The current scene seed contains a single `UnityObservatoryBootstrap` object. At
Play time the bootstrap creates the main camera when needed, configures one
runtime Observatory shell, and renders a governed control-panel surface from
static launch-baseline values while leaving packet ingestion to `#4032`.

The UXML and USS files are tracked as reference assets for the same governed
shell structure. This issue does not claim that the runtime path already loads
those assets directly.

## Repository Boundary

The Unity Observatory launch baseline consumes bounded Observatory contract
inputs. It is not the authority for raw runtime mutation, identity rebinding,
or private-state inspection.

### Read-only evidence inputs

Initial bounded input for the launch baseline:

- `demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json`

Canonical packet schema consumed by the Unity lane:

- `adl.csm_visibility_packet.v1`
- validator and renderer authority:
  - `adl/src/csm_observatory.rs`
  - `adl/tools/render_csm_observatory_report.py`
  - `adl/tools/validate_csm_governed_observatory.py`

Runtime-backed golden fixture family that later issues may consume without
introducing hidden local state:

- `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`

### Current launch-baseline wiring

The current scaffold intentionally binds only static launch-baseline values:

- governed packet reference path
- packet schema family
- baseline citizen and episode counts
- default room and lens labels
- review/non-claim boundary copy

Actual packet parsing and richer surface composition remain issue-owned follow-on
work for `#4032` and `#4033`.

### Non-authoritative boundary

The Unity surface may present:

- runtime status
- citizen and episode summaries
- governed operator actions that remain read-only by default
- review/proof classification
- bounded logging and trace evidence once issue-owned proof lands

The Unity surface may not claim:

- live Runtime v2 capture by default
- private memory or profile inspection
- direct mutation authority
- closed WP-09 readiness from baseline definition alone

## Downstream Ownership

This launch baseline intentionally leaves the following issue boundaries explicit:

- `#4032` owns the ADL evidence/data contract and fixture-loading path
- `#4033` owns inhabitant-facing world, status, identity, and capability
  surfaces
- `#4034` owns logging/OTel/security consumption proof
- `#4035` owns final working Unity Observatory closeout truth

## Open In Unity

1. Open Unity Hub.
2. Add project from disk: `demos/v0.91.6/unity-observatory`.
3. Use Unity `2022.3 LTS` or a compatible editor.
4. Open `Assets/Scenes/UnityObservatory.unity`.
5. Press Play.

Current launch-baseline behavior:

- the bootstrap creates a calm document-panel Observatory shell
- the shell shows governed packet/schema references and baseline summary counts
- the shell presents room/lens navigation labels and non-authoritative proposal
  language
- no live runtime mutation, snapshot, or profile inspection is performed

## Validation Entry Points

Focused proof that already exists for the bounded Observatory packet contract:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture
```

Focused CLI proof for bundle/report generation from the same contract family:

```bash
cargo test --manifest-path adl/Cargo.toml csm_observatory_cli_writes_fixture_backed_bundle -- --nocapture
```

Focused governed-prototype guardrail check:

```bash
bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh
```

Focused baseline-coordinate guardrail for this issue:

```bash
bash adl/tools/test_v0916_unity_observatory_baseline.sh
```

## Validation Truth

Repository structure validation: passed by focused file/content checks during
issue execution.

Unity editor validation: not run.

Unity build validation: not run.

C# compiler validation outside Unity: not run.

This means `#4031` records a deterministic launchable-equivalent scaffold with
setup/run instructions and tracked proof surfaces, not a false claim that the
Unity editor or build pipeline already succeeded on this machine.

## Non-Claims

- No live Runtime v2 capture is claimed.
- No completed ADL evidence ingestion is claimed.
- No inhabitant-safe display or input closure is claimed.
- No Unity editor success is claimed.
- No Unity build success is claimed.
- No WP-09 closeout readiness is claimed.
