# Unity Observatory

## Status

Launchable-baseline scaffold for WP-09 issue `#4031`.

This directory now contains the first Unity Observatory project scaffold for
`v0.91.6`. It is intended to open as a Unity `6.5` project on the current
local baseline `6000.5.1f1` and provide one launchable-equivalent shell for
the richer Observatory lane.

This issue now adds one bounded inhabitant-readiness projection for `#4033`
through the same checked-in Unity-facing contract seed. The shell now presents:

- inhabited world and lens framing
- runtime/status posture
- a reviewed inhabitant-readiness checklist
- redacted inhabitant-lane capability projections with explicit identity limits

This bounded scaffold still does not claim:

- live ADL evidence ingestion into Unity
- identity-safe inhabitant/profile closure
- production Observatory readiness

Those remain owned by downstream runtime/demo integration work. WP-09 closeout
truth itself is now complete through `#3974`.

Update for `#4032`: the scaffold now includes one deterministic Unity-facing
contract seed at `Assets/Resources/observatory_contract.json`, derived from the
same governed Observatory packet family and loaded through Unity `Resources`
instead of private machine-local paths. The same contract is emitted by the ADL
Observatory bundle as `unity_observatory_contract.json`.

## Purpose

Provide one concrete Unity project scaffold that downstream WP-09 issues can use
instead of reconstructing the launch surface from planning prose alone.

## Project Surface

- Unity project root: `demos/v0.91.6/unity-observatory`
- Intended editor family: Unity `6.5`
- Local proof baseline: Unity `6000.5.1f1`
- Primary scene path: `Assets/Scenes/UnityObservatory.unity`
- Primary bootstrap script: `Assets/Scripts/UnityObservatoryBootstrap.cs`
- Runtime controller: `Assets/Scripts/UnityObservatoryShellController.cs`
- Batch validation script: `Assets/Editor/UnityObservatoryBatchValidator.cs`
- Reference UI asset: `Assets/UI/ObservatoryShell.uxml`
- Reference style asset: `Assets/UI/ObservatoryShell.uss`
- Unity contract seed: `Assets/Resources/observatory_contract.json`
- Proof packet: `PROOF_PACKET.md`

The current scene seed contains a single `UnityObservatoryBootstrap` object. At
Play time the bootstrap creates the main camera when needed, creates explicit
runtime UI Toolkit panel settings, loads the seeded Unity-facing contract from
`Resources/observatory_contract.json`, configures one runtime Observatory shell,
and renders a governed control-panel surface from that read-only contract while
leaving richer inhabitant-specific expansion to later issues. If the contract
resource is missing, empty, or malformed, the shell falls back to bounded
deterministic defaults rather than mutating runtime state.

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

### Current contract-backed wiring

The current scaffold now consumes a bounded Unity-facing contract with:

- governed packet reference path
- packet schema family
- runtime artifact root
- citizen and episode counts
- default room and lens labels
- proposal-mode boundary copy
- Freedom Gate summary counts
- review/operator-report references

The contract seed is read-only and fixture-backed. `#4033` now expands it with
bounded world, status, checklist, and redacted inhabitant-lane surfaces while
keeping identity/profile safety explicitly routed. Full identity-safe display
and live runtime consumption still remain follow-on proof work.

Update for `#4034`: the same contract now makes Observatory logging, OTel, and
security-consumption posture explicit through repository-relative proof refs for:

- the bounded OTel and event-stream floor from `#3999`
- the logging-validation and redaction floor from `#4000`
- the consumed WP-07 security review from `#4023`
- the issue-owned reviewer packet
  `docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md`

The current checked-in contract and Unity shell now render this as a governed
observability/security card when that contract section is actually present, so
reviewers can inspect the non-claim boundary without opening raw runtime logs.

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

This Unity surface intentionally leaves the following issue boundaries explicit:

- `#4032` owns the bounded ADL evidence/data contract and fixture-loading path
- `#4033` owns the bounded inhabitant-facing world, status, and redacted
  capability surfaces
