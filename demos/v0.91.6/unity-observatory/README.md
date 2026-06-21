# Unity Observatory Baseline

## Status

Baseline contract for WP-09 issue `#4030`.

This directory names the intended Unity Observatory project root for
`v0.91.6`. It does not yet claim a launchable Unity scene, a completed
inhabitant-facing surface, or closed Observatory ingestion/security proof.
Those remain owned by `#4031`, `#4032`, `#4033`, `#4034`, and `#4035`.

## Purpose

Provide one concrete repository location and implementation boundary for the
Unity Observatory so downstream issues stop reconstructing the baseline from
planning prose.

## Project Root

- Unity project root: `demos/v0.91.6/unity-observatory`
- Intended editor family: Unity `2022.3 LTS`
- Primary scene path: `Assets/Scenes/UnityObservatory.unity`
- Primary UI surface: `Assets/UI/ObservatoryShell.uxml`
- Primary style sheet: `Assets/UI/ObservatoryShell.uss`
- Primary bootstrap script: `Assets/Scripts/UnityObservatoryBootstrap.cs`

The concrete Unity project files above are implementation targets for `#4031`.
This baseline issue intentionally names them before they exist so the launchable
baseline, ingestion contract, and inhabitant surfaces share one stable target.

## Repository Boundary

The Unity Observatory consumes bounded ADL evidence. It is not the authority
for raw runtime mutation, identity rebinding, or private-state inspection.

### Read-only evidence inputs

Initial bounded input for the first governed baseline:

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

This baseline intentionally leaves the following issue boundaries explicit:

- `#4031` owns the first launchable Unity project or deterministic equivalent
- `#4032` owns the ADL evidence/data contract and fixture-loading path
- `#4033` owns inhabitant-facing world, status, identity, and capability
  surfaces
- `#4034` owns logging/OTel/security consumption proof
- `#4035` owns final working Unity Observatory closeout truth
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

These commands prove the current contract family and guardrails. They do not
prove that the Unity project itself launches; `#4031` owns that proof.
