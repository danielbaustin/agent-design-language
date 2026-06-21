# Unity Observatory Implementation Baseline for #4030

## Scope

Define the concrete Unity Observatory implementation baseline for WP-09 without
overclaiming launch, ingestion, inhabitant-readiness, or closeout completion.

## Source evidence

- issue `#3974` WP-09 umbrella
- issue `#4030` source prompt and cards
- `docs/milestones/v0.91.6/review/planning/TBD_ACTIVE_DOC_ROUTING_4234.md`
- `demos/v0.91.4/celestial-rescue-unity/README.md`
- `demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json`
- `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`
- `adl/src/csm_observatory.rs`
- `docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md`

## Decision

WP-09 will treat `demos/v0.91.6/unity-observatory` as the single repository
root for the richer Unity Observatory lane.

This baseline names:

- the future Unity project root
- the intended primary scene and UI shell paths
- the initial bounded packet contract entrypoint
- the validation surfaces downstream issues must preserve
- the explicit split between Unity, HTML/mobile, ingestion, and closeout work

## Why this baseline

1. The repository already has a bounded Observatory packet contract under
   `adl.csm_visibility_packet.v1`, plus deterministic fixtures and CLI proof.
2. The repository already has one Unity project scaffold in
   `demos/v0.91.4/celestial-rescue-unity`, which is useful as setup precedent
   but is not the Observatory product surface.
3. The WP-09 child issues need one stable target so `#4031` through `#4035`
   stop re-deciding file locations, ingress shape, and non-claim boundaries.
4. This baseline should stay narrow enough that adjacent HTML/mobile work can be
   routed separately without being mistaken for Unity completion.

## Baseline contract

### Unity project target

- Project root: `demos/v0.91.6/unity-observatory`
- Editor family: Unity `2022.3 LTS`
- Primary scene target: `Assets/Scenes/UnityObservatory.unity`
- Bootstrap target: `Assets/Scripts/UnityObservatoryBootstrap.cs`
- UI shell target:
  - `Assets/UI/ObservatoryShell.uxml`
  - `Assets/UI/ObservatoryShell.uss`

### ADL evidence entrypoint

The first bounded ingress contract for the Unity lane is the governed
fixture-backed Observatory packet:

- `demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json`

The packet must remain within the validated `adl.csm_visibility_packet.v1`
family, with `adl/src/csm_observatory.rs` treated as the current shape and
guardrail authority.

O-02 may deepen the Unity-facing loader, normalization, or fixture set, but it
must not silently replace the contract family with an unreviewed ad hoc format.

### Safety boundary

The baseline permits only bounded, read-only Observatory evidence consumption.

It does not permit:

- direct runtime mutation authority
- raw private-state or profile display
- hidden machine-local paths or credential requirements
- treating a baseline scene as final closeout proof

## Downstream execution map

| Issue | Role after this baseline |
| --- | --- |
| `#4031` | create the launchable Unity project at the named root and prove it launches or has an accepted deterministic equivalent |
| `#4032` | wire the Unity lane to the bounded ADL evidence/data contract |
| `#4033` | build inhabitant-facing surfaces using the same root, scene, and contract family |
| `#4034` | prove logging/OTel/security consumption without overclaiming live observability |
| `#4035` | record final working Unity Observatory closeout truth |
## Validation for #4030

This issue proves the baseline by making the target root and contract explicit,
not by claiming that Unity already launches.

Focused proof surfaces:

```bash
test -f demos/v0.91.6/unity-observatory/README.md
test -f demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json
bash adl/tools/test_v0916_unity_observatory_baseline.sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture
```

Optional companion contract proof:

```bash
cargo test --manifest-path adl/Cargo.toml csm_observatory_cli_writes_fixture_backed_bundle -- --nocapture
```

## Non-claims

This packet does not prove:

- a launchable Unity Observatory
- completed inhabitant-facing surfaces
- completed ADL ingestion into Unity
- completed logging/OTel/security consumption closure
- HTML/mobile Observatory completion
- WP-09 umbrella closeout readiness

## Reviewer takeaway

`#4030` is successful when reviewers can confirm that WP-09 now has one
concrete Unity Observatory target with explicit repo boundaries, ingress
contract, and downstream ownership, while all remaining implementation proof
surfaces stay honestly open.