- `#4035` owns the retained working Unity Observatory child closeout proof
- `#3974` owns the now-closed WP-09 umbrella closeout truth

## Open In Unity

1. Open Unity Hub.
2. Add project from disk: `demos/v0.91.6/unity-observatory`.
3. Use Unity `6.5` with the local proof baseline `6000.5.1f1` or a compatible
   Unity 6 editor.
4. Open `Assets/Scenes/UnityObservatory.unity`.
5. Press Play.

Current contract-backed behavior:

- the bootstrap creates a calm document-panel Observatory shell
- the bootstrap creates explicit runtime UI Toolkit panel settings for the shell
- the shell loads a deterministic Unity-facing contract from `Resources`
- missing or malformed contract data falls back to deterministic bounded state
- the shell shows governed packet/schema references, artifact root, and summary
  counts sourced from that contract
- the shell presents room/lens navigation labels, proposal-mode language, and
  Freedom Gate summary counts from the same contract
- the shell presents inhabitant-readiness checklist items and redacted
  inhabitant-lane capability projections from the same contract
- the shell presents observability/security consumption status, proof refs, and
  private-state posture from the same contract
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

Focused contract-seed guardrail for this issue:

```bash
bash adl/tools/test_v0916_unity_observatory_contract.sh
```

Focused Unity 6.5 working-scene smoke proof for this issue:

```bash
bash adl/tools/test_v0916_unity_observatory_unity65_smoke.sh
```

Focused O-04 review packet inspection:

```bash
test -f docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md
cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource -- --nocapture
```

Focused Unity compatibility-canvas proof for `#4524`:

```bash
/Applications/Unity/Hub/Editor/2022.3.62f3/Unity.app/Contents/MacOS/Unity -batchmode -nographics -quit -projectPath demos/v0.91.6/unity-observatory -executeMethod ADL.Demos.UnityObservatory.Editor.UnityObservatoryCompatibilityVerifier.Run -logFile /tmp/unity_observatory_compatibility.log
```

If the project is already open in the Unity editor, run the same proof through
the editor menu instead of batch mode:

- `ADL -> Observatory -> Verify Compatibility Canvas`

## Validation Truth

Repository structure validation: passed by focused file/content checks during
issue execution.

Unity editor validation now has two bounded proving lanes:

- Unity `2022.3.62f3` compatibility fallback proof from `#4524`, driven by
  `Assets/Editor/UnityObservatoryCompatibilityVerifier.cs` through
  `ADL -> Observatory -> Verify Compatibility Canvas`. The observed proof
  asserted `shouldUseCompatibilityCanvas=True`, a non-empty compatibility
  payload, and `sortingOrder=10`.
- Unity `6000.5.1f1` working-scene migration proof from `#4529`, driven by
  `bash adl/tools/test_v0916_unity_observatory_unity65_smoke.sh`. That proof
  compiles the migrated project under Unity `6.5`, opens
  `Assets/Scenes/UnityObservatory.unity`, loads
  `Assets/Resources/observatory_contract.json`, and executes the checked-in
  editor validator at `Assets/Editor/UnityObservatoryBatchValidator.cs`, which
  now drives the runtime bootstrap path and confirms the theme/style-backed
  Observatory shell builds the expected title, packet-contract, and
  observability cards.

Unity build validation: not run.

C# compiler validation outside Unity: not run.

This means the checked-in Unity Observatory demo now retains a proved
Unity `2022.3.x` compatibility fallback while also carrying a focused
Unity `6000.5.1f1` working-scene migration proof. It still does not claim a
player build or broader production readiness.

## Non-Claims

- No live Runtime v2 capture is claimed.
- No live Runtime v2 ingestion is claimed.
- No live OpenTelemetry collector or exporter integration is claimed.
- No inhabitant-safe profile or memory display closure is claimed.
- No standalone Unity player build success is claimed.
- No Unity build success is claimed.
- No production Observatory readiness is claimed.
